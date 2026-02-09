#!/usr/bin/env bash
set -euo pipefail

if [[ "${OSTYPE:-}" != darwin* ]]; then
	echo "This script is macOS-only." >&2
	exit 1
fi

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

APP_NAME="Kaku"
TARGET_DIR="${TARGET_DIR:-target}"
PROFILE="${PROFILE:-release}"
OUT_DIR="${OUT_DIR:-dist}"
OPEN_APP="${OPEN_APP:-0}"

if [[ "${1:-}" == "--open" ]]; then
	OPEN_APP=1
fi

APP_BUNDLE_SRC="assets/macos/Kaku.app"
APP_BUNDLE_OUT="$OUT_DIR/$APP_NAME.app"

echo "[1/6] Building binaries ($PROFILE)..."
if [[ "$PROFILE" == "release" ]]; then
	cargo build --release -p kaku-gui -p kaku
	BIN_DIR="$TARGET_DIR/release"
elif [[ "$PROFILE" == "release-opt" ]]; then
	cargo build --profile release-opt -p kaku-gui -p kaku
	BIN_DIR="$TARGET_DIR/release-opt"
else
	cargo build -p kaku-gui -p kaku
	BIN_DIR="$TARGET_DIR/debug"
fi

echo "[2/6] Preparing app bundle..."
rm -rf "$APP_BUNDLE_OUT"
mkdir -p "$OUT_DIR"
cp -R "$APP_BUNDLE_SRC" "$APP_BUNDLE_OUT"

# Move libraries from root to Frameworks (macOS requirement)
if ls "$APP_BUNDLE_OUT"/*.dylib 1>/dev/null 2>&1; then
	mkdir -p "$APP_BUNDLE_OUT/Contents/Frameworks"
	mv "$APP_BUNDLE_OUT"/*.dylib "$APP_BUNDLE_OUT/Contents/Frameworks/"
fi

mkdir -p "$APP_BUNDLE_OUT/Contents/MacOS"
mkdir -p "$APP_BUNDLE_OUT/Contents/Resources"

echo "[2.5/6] Syncing version from Cargo.toml..."
# Extract version from kaku/Cargo.toml (assuming it's the source of truth)
VERSION=$(grep '^version =' kaku/Cargo.toml | head -n 1 | cut -d '"' -f2)
if [[ -n "$VERSION" ]]; then
	echo "Stamping version $VERSION into Info.plist"
	/usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $VERSION" "$APP_BUNDLE_OUT/Contents/Info.plist"
	/usr/libexec/PlistBuddy -c "Set :CFBundleVersion $VERSION" "$APP_BUNDLE_OUT/Contents/Info.plist"
else
	echo "Warning: Could not detect version from kaku/Cargo.toml"
fi

echo "[3/6] Downloading vendor dependencies..."
./scripts/download_vendor.sh

echo "[4/6] Copying resources and binaries..."
cp -R assets/shell-integration/* "$APP_BUNDLE_OUT/Contents/Resources/"
cp -R assets/shell-completion "$APP_BUNDLE_OUT/Contents/Resources/"
cp -R assets/fonts "$APP_BUNDLE_OUT/Contents/Resources/"
mkdir -p "$APP_BUNDLE_OUT/Contents/Resources/vendor"
cp -R assets/vendor/* "$APP_BUNDLE_OUT/Contents/Resources/vendor/"
cp assets/shell-integration/first_run.sh "$APP_BUNDLE_OUT/Contents/Resources/"
chmod +x "$APP_BUNDLE_OUT/Contents/Resources/first_run.sh"

# Explicitly use the logo.icns from assets if available
if [[ -f "assets/logo.icns" ]]; then
	cp "assets/logo.icns" "$APP_BUNDLE_OUT/Contents/Resources/terminal.icns"
fi

tic -xe kaku -o "$APP_BUNDLE_OUT/Contents/Resources/terminfo" termwiz/data/kaku.terminfo

for bin in kaku kaku-gui; do
	cp "$BIN_DIR/$bin" "$APP_BUNDLE_OUT/Contents/MacOS/$bin"
	chmod +x "$APP_BUNDLE_OUT/Contents/MacOS/$bin"
done

# Clean up xattrs to prevent icon caching issues or quarantine
xattr -cr "$APP_BUNDLE_OUT"

echo "[5/6] Signing app bundle..."
codesign --force --deep --sign - "$APP_BUNDLE_OUT"

touch "$APP_BUNDLE_OUT/Contents/Resources/terminal.icns"
touch "$APP_BUNDLE_OUT/Contents/Info.plist"
touch "$APP_BUNDLE_OUT"

echo "[6/6] Creating DMG..."
DMG_NAME="$APP_NAME.dmg"
DMG_PATH="$OUT_DIR/$DMG_NAME"
STAGING_DIR="$OUT_DIR/dmg_staging"

cleanup_volumes() {
	local vol_pattern="/Volumes/$APP_NAME"
	local max_attempts=15
	local attempt=1

	while [ $attempt -le $max_attempts ]; do
		if hdiutil info | grep -q "$vol_pattern"; then
			echo "Detaching existing volumes (Attempt $attempt/$max_attempts)..."
			hdiutil info | grep "$vol_pattern" | awk '{print $1}' | while read -r dev; do
				echo "Force detaching $dev..."
				hdiutil detach "$dev" -force || true
			done
			sleep 1
		else
			if [ -d "$vol_pattern" ]; then
				echo "Removing stale mount point directory $vol_pattern..."
				rmdir "$vol_pattern" || true
			fi
			return 0
		fi
		attempt=$((attempt + 1))
	done
	echo "Warning: Failed to fully detach volumes after $max_attempts attempts."
}

cleanup_volumes

sync

rm -rf "$DMG_PATH" "$STAGING_DIR"
mkdir -p "$STAGING_DIR"

cp -R "$APP_BUNDLE_OUT" "$STAGING_DIR/"
ln -s /Applications "$STAGING_DIR/Applications"

mdutil -i off "$STAGING_DIR" >/dev/null 2>&1 || true

echo "Creating DMG..."
MAX_RETRIES=3
RETRY_COUNT=0

while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
	if hdiutil create -volname "$APP_NAME" \
		-srcfolder "$STAGING_DIR" \
		-ov -format UDZO \
		"$DMG_PATH"; then
		break
	else
		echo "hdiutil create failed. Retrying in 2 seconds... ($((RETRY_COUNT + 1))/$MAX_RETRIES)"
		cleanup_volumes
		sleep 2
		RETRY_COUNT=$((RETRY_COUNT + 1))
	fi
done

if [ ! -f "$DMG_PATH" ]; then
	echo "Error: Failed to create DMG after retries."
	exit 1
fi

rm -rf "$STAGING_DIR"

echo "DMG created: $DMG_PATH"

echo "Done: $APP_BUNDLE_OUT"
if [[ "$OPEN_APP" == "1" ]]; then
	echo "Opening app..."
	open "$APP_BUNDLE_OUT"
fi
