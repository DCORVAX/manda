#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

echo "z_jump_provider: starting (zsh=$(command -v zsh 2>/dev/null || echo MISSING), bash=$BASH_VERSION)" >&2

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/kaku-z-jump-provider.XXXXXX")"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

HOME="$tmp_dir/home"
ZDOTDIR="$HOME"
mkdir -p "$HOME"

vendor_dir="$tmp_dir/vendor"
mkdir -p "$vendor_dir/zsh-z" "$vendor_dir/zsh-autosuggestions" \
         "$vendor_dir/zsh-syntax-highlighting" "$vendor_dir/zsh-completions"

cat >"$vendor_dir/zsh-z/zsh-z.plugin.zsh" <<'EOF'
typeset -g KAKU_TEST_ZSH_Z_SOURCED=1
zshz() { :; }
EOF

echo "z_jump_provider: running setup_zsh.sh" >&2
setup_out=""
setup_status=0
setup_out="$(
  HOME="$HOME" \
  ZDOTDIR="$ZDOTDIR" \
  KAKU_INIT_INTERNAL=1 \
  KAKU_SKIP_TOOL_BOOTSTRAP=1 \
  KAKU_SKIP_TERMINFO_BOOTSTRAP=1 \
  KAKU_VENDOR_DIR="$vendor_dir" \
  bash "$REPO_ROOT/assets/shell-integration/setup_zsh.sh" --update-only 2>&1
)" || setup_status=$?
if [[ "$setup_status" -ne 0 ]]; then
  echo "z_jump_provider: setup_zsh.sh failed (exit $setup_status):" >&2
  echo "$setup_out" >&2
  exit 1
fi

kaku_zsh="$HOME/.config/kaku/zsh/kaku.zsh"
if [[ ! -f "$kaku_zsh" ]]; then
  echo "z_jump_provider: kaku.zsh not created at $kaku_zsh" >&2
  exit 1
fi

with_existing_provider=""
if ! with_existing_provider="$(
  TERM=xterm-256color \
  HOME="$HOME" \
  ZDOTDIR="$ZDOTDIR" \
  zsh -f -c '
__zoxide_z() { :; }
alias z=__zoxide_z
source "$HOME/.config/kaku/zsh/kaku.zsh"
print -r -- "__KAKU_EXISTING__:${KAKU_TEST_ZSH_Z_SOURCED:-0}"
' 2>&1
)"; then
  echo "z_jump_provider: zsh with existing provider exited non-zero:" >&2
  echo "$with_existing_provider" >&2
  exit 1
fi

case "$with_existing_provider" in
  *__KAKU_EXISTING__:0* ) ;;
  * )
    echo "z_jump_provider: bundled zsh-z loaded despite existing z provider:" >&2
    echo "$with_existing_provider" >&2
    exit 1
    ;;
esac

without_existing_provider=""
if ! without_existing_provider="$(
  TERM=xterm-256color \
  HOME="$HOME" \
  ZDOTDIR="$ZDOTDIR" \
  zsh -f -c '
source "$HOME/.config/kaku/zsh/kaku.zsh"
print -r -- "__KAKU_FALLBACK__:${KAKU_TEST_ZSH_Z_SOURCED:-0}"
' 2>&1
)"; then
  echo "z_jump_provider: zsh without existing provider exited non-zero:" >&2
  echo "$without_existing_provider" >&2
  exit 1
fi

case "$without_existing_provider" in
  *__KAKU_FALLBACK__:1* ) ;;
  * )
    echo "z_jump_provider: bundled zsh-z did not load as fallback:" >&2
    echo "$without_existing_provider" >&2
    exit 1
    ;;
esac

echo "z_jump_provider smoke test passed"
