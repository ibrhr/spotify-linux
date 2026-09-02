# AGENTS.md

Native Spotify client for Linux — Rust + GTK4/Relm4, librespot for playback. Package manager: cargo.

## Non-standard commands

```sh
cargo run                                  # default build: NO playback (librespot is feature-gated)
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
```

Use the exact CI invocations above everywhere (CI runs `--locked` and `-D warnings`). CI runs tests via `xvfb-run -a cargo nextest run ...` because the GTK smoke test needs a display — locally `cargo test` works (the UI test skips without a display).

## Rule that applies to every code change

`unsafe_code` forbidden; clippy denies `unwrap_used`, `expect_used`, `panic`, `dbg_macro`, `print_stdout/stderr`. Use `anyhow`/`thiserror` and `tracing`.

## Guides (read when relevant)

- `docs/agents/testing.md` — test layout, nextest/Xvfb, env-mutation ban, mocking boundaries
- `docs/agents/architecture.md` — two-API split, message-passing boundaries, scaffold-phase notes
- `docs/agents/build-and-ci.md` — build prerequisites, lockfile policy, pre-commit, releases
- `docs/agents/spotify-gotchas.md` — OAuth redirect rules, env config, rspotify/deny.toml traps
- `docs/agents/linear.md` — Linear workspace/team/project layout and MCP conventions
