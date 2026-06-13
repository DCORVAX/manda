# V0.12.2 Steadier

<div align="center">
  <img src="https://raw.githubusercontent.com/tw93/Kaku/main/assets/logo.png" alt="Kaku Logo" width="120" height="120" />
  <h1 style="margin: 12px 0 6px;">Kaku V0.12.2</h1>
  <p><em>A fast, out-of-the-box terminal built for AI coding.</em></p>
</div>

### Changelog

1. **Sleep Wake**: After the Mac sleeps or an external display is unplugged, Kaku no longer freezes on a stale frame; rendering recovers right away while your tasks keep running.
2. **External Displays**: Dragging a window on an external display no longer makes it jump at the first move, and Window Center plus `--position` land on the right spot when screens use different scales.
3. **Selection**: In mouse-enabled apps such as Claude Code, holding the left button while scrolling no longer paints a selection highlight that cannot be cleared, and a plain click clears any leftover one.
4. **Yazi**: Newer Yazi versions refused to start with config files written by earlier Kaku; those files are now repaired automatically.
5. **Windows**: A new window opened into a fullscreen Space fills the screen right away, and cold start no longer flashes a second empty window.
6. **Updates**: Clicking the update notification now asks for confirmation first, so running tasks are no longer interrupted by a stray click.

### 更新日志

1. **休眠唤醒**：合盖休眠或拔掉外接显示器后，Kaku 不再停在旧画面上假死，渲染会立即恢复，正在跑的任务不受影响。
2. **外接屏**：在外接显示器上拖动窗口不再在起手时跳一下，屏幕缩放不同时 Window Center 和 `--position` 也会落在正确位置。
3. **选区**：在 Claude Code 这类启用鼠标的应用里，按住左键滚动不再误画出无法清除的选区高亮，普通单击会清掉残留的高亮。
4. **Yazi**：新版 Yazi 会拒绝旧版 Kaku 写入的配置导致无法启动，现在这些配置文件会被自动修复。
5. **窗口**：在全屏 Space 中新开窗口会立即铺满整屏，冷启动也不再闪现第二个空窗口。
6. **更新**：点击更新通知会先弹出确认，不会再因误点直接关闭所有窗口、打断正在运行的任务。

> https://github.com/tw93/Kaku
