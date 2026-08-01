#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../state_common.sh
source "$SCRIPT_DIR/state_common.sh"

output="$(KAKU_CONFIG_UPDATE_LANGUAGE=en print_config_update_highlights "$SCRIPT_DIR" 12 15)"

[[ "$output" != *"  v12"* ]]
[[ "$output" != *"  v13"* ]]
[[ "$output" != *"  v14"* ]]
[[ "$output" == *"Shell integration compatibility is improved for SSH"* ]]
[[ "$output" == *"Starship prompt and AI shell hooks are more reliable"* ]]
[[ "$output" == *"regenerate the managed script correctly"* ]]
[[ "$output" == *"Yazi now follows Kaku dark and light themes automatically"* ]]

english_output="$(KAKU_CONFIG_UPDATE_LANGUAGE=en print_config_update_highlights "$SCRIPT_DIR" 20 21)"
[[ "$english_output" == *"Tab and pane close confirmation now support Never, Smart, and Always modes"* ]]
[[ "$english_output" == *"Kaku Dark now reports a dark terminal background to Hermes"* ]]
[[ "$english_output" != *"标签页和面板关闭确认"* ]]

chinese_output="$(KAKU_CONFIG_UPDATE_LANGUAGE=zh print_config_update_highlights "$SCRIPT_DIR" 20 21)"
[[ "$chinese_output" == *"标签页和面板关闭确认现在支持"* ]]
[[ "$chinese_output" == *"Kaku Dark 现在会向 Hermes 正确报告深色终端背景"* ]]
[[ "$chinese_output" != *"Tab and pane close confirmation now support"* ]]

state_test_dir="$(mktemp -d)"
trap 'rm -rf "$state_test_dir"' EXIT
CONFIG_DIR="$state_test_dir/config"
STATE_FILE="$CONFIG_DIR/state.json"
LEGACY_VERSION_FILE="$CONFIG_DIR/.kaku_config_version"
LEGACY_GEOMETRY_FILE="$CONFIG_DIR/.kaku_window_geometry"
CURRENT_CONFIG_VERSION=22
mkdir -p "$CONFIG_DIR"
printf '%s\n' '{"config_version":21,"managed_shell":"fish","window_geometry":{"width":120,"height":40},"window_position":{"x":10,"y":20,"screen_id":7},"future_setting":{"enabled":true}}' >"$STATE_FILE"

[[ "$(read_managed_shell)" == "fish" ]]
persist_config_version
[[ "$(read_managed_shell)" == "fish" ]]
grep -Eq '"config_version"[[:space:]]*:[[:space:]]*22' "$STATE_FILE"
grep -q '"width":120' "$STATE_FILE"
grep -q '"screen_id":7' "$STATE_FILE"
grep -q '"future_setting":{"enabled":true}' "$STATE_FILE"

echo "config_update_highlights smoke test passed"
