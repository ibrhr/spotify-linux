//! MPRIS D-Bus integration (org.mpris.MediaPlayer2.spotify-linux).
//!
//! Exposes the player to the desktop: GNOME media controls, media keys,
//! `playerctl`, lock screens, etc.
//!
//! Implementation will use the `mpris-server` crate:
//! - Owns the `MprisServer` on a Tokio task.
//! - Mirrors [`crate::player::PlayerEvent`]s into MPRIS properties.
//! - Translates MPRIS commands into [`crate::player::PlaybackCommand`]s.
//!
//! TODO: implement.
