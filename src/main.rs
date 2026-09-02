//! spotify-linux -- a Linux-native Spotify client.
//!
//! Entry point: initializes logging, ensures app directories exist,
//! and hands off to the root Relm4 component ([`app::App`]).

// Scaffold phase: module APIs are defined ahead of their call sites.
// TODO: remove once auth/playback/API layers are wired into the UI.
#![allow(dead_code)]

mod api;
mod app;
mod auth;
mod config;
mod models;
mod mpris;
mod player;

use relm4::RelmApp;

fn main() {
    // Logging: honors RUST_LOG (e.g. RUST_LOG=debug cargo run).
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    if let Err(err) = config::init_dirs() {
        tracing::warn!("could not create app directories: {err:#}");
    }

    if config::client_id().is_none() {
        tracing::warn!(
            "SPOTIFY_CLIENT_ID is not set -- create an app at \
             https://developer.spotify.com/dashboard and set it to enable the Web API"
        );
    }

    tracing::info!("starting {}", config::APP_ID);

    let app = RelmApp::new(config::APP_ID);
    app.run::<app::App>(());
}
