# spotify-linux

A Linux-native Spotify client, built for my taste rather than Spotify's design language.

**Status: scaffold.** The architecture is in place and it compiles; features come next. See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

[![CI](https://github.com/ibrhr/spotify-linux/actions/workflows/ci.yml/badge.svg)](https://github.com/ibrhr/spotify-linux/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Stack

| Layer | Choice | Why |
|---|---|---|
| Language | Rust | Reliability, and the ecosystem for this exact problem exists |
| UI | GTK4 + libadwaita via [Relm4](https://relm4.org) | Native GNOME look & feel, Elm-style components |
| Metadata / library / playlists / search | [rspotify](https://github.com/ramsayleung/rspotify) (Spotify Web API) | Mature Web API wrapper; all auth flows |
| Audio playback | [librespot](https://github.com/librespot-org/librespot) | Open-source Spotify client library; streams directly and registers as a Spotify Connect device |
| Desktop integration | MPRIS (D-Bus) | Media keys, GNOME media controls, `playerctl` |
| Secrets | system keyring (Secret Service) | Refresh tokens never touch disk in plaintext |

> **Premium required.** librespot (and Spotify's playback endpoints) only work with paid
> accounts. That is a Spotify restriction, not a project choice.

## Requirements

- Rust 1.88+ (MSRV; any current stable works)
- GTK4 (≥ 4.12) and libadwaita dev packages

  ```sh
  # Fedora
  sudo dnf install gtk4-devel libadwaita-devel
  # Debian/Ubuntu
  sudo apt install libgtk-4-dev libadwaita-1-dev
  ```

- For the `playback` feature (local audio via librespot), also:

  ```sh
  # Fedora
  sudo dnf install alsa-lib-devel
  # Debian/Ubuntu
  sudo apt install libasound2-dev
  ```

  Until then, the default build excludes playback: `cargo build` works, `cargo build --features playback` needs ALSA.

## Spotify app registration (one-time)

1. Create an app at <https://developer.spotify.com/dashboard>.
2. Add the redirect URI: `http://127.0.0.1:8899/login` (loopback `127.0.0.1` is required; `localhost` is rejected).
3. In *User Management*, add your own account — development-mode apps can only be used by allow-listed users (≤ 25).
4. Export the client id when running:

   ```sh
   export SPOTIFY_CLIENT_ID=your_client_id
   ```

## Running

```sh
cargo run
```

Logging follows `RUST_LOG` (default `info`), e.g. `RUST_LOG=debug cargo run`.

## Contributing

Quality gates (all enforced locally via pre-commit and in CI):

- `cargo fmt` — rustfmt, standard settings
- `cargo clippy -- -D warnings` — with a curated deny-list in `[lints]` (no `unwrap`/`expect`/`panic` in app code, no stdout/stderr in the GUI)
- `cargo test` — on pre-push and in CI
- `cargo-deny` — licenses, RustSec advisories, duplicate/banned crates (see `deny.toml`)

One-time setup:

```sh
pre-commit install --install-hooks   # pre-commit + pre-push hooks
```

CI runs on every push/PR to `main` (fmt, clippy `--all-features`, tests, cargo-deny). Tagging `v*` triggers a release build with packaged artifacts.

## Roadmap

1. **Auth** — PKCE login flow + keyring token storage (`src/auth.rs`)
2. **Playback engine** — librespot session, command/event loop (`src/player/`)
3. **Now Playing bar** — transport controls, position/volume
4. **MPRIS** — desktop media integration (`src/mpris.rs`)
5. **Library & playlists** — Web API browsing pages (`src/api/`, `src/ui/`)
6. **Search** — Web API search page
7. **Queue, liked songs, artist/album drill-down**

## License

MIT — see [LICENSE](LICENSE).
