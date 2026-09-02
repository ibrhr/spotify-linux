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
    normalize_client_id(std::env::var("SPOTIFY_CLIENT_ID").ok())
}

/// Pure logic behind [`client_id`], split out so tests don't need to mutate
/// the environment (`unsafe_code` is forbidden crate-wide, and Rust 2024
/// made env mutation unsafe anyway).
fn normalize_client_id(raw: Option<String>) -> Option<String> {
    raw.filter(|s| !s.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_dirs_live_under_spotify_linux() {
        let data = data_dir().unwrap();
        let cache = cache_dir().unwrap();
        assert!(
            data.ends_with("spotify-linux"),
            "unexpected data dir: {data:?}"
        );
        assert!(
            cache.ends_with("spotify-linux"),
            "unexpected cache dir: {cache:?}"
        );
    }

    #[test]
    fn redirect_port_matches_documented_redirect_uri() {
        // The dashboard must register exactly this URI; keep in sync with README.
        assert_eq!(OAUTH_REDIRECT_PORT, 8899);
        assert_eq!(crate::auth::redirect_uri(), "http://127.0.0.1:8899/login");
    }

    #[test]
    fn client_id_rejects_blank_and_missing() {
        assert_eq!(
            normalize_client_id(Some("abc".into())).as_deref(),
            Some("abc")
        );
        assert_eq!(normalize_client_id(Some("   ".into())), None);
        assert_eq!(normalize_client_id(Some("".into())), None);
        assert_eq!(normalize_client_id(None), None);
    }
}
