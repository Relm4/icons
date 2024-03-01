//! More than 2500 icons, ready for use in your app!
//!
//! # Sources
//!
//! Icons are from
//!
//! + [icon-development-kit](https://gitlab.gnome.org/Teams/Design/icon-development-kit) ([CC0 license](https://gitlab.gnome.org/Teams/Design/icon-development-kit/-/blob/main/COPYING.md))
//! + [fluentui-system-icons](https://github.com/microsoft/fluentui-system-icons) ([MIT license](https://github.com/microsoft/fluentui-system-icons/blob/main/LICENSE))

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub,
    unused_qualifications,
    clippy::cargo,
    clippy::must_use_candidate
)]
#![allow(clippy::negative_feature_names)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// Module containing constants for icons names.
pub mod icon_names;

use gtk::{gdk, gio};

/// Initialized the icons and registers them globally for your application.
pub fn initialize_icons() {
    gio::resources_register_include!("resources.gresource").unwrap();

    if icon_names::APP_ID.is_empty() && icon_names::BASE_RESOURCE_PATH.is_empty() {
        gtk::init().unwrap();

        let display = gdk::Display::default().unwrap();
        let theme = gtk::IconTheme::for_display(&display);
        theme.add_resource_path("/org/gtkrs/icons/");
        theme.add_resource_path("/org/gtkrs/icons/scalable/actions/");
    }
}
