# V0.10.0 Polish 🪄

<div align="center">
  <img src="https://raw.githubusercontent.com/tw93/Kaku/main/assets/logo.png" alt="Kaku Logo" width="120" height="120" />
  <h1 style="margin: 12px 0 6px;">Kaku V0.10.0</h1>
  <p><em>A fast, out-of-the-box terminal built for AI coding.</em></p>
</div>

### Changelog

1. **AI Chat Panel**: A built-in chat overlay for interacting with AI directly inside the terminal window. Press `Cmd+L` to open it. Supports streaming output and scroll.
2. **Tab Rename**: Single double-click to rename a tab; drag and hover now show the correct cursor to avoid accidental rename triggers.
3. **macOS Stability**: Fixed a crash when closing a fullscreen window, a main-thread hang after lock-screen return, and incorrect terminal size after entering fullscreen.
4. **Copy Improvement**: Visually-wrapped TUI rows are joined into a single line when `copy_unwrap_tui_lines = true`.
5. **Shell Integration**: Fixed zsh-syntax-highlighting comment color invisibility in dark themes, autosuggest recursion with external providers, and `RUNEWIDTH_EASTASIAN` alignment with Kaku's width setting.
6. **Performance**: Reduced input latency, lock contention, and memory overhead; macOS runloop and opacity optimizations lower idle power consumption.
7. **Window Behavior**: On macOS, closing a window now hides it instead of destroying tabs, so state is preserved.
8. **MacPorts Support**: Tool detection now works correctly with MacPorts and other non-Homebrew package managers.

### 更新日志

1. **AI 对话面板**：内置 AI Chat 悬浮层，按 `Cmd+L` 打开，支持流式输出和历史滚动。
2. **Tab 重命名体验**：连续双击才进入重命名；拖拽和悬停时显示对应光标，避免误触重命名。
3. **macOS 稳定性**：修复全屏关闭窗口崩溃、锁屏返回后主线程卡住、进入全屏后终端内容大小不变等问题。
4. **复制优化**：开启 `copy_unwrap_tui_lines = true` 时，视觉换行的 TUI 行现在会正确拼接为一行复制。
5. **Shell 集成**：修复深色主题下 zsh 注释颜色不可见、外部 autosuggest 递归调用、`RUNEWIDTH_EASTASIAN` 与 Kaku 宽字符设置不一致等问题。
6. **性能优化**：降低输入延迟、锁竞争和内存占用；macOS runloop 与透明度优化减少空闲功耗。
7. **窗口行为**：macOS 关闭窗口改为隐藏而非销毁 tabs，状态得以保留。
8. **MacPorts 支持**：工具检测现在兼容 MacPorts 及其他非 Homebrew 包管理器。

> https://github.com/tw93/Kaku
