# Build & CI

## Commands (CI-exact — use these everywhere)

```sh
cargo run                                  # default build: NO playback (librespot is feature-gated)
cargo build --features playback            # local playback via librespot/rodio
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
cargo deny check --all-features            # licenses/advisories gate in CI
```

## Build prerequisites

- **GTK4 + libadwaita dev packages are required for every build**, not just playback:
  - Fedora: `sudo dnf install gtk4-devel libadwaita-devel`
  - Debian/Ubuntu: `sudo apt install libgtk-4-dev libadwaita-1-dev pkg-config`
- The `playback` feature additionally needs ALSA headers: `alsa-lib-devel` (Fedora) / `libasound2-dev` (Debian/Ubuntu).

## Lockfile policy

CI always passes `--locked`. If you change `Cargo.toml`, commit the updated `Cargo.lock` in the same change or CI fails.

## Pre-commit hooks

`pre-commit install --install-hooks` enables them. On commit: fmt + clippy + hygiene checks. On push: `cargo test --workspace --locked`. Note pre-commit runs clippy with default features only — the CI-exact command above is what you must run yourself.

## CI test runner

The `test` job installs `xvfb` + `cargo-nextest` and runs `xvfb-run -a cargo nextest run --workspace --all-features --locked` (GTK4 needs a display; nextest gives each test its own process and a fresh `gtk::init()`). See `docs/agents/testing.md`.

## Releases

Tag-driven (`.github/workflows/release.yml`): pushing a `v*` tag builds `--release --all-features --locked` and publishes a GitHub release with a tarball + sha256. No manual release steps.
