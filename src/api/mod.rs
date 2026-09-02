//! Spotify Web API layer (metadata, library, playlists, search).
//!
//! Backed by [`rspotify`] using the **Authorization Code with PKCE** flow
//! (no client secret required for desktop apps). Spotify's *playback
//! control* endpoints require Premium -- we don't rely on them for audio;
//! playback runs locally through [`crate::player`] (librespot).
//!
//! Design notes:
//! - All calls are async and executed on the shared Tokio runtime.
//! - Results are converted into [`crate::models`] types at this boundary.
//! - A thin cache (album art, library snapshots) will live here later.

use rspotify::AuthCodePkceSpotify;

/// Wrapper around the authenticated Web API client.
///
/// TODO: construct from [`crate::auth`] once the OAuth flow is wired up;
/// expose typed methods returning `crate::models` types.
#[derive(Default)]
pub struct Api {
    client: AuthCodePkceSpotify,
}

impl Api {
    /// Access to the underlying client for endpoints this wrapper doesn't
    /// cover yet.
    pub fn client(&self) -> &AuthCodePkceSpotify {
        &self.client
    }
}
