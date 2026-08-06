# FAQ

## Is there a Windows or Linux version?

Not currently. MANDA is macOS-only while the macOS experience is being polished. Windows and Linux may come later.

## Can I use a transparent window?

Yes. Add to `~/.config/manda/manda.lua`:

```lua
local config = require("manda").config
config.window_background_opacity = 0.92
config.macos_window_background_blur = 20  -- optional blur, 0–100
return config
```

## How do I turn off copy on select?

```lua
config.copy_on_select = false
```

## How do I customize keybindings?

Append to `config.keys`, do not replace it:

```lua
config.keys[#config.keys + 1] = {
  key = "RightArrow",
  mods = "CMD|SHIFT",
  action = wezterm.action.ActivatePaneDirection("Right"),
}
```

See [keybindings.md](keybindings.md) and [configuration.md](configuration.md) for more examples.

## Can I control working directory inheritance?

Yes, individually for windows, tabs, and splits:

```lua
config.window_inherit_working_directory = true
config.tab_inherit_working_directory = true
config.split_pane_inherit_working_directory = true
```

All are enabled by default.

## How do I disable MANDA Assistant?

Run `manda ai`, open MANDA Assistant settings, and set Enabled to Off. Or edit `~/.config/manda/assistant.toml` directly:

```toml
enabled = false
```

## How do I use a custom LLM provider?

Run `manda ai`, keep Auth Type set to API key, and enter your Base URL, API Key,
Simple Model, and Deep Model manually. Choose **API Mode** `chat_completions`
for `/v1/chat/completions`, or `responses` for `/v1/responses`. If the Responses
provider supports hosted search, set **Native Web Search** to On; no separate
search provider or search API key is required.

## How do I restore default config?

```bash
manda reset
```

This removes MANDA-managed shell and tmux integration, MANDA-managed git delta
defaults, selected MANDA state, and managed theme blocks in
`~/.config/manda/manda.lua`. User-authored Lua outside managed blocks is
preserved. Run `manda init` again if you want shell integration back.

## The `manda` command is missing. How do I recover it?

```bash
/Applications/Manda.app/Contents/MacOS/manda init --update-only
exec zsh -l
```

Then run `manda doctor` to verify everything is healthy.

## How do I use MANDA's CLI from scripts?

```bash
manda cli split-pane
manda cli split-pane -- bash -c "echo hello"
manda cli --help
```

See [cli.md](cli.md) for full reference.

## How do I enable the scrollbar?

Open `manda config` and toggle the scrollbar option, or add to `~/.config/manda/manda.lua`:

```lua
config.enable_scroll_bar = true
```

## How do I scroll inside nano, vim, or another full-screen terminal app?

Enable alternate-screen wheel forwarding:

```lua
config.alternate_screen_wheel_scrolls_terminal = true
```

## How do I change the font? My font change isn't taking effect.

Font changes require explicitly setting `config.font` in your config:

```lua
config.font = wezterm.font('Your Font Name')
```

Note: MANDA's theme-aware font weight system only applies to the default JetBrains Mono stack. Once you set a custom font, MANDA will no longer override its weight automatically.

## My `window_padding` change isn't working.

`window_padding` values require a `'px'` unit suffix:

```lua
config.window_padding = { left = '24px', right = '24px', top = '40px', bottom = '20px' }
```

Plain numbers (without `'px'`) are interpreted as terminal cell units, which may not match your intent.

## The screen jumps to the top while Claude Code is generating output.

This is a known interaction between trackpad scroll and Claude Code's streaming output. If you accidentally scroll to the top mid-stream, pressing the down arrow or scrolling back down returns you to the current output. A fix for the jump behavior has been tracked and shipped in recent releases.

## Cmd+Shift+Y sends a local path when inside an SSH session.

The yazi remote-files feature (`Cmd+Shift+R`) is designed for SSH sessions and mounts the remote filesystem via sshfs. `Cmd+Shift+Y` is for local yazi. Use `Cmd+Shift+R` when you are inside an SSH pane.

## The `y` shell wrapper doesn't sync my directory on exit.

Make sure the MANDA fish/zsh shell integration is sourced. Check with `manda doctor`. The `y` wrapper requires the shell init to be loaded. A bare `yazi` call will not sync the directory.

## Homebrew installs the wrong `manda`.

There is an older unrelated package named `manda` on Homebrew. Install MANDA with the one-liner installer to avoid conflicts:

```bash
curl -fsSL https://raw.githubusercontent.com/WILFREDY-X/manda/main/install/install.sh | bash
```

The installer always downloads the MANDA DMG from GitHub Releases.

## Claude Code notifications don't appear.

MANDA's notification permission may not be granted. Go to System Settings > Notifications > MANDA and enable Allow Notifications. Then restart MANDA.

## The global hotkey doesn't work on non-QWERTY keyboards (e.g. Colemak).

`Cmd + Opt + Ctrl + K` uses the physical QWERTY K position. On Colemak, this corresponds to a different key. Remap it in your config:

```lua
table.insert(config.keys, {
  key = 'k',  -- adjust to your layout's physical key
  mods = 'CMD|OPT|CTRL',
  action = wezterm.action.EmitEvent('toggle-global-window'),
})
```

## QR codes and terminal graphics look vertically stretched.

MANDA's default `line_height = 1.28` favors comfortable text spacing. Terminal graphics built from characters, such as QR codes, `neofetch` logos, and TUI bar charts, scale with the row height, so they render about 28% taller than in terminals with no extra line spacing. This is a typography trade-off, not a rendering bug: block characters must fill the whole cell so TUI borders and progress bars stay seamless.

If you want near-square graphics, lower the line height in `~/.config/manda/manda.lua`:

```lua
config.line_height = 1.1  -- or 1.0 to match terminals without extra spacing
```

Note that no terminal renders half-block QR codes perfectly square: with common monospace fonts the cell is naturally a bit taller than 2:1 even at `line_height = 1.0`.

## Can I use MANDA with tiling window managers (yabai, AeroSpace)?

MANDA is compatible with yabai and AeroSpace. If you see continuous flickering, it is usually caused by the tiling WM fighting with MANDA's fullscreen/resize logic. Disabling MANDA's native fullscreen (`config.native_macos_fullscreen_mode = false`) or excluding MANDA from the tiling WM's managed window list typically resolves it.
