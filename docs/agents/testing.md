# Testing

Stack: `cargo-nextest` in CI (process-per-test), `cargo test` locally, Xvfb for GTK in CI.

## Layout

- **Unit tests** live inline as `#[cfg(test)] mod tests` next to the code (`models.rs`, `config.rs`, `auth.rs`). Keep pure logic here — no GTK, no env mutation.
- **Integration tests** live in `tests/` (`tests/smoke.rs` boots the real root component). Possible because the crate is lib+bin (`src/lib.rs` exposes modules; `main.rs` stays thin).

## Running

```sh
cargo test --workspace --all-features --locked     # what CI does (under xvfb-run)
cargo test --workspace --locked                    # default features, fine locally
```

- The UI smoke test skips itself when no `DISPLAY`/`WAYLAND_DISPLAY` is set — it runs for real on a desktop session.
- Prefer `cargo nextest run` when installed: each test gets its own process, hence a fresh `gtk::init()`. With plain `cargo test`, **keep at most one GTK-booting test per `tests/` file** — a second `gtk::init()` in the same process panics.

## Project-specific rules

- `unsafe_code` is forbidden crate-wide, and Rust 2024 made `std::env::set_var` unsafe: **don't mutate the environment in tests.** Instead, split the logic into a pure function taking the value as a parameter (see `config.rs::normalize_client_id`) and test that.
- `clippy.toml` allows `unwrap`/`expect`/`panic`/`print!` in tests only (CI denies them in production code). Use them freely inside `#[test]` fns.
- Never test against live Spotify services or the real keyring — mock at the boundary (`src/api/` converts to `models.rs` types; `src/player/` speaks `PlaybackCommand`/`PlayerEvent`), or inject a fake. librespot/credentials paths are out of scope for tests.
- Tests that touch the filesystem should use `tempfile` (dev-dependency) for scratch XDG dirs, not the real home.

## CI

The `test` job (`.github/workflows/ci.yml`) installs `xvfb`, installs `cargo-nextest` via `taiki-e/install-action`, and runs:

```sh
xvfb-run -a cargo nextest run --workspace --all-features --locked
```

GTK4 has no headless mode; `xvfb-run` provides the display. Doctest support is not needed (none exist); if you add doctests, run `cargo test --doc` as a separate step — nextest doesn't run them.
