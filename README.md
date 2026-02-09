<div align="center">
  <h1>Kaku</h1>
  <p><em>A fast, out-of-the-box terminal built for AI coding.</em></p>

  <p>🚧 Early Development - Features and APIs may change</p>
</div>

<p align="center">
  <a href="https://github.com/tw93/Kaku/stargazers"><img src="https://img.shields.io/github/stars/tw93/Kaku?style=flat-square" alt="Stars"></a>
  <a href="https://github.com/tw93/Kaku/releases"><img src="https://img.shields.io/github/v/tag/tw93/Kaku?label=version&style=flat-square" alt="Version"></a>
  <a href="LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square" alt="License"></a>
  <a href="https://github.com/tw93/Kaku/commits"><img src="https://img.shields.io/github/commit-activity/m/tw93/Kaku?style=flat-square" alt="Commits"></a>
  <a href="https://twitter.com/HiTw93"><img src="https://img.shields.io/badge/follow-Tw93-red?style=flat-square&logo=Twitter" alt="Twitter"></a>
</p>

<p align="center">
  <img src="assets/kaku.jpeg" alt="Kaku Screenshot" width="1000" />
  <br/>
  Kaku is a deeply customized fork of <a href="https://github.com/wez/wezterm">WezTerm</a>, designed for an <b>out-of-the-box</b> experience.
</p>

## Features

- **Zero Config**: Polished defaults with JetBrains Mono, **optimized macOS font rendering**, and smooth animations.
- **Built-in Shell Suite**: Comes pre-loaded with **Starship** prompt, **z** smart jumper, **Delta** git diff beautifier, **syntax highlighting**, and **autosuggestions**.
- **Fast & Lightweight**: **40% smaller binary**, instant startup with lazy loading, and a stripped-down GPU-accelerated core.
- **Lua Scripting**: Retains the full power of WezTerm's Lua engine for infinite customization.

## Quick Start

1. [**Download Kaku DMG**](https://github.com/tw93/Kaku/releases/latest) & Drag to Applications
2. Open Kaku - Right-click Open if blocked
3. Run `sudo xattr -d com.apple.quarantine /Applications/Kaku.app` if needed
4. On first launch, Kaku will automatically set up your shell environment

## Usage Guide

Kaku comes with intuitive macOS-native shortcuts:

| Action | Shortcut |
| :--- | :--- |
| **New Tab** | `Cmd + T` |
| **New Window** | `Cmd + N` |
| **Split Pane Vertical** | `Cmd + D` |
| **Split Pane Horizontal** | `Cmd + Shift + D` |
| **Zoom/Unzoom Pane** | `Cmd + Shift + Enter` |
| **Resize Pane** | `Cmd + Ctrl + Arrows` |
| **Close Tab/Pane** | `Cmd + W` |
| **Navigate Tabs** | `Cmd + [`, `Cmd + ]` or `Cmd + 1-9` |
| **Navigate Panes** | `Cmd + Opt + Arrows` |
| **Clear Screen** | `Cmd + R` |
| **Font Size** | `Cmd + +`, `Cmd + -`, `Cmd + 0` |
| **Smart Jump** | `z <dir>` |
| **Smart Select** | `z -l <dir>` |
| **Recent Dirs** | `z -t` |

## Configuration

Kaku automatically configures these built-in tools:

- **Starship** - Fast prompt with git status and language versions
- **z** - Jump to directories with `z <name>`
- **Delta** - Syntax-highlighted git diffs, works automatically
- **Syntax Highlighting** - Real-time command validation
- **Autosuggestions** - Fish-style suggestions from history

Config loading priority:

1. **Environment Variable**: `KAKU_CONFIG_FILE`
2. **Bundled Config**: `Kaku.app/Contents/Resources/kaku.lua`
3. **User Config**: `~/.config/kaku/kaku.lua`

## Why Kaku?

I really enjoy using the CLI to handle everything in my work and life. I've even built open-source tools like [tw93/mole](https://github.com/tw93/mole) and [tw93/pake](https://github.com/tw93/pake) to stay in the terminal.

I used **Alacritty** for a long time but its lack of multi-tab support makes it inconvenient when doing AI coding. **Kitty** has some aesthetic and window positioning issues I couldn't get past. **Ghostty** is promising but I haven't found a good solution for its font rendering yet. **Warp** feels too heavy and requires a login. **iTerm2** is reliable but shows its age and is harder to deeply customize.

**WezTerm** is fantastic and very hackable. However, I wanted a more "out-of-the-box" experience that didn't require extensive configuration.

So I built Kaku for AI coding. Fast, beautiful, and ready to use.

### Performance

| Metric | Upstream | Kaku | Improvement |
| :--- | :--- | :--- | :--- |
| **Binary Size** | ~67 MB | **~40 MB** | **40% Smaller** |
| **App Bundle** | ~100 MB | **~80 MB** | **20% Smaller** |
| **Startup** | Standard | **Instant** | **Lazy Loading** |
| **Shell Init** | Standard | **~100ms** | **50% Faster** |

Achieved through aggressive stripping of unused features, lazy loading of color schemes, and shell optimizations.

## Support

- If Kaku helped you, star the repo or [share it](https://twitter.com/intent/tweet?url=https://github.com/tw93/Kaku&text=Kaku%20-%20A%20fast%20terminal%20built%20for%20AI%20coding.) with friends.
- Got ideas or found bugs? Open an issue/PR or check [CONTRIBUTING.md](CONTRIBUTING.md) for details.
- Like Kaku? <a href="https://miaoyan.app/cats.html?name=Kaku" target="_blank">Buy Tw93 a Coke</a> to support the project!

## License

MIT License, feel free to enjoy and participate in open source.
