# V0.15.0 Connected

<div align="center">
  <img src="https://raw.githubusercontent.com/tw93/Kaku/main/assets/logo.png" alt="Kaku Logo" width="120" height="120" />
  <h1 style="margin: 12px 0 6px;">Kaku V0.15.0</h1>
  <p><em>A fast, out-of-the-box terminal built for AI coding.</em></p>
</div>

### Changelog

1. **Remote Tabs**: Tabs connected over ssh now show a dedicated icon with the host name, split tabs reveal which pane is remote, and mosh, autossh, and et sessions are recognized too.
2. **AI in SSH Sessions**: AI chat now understands when the current directory lives on a remote host, stops running local commands against it, and answers from the terminal context instead.
3. **SSH Everywhere**: The fish integration keeps your own ssh function and gains the 1Password fix, env-prefixed ssh aliases work again, and mosh gets the same terminfo fallback as ssh.
4. **Session Restore**: Restored ssh panes return to their remote working directory, and when a window cannot come back Kaku says how many were kept and retries them on the next launch.
5. **Fixes**: Text selection clears right after Ctrl+L instead of lingering, Cmd+, focuses the existing settings window instead of stacking new ones, and AI shell commands work for fish users again.

### 更新日志

1. **远程标签**：通过 ssh 连接的标签会显示专属图标和主机名，分屏里也能看出哪个窗格在远程，mosh、autossh、et 会话同样能识别。
2. **SSH 里的 AI**：AI 聊天现在能识别当前目录在远程主机上，不再把本地命令跑在远程路径里，而是基于终端上下文直接回答。
3. **SSH 一致体验**：fish 集成不再覆盖你自定义的 ssh 函数并支持 1Password 修复，带环境变量前缀的 ssh 别名恢复可用，mosh 也获得与 ssh 相同的 terminfo 回退。
4. **会话恢复**：恢复的 ssh 窗格会回到原来的远程目录，有窗口无法恢复时会说明数量并在下次启动时重试。
5. **问题修复**：Ctrl+L 清屏后选区高亮立即消失，Cmd+, 会聚焦已有设置窗口而不是不断新开，fish 用户的 AI 命令执行恢复正常。

Special thanks to @shlroland, @mortalYoung, and @darion-yaphet for their contributions to this release.

> https://github.com/tw93/Kaku
