# Contributing to MANDA

Thank you for your interest in MANDA! This guide covers everything you need to
build, test, and validate your changes before opening a Pull Request.

> **Good first issues:** browse [issues labeled `good first issue`](https://github.com/DCORVAX/manda/labels/good%20first%20issue)
> and ask to be assigned before starting.

## Prerequisites

- **macOS 11+** — [`scripts/build.sh`](scripts/build.sh) and the app bundle are macOS-only.
- **Rust toolchain** via [rustup](https://rustup.rs) — the pinned channel is declared in
  [`rust-toolchain.toml`](rust-toolchain.toml) (1.95.0). `rustup` installs it automatically on first `cargo` invocation.
- **Xcode Command Line Tools** — required for code signing and `PlistBuddy`.
- **LuaJIT** — required to validate the bundled `manda.lua` config: `brew install luajit`.
- **Vendored deps** — `deps/` is already tracked in git. A fresh clone builds without extra downloads.

## Setup

```bash
# Clone the repository
git clone https://github.com/DCORVAX/manda.git
cd manda

# Install Rust if it isn't already available
brew install rustup
rustup toolchain install 1.95.0

# Install required tools (cargo-nextest, cargo-watch, nightly rustfmt)
make install-tools

# Install pre-commit hook (format + test before each commit)
make install-hooks

# Optional: LuaJIT for the bundled config validation
brew install luajit
```

## Development

| Command | Purpose |
|---------|---------|
| `make fmt` | Auto-format code (requires nightly) |
| `make fmt-check` | Check formatting without modifying files |
| `make check` | Compile check, catch type/syntax errors |
| `make test` | Run unit tests (cargo-nextest) |
| `make dev` | Fast local debug: build `manda-gui` and run from `target/debug` |
| `make build` | Compile binaries (no app bundle) |
| `make app` | Build debug app bundle → `dist/Manda.app` |

**Recommended workflow:**

```bash
make fmt        # format first
make check      # verify it compiles
make test       # run tests
make dev        # fast local run without packaging
```

You can override the log level for `make dev`:

```bash
RUST_LOG=debug make dev
```

### Targeted cargo check

CI checks the GUI crate explicitly — run it locally before pushing:

```bash
cargo check --locked -p manda -p manda-gui
```

## Web (landing pages)

The landing page is a plain static site in [`web/`](web/index.html) (no build step).
To preview it locally:

```bash
cd web
python3 -m http.server 8000
# open http://localhost:8000
```

Both English (`web/index.html`) and Spanish (`web/es/index.html`) versions are
kept in sync — if you change a string, update both.

## Bundled Lua config validation

The bundled `manda.lua` (`assets/macos/Manda.app/Contents/Resources/manda.lua`)
is loaded by every user's shell at startup. After editing it, verify it still loads:

```bash
bash scripts/check_manda_lua_loads.sh assets/macos/Manda.app/Contents/Resources/manda.lua
```

> LuaJIT caps a chunk at 200 local variables and the top-level chunk is already
> at capacity — one new top-level `local` breaks startup while the edit itself
> looks fine. See "Bundled manda.lua Pitfalls" in `config/AGENTS.md`.

## Build Release

```bash
# Build application and DMG (release, universal binary)
./scripts/build.sh
# Outputs: dist/Manda.app and dist/MANDA.dmg

# Build for current architecture only (faster, for local testing)
./scripts/build.sh --native-arch

# Build app bundle only (skip DMG creation)
./scripts/build.sh --native-arch --app-only

# Build and open the app automatically
./scripts/build.sh --native-arch --open
```

## Pull Requests

1. Fork the repository and create a branch from `main`.
2. Make your changes — one logical change per PR is preferred.
3. Run the full validation suite before pushing:

```bash
make fmt && make fmt-check && make check && make test
cargo check --locked -p manda -p manda-gui
```

4. If you touched the bundled config: `bash scripts/check_manda_lua_loads.sh assets/macos/Manda.app/Contents/Resources/manda.lua`
5. If you touched the web: preview it locally with `cd web && python3 -m http.server 8000`.
6. Commit and push to your fork.
7. Open a PR targeting `main` and fill in the PR checklist below.

### PR checklist

- [ ] `make fmt` and `make fmt-check` pass
- [ ] `make check` passes
- [ ] `make test` passes
- [ ] `cargo check --locked -p manda -p manda-gui` passes
- [ ] Web changes previewed locally (`cd web && python3 -m http.server 8000`) — both EN and ES
- [ ] Bundled `manda.lua` validated with `scripts/check_manda_lua_loads.sh` (if touched)
- [ ] No references to removed features or stale version numbers
- [ ] Changes documented in README/docs if user-facing

## CI pipeline

CI runs, in order: format check → unit tests → cargo check → Clippy → smoke
scripts → config/release checks → universal build validation. Anything green on
CI locally via the commands above will be green on the pull request.
