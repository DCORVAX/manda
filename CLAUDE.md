# Kaku

macOS-native terminal emulator forked from WezTerm, optimized for AI coding workflows.

## 启动前

- 全局规则在 `~/.claude/CLAUDE.md`（写作 / 提交 / 安全 / 验证 / 响应风格）。
- Rust 通用规则在 `~/.claude/rules/rust.md`。
- macOS 平台陷阱（AppKit 自动注入、menubar 时序）在 `.claude/rules/macos.md`，改 menu / window handling 前必读。
- 仓库地图、Working Rules、Verification、Subsystem Guides、Current Risk Areas 在 `AGENTS.md`。
- 每个 crate 有自己的 `<crate>/AGENTS.md`，改某个 crate 前先读它。
- 发版完整 runbook 在 `.claude/skills/release/SKILL.md`（通过 `/release` 调用）。用户向导式说明在 `docs/release-checklist.md`。
- Issue / PR 批量维护、push 后等 CI、回复并关闭的流程在 `.claude/skills/maintainer-sweep/SKILL.md`。

## 常用命令

```bash
make fmt-check     # 格式校验
make check         # 编译
make test          # 测试
make app           # 构建 app bundle，GUI/渲染/AI overlay 验证用
./scripts/build.sh
./scripts/check_release_notes.sh
./scripts/check_release_config.sh
./scripts/check_config_release_readiness.sh
```

`make fmt` 需要 nightly Rust toolchain。

## 项目独有硬规则

- AI 聊天与 shell 流是核心，改 `kaku-gui/src/ai_*` / `ai_chat_engine/` / `cli_chat/` / `overlay/ai_chat/` 前先读 `kaku-gui/AGENTS.md`。
- Config 发版当前在 `config_version: 19`。改 config schema 必须同步更新 bundled defaults、docs、release checks、migration 行为。
- 启动性能依赖 shell user vars 缓存、Lua bytecode、early appearance queries、GLSL version、内置字体的缓存。不要在没有性能测量的情况下使它们失效。
- 通知 action 回调 Kaku 时，要相对运行中的 app 解析 bundled executable，不要假设系统路径。
- macOS 26 上 AppKit 会向 Window/Help menu 自动注入项，可能 PAC-fault 崩溃。改 menubar 相关代码前先读 `.claude/rules/macos.md`。

## 提交习惯

- 用户说"帮我提交"或"提交"时，commit + push 一步完成,不需要分两步确认。
- 多 issue / 多 bug 一起修时，**每个 issue / bug 一个独立 commit**，不要打包成一个 mega-commit。subject 写"fix(<scope>): #issue-number 简述"。这样 release notes 自动 list 出来更清楚。
- Issue / PR 的公开回复和关闭动作默认先 draft；如果用户已经明确给出文案、关闭决策和 CI gate，就在 gate 通过后直接执行。
