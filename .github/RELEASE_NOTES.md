# V0.14.0 Focused

<div align="center">
  <img src="https://raw.githubusercontent.com/tw93/Kaku/main/assets/logo.png" alt="Kaku Logo" width="120" height="120" />
  <h1 style="margin: 12px 0 6px;">Kaku V0.14.0</h1>
  <p><em>A fast, out-of-the-box terminal built for AI coding.</em></p>
</div>

### Changelog

1. **Pane Navigation**: Tab Navigator now lists every pane inside split tabs and lets you jump directly to the one you need, while narrow tab titles keep the active pane visible.
2. **Tab Renaming**: The rename dialog has clearer text and cursor behavior, canceling with Escape or the mouse no longer leaves a phantom selection, and automatic split-pane titles stay intact.
3. **Tabs & Selection**: Tab separators are cleaner, selected text is easier to see in dark themes, and double-click selection now respects boundaries between CJK and Latin text.
4. **Session Restore**: Closed tabs stay closed after restart, incomplete restores preserve the original recovery data, and SSH sessions can be restored from saved snapshots again.
5. **Security & Stability**: Dependency advisories for `anyhow` and `crossbeam-epoch` are resolved, while pane actions and mouse releases handle disappearing UI state more safely.

### 更新日志

1. **分屏导航**：Tab Navigator 现在会列出标签内的每个分屏，可以直接跳到需要的分屏；标签很窄时也会优先保留当前分屏标题。
2. **标签重命名**：重命名窗口的文字与光标表现更清楚，按 Esc 或用鼠标取消后不再留下错误选区，自动生成的分屏标题也不会被意外覆盖。
3. **标签与选择**：标签分隔线更简洁，深色主题下的选区更清楚，双击选择也会正确区分中日韩文字与拉丁文字的边界。
4. **会话恢复**：重启后不会再恢复已经关闭的标签，恢复不完整时会保留原始恢复数据，SSH 会话也能再次从快照中恢复。
5. **安全与稳定性**：已解决 `anyhow` 与 `crossbeam-epoch` 的安全公告，分屏操作和鼠标释放在界面状态变化时也会更稳。

> https://github.com/tw93/Kaku
