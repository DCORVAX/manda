#!/usr/bin/env bash
# MANDA — instalador para macOS
# Uso:  curl -fsSL https://raw.githubusercontent.com/WILFREDY-X/manda/main/install/install.sh | bash
#
# Descarga el DMG desde GitHub Releases, monta la imagen y copia
# MANDA.app a /Applications. También configura la integración de shell.

set -euo pipefail

REPO="WILFREDY-X/manda"
RELEASE_TAG="v0.1.0"
DMG_NAME="MANDA.dmg"
APP_NAME="MANDA.app"
INSTALL_DIR="/Applications"
MANDA_CONFIG_DIR="${HOME}/.config/manda"

echo "════════════════════════════════════════════════"
echo "  MANDA — instalador macOS"
echo "════════════════════════════════════════════════"

# ---------------------------------------------------------------------------
# 1) Verificar que no estemos en Linux (por ahora solo macOS)
# ---------------------------------------------------------------------------
if [[ "$(uname)" != "Darwin" ]]; then
  echo "❌ MANDA solo está disponible para macOS por ahora."
  exit 1
fi

# ---------------------------------------------------------------------------
# 2) Verificar si MANDA ya está instalado y offer upgrade
# ---------------------------------------------------------------------------
if [[ -d "${INSTALL_DIR}/${APP_NAME}" ]]; then
  INSTALLED_VERSION=$(/usr/libexec/PlistBuddy -c "Print :CFBundleShortVersionString" "${INSTALL_DIR}/${APP_NAME}/Contents/Info.plist" 2>/dev/null || echo "unknown")
  echo "ℹ  MANDA ya está instalado (versión: ${INSTALLED_VERSION})"
  echo "   Se actualizará a la última versión."
fi

# ---------------------------------------------------------------------------
# 3) Descargar el DMG desde GitHub Releases
# ---------------------------------------------------------------------------
echo "→ Descargando MANDA ${RELEASE_TAG}..."
DMG_URL="https://github.com/${REPO}/releases/download/${RELEASE_TAG}/${DMG_NAME}"
TMP_DIR="$(mktemp -d)"
DMG_PATH="${TMP_DIR}/${DMG_NAME}"

if command -v curl >/dev/null 2>&1; then
  curl -fSL "${DMG_URL}" -o "${DMG_PATH}" \
    || { echo "❌ No se pudo descargar MANDA desde GitHub."; exit 1; }
elif command -v wget >/dev/null 2>&1; then
  wget -qO "${DMG_PATH}" "${DMG_URL}" \
    || { echo "❌ No se pudo descargar MANDA desde GitHub."; exit 1; }
else
  echo "❌ Necesitas curl o wget para instalar MANDA."
  exit 1
fi
echo "✅ DMG descargado ($(du -h "${DMG_PATH}" | cut -f1))"

# ---------------------------------------------------------------------------
# 4) Montar el DMG y copiar la app
# ---------------------------------------------------------------------------
echo "→ Montando imagen de disco..."
MOUNT_DIR="${TMP_DIR}/mount"
mkdir -p "${MOUNT_DIR}"
hdiutil attach "${DMG_PATH}" -mountpoint "${MOUNT_DIR}" -nobrowse -quiet

echo "→ Copiando ${APP_NAME} a ${INSTALL_DIR}..."
# Matar la app si está corriendo para poder reemplazar
if pgrep -x "manda-gui" >/dev/null 2>&1; then
  echo "   Cerrando MANDA en ejecución..."
  pkill -x "manda-gui" 2>/dev/null || true
  sleep 1
fi

rm -rf "${INSTALL_DIR}/${APP_NAME}"
cp -R "${MOUNT_DIR}/${APP_NAME}" "${INSTALL_DIR}/${APP_NAME}"

echo "→ Desmontando imagen..."
hdiutil detach "${MOUNT_DIR}" -quiet

# ---------------------------------------------------------------------------
# 5) Configurar integración de shell
# ---------------------------------------------------------------------------
echo "→ Configurando integración de shell..."
MANDA_BIN="${INSTALL_DIR}/${APP_NAME}/Contents/MacOS/manda"

if [[ -x "${MANDA_BIN}" ]]; then
  # Inicializar la config si no existe
  if [[ ! -d "${MANDA_CONFIG_DIR}" ]]; then
    mkdir -p "${MANDA_CONFIG_DIR}"
  fi

  # Ejecutar init para configurar shell integration
  "${MANDA_BIN}" init --update-only 2>/dev/null || true
  echo "✅ Shell integration configurada."
else
  echo "⚠  No se pudo ejecutar manda init. Hazlo manualmente:"
  echo "   ${MANDA_BIN} init --update_only && exec \${SHELL} -l"
fi

# ---------------------------------------------------------------------------
# 6) Verificar la instalación
# ---------------------------------------------------------------------------
echo ""
echo "→ Verificando instalación..."
if [[ -x "${MANDA_BIN}" ]]; then
  "${MANDA_BIN}" --version 2>/dev/null || echo "MANDA instalado (versión no disponible)"
fi

echo ""
echo "════════════════════════════════════════════════"
echo "  ✅ MANDA instalado en ${INSTALL_DIR}/${APP_NAME}"
echo ""
echo "  Para usarlo:"
echo "    • Abre MANDA desde Applications"
echo "    • O ejecuta: ${MANDA_BIN}"
echo ""
echo "  Configurar IA:"
echo "    ${MANDA_BIN} ai"
echo "════════════════════════════════════════════════"

# Limpiar
rm -rf "${TMP_DIR}"
