---
name: product-docs
description: "Keep the MANDA website (manda-term.vercel.app) accurate and user-facing: gather verified features from the current commit, refresh the landing page in web/, and verify before handing the diff to the maintainer."
when_to_use: "update website, update docs, 更新官网, 完善官网, 产品说明, 使用文档, 上手指南, product guide, document a feature, refresh manda-term.vercel.app, write user docs, site docs, 写文档, 写官网"
---

# MANDA Product Docs

Use this skill to write or update the user-facing landing page for MANDA. The site is the deliverable; this skill is the repeatable flow for keeping it correct and adding the next feature without drift.

The audience is normal users, not contributors. Explain what each feature **is**, how to **reach** it, and what happens **after** you use it. Avoid implementation detail.

## Where things live

- **Site source**: `web/index.html` in this repository — a single self-contained static page (inline CSS, no build step). It deploys to `manda-term.vercel.app`; point the Vercel project root at the `web/` directory.
- **Keep it self-contained**: no external font/CDN dependencies; the page must render from the file alone. Reuse the existing visual language (dark terminal theme, gradient accent, monospace terminal mockups); do not invent new components.
- **Content source of truth**: `README.md`, `docs/`, and the bundled config in `assets/`. Verify every claim against the code before publishing.
- **Screenshots**: `assets/manda.jpg` is the canonical app screenshot. MANDA is a native terminal, so CDP/browser screenshots of the app do not apply; only capture new app shots if the maintainer explicitly asks (build `make app` or use `/Applications/Manda.app`, then `screencapture`).

## Source of truth: verify, never infer

User-facing docs are public. Do not copy feature claims from a subagent summary or from a feature's name. Confirm each claim against the running code before publishing:

- Defaults and behavior flips: grep `config/src/config.rs` (and its tests, e.g. `*_defaults_to_*`) and bundled config in `assets/`.
- Shell behavior: `assets/shell-integration/setup_zsh.sh`.
- Keybindings: `docs/keybindings.md` is the maintainer's authored ground truth; reuse its wording rather than re-deriving.
- Provider presets and AI setup: `README.md` and `manda/src/ai_config/`.

## Workflow

1. **Scope**: read `git log` for what changed since the page was last updated. Read the nearest crate `AGENTS.md` and `CLAUDE.md` for feature notes.
2. **Edit `web/index.html`** in place: update the copy, feature grid, keybinding table, or download links. Keep the section structure and class names stable.
3. **Version pointers** (only when a release shipped): bump the version badge in the hero and the footer; leave historical wording alone.
4. **Verify before declaring done**:
   - Open `web/index.html` in a browser (or `python3 -m http.server` in `web/`) and confirm it renders without errors.
   - Confirm every link resolves (GitHub URLs, docs paths, the download URL).
   - Confirm responsive layout at mobile width (375px) and desktop (1280px).
5. **Hand off**: show the diff. Do NOT commit or push unless the maintainer says so this turn.

## Adding the next feature later

- A daily-use behavior → a bullet/step in the Features or AI section.
- A configurable surface or new tool → a section in the feature grid (+ Keybindings/Configuration links if it adds a shortcut).
- Re-run the verify checklist. Keep diffs minimal and atomic per behavior.
