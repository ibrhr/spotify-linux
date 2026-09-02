//! Root application component. Owns global state (player, session, navigation)
//! and hosts the main window layout.
//!
//! Planned layout:
//! - Sidebar navigation (Library / Search / Playlists / Queue ...)
//! - Main content area (swappable pages)
//! - Bottom "now playing" bar (transport controls + track info)
//!
//! Everything below is a placeholder until the real UI lands.

use gtk::prelude::*;
use relm4::prelude::*;

/// Messages the root component understands.
#[derive(Debug)]
pub enum AppMsg {
    /// No-op placeholder; replaced as features land.
    Nop,
}

/// Root model.
pub struct App;

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::ApplicationWindow {
            set_title: Some("Spotify for Linux"),
            set_default_width: 1280,
            set_default_height: 800,

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},
                #[wrap(Some)]
                set_content = &gtk::Label {
                    set_label: "spotify-linux -- scaffold\n\nUI comes next; see docs/ARCHITECTURE.md",
                    set_valign: gtk::Align::Center,
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {
        // Routing for global messages (player events, navigation) lands here.
    }
}
