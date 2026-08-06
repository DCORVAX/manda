#!/usr/bin/env bash
# scripts/check_logs.sh
#
# Forbid `eprintln!` / `println!` from creeping into production paths where
# `log::warn!` / `log::info!` is the right tool. Exit 0 if clean, 1 with the
# offending lines printed if any new violation appears.
#
# Allowlist rationale:
#   - CLI binaries (manda/src/main.rs, manda/src/cli/*, manda-gui/src/bin/k.rs,
#     manda-gui/src/cli_chat) speak directly to the user on stdout/stderr;
#     log:: would either be filtered out or end up double-printed.
#   - Test functions (#[test], tests/, *_test.rs) use println! as expected
#     by cargo test output.
#   - Startup trace (manda-gui/src/startup_trace.rs) is env-var-gated diagnostic
#     output; routing it through log:: would be suppressed by default level.
#   - Stats dump (manda-gui/src/stats.rs) tabulates to stderr by design.
#
# Anything outside the allowlist is a regression. Add a new path to
# ALLOW_FILES only after explaining why log:: doesn't fit.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

# Files (or path prefixes) explicitly permitted to use eprintln!/println!
# outside of test bodies. Reasons:
#   - CLI binaries / commands speak directly to the user on stdout/stderr.
#   - Startup trace and stats dump are env-var-gated diagnostic output;
#     routing them through log:: would be suppressed by default level.
#   - Test directories (term/src/test/*) print verification context.
#   - The configmeta proc-macro derive prints generated tokens for debugging
#     macro expansion.
ALLOW_FILES=(
  'manda/src/main\.rs'
  'manda/src/cli/'
  'manda/src/config_cmd\.rs'
  'manda/src/doctor\.rs'
  'manda/src/init\.rs'
  'manda/src/reset\.rs'
  'manda/src/update\.rs'
  'manda/src/utils\.rs'
  'manda/src/shell\.rs'
  'manda/src/chat\.rs'
  'manda/src/tui_splash\.rs'
  'manda/src/ai_config/'
  'manda-gui/src/bin/'
  'manda-gui/src/cli_chat/'
  'manda-gui/src/startup_trace\.rs'
  'manda-gui/src/stats\.rs'
  'manda-gui/src/update\.rs'
  'manda-gui/src/shapecache\.rs'
  'config/src/config\.rs'          # MANDA_STARTUP_TRACE env-gated trace
  'config/src/lua\.rs'             # MANDA_STARTUP_TRACE env-gated trace
  'config/derive/'                 # proc-macro derive debug
  'term/src/test/'                 # test infrastructure helpers
)

allow_pattern="$(IFS='|'; echo "${ALLOW_FILES[*]}")"

# Scan production crates only. Skip lines whose `println!` / `eprintln!`
# token is inside a Rust string literal (preceded by `"` on the same line,
# heuristic but sufficient for the call sites that exist today).
violations=$(
  grep -rnE 'eprintln!|println!' --include='*.rs' \
    manda-gui/src manda/src config mux term \
    2>/dev/null \
    | grep -vE "($allow_pattern)" \
    | grep -vE '"[^"]*(eprintln!|println!)' \
    || true
)

if [ -n "$violations" ]; then
  echo "ERROR: eprintln!/println! found in production paths." >&2
  echo "Use log::warn! or log::info! instead, or add the file to the" >&2
  echo "allowlist in scripts/check_logs.sh with a justification." >&2
  echo "" >&2
  echo "$violations" >&2
  exit 1
fi

echo "check_logs.sh: clean."
