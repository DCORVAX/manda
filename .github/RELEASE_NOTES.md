# V0.1.2 🚀

<div align="center">
  <img src="https://raw.githubusercontent.com/DCORVAX/manda/main/assets/manda.jpg" alt="MANDA Logo" width="120" height="120" />
  <h1 style="margin: 12px 0 6px;">MANDA V0.1.2</h1>
  <p><em>A fast, out-of-the-box terminal built for AI coding.</em></p>
</div>

### Changelog

1. **Ícono de macOS corregido**: el contenido visible del ícono en el Launchpad se redujo al 81.5% (de 128px a ~104px aparentes), quedando centrado con margen transparente uniforme. Se regeneró `assets/logo.icns` con las 10 representaciones estándar (16–1024 px).
2. **Footer rediseñado**: nueva estructura editorial (línea de cierre serif, grupos de enlaces como colofón con separadores, logo `>_` y marca de agua ASCII) aplicada en las 8 páginas del sitio (EN + ES).
3. **Deploy automático a Vercel**: nuevo workflow `vercel-deploy.yml` + `vercel.json` + `.vercelignore` — cada push a `main` publica en GitHub Pages y Vercel simultáneamente.
4. **CI robusto**: instalación explícita de targets rustup en el build universal (arregla `E0463` en runners arm64 de macOS).
5. **Historia del repositorio limpia**: reescritura completa de mensajes y metadatos de los 1590 commits (sin referencias al proyecto upstream), release v0.1.1 y tag verificados.

### 更新日志

1. **修复 macOS 图标大小**：Launchpad 中图标可见内容缩放至 81.5%（视觉从 128px 降至约 104px），居中并保留均匀透明边距；使用 10 种标准尺寸（16–1024px）重新生成 `assets/logo.icns`。
2. **全新页脚设计**：编辑风格页脚（衬线收尾语句、圆点分隔的链接组、`>_` 标识与 ASCII 水印），已应用于全部 8 个页面（英文 + 西班牙文）。
3. **Vercel 自动部署**：新增 `vercel-deploy.yml` workflow 及 `vercel.json`/`.vercelignore` —— 每次推送到 `main` 同时发布 GitHub Pages 与 Vercel。
4. **CI 稳定性**：在通用构建中显式安装 rustup targets（修复 arm64 macOS runner 上的 `E0463`）。
5. **仓库历史清理**：重写全部 1590 个提交的消息与元数据（移除上游项目引用），v0.1.1 发布与标签已验证完好。

Special thanks to MANDA Developers and all contributors.

> https://github.com/DCORVAX/manda
