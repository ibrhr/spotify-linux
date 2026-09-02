# spotify-linux

A native Spotify client for Linux built with Rust and GTK4. Not affiliated with Spotify.

**Status:** early development. The architecture is in place; there is no usable functionality yet. See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for the design and roadmap.

A Spotify Premium account is required for playback (a limitation of Spotify's APIs, not this project).

## Building

Requires Rust 1.88+ and the GTK4/libadwaita development packages:

```sh
# Fedora
sudo dnf install gtk4-devel libadwaita-devel
# Debian/Ubuntu
sudo apt install libgtk-4-dev libadwaita-1-dev
```

The default build excludes audio playback. To enable it (librespot, ALSA backend):

```sh
sudo dnf install alsa-lib-devel        # or: sudo apt install libasound2-dev
cargo build --features playback
```

## Running

```sh
cargo run
```

Logging follows `RUST_LOG` (default `info`).

## Spotify app setup

The Web API client requires a registered application:

1. Create an app at https://developer.spotify.com/dashboard
2. Add the redirect URI `http://127.0.0.1:8899/login` (loopback IPs only; Spotify rejects `localhost`)
3. Under *User Management*, add the account(s) that will use the app (development-mode limit: 25 users)
4. Provide the client id via the environment: `export SPOTIFY_CLIENT_ID=...`

## Architecture

The project uses two Spotify APIs:

- **Web API** (via `rspotify`) for metadata, library, playlists, and search
- **librespot** (behind the `playback` feature) for actual audio streaming, registering the app as a Spotify Connect device

Design details and the module map are documented in [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Contributing

The repository enforces `cargo fmt`, `cargo clippy -D warnings`, `cargo test`, and `cargo-deny` in CI, and via pre-commit locally:

```sh
pre-commit install --install-hooks
```

`deny.toml` documents two known advisories inherited from librespot's dependency tree that cannot be fixed from this project.

## License

MIT — see [LICENSE](LICENSE).
