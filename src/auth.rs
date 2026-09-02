//! Authentication: OAuth 2.0 Authorization Code with PKCE + token storage.
//!
//! Plan:
//! 1. Read `SPOTIFY_CLIENT_ID` (see [`crate::config::client_id`]).
//! 2. Run rspotify's `AuthCodePkceSpotify` prompt flow with the loopback
//!    redirect `http://127.0.0.1:<OAUTH_REDIRECT_PORT>/login` (registered in
//!    the Spotify dashboard). Spotify requires `127.0.0.1` loopback URIs --
//!    plain `localhost` is rejected.
//! 3. Persist the refresh token in the system keyring (`keyring` crate,
//!    Secret Service on GNOME/KDE). Access tokens are refreshed in-band.
//!
//! Note: apps in "development mode" are limited to users explicitly
//! allow-listed in the dashboard (up to 25) until a quota extension is
//! granted. Add your own account to the app's user list before testing.
//!
//! TODO: implement the flow + storage below.

use crate::config::{OAUTH_REDIRECT_PORT, client_id};

/// Loopback redirect URI registered in the Spotify dashboard.
pub fn redirect_uri() -> String {
    format!("http://127.0.0.1:{OAUTH_REDIRECT_PORT}/login")
}

/// Scopes we ask for. Playback itself goes through librespot, so we only
/// need metadata/library scopes; extras can be added as features land.
pub const SCOPES: &[&str] = &[
    "user-read-private",
    "user-read-email",
    "user-library-read",
    "user-library-modify",
    "playlist-read-private",
    "playlist-read-collaborative",
    "playlist-modify-private",
    "playlist-modify-public",
    "user-top-read",
    "user-follow-read",
    "user-follow-modify",
];

/// Errors surfaced from the auth flow.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("SPOTIFY_CLIENT_ID is not configured")]
    MissingClientId,
    #[error("login was cancelled or timed out")]
    Cancelled,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Ensure we have a valid session: load a stored refresh token, or start the
/// interactive login flow.
///
/// TODO: return an authenticated `rspotify::AuthCodePkceSpotify`.
pub async fn ensure_session() -> Result<(), AuthError> {
    if client_id().is_none() {
        return Err(AuthError::MissingClientId);
    }
    todo!("PKCE flow + keyring storage")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redirect_uri_is_loopback_not_localhost() {
        // Spotify rejects `localhost`; the URI must use 127.0.0.1.
        let uri = redirect_uri();
        assert!(uri.starts_with("http://127.0.0.1:"), "got {uri}");
        assert!(!uri.contains("localhost"), "got {uri}");
        assert!(uri.ends_with("/login"), "got {uri}");
    }

    #[test]
    fn scopes_are_unique_and_nonempty() {
        assert!(!SCOPES.is_empty());
        let mut seen = SCOPES.to_vec();
        seen.sort_unstable();
        seen.dedup();
        assert_eq!(seen.len(), SCOPES.len(), "duplicate scopes in SCOPES");
    }

    #[tokio::test]
    async fn missing_client_id_maps_to_missing_client_id_error() {
        // Only assert when the var is absent: the user's environment may set
        // it, in which case ensure_session would proceed to its todo!().
        if std::env::var_os("SPOTIFY_CLIENT_ID").is_none() {
            assert!(matches!(
                ensure_session().await,
                Err(AuthError::MissingClientId)
            ));
        }
    }
}
