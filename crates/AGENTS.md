# Crates Agent Guide

`crates/` contains shared utility crates used across the workspace. Inspect the target crate's `Cargo.toml` before editing because MANDA keeps some WezTerm-derived crates and adds its own helpers over time.

## Scope

Key groups:

| Group | Crates |
|-------|--------|
| Terminal core | `codec`, `color-types`, `vtparse`, `wezterm-cell`, `wezterm-escape-parser`, `wezterm-surface` |
| GUI and input | `wezterm-font`, `wezterm-input-types`, `wezterm-gui-subcommands`, `wezterm-open-url`, `wezterm-toast-notification` |
| Mux, client, and remote | `wezterm-client`, `wezterm-mux-server-impl`, `wezterm-ssh`, `wezterm-uds`, `manda-relay`, `manda-remote` |
| Platform and process | `pty`, `filedescriptor`, `procinfo`, `env-bootstrap`, `umask`, `async_ossl` |
| Lua and config support | `luahelper`, `wezterm-dynamic` |
| Shared utilities | `promise`, `base91`, `bidi`, `bintree`, `frecency`, `lfucache`, `rangeset`, `ratelim`, `tabout`, `wezterm-blob-leases`, `wezterm-char-props`, `wezterm-version` |
| MANDA-specific utilities | `manda-ai-utils` |

## Practical Rules

- Keep each crate focused and small in scope.
- Prefer reusing existing workspace crates before adding new ones.
- Preserve public API stability for cross-crate consumers.
- For macOS notification actions that launch MANDA commands, prefer resolving bundled executables relative to the running `manda-gui` binary.
- Keep `manda-ai-utils` provider-agnostic; provider policy belongs in GUI/CLI call sites and config, not in low-level helpers.
- When shared cache helpers affect startup, verify the GUI consumer and avoid invalidating hot-path caches unnecessarily.

## Notes

- Not every crate needs deep inspection for each task.
- Start from call-sites in top-level crates, then dive into utilities as needed.

## Cross-References

- [`term/AGENTS.md`](../term/AGENTS.md) - Uses codec, vtparse, and color types.
- [`manda-gui/AGENTS.md`](../manda-gui/AGENTS.md) - Uses font, input, and surface crates.
- [`lua-api-crates/AGENTS.md`](../lua-api-crates/AGENTS.md) - Uses Lua helper and dynamic binding crates.
