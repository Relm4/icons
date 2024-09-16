//! More than 3000 icons, ready for use in your app!
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
#![allow(clippy::negative_feature_names, clippy::multiple_crate_versions)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use gtk::{
    gdk,
    gio::{resources_register, Resource},
    glib,
};

/// Initialized the icons and registers them globally for your application.
pub fn initialize_icons(gresource_bytes: &'static [u8]) {
    let bytes = glib::Bytes::from(gresource_bytes);
    let resource = Resource::from_data(&bytes).unwrap();
    resources_register(&resource);

    gtk::init().unwrap();

    let display = gdk::Display::default().unwrap();
    let theme = gtk::IconTheme::for_display(&display);
    theme.add_resource_path("/org/gtkrs/icons/");
    theme.add_resource_path("/org/gtkrs/icons/scalable/actions/");
}
