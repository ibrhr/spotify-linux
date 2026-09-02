//! App-wide constants, XDG paths, and environment-based configuration.

use std::path::PathBuf;

use anyhow::Context;

/// D-Bus / GTK application id. Keep in sync with any future .desktop file.
pub const APP_ID: &str = "io.github.spotify-linux";

/// Port for the OAuth loopback redirect (must match the dashboard-registered
/// redirect URI: `http://127.0.0.1:8899/login`).
pub const OAUTH_REDIRECT_PORT: u16 = 8899;

/// XDG data dir for this app (databases, cached credentials metadata).
pub fn data_dir() -> anyhow::Result<PathBuf> {
    Ok(dirs::data_dir()
        .context("could not resolve XDG data directory")?
        .join("spotify-linux"))
}

/// XDG cache dir (librespot audio cache, album art cache).
pub fn cache_dir() -> anyhow::Result<PathBuf> {
    Ok(dirs::cache_dir()
        .context("could not resolve XDG cache directory")?
        .join("spotify-linux"))
}

/// Create the app directories if missing.
pub fn init_dirs() -> anyhow::Result<()> {
    std::fs::create_dir_all(data_dir()?)?;
    std::fs::create_dir_all(cache_dir()?)?;
    Ok(())
}

/// Spotify app client id from the environment.
///
/// Register an app at <https://developer.spotify.com/dashboard> and export
/// `SPOTIFY_CLIENT_ID`. (No client secret needed: we use the PKCE flow.)
pub fn client_id() -> Option<String> {
    std::env::var("SPOTIFY_CLIENT_ID")
        .ok()
        .filter(|s| !s.trim().is_empty())
}
