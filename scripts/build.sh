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

# Detach any existing volumes to prevent "Resource busy" errors
if hdiutil info | grep "/Volumes/$APP_NAME" >/dev/null; then
	echo "Detaching existing volumes..."
	hdiutil info | grep "/Volumes/$APP_NAME" | awk '{print $1}' | while read -r dev; do
		echo "Detaching $dev..."
		hdiutil detach "$dev" -force || true
	done
fi

rm -rf "$DMG_PATH" "$STAGING_DIR"
mkdir -p "$STAGING_DIR"

cp -R "$APP_BUNDLE_OUT" "$STAGING_DIR/"
ln -s /Applications "$STAGING_DIR/Applications"

hdiutil create -volname "$APP_NAME" \
	-srcfolder "$STAGING_DIR" \
	-ov -format UDZO \
	"$DMG_PATH"

rm -rf "$STAGING_DIR"

echo "DMG created: $DMG_PATH"

echo "Done: $APP_BUNDLE_OUT"
if [[ "$OPEN_APP" == "1" ]]; then
	echo "Opening app..."
	open "$APP_BUNDLE_OUT"
fi
