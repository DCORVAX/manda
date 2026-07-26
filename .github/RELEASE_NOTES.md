# V0.16.0 Trusted

<div align="center">
  <img src="https://raw.githubusercontent.com/tw93/Kaku/main/assets/logo.png" alt="Kaku Logo" width="120" height="120" />
  <h1 style="margin: 12px 0 6px;">Kaku V0.16.0</h1>
  <p><em>A fast, out-of-the-box terminal built for AI coding.</em></p>
</div>

### Changelog

1. **Inline AI Uses Your Provider**: The `#` prompt and the automatic quick-fix now go through the same transport as Cmd+L, so Codex, Copilot, and API-key setups all just work.
2. **Authenticated Controls**: Shell control messages now carry a local capability, so terminal output can no longer borrow your configured credentials to fire assistant requests.
3. **Close Tabs From The Navigator**: Press Backspace on a highlighted tab to close it, with the same confirmation you get everywhere else.
4. **Display Fixes**: Titlebar spacing follows the display, AI chat picks up a live appearance switch, overlays resize after a pane split, and link underlines survive hover.
5. **Fixes**: Starship setup stays inside Kaku instead of taking over your other terminals, white backgrounds stay readable, and menu commands run the bundled CLI.

### 更新日志

1. **内联 AI 走你的模型**：`#` 生成和自动修复现在与 Cmd+L 走同一条链路，Codex、Copilot 和 API key 三种配置都能直接用。
2. **控制消息需要认证**：Shell 发给终端的控制消息带上了本地凭据，终端里的输出无法再借用你配置的密钥触发助手请求。
3. **在标签导航里关标签**：选中标签按退格即可关闭，确认提示与别处一致。
4. **显示修复**：标题栏间距随显示器调整，AI 聊天会跟随系统外观切换，分屏后浮层尺寸正确，链接下划线在悬停时不再丢失。
5. **问题修复**：Starship 配置只在 Kaku 内生效，不再接管你的其他终端；白色背景上的文字保持可读；菜单命令改用随包分发的 CLI。

Special thanks to @zwrong and @mortalYoung for their contributions to this release.

> https://github.com/tw93/Kaku
