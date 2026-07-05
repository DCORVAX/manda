# V0.13.0 Faster

<div align="center">
  <img src="https://raw.githubusercontent.com/tw93/Kaku/main/assets/logo.png" alt="Kaku Logo" width="120" height="120" />
  <h1 style="margin: 12px 0 6px;">Kaku V0.13.0</h1>
  <p><em>A fast, out-of-the-box terminal built for AI coding.</em></p>
</div>

### Changelog

1. **Startup & Prompt**: Kaku now does less work before the first frame and keeps shell, config, and font caches warm, so new windows and prompts feel faster.
2. **Default Prompt**: The bundled prompt now shows Git status and Node version while keeping the same compact terminal feel.
3. **Tab Titles**: Tabs stay path-focused by default, with an optional Command Tab Titles setting for showing running tools such as `project·claude`; split panes also keep their titles readable.
4. **AI Chat**: Long streaming conversations no longer re-highlight the whole chat on every update, keeping the AI panel smoother during large sessions.
5. **Window & Terminal Stability**: Moving between displays, half-intensity text, resizing terminals smaller, and deferred config setup are handled more reliably.

### 更新日志

1. **启动与提示符**：Kaku 现在会在首帧前少做一些工作，并缓存 shell、配置和字体相关结果，新窗口和提示符响应会更快。
2. **默认提示符**：内置提示符现在会显示 Git 状态和 Node 版本，同时保留原本紧凑的终端观感。
3. **标签标题**：标签默认继续以路径为主，需要时可以打开 Command Tab Titles 来显示 `project·claude` 这类正在运行的工具，分屏标题也会保持可读。
4. **AI 聊天**：长对话流式输出时不再每次都重新高亮整段聊天，AI 面板在大段会话里会更顺。
5. **窗口与终端稳定性**：跨显示器缩放、半亮文本、终端缩小时的滚动历史，以及延迟配置初始化都处理得更稳。

> https://github.com/tw93/Kaku
