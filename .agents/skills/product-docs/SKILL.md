---
name: product-docs
description: "Keep the MANDA website (wilfredy-x.github.io/manda) accurate and user-facing: gather verified features from the current commit, refresh the EN and ES pages in web/, and verify before handing the diff to the maintainer."
when_to_use: "update website, update docs, 更新官网, 完善官网, 产品说明, 使用文档, 上手指南, product guide, document a feature, refresh wilfredy-x.github.io/manda, write user docs, site docs, 写文档, 写官网"
---

# MANDA Product Docs

Use this skill to write or update the MANDA marketing site. The site is the deliverable; this skill is the repeatable flow for keeping it correct and adding the next feature without drift.

The audience is normal users, not contributors. Explain what each feature **is**, how to **reach** it, and what happens **after** you use it. Avoid implementation detail.

## Where things live

- **Site source**: `web/` in this repository — a small static multi-page site with no build step:
  - `index.html` (EN home — keep short: hero, three highlights, compact Why/AI teasers, download), `features.html` (full feature grid + AI section), `shortcuts.html` (full keybinding table), `faq.html` (accordion with the top 6 FAQs distilled from `docs/faq.md`).
  - `es/` — Spanish mirrors of the same four pages, kept in lockstep (same structure, section order, and anchors).
  - Shared `web/style.css` (design system) and `web/script.js` (theme toggle + mobile nav).
  - Deploys to GitHub Pages at `https://wilfredy-x.github.io/manda/` (gh-pages branch from `web/`); point the Vercel project root at the `web/` directory.
- **Design system**: warm monochrome editorial. Light is the primary theme; dark is secondary via the theme toggle (persisted in localStorage). Tokens live at the top of `web/style.css`: warm cream `#faf9f6` surface, ink `#121212`, sand/stone dividers, Hedvig Letters Serif for display headings, Hedvig Letters Sans for body/UI, monospace for code. Flat surfaces (no shadows), radius 12–16px cards, pill buttons. Reuse the existing tokens and components; do not invent new ones.
- **Bilingual parity**: every EN page has an ES twin under `es/`. Update both together; mirror anchors and copy tone.
- **Content source of truth**: `README.md`, `docs/`, and the bundled config in `assets/`. Verify every claim against the code before publishing.
- **Screenshots**: `assets/manda.jpg` is the canonical app screenshot. MANDA is a native terminal, so CDP/browser screenshots of the app do not apply; only capture new app shots if the maintainer explicitly asks (build `make app` or use `/Applications/Manda.app`, then `screencapture`).

## Source of truth: verify, never infer

User-facing docs are public. Do not copy feature claims from a subagent summary or from a feature's name. Confirm each claim against the running code before publishing:

- Defaults and behavior flips: grep `config/src/config.rs` (and its tests, e.g. `*_defaults_to_*`) and bundled config in `assets/`.
- Shell behavior: `assets/shell-integration/setup_zsh.sh`.
- Keybindings: `docs/keybindings.md` is the maintainer's authored ground truth; reuse its wording rather than re-deriving.
- Provider presets and AI setup: `README.md` and `manda/src/ai_config/`.

## Workflow

1. **Scope**: read `git log` for what changed since the site was last updated. Read the nearest crate `AGENTS.md` and `CLAUDE.md` for feature notes.
2. **Edit the pages**: keep the home page short; put depth on `features.html` and `shortcuts.html`. Change copy in the EN page and its `es/` twin together. Preserve the section structure, class names, and anchor names. The FAQ pages (`faq.html` + `es/faq.html`) distill the top 6 questions from `docs/faq.md`; keep answers short and pick the most user-facing questions. The FAQ is linked from the footer of every page.
3. **Version pointers** (only when a release shipped): bump the version badge in the hero and the footer of both languages; leave historical wording alone.
4. **Verify before declaring done**:
   - Open `web/index.html` in a browser (or `python3 -m http.server` from `web/`) and confirm it renders without errors in light and dark (toggle in the nav).
   - Confirm every link resolves (Features, Shortcuts, FAQ, EN↔ES, GitHub URLs, docs paths, download URL) on both languages.
   - Confirm responsive layout at mobile width (375px) and desktop (1280px).
5. **Hand off**: show the diff. Do NOT commit or push unless the maintainer says so this turn.

## Adding the next feature later

- A daily-use behavior → a bullet in the home teaser and a card in `features.html` (+ a row in `shortcuts.html` if it adds a shortcut).
- A configurable surface or new tool → a card in the feature grid or a step in the AI section; link from the home page teaser.
- Always update EN and ES together, then re-run the verify checklist. Keep diffs minimal and atomic per behavior.
