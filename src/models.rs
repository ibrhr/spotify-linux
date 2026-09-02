//! Domain models decoupled from both `rspotify` and `librespot` types.
//!
//! API-layer types are converted into these at the boundary so the UI and
//! player never depend on the shape of Spotify's JSON payloads.

/// A playable track.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Track {
    /// Spotify base-62 id.
    pub id: String,
    /// `spotify:track:<id>` URI, used by both the Web API and librespot.
    pub uri: String,
    pub title: String,
    pub artists: Vec<Artist>,
    pub album: Option<Album>,
    /// Duration in milliseconds.
    pub duration_ms: u64,
}

impl Track {
    /// "A, B and C" style artist listing for display.
    pub fn artist_names(&self) -> String {
        match self.artists.as_slice() {
            [] => String::new(),
            [one] => one.name.clone(),
            [first, rest @ ..] => {
                let names: Vec<&str> = rest.iter().map(|a| a.name.as_str()).collect();
                format!("{}, {}", first.name, names.join(", "))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artist {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Album {
    pub id: String,
    pub name: String,
    /// URL of the largest available cover image, if any.
    pub cover_url: Option<String>,
    /// Release year, when known.
    pub year: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub track_count: u32,
}

/// High-level playback state mirrored into the UI and MPRIS.
#[derive(Debug, Clone, PartialEq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

/// What the player is currently acting on.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NowPlaying {
    pub track: Option<Track>,
    /// Elapsed playback position in seconds (UI-side estimate).
    pub position_s: f64,
    pub volume: f32,
    pub shuffle: bool,
    pub repeat: RepeatMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RepeatMode {
    #[default]
    Off,
    Track,
    Context,
}
