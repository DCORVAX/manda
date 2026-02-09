# Contributing to Kaku

## Setup

```bash
# Clone the repository
git clone https://github.com/tw93/Kaku.git
cd Kaku
```

## Development

```bash
# Verify code
cargo check

# Run tests
cargo test
```

## Build Release

```bash
# Build application and DMG
./scripts/build.sh
# Outputs: dist/Kaku.app and dist/Kaku.dmg
```

## Pull Requests

> **Important:** Please submit PRs to the `dev` branch, not `main`. We merge `dev` into `main` after testing.

1. Fork and create branch from `dev`
2. Make changes
3. Run checks: `cargo check && cargo test`
4. Commit and push
5. Open PR targeting `dev`

CI will verify formatting, linting, and tests.
