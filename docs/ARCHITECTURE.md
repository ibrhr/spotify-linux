# Architecture

## The two-API split (core design decision)

Spotify's surface is really two services, and this project uses both where each is strongest:

1. **Web API** (`src/api/`, via `rspotify`) — metadata, library, playlists, search, profile.
   No audio. Auth: OAuth 2.0 Authorization Code **with PKCE** (desktop-app friendly, no client secret).
2. **librespot** (`src/player/`) — the actual Spotify protocol client. Streams audio locally,
   registers the app as a Spotify Connect device (so this player shows up in the official
   clients' device picker, and vice-versa). Requires Premium.

Attempts to play through the Web API alone would require another official client to receive
playback, which defeats the point of a native client — hence the split. Precedents: `ncspot`, `Spot`.

## Module map

```
src/
├── main.rs        Entry point: logging, dirs, RelmApp bootstrap
├── app.rs         Root Relm4 component; global state, navigation, message routing
├── config.rs      App id, XDG paths, env config (SPOTIFY_CLIENT_ID, OAuth port)
├── models.rs      Domain types (Track, Album, Artist, Playlist, NowPlaying, PlaybackState)
├── auth.rs        OAuth PKCE flow, refresh-token storage in system keyring
├── api/           Web API layer: rspotify wrapper -> domain models
├── player/        Playback engine: librespot session behind a command/event channel
├── mpris.rs       MPRIS D-Bus bridge (media keys, GNOME media controls)
└── ui/            Future: sidebar, pages, now-playing bar (Relm4 components)
```

## Data flow

```
            commands (mpsc)                  events (watch/mpsc)
 UI (Relm4) ──────────────► player task (librespot Session+Player) ────────► UI / MPRIS
     │                                                                        ▲
     │  async calls                       PlayerEvent                         │
     ├──────────────► api/ (rspotify, Tokio runtime) ──► models.rs ────────────┘
     │
     └─ auth.rs: PKCE login + keyring storage ─► tokens used by both api/ and player/
```

- **UI never touches librespot or rspotify types directly.** `api/` converts into `models.rs`
  domain types; the player exposes `PlaybackCommand` / `PlayerEvent` enums. Boundaries are
  message-passing, which fits Relm4's Elm-style `Input`/`Output` model.
- The player runs in a dedicated Tokio task (librespot is callback-heavy internally); the root
  `App` component subscribes to `PlayerEvent` and fans state out to UI widgets and MPRIS.

## Key decisions & constraints

- **librespot as a library, not a daemon.** Unlike Spotifyd/raspotify, we link librespot
  directly (like `ncspot`/`Spot`) so playback, queue, and Connect state live in-process.
- **Audio backend:** rodio (default) → ALSA/PipeWire via cpal. Kept behind a cargo feature so
  PulseAudio/GStreamer backends can be swapped later.
- **Auth flow:** PKCE only. Spotify now rejects `localhost` redirect URIs — use
  `http://127.0.0.1:8899/login`. Refresh token goes in the system keyring (`keyring` crate).
- **Dev-mode quota:** until Spotify grants a quota extension, only ≤ 25 allow-listed users
  (added in the dashboard's User Management) can use the app. Fine for a personal client.
- **Feb 2026 Web API changes:** playlist endpoints renamed `/tracks` → `/items`, and
  save/follow endpoints consolidated into generic `PUT /me/library` URI-based calls.
  rspotify 0.16 is current but in maintenance mode — if an endpoint drifts, we can drop to
  raw `reqwest` calls for just that endpoint rather than forking.
- **Caching:** librespot gets an audio cache dir (`~/.cache/spotify-linux`); album art gets its
  own cache. Library snapshots cached in SQLite later, if needed.

## Relm4 component tree (planned)

```
App (root)
├── Sidebar (Library / Search / Playlists / Queue)
├── Pages
│   ├── LibraryPage
│   ├── PlaylistsPage → PlaylistDetail
│   ├── SearchPage
│   └── ArtistPage / AlbumPage
└── NowPlayingBar
    ├── TrackInfo (art, title, artists)
    ├── Transport (prev/play/next, seek bar, shuffle/repeat)
    └── VolumeControl
```

Each is an independent Relm4 `Component`; cross-component traffic (play this track, player
state changed) routes through `App`'s `Input` messages or shared `PlayerHandle`s.

## Milestones

1. **M1 — Auth:** PKCE flow, keyring storage, session bootstrap. (`auth.rs`)
2. **M2 — Playback:** librespot session + command/event loop; play a hardcoded URI.
   (`player/`)
3. **M3 — Now Playing bar:** UI wired to player events; MPRIS alongside.
4. **M4 — Library:** playlists + liked songs via Web API.
5. **M5 — Search & drill-down pages.**
6. **M6 — Polish:** Connect device naming, audio settings, packaging (.desktop, icon).
