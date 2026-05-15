---
name: maintainer-sweep
description: "Run Kaku maintainer follow-up for live GitHub issues and pull requests: triage latest open items, map them to local fixes, verify, push main safely, wait for GitHub Actions, then post concise replies and close items when appropriate."
when_to_use: "latest issues, latest PRs, triage issues, triage PRs, close issue, reply issue, reply PR, GitHub follow-up, push then close, maintainer sweep, 看看最新 issue, 看看最新 PR, 回复 issue, 关闭 issue, 处理 PR"
---

# Kaku Maintainer Sweep

Use this for post-release or pre-release maintenance work that involves current GitHub issues/PRs, local fixes, CI-gated push follow-up, and public replies. It complements `.claude/skills/release/SKILL.md`; do not use it for the notarized release flow itself.

## Workflow

1. **Refresh live state**
   - Run `gh issue list --state open --limit 20` and `gh pr list --state open --limit 20`.
   - For each item you may act on, read `gh issue view <n>` or `gh pr view <n>` and confirm title, author, state, body, and recent comments.
   - If the task says "latest", refresh these lists again before final decisions because the queue may change during the session.

2. **Map to code and commits**
   - Find the latest release tag with `git tag --sort=-version:refname | head -1`, ignoring rolling tags such as `nightly` unless the maintainer explicitly asks about them.
   - Compare relevant changes with `git log <tag>..HEAD --oneline`, `git show`, and targeted `rg`.
   - Do not treat a closed issue as proof by itself; identify the actual fix mechanism or the commit that changed behavior.

3. **Fix and verify**
   - Keep fixes scoped and atomic. For multiple unrelated issues, prefer one commit per issue or behavior.
   - For shell integration changes, run the affected smoke test plus the shell smoke group when feasible.
   - For AI provider or config changes, run targeted Cargo tests first, then the repository checks that match `AGENTS.md`.
   - Before shipping release-adjacent work, prefer `git diff --check`, `make fmt-check`, `make check`, `make test`, and `make app` unless the maintainer narrows the gate.

4. **Push safely**
   - Ensure the tree contains only intended changes.
   - Run `git fetch origin main`, then compare `origin/main` with the expected base before `git push origin main`.
   - If `origin/main` moved, stop and review `origin/main..HEAD` before pushing.
   - After pushing, find the new main run with `gh run list --branch main --limit 5` and watch it with `gh run watch <id> --exit-status`.

5. **Public follow-up**
   - Only post fixed/closed replies after the relevant GitHub Actions run on `main` is green.
   - Confirm each item identity again before posting.
   - Match the opener's language when it is Chinese or English; use English for Japanese/Korean unless the maintainer says otherwise.
   - Start with `@login`, one short thanks, the concrete fix or reason, and the next release/nightly/verification step.
   - Close issues with `--reason completed` when fixed on `main`. Close PRs without merging only when the fix is already covered on `main`, the direction is no longer needed, unsafe, duplicate, or explicitly rejected.
   - If a contributor PR's accepted fix is landed through a maintainer commit, mention the landed commit and co-author credit in the PR comment.

## Final Report

Report the pushed branch and commit, the CI run URL and conclusion, each issue/PR decision, and whether any open issues or PRs remain.
