# V0.10.0 Chat 🪄

<div align="center">
  <img src="https://raw.githubusercontent.com/tw93/Kaku/main/assets/logo.png" alt="Kaku Logo" width="120" height="120" />
  <h1 style="margin: 12px 0 6px;">Kaku V0.10.0</h1>
  <p><em>A fast, out-of-the-box terminal built for AI coding.</em></p>
</div>

### Changelog

1. **AI Chat Panel**: Press `Cmd+L` for a terminal-native chat overlay with streaming Markdown, syntax highlighting, shell context, project tools, web search, and memory.
2. **`k` CLI**: The new `k` binary brings the same AI engine into an alternate-screen TUI, with theme detection and safer cancel/approval behavior.
3. **AI Configuration**: Assistant settings now support chat and fast models, live model loading, proxy-aware requests, OAuth setup, and broader provider responses.
4. **AI Safety and Context**: Shell approvals, sensitive-path guards, file writes, patches, tool limits, failed-command context, and parse errors are all more defensive.
5. **Window Snapshots**: Kaku saves multi-tab, multi-pane layouts automatically; restore one with `Cmd+Option+Shift+T`, Shell → Restore Previous Window, or the Command Palette.
6. **macOS and Terminal UX**: Fixed fullscreen crashes and hangs, display races, resize gaps, tab drag state, cursor reflow, links, selection, and TUI copy.
7. **Updates, Shell, and Performance**: Updates download in the background, checksums fail closed, proxy and MacPorts detection improved, shell state is cached, and startup is lighter.
8. **Tests and Release Tooling**: 52 regression tests landed, config version 19 is documented, update archives are refreshed after stapling, and release scripts are safer to resume.

### 更新日志

1. **AI 对话面板**：按 `Cmd+L` 打开终端内 AI Chat，支持流式 Markdown、语法高亮、shell 上下文、项目工具、网页搜索和本地记忆。
2. **`k` CLI**：新增 `k` 二进制，把同一套 AI 引擎放进 alternate-screen TUI，并补上主题识别、取消和审批语义。
3. **AI 配置**：Assistant 设置支持 chat model、fast model、在线模型加载、代理感知请求、OAuth 配置，以及更多 provider 响应格式。
4. **AI 安全与上下文**：shell 审批、敏感路径保护、文件写入、patch、工具参数上限、失败命令上下文和解析错误都更稳。
5. **窗口快照**：Kaku 会自动保存多 tab、多 pane 布局，需要时按 `Cmd+Option+Shift+T`，或从 Shell → Restore Previous Window、命令面板恢复。
6. **macOS 与终端体验**：修复全屏崩溃和卡住、显示器竞态、resize 缝隙、tab 拖拽残留、光标 reflow、链接、选择和 TUI 复制。
7. **更新、Shell 与性能**：更新改为后台下载，checksum 失败时关闭风险路径，代理与 MacPorts 检测更稳，shell 状态缓存，冷启动更轻。
8. **测试与发布工具**：新增 52 个回归测试，config version 19 的变更已写入，staple 后会刷新更新包，release 脚本也更容易恢复。

Special thanks to @s010s, @SherlockSalvatore, @darion-yaphet, @ddotz, @beautifulrem, @yxspace, and @fanweixiao for their contributions to this release.

> https://github.com/tw93/Kaku
