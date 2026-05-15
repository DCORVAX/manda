# V0.11.0 Steady 🧭

<div align="center">
  <img src="https://raw.githubusercontent.com/tw93/Kaku/main/assets/logo.png" alt="Kaku Logo" width="120" height="120" />
  <h1 style="margin: 12px 0 6px;">Kaku V0.11.0</h1>
  <p><em>A fast, out-of-the-box terminal built for AI coding.</em></p>
</div>

### Changelog

1. **Chinese UI**: Kaku now includes Simplified Chinese localization for the app shell, command palette, settings, and AI chat surfaces.
2. **Session Restore**: New restore settings make window snapshots more explicit and easier to control from config.
3. **AI Reasoning**: Deep reasoning from Fireworks, GLM, Kimi, and DeepSeek-compatible streams stays hidden while the visible answer remains clean.
4. **Kaku Chat**: `kaku chat` shows a compact thinking status during hidden reasoning and no longer records empty AI turns.
5. **Shell Setup**: `kaku init` is safer around read-only config paths, existing jump providers, Cmd+Backspace, Yazi theme blocks, and `#` AI queries.
6. **Window Control**: Cmd+W now closes the last fullscreen tab instead of hiding the app, while title-bar dragging avoids accidental snap or maximize.
7. **Mouse and Tabs**: Integrated title buttons, top-tab hit testing, tab drag animation, and scrollback selection behavior are more stable.
8. **Rendering**: Bar cursors, low-DPI toast text, color emoji sizing, and pane background alignment have been tightened.
9. **AI Transport**: Streaming, IME composition, empty API keys, proxy handling, and shell query setup are more robust across providers.
10. **Maintenance**: Agent guides, config docs, release checks, and contributor metadata are updated for the current maintainer workflow.

### 更新日志

1. **中文界面**：Kaku 现在内置简体中文，覆盖应用外壳、命令面板、设置和 AI 对话界面。
2. **会话恢复**：新的恢复设置让窗口快照行为更明确，也更容易从配置里控制。
3. **AI 推理**：Fireworks、GLM、Kimi、DeepSeek 兼容流里的深度推理会继续隐藏，最终回答保持干净。
4. **Kaku Chat**：`kaku chat` 在隐藏推理时只显示紧凑 thinking 状态，不再记录空的 AI 回合。
5. **Shell 初始化**：`kaku init` 对只读配置路径、已有跳转工具、Cmd+Backspace、Yazi 主题块和 `#` AI 查询更稳。
6. **窗口控制**：全屏最后一个 tab 下 Cmd+W 会关闭页面而不是隐藏应用，标题栏拖动也避免误触发 snap 或最大化。
7. **鼠标与标签页**：集成标题按钮、顶部标签栏命中、标签拖拽动画和 scrollback 选择滚动都更稳定。
8. **渲染细节**：条形光标、低 DPI toast 文本、彩色 emoji 尺寸和 pane 背景对齐都做了收紧。
9. **AI 传输**：流式输出、输入法组合、空 API key、代理处理和 shell 查询初始化在更多 provider 下更稳。
10. **维护工作**：Agent 指南、配置文档、release 检查和 contributor 元数据都同步到当前维护流程。

Special thanks to @t0m-car for the low-DPI toast clipping fix.

> https://github.com/tw93/Kaku
