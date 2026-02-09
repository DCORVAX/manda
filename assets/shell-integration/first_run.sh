#!/bin/bash
# Kaku First Run Experience
# This script is launched automatically on the first run of Kaku.

set -euo pipefail

# Resources directory resolution
if [[ -d "../../../Contents/Resources" ]]; then
	RESOURCES_DIR="$(cd ../../../Contents/Resources && pwd)"
elif [[ -d "/Applications/Kaku.app/Contents/Resources" ]]; then
	RESOURCES_DIR="/Applications/Kaku.app/Contents/Resources"
else
	# Fallback for dev environment
	DIR="$(dirname "$0")"
	RESOURCES_DIR="$DIR"
fi

SETUP_SCRIPT="$RESOURCES_DIR/setup_zsh.sh"

# Clear screen
clear

# Display Welcome Message
echo -e "\033[1;35m"
echo "  _  __      _          "
echo " | |/ /     | |         "
echo " | ' / __ _ | | __ _   _ "
echo " |  < / _\` || |/ /| | | |"
echo " | . \ (_| ||   < | |_| |"
echo " |_|\_\__,_||_|\_\ \__,_|"
echo -e "\033[0m"
echo "Welcome to Kaku!"
echo "The terminal built for the AI coding era."
echo "--------------------------------------------------------"
echo "Would you like to install Kaku's enhanced shell features?"
echo "This includes:"
echo "  - Starship Prompt"
echo "  - z - Smart Directory Jumper"
echo "  - Zsh Syntax Highlighting"
echo "  - Zsh Autosuggestions"
echo "--------------------------------------------------------"
echo ""

# Interactive Prompt
read -p "Install enhanced shell features? [Y/n] " -n 1 -r
echo ""

INSTALL_SHELL=false
if [[ $REPLY =~ ^[Yy]$ ]] || [[ -z $REPLY ]]; then
	INSTALL_SHELL=true
fi

# Check if OpenCode is installed
HAS_OPENCODE=false
if command -v opencode >/dev/null 2>&1 || [[ -d "$HOME/.config/opencode" ]] || [[ -d "$HOME/.opencode" ]]; then
	HAS_OPENCODE=true
fi

# OpenCode Theme Prompt (Only if OpenCode is detected)
INSTALL_THEME=false
if [[ "$HAS_OPENCODE" == "true" ]]; then
	echo "--------------------------------------------------------"
	echo "OpenCode detected! Would you like to use the OpenCode Theme?"
	echo "A modern, high-contrast dark theme optimized for AI coding."
	echo "--------------------------------------------------------"
	read -p "Apply OpenCode Theme? [y/N] " -n 1 -r
	echo ""

	if [[ $REPLY =~ ^[Yy]$ ]]; then
		INSTALL_THEME=true
	fi
fi

# Process Shell Features
if [[ "$INSTALL_SHELL" == "true" ]]; then
	if [[ -f "$SETUP_SCRIPT" ]]; then
		"$SETUP_SCRIPT"
	else
		echo "Error: setup_zsh.sh not found at $SETUP_SCRIPT"
	fi
else
	echo ""
	echo "Skipping shell setup. You can run it manually later:"
	echo "$SETUP_SCRIPT"
fi

mkdir -p "$HOME/.config/kaku"

# Process OpenCode Theme
if [[ "$INSTALL_THEME" == "true" ]]; then
	KAKU_LUA_SRC="$RESOURCES_DIR/kaku.lua"
	KAKU_LUA_DEST="$HOME/.config/kaku/kaku.lua"

	if [[ -f "$KAKU_LUA_SRC" ]]; then
		echo "Installing OpenCode theme..."
		cp "$KAKU_LUA_SRC" "$KAKU_LUA_DEST"

		# Inject OpenCode theme before the return statement
		# We use a temporary file to construct the new content
		TMP_FILE=$(mktemp)

		# Read all lines except the last one (return config)
		sed '$d' "$KAKU_LUA_DEST" >"$TMP_FILE"

		# Append OpenCode theme config
		cat <<EOF >>"$TMP_FILE"

-- ===== OpenCode Theme =====
config.colors = {
  foreground = '#d4d4d4',
  background = '#1e1e1e',
  cursor_bg = '#569cd6',
  cursor_fg = '#1e1e1e',
  cursor_border = '#569cd6',
  selection_bg = '#264f78',
  selection_fg = '#d4d4d4',
  ansi = {'#000000', '#cd3131', '#0dbc79', '#e5e510', '#2472c8', '#bc3fbc', '#11a8cd', '#e5e5e5'},
  brights = {'#666666', '#f14c4c', '#23d18b', '#f5f543', '#3b8eea', '#d670d6', '#29b8db', '#e5e5e5'},
  tab_bar = {
    background = '#1e1e1e',
    active_tab = {
      bg_color = '#1e1e1e',
      fg_color = '#569cd6',
      intensity = 'Bold',
    },
    inactive_tab = {
      bg_color = '#2d2d2d',
      fg_color = '#858585',
    },
    inactive_tab_hover = {
      bg_color = '#2d2d2d',
      fg_color = '#d4d4d4',
    },
    new_tab = {
      bg_color = '#1e1e1e',
      fg_color = '#858585',
    },
    new_tab_hover = {
      bg_color = '#2d2d2d',
      fg_color = '#d4d4d4',
    },
  },
}
config.window_frame = {
  active_titlebar_bg = '#1e1e1e',
  inactive_titlebar_bg = '#1e1e1e',
  button_bg = '#1e1e1e',
  button_fg = '#cccccc',
}

return config
EOF
		mv "$TMP_FILE" "$KAKU_LUA_DEST"
		echo "OpenCode theme applied!"
	else
		echo "Warning: Could not find kaku.lua source at $KAKU_LUA_SRC"
	fi
fi

touch "$HOME/.config/kaku/.kaku_setup_v1_completed"

echo -e "\n\033[1;32m❤️  Kaku environment is ready! Enjoy coding.\033[0m"

# Replace current process with zsh to enter the shell
exec /bin/zsh -l
