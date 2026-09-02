<div align="center">

# 🎧 spotify-linux 🎧

### *the Spotify client Linux DESERVED (but definitely not the one it expected)*

[![CI](https://github.com/ibrhr/spotify-linux/actions/workflows/ci.yml/badge.svg?style=for-the-badge)](https://github.com/ibrhr/spotify-linux/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](LICENSE)
[![Crates.io](https://img.shields.io/badge/crates.io-coming%20soon%E2%84%A2-orange?style=for-the-badge)](https://crates.io)
[![Rust](https://img.shields.io/badge/made%20with-Rust%20%F0%9F%A6%80-dea584?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
[![GTK4](https://img.shields.io/badge/UI-GTK4%20%2B%20libadwaita-4a86cf?style=for-the-badge&logo=gnome)](https://gtk.org)
[![Premium Required](https://img.shields.io/badge/Spotify-PREMIUM%20ONLY-1db954?style=for-the-badge&logo=spotify)](https://www.spotify.com/premium/)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=for-the-badge)](https://github.com/ibrhr/spotify-linux/pulls)
[![Status](https://img.shields.io/badge/status-vaporware%20%F0%9F%92%A4-red?style=for-the-badge)](#-roadmap--the-hype-train)
[![WIP](https://img.shields.io/badge/WIP-do%20NOT%20dogfood%20(yet)-yellow?style=for-the-badge)](#-running-the-thing)
[![Vibes](https://img.shields.io/badge/vibes-immaculate-ff69b4?style=for-the-badge)](#)

---

```
        _             _         _       _
       | |_ _   _ ___| |_ _   _| | __ _| |_ ___  _ __ ___
   _   | __| | | / __| __| | | | |/ _` | __/ _ \| '__/ _ \
  | |__| |_| |_| \__ \ |_| |_| | | (_| | || (_) | | |  __/
   \____/\__,_|___/\__|\__,_|_|\__,_|\__\___/|_|  \___|
              (but for linux this time, i promise)
```

**⬇️ scroll down, there's actual information down there ⬇️**

</div>

---

## 🚨📢 ATTENTION / TL;DR / README-INCEPTION 📢🚨

> [!WARNING]
> 🚧 **THIS PROJECT IS IN PRE-ALPHA EARLY SKELETON PHASE** 🚧
> There is no music. There is barely a window. The window says a placeholder sentence.
> And yet — the CI is *immaculate*. We have our priorities perfectly in order. 💅

> [!IMPORTANT]
> 💸 **Spotify PREMIUM required.** Yes, really. librespot and Spotify's playback
> endpoints only work with paid accounts — that's a Spotify rule, not our vibe.
> Blame Stockholm, not us. 🇸🇪

---

## 🤔 Wait, why does this exist?

Because the official Spotify Linux client is a CEF wrapper that ships a whole Chromium
with it, and because *I* wanted a client built for **MY** taste — not Spotify's Design
Language™, not a webview in a trench coat. A real, native, GTK4 app. You know. Like
it's 2009 again, but with Rust. 🦀✨

Also because every project needs a README this cluttered. It's the law. ⚖️

---

## 🧱 The Big Brain Stack 🧠

| Layer | Choice | Why (reasons we totally thought through) |
|---|---|---|
| 🦀 Language | Rust | So the compiler can yell at us instead of segfaults at runtime |
| 🪟 UI | GTK4 + libadwaita via [Relm4](https://relm4.org) | Native GNOME goodness, Elm-style components, zero Chromium |
| 🌐 Metadata / library / playlists / search | [rspotify](https://github.com/ramsayleung/rspotify) (Spotify Web API) | The mature wrapper. It's in maintenance mode and we love a survivor 🫡 |
| 🔊 Actual audio | [librespot](https://github.com/librespot-org/librespot) | Streams real Spotify audio, registers as a Spotify Connect device. Magic. 🪄 |
| ⌨️ Desktop integration | MPRIS (D-Bus) | Media keys!! GNOME media controls!! `playerctl`!! |
| 🔐 Secrets | system keyring (Secret Service) | Refresh tokens in GNOME Keyring, NOT in a plaintext dotfile like animals 🙅 |

### 🧬 The Two-API Split™ (patent pending)

Spotify is secretly TWO services wearing a trench coat:

1. **Web API** → metadata, library, playlists, search (no audio, vibes only)
2. **librespot** → the actual Spotify protocol, streams audio, shows up in your
   Spotify Connect device picker like a real boy 🤥

Playing through the Web API alone would need the *official* client to receive playback —
which defeats the entire point. So we do both. Precedents: `ncspot`, `Spot`. We stand
on the shoulders of giants and we're not even sorry. 🏔️

---

## 📦 Requirements (the toll booth 🎟️)

- 🦀 **Rust 1.88+** (MSRV; any current stable works fine, live your life)
- 🪟 **GTK4 (≥ 4.12)** and **libadwaita** dev packages:

  ```sh
  # Fedora (the superior distro, fight me) 🎩
  sudo dnf install gtk4-devel libadwaita-devel
  # Debian/Ubuntu (also fine, we don't discriminate... much) 🐧
  sudo apt install libgtk-4-dev libadwaita-1-dev
  ```

- 🔊 For the `playback` feature (librespot audio) — until then the default build
  compiles WITHOUT playback, which is perfect for a client with no audio:

  ```sh
  # Fedora
  sudo dnf install alsa-lib-devel
  # Debian/Ubuntu
  sudo apt install libasound2-dev
  ```

  then: `cargo build --features playback` 🎶 (eventually)

---

## 🪪 Spotify app registration (one-time bureaucracy 🧾)

1. 🏗️ Create an app at <https://developer.spotify.com/dashboard>
2. 🔗 Add the redirect URI: `http://127.0.0.1:8899/login`
   — Spotify **rejects** `localhost` now (yes, really, they said loopback only, we said ok fine)
3. 🧑‍🤝‍🧑 In *User Management*, add your own account — dev-mode apps only work for
   allow-listed users (≤ 25 of them). Add yourself. You're one of the 25. You made it. 🎉
4. 📤 Export the client id:

   ```sh
   export SPOTIFY_CLIENT_ID=your_client_id
   ```

---

## 🏃 Running the thing

```sh
cargo run          # 🔮 witness... a window
```

Logging follows `RUST_LOG` (default `info`):

```sh
RUST_LOG=debug cargo run   # 🔬 now with 340% more output
```

**Current feature set:** a window. It has a title. It has a header bar. It's honest work. 👨‍🌾

---

## 🗺️ Roadmap — The Hype Train 🚂💨

- [ ] 🔑 **Auth** — PKCE login flow + keyring token storage (`src/auth.rs`)
- [ ] 🔊 **Playback engine** — librespot session, command/event loop (`src/player/`)
- [ ] 🎛️ **Now Playing bar** — transport controls, position, volume (the basics, but sexy)
- [ ] ⌨️ **MPRIS** — desktop media integration (`src/mpris.rs`)
- [ ] 📚 **Library & playlists** — Web API browsing pages (`src/api/`, `src/ui/`)
- [ ] 🔍 **Search** — the Web API search page
- [ ] ⏭️ **Queue, liked songs, artist/album drill-down**

---

## 🛡️ Quality gates (we said "highest standard" and we MEANT it 😤)

Every commit gets bullied by:

| Gate | Local (pre-commit) | CI |
|---|---|---|
| 🧹 Repo hygiene (whitespace, YAML/TOML/JSON, private keys, merge markers) | ✅ pre-commit | — |
| 🎨 `cargo fmt` | ✅ | ✅ |
| 📎 `cargo clippy -D warnings` (strict lints: no unwrap/expect/panic/stdout in the GUI) | ✅ | ✅ `--all-features` |
| 🧪 `cargo test` | ✅ pre-push | ✅ |
| ⚖️ cargo-deny (licenses, RustSec advisories, duplicate crates) | manual | ✅ |

One-time setup:

```sh
pre-commit install --install-hooks   # ⛓️ chain yourself to quality
```

`cargo-deny` notes: two advisories are inherited from librespot (`quick-xml` version pin +
the unpatched `rsa` timing thing) and are documented in `deny.toml` — not fixable from here,
revisited on every librespot release. Transparency! 🪟

---

## 🤝 Contributing

It's pre-alpha, so the best contribution right now is... stars. ⭐ And vibes. ✨
But if you insist: fork → branch → let the pre-commit hooks judge you → PR.

---

## 📜 License

MIT — see [LICENSE](LICENSE). Do whatever, just don't blame us when your
pre-alpha music app plays nothing. 🤷

---

<div align="center">

**⭐ star this repo to spiritually accelerate development ⭐**

*~ built with 🦀 rust, 🪟 gtk4, and an unreasonable amount of badges ~*

</div>
