#!/bin/bash
# Reset Kaku First Run Experience
# This script is for testing purposes. It removes the completion flag
# so that Kaku will trigger the first run setup again.

set -e

CONFIG_DIR="$HOME/.config/kaku"
FLAG_FILE="$CONFIG_DIR/.kaku_setup_v1_completed"

echo "Resetting Kaku First Run..."

if [[ -f "$FLAG_FILE" ]]; then
	rm "$FLAG_FILE"
	echo "✅ Removed flag file: $FLAG_FILE"
else
	echo "ℹ️  Flag file not found: $FLAG_FILE"
fi

echo "Now relaunch Kaku to see the First Run experience."
