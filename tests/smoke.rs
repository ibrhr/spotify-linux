//! UI smoke test: boots the real root Relm4 component and shuts it down.
//!
//! GTK4 cannot run headless -- these tests need a display. In CI the test
//! job runs under `xvfb-run -a`. Locally without a display the test skips
//! itself instead of failing.
//!
//! Prefer running the suite with `cargo nextest run`: nextest executes every
//! test in its own process, so each test gets a fresh `gtk::init()`. If you
//! run `cargo test` instead, keep at most one GTK-booting test per file --
//! `gtk::init()` panics if called twice in one process.

use std::time::Duration;

use relm4::RelmApp;
use relm4::gtk::prelude::ApplicationExt;

use spotify_linux::{app, config};

fn has_display() -> bool {
    std::env::var_os("DISPLAY").is_some() || std::env::var_os("WAYLAND_DISPLAY").is_some()
}

#[test]
fn app_boots_and_shuts_down() {
    if !has_display() {
        eprintln!("skipping: no DISPLAY/WAYLAND_DISPLAY available");
        return;
    }

    let app = RelmApp::new(config::APP_ID);

    // Quit the main loop shortly after it starts; `run` blocks until then.
    relm4::gtk::glib::timeout_add_local_once(Duration::from_millis(200), || {
        relm4::main_application().quit();
    });

    // Panics on any component init/view-build failure -- that's the point.
    app.run::<app::App>(());
}
