# Config Version History

`config_version` is a monotonically increasing integer. The single source of truth is
`assets/shell-integration/config_version.txt`; the release gate
(`scripts/check_release_config.sh`) requires it to increment on every release, even when
nothing in the schema changed. Do not hardcode the current number in agent guides or
instruction files; read the txt file instead.

## Rules for bumping

- A schema change must update the bundled defaults (`assets/macos/Kaku.app/Contents/Resources/kaku.lua`),
  user docs, release checks, and migration behavior together, in one change.
- Only keys that existed in the previous released version need migration code. A feature
  introduced in the current cycle ships its default directly: no migration, no version-gated
  branches, no release-note migration mention. Check with
  `git show V<previous release>:assets/macos/Kaku.app/Contents/Resources/kaku.lua | grep <key>`.
- A bump with no schema change is normal and expected; record why in this file.

## History

| Version | Release | Change |
|---|---|---|
| v21 | - | Adds `smart_tab_mode`. Introduces the optional `SmartPrompt` value for `window_close_confirmation` (the bundled default later flipped to `SmartPrompt` so a stateful pane is no longer dropped silently on Cmd+Q). Accepts the removed `language` option as a deprecated field for backward compat. |
| v22 | - | Adds a precmd guard so the dark-theme comment color override still applies when the user pre-loads fast-syntax-highlighting or zsh-syntax-highlighting in their own `.zshrc`. |
| v23 | - | Flips the bundled `smart_tab_mode` default to `suggestion_first` so Tab accepts a visible autosuggestion, falling back to completion. No schema change, so no migration; users who set `completion_first` keep it. |
| v24 | - | Migrates `$schema` keys in user yazi configs to the `#:schema` comment form that yazi 26.5.6+ requires. No schema change; the repair runs in three places that must stay in sync: setup scripts, the yazi wrapper, and `kaku.lua`. |
| v25 | - | No schema change. Bumps so an updated install regenerates the bundled zsh integration, picking up the fast-syntax-highlighting `path-to-dir` style that no longer underlines existing directories. |
| v26 | V0.12.4 | No schema or bundled-integration change; increments only because the release gate requires a bump every release. The release highlight reuses the Cmd+Click-opens-links improvement. |
| v27 | V0.13.0 | Adds `tab_title_show_foreground_process`, an opt-in setting for showing foreground process names in auto-generated tab titles. No migration is needed because this key did not exist in V0.12.4. |
| v28 | - | No schema change. Bumps so existing installs regenerate the bundled zsh integration with the self-contained SSH wrapper required by shell snapshot tools. |

When you bump the version, add a row here in the same change.
