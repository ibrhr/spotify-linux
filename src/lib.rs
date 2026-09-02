//! spotify-linux -- a Linux-native Spotify client.
//!
//! Library crate holding all app modules so integration tests (and the
//! binary) can exercise them. The binary in `main.rs` stays thin.

// Scaffold phase: module APIs are defined ahead of their call sites.
// TODO: remove once auth/playback/API layers are wired into the UI.
#![allow(dead_code)]

pub mod api;
pub mod app;
pub mod auth;
pub mod config;
pub mod models;
pub mod mpris;
pub mod player;
