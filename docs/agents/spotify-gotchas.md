# Spotify-specific gotchas

## OAuth & credentials

- Redirect URI must be exactly `http://127.0.0.1:8899/login` — Spotify **rejects `localhost`**, only loopback IPs work.
  - Port lives in `config.rs::OAUTH_REDIRECT_PORT`. Changing it also requires updating the redirect URI registered in the Spotify developer dashboard.
- Client ID comes from the `SPOTIFY_CLIENT_ID` env var (app must be registered at https://developer.spotify.com/dashboard — setup steps are in the README).
- No client secret: OAuth 2.0 Authorization Code with PKCE.

## Dependency traps

- `rspotify` 0.16 is in maintenance mode. If an endpoint drifts (note the Feb 2026 Web API changes: `/tracks` → `/items` renames, consolidated `PUT /me/library` endpoints), call `reqwest` directly for just that endpoint rather than forking rspotify.
- `deny.toml` pins advisory ignores (quick-xml `RUSTSEC-2026-0194`/`-0195`, rsa `RUSTSEC-2023-0071`) that are unfixable transitive deps of librespot. **Revisit on every librespot version bump** and remove ignores that no longer apply.
