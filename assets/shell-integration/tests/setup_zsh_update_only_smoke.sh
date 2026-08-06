#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SHELL_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
REPO_ROOT="$(cd "$SHELL_DIR/../.." && pwd)"

fail() {
  echo "FAIL: $*" >&2
  exit 1
}

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/manda-setup-zsh-smoke.XXXXXX")"
trap 'rm -rf "$tmp_dir"' EXIT

tmp_home="$tmp_dir/home"
tmp_vendor="$tmp_dir/vendor"
mkdir -p "$tmp_home" "$tmp_vendor"

for plugin in fast-syntax-highlighting zsh-autosuggestions zsh-completions zsh-z; do
  mkdir -p "$tmp_vendor/$plugin"
done

if [[ -f "$REPO_ROOT/assets/vendor/starship.toml" ]]; then
  cp "$REPO_ROOT/assets/vendor/starship.toml" "$tmp_vendor/starship.toml"
else
  printf '# test starship config\n' >"$tmp_vendor/starship.toml"
fi

output_log="$tmp_dir/output.log"
error_log="$tmp_dir/error.log"

if ! HOME="$tmp_home" \
  MANDA_INIT_INTERNAL=1 \
  MANDA_SKIP_TOOL_BOOTSTRAP=1 \
  MANDA_SKIP_TERMINFO_BOOTSTRAP=1 \
  MANDA_VENDOR_DIR="$tmp_vendor" \
  bash "$SHELL_DIR/setup_zsh.sh" --update-only >"$output_log" 2>"$error_log"; then
  cat "$output_log" >&2
  cat "$error_log" >&2
  fail "setup_zsh.sh --update-only failed"
fi

if grep -Fq "local: can only be used in a function" "$output_log" "$error_log"; then
  cat "$output_log" >&2
  cat "$error_log" >&2
  fail "setup_zsh.sh used local outside a function"
fi

[[ -f "$tmp_home/.config/starship.toml" ]] || fail "starship.toml was not initialized"
[[ -f "$tmp_home/.config/manda/zsh/manda.zsh" ]] || fail "manda.zsh was not generated"
[[ -f "$tmp_home/.zshrc" ]] || fail ".zshrc was not patched"

if ! grep -Fq "fg=249" "$tmp_home/.config/manda/zsh/manda.zsh"; then
  fail "generated manda.zsh did not set readable comment color fg=249"
fi
if grep -Fq "fg=244" "$tmp_home/.config/manda/zsh/manda.zsh"; then
  fail "generated manda.zsh still contains old comment color fg=244"
fi
grep -Fq 'if [[ "${TERM_PROGRAM:-}" == "MANDA" ]] && command -v starship' \
  "$tmp_home/.config/manda/zsh/manda.zsh" \
  || fail "generated manda.zsh did not preserve the runtime MANDA session guard"
grep -Fq 'local capability_file="$HOME/.config/manda/ai_inline_capability"' \
  "$tmp_home/.config/manda/zsh/manda.zsh" \
  || fail "generated manda.zsh did not read the inline AI capability"
grep -Fq '_manda_set_ai_user_var "manda_ai_query" "[mode:${mode}] ${body}"' \
  "$tmp_home/.config/manda/zsh/manda.zsh" \
  || fail "generated manda.zsh did not authenticate inline AI queries"

# The generated file is sourced by the user's real zsh, so it must parse under
# zsh. A corrupted heredoc (e.g. an unescaped backtick that bash expanded at
# generation time, #450) or a stray top-level construct surfaces here even when
# setup_zsh.sh itself exited 0.
if command -v zsh >/dev/null 2>&1; then
  if ! zsh -n "$tmp_home/.config/manda/zsh/manda.zsh" 2>"$tmp_dir/zsh_parse.log"; then
    cat "$tmp_dir/zsh_parse.log" >&2
    fail "generated manda.zsh failed 'zsh -n' parse check"
  fi

  starship_stub_dir="$tmp_dir/starship-bin"
  starship_marker="$tmp_dir/starship-initialized"
  mkdir -p "$starship_stub_dir"
  cp /dev/null "$starship_marker"
  cat >"$starship_stub_dir/starship" <<'EOF'
#!/usr/bin/env bash
if [[ "${1:-}" == "init" ]]; then
  printf 'print -r -- initialized >> "$MANDA_STARSHIP_TEST_MARKER"\n'
fi
EOF
  chmod +x "$starship_stub_dir/starship"

  HOME="$tmp_home" \
    TERM_PROGRAM="Apple_Terminal" \
    MANDA_STARSHIP_TEST_MARKER="$starship_marker" \
    PATH="$starship_stub_dir:$PATH" \
    zsh -dfc 'add-zsh-hook() { :; }; source "$HOME/.config/manda/zsh/manda.zsh"'
  [[ ! -s "$starship_marker" ]] \
    || fail "generated manda.zsh initialized Starship outside MANDA"

  HOME="$tmp_home" \
    TERM_PROGRAM="MANDA" \
    MANDA_STARSHIP_TEST_MARKER="$starship_marker" \
    PATH="$starship_stub_dir:$PATH" \
    zsh -dfc 'add-zsh-hook() { :; }; source "$HOME/.config/manda/zsh/manda.zsh"'
  [[ "$(cat "$starship_marker")" == "initialized" ]] \
    || fail "generated manda.zsh did not initialize Starship inside MANDA"
else
  echo "warning: zsh not found; skipping manda.zsh parse check" >&2
fi

echo "setup_zsh update-only smoke test passed"
