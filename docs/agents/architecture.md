# Architecture

Full design lives in `docs/ARCHITECTURE.md` (source of truth). This file is the working-rule digest.

## The two-API split

- **Web API** (`src/api/`, via `rspotify`) — metadata, library, playlists, search, profile. No audio.
- **librespot** (`src/player/`, behind the `playback` cargo feature) — actual audio streaming + Spotify Connect device registration. Linked in-process, not run as a daemon.

## Boundary rule (enforced by convention, not the compiler)

UI/Relm4 components never touch `rspotify` or librespot types directly:

- `src/api/` converts everything into `models.rs` domain types
- the player exposes only `PlaybackCommand` / `PlayerEvent` message-passing
- cross-component traffic routes through the root `App` component's `Input` messages (Relm4 Elm-style), which also fans out to MPRIS
- the player runs in a dedicated Tokio task (librespot is callback-heavy internally)

## Scaffold-phase notes

- `main.rs` carries `#![allow(dead_code)]` with a TODO to remove once auth/playback/API are wired into the UI. Don't extend dead code; remove the allow when it's no longer needed.
- Module APIs are defined ahead of their call sites during this phase — check the module doc comments (`//!`) before assuming how something is wired.
