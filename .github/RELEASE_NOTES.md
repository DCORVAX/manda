# V0.12.0 Sharper ✂️

<div align="center">
  <img src="https://raw.githubusercontent.com/tw93/Kaku/main/assets/logo.png" alt="Kaku Logo" width="120" height="120" />
  <h1 style="margin: 12px 0 6px;">Kaku V0.12.0</h1>
  <p><em>A fast, out-of-the-box terminal built for AI coding.</em></p>
</div>

### Changelog

1. **Session Scrollback**: Reopening restores each pane's scrollback up to 1,500 lines, reflowing when the width changed.
2. **Codex Backend**: Kaku Assistant runs on your existing Codex / ChatGPT login, no separate API key needed.
3. **AI Chat**: The chat overlay streams answers in real time, suggests your next message with `/suggest`, and reopens reliably every time.
4. **AI Shell**: The `#` quick-fix adds injection detection, and tool access blocks credential files like `.env` and SSH keys in any directory.
5. **SmartPrompt**: Cmd+Q quits instantly when every pane sits at a shell prompt, and asks first when an agent or editor is running.
6. **macOS Window**: Theme flips refresh all windows at once, fullscreen exit is clean, and a title-bar click no longer maximizes.
7. **Document Open**: PDFs, images, media, archives, and Office files open in their default app instead of VS Code.
8. **Tidy**: `smart_tab_mode` is added, Simplified Chinese localization is removed, the zsh comment highlight is brighter, and new CI gates are in place.

### 更新日志

1. **会话历史恢复**：重新打开窗口后，会恢复每个面板最多 1,500 行滚动历史，列宽变化时也会自动重排。
2. **Codex 后端**：Kaku Assistant 现在可以直接复用你已有的 Codex / ChatGPT 登录，不用再单独配置一个 API key。
3. **AI 聊天**：聊天面板的回答实时流式输出，可以用 `/suggest` 预测你接下来想发的内容，每次也都能稳定重新打开。
4. **AI Shell**：`#` 生成命令新增了注入检测，工具访问会在任意目录拦截 `.env`、SSH 私钥等敏感文件。
5. **SmartPrompt**：所有面板都停在 shell 提示符时，Cmd+Q 会直接退出；仍有 agent 或编辑器在运行时，则会先询问一下。
6. **macOS 窗口**：浅深色切换会一次性刷新所有窗口，退出全屏更干净，标题栏单击也不再误触最大化。
7. **文档默认打开**：PDF、图片、音视频、压缩包、Office 文档现在都用系统默认 app 打开，不再被 VS Code 抢走。
8. **轻装**：新增 `smart_tab_mode`，移除简体中文本地化，zsh 注释高亮更清晰，并新增了多道 CI 检查。

Special thanks to @t0m-car for the non-fancy tab bar fixes.

> https://github.com/tw93/Kaku
