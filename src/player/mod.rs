//! Local playback engine built on [`librespot`].
//!
//! Architecture:
//! - A dedicated Tokio task owns the librespot `Session` + `Player`.
//! - The UI (and MPRIS) talk to it through an mpsc channel of
//!   [`PlaybackCommand`]s; state changes come back as [`PlayerEvent`]s.
//! - This keeps librespot's callback-heavy API behind a clean message API
//!   that Relm4 components can consume.
//!
//! librespot logs in with Spotify credentials and registers this app as a
//! **Spotify Connect** device, so playback here behaves like the official
//! client (and other devices can cast to it). Premium account required.
//!
//! TODO: implement the session bootstrap (credentials via stored OAuth
//! token from [`crate::auth`]), event loop, and queue management.

use crate::models::Track;

/// Commands sent into the player task.
#[derive(Debug, Clone)]
pub enum PlaybackCommand {
    /// Load and play the given track (or context at index, later).
    Load(Track),
    Play,
    Pause,
    Stop,
    /// Seek to an absolute position in seconds.
    Seek(f64),
    /// Volume in `[0.0, 1.0]`.
    SetVolume(f32),
    ToggleShuffle,
    CycleRepeat,
}

/// Events emitted by the player task.
#[derive(Debug, Clone)]
pub enum PlayerEvent {
    Started(Track),
    Paused,
    Resumed,
    Stopped,
    Position(f64),
    Volume(f32),
    EndOfTrack,
    /// Error text suitable for surfacing in the UI.
    Error(String),
}
