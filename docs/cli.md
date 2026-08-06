# CLI Reference

Run `manda` in your terminal to see all available commands.

## manda ai

Open the AI settings panel inside MANDA. Configure external coding tools (Claude Code, Codex, Gemini CLI, Copilot CLI, Kimi Code, etc.) and MANDA Assistant.

```bash
manda ai
```

## manda chat

Start MANDA's standalone AI chat from any shell. This is a discoverable alias for
the bundled `k` helper, so it works even when `k` is not on your PATH.

```bash
manda chat                 # open interactive chat
manda chat "explain this"  # one-shot prompt
```

The chat uses `~/.config/manda/assistant.toml`, shares the same conversation and
memory files as the `Cmd + L` overlay, and supports `/new`, `/resume`, `/clear`,
`/status`, `/memory`, and `/exit` in interactive mode.

## manda config

Open the MANDA configuration TUI for common settings and Lua overrides. It
ensures `~/.config/manda/manda.lua` exists and is also accessible from the
settings panel with `Cmd + ,`.

```bash
manda config
```

## manda doctor

Run diagnostics and verify that MANDA's shell integration, PATH entries, and optional tool installations are healthy. Use this first if something feels broken.

```bash
manda doctor
manda doctor --shell fish       # check fish even when $SHELL points to zsh
manda doctor --shell fish --fix # repair the selected integration
```

## manda update

Check for and install the latest MANDA release.

```bash
manda update
```

## manda reset

Remove MANDA-managed shell and tmux integration, MANDA-managed git delta defaults,
selected MANDA state, and managed theme blocks in `~/.config/manda/manda.lua`.
User-authored Lua outside managed blocks is preserved. Use with caution and run
`manda init` again if you want shell integration back.

```bash
manda reset
manda reset --shell fish # use fish for restart and restore guidance
```

## manda init

Set up MANDA's shell integration for zsh or fish. When both shells are installed,
an interactive run asks which one to configure. Use `--shell` to make the choice
explicit in scripts or when `$SHELL` does not match your daily shell. Also
installs optional CLI tools (Starship, Delta, Lazygit, Yazi) via Homebrew.

```bash
manda init
manda init --shell fish
manda init --shell zsh --update-only
```

If the `manda` command goes missing from your shell, restore it with:

```bash
/Applications/Manda.app/Contents/MacOS/manda init --update-only
exec zsh -l
```

## manda cli

Interact with the MANDA multiplexer from scripts and external tools.

```bash
manda cli split-pane                          # split current pane
manda cli split-pane -- bash -c "echo hello"  # split and run a command
manda cli --help                              # list all subcommands
manda cli split-pane --help                   # help for a specific subcommand
```

Useful for integrating MANDA with AI tools or shell scripts that need to open panes or tabs programmatically.
