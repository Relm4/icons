# Relm4 icons - Icons for your Relm4 and gtk-rs applications!

[![CI](https://github.com/Relm4/icons/actions/workflows/rust.yml/badge.svg)](https://github.com/Relm4/icons/actions/workflows/rust.yml)
[![Matrix](https://img.shields.io/matrix/relm4:matrix.org?label=matrix%20chat)](https://matrix.to/#/#relm4:matrix.org)
[![Relm4 icons on crates.io](https://img.shields.io/crates/v/relm4-icons.svg)](https://crates.io/crates/relm4-icons)
[![Relm4 icons docs](https://img.shields.io/badge/rust-documentation-blue)](https://docs.rs/relm4_icons/)
![Minimum Rust version 1.65](https://img.shields.io/badge/rustc-1.65+-06a096.svg)
[![dependency status](https://deps.rs/repo/github/Relm4/icons/status.svg)](https://deps.rs/repo/github/Relm4/icons)

More than 2500 icons, ready for use in your app!

## Usage

### 1. Find your icons 🔍

Use one of the following methods:

For icons from the GNOME [icon-development-kit](https://gitlab.gnome.org/Teams/Design/icon-development-kit):
+ Install [Icon library](https://flathub.org/apps/details/org.gnome.design.IconLibrary)
+ Browse the [GNOME icon resource](https://teams.pages.gitlab.gnome.org/Design/icon-development-kit-www/)

For icons from [Fluent UI System Icons](https://github.com/microsoft/fluentui-system-icons):
+ Browse the [fluent icon library catalog](https://aka.ms/fluentui-system-icons)

For browsing all icons:
+ Use the icon previews provided by in this repo: [16x16](icons16.md), [32x32](icons32.md) and [64x64](icons64.md)
+ Search the [Rust documentation](https://docs.rs/relm4_icons/) which also includes icon previews

### 2. Get relm4-icons ✍

For each icon you want to use, you can add a feature flag.
Only the icons you specify will be compiled into your application.

```toml
relm4-icons = { version = "0.6.0-beta.6", features = ["<icon1>", "<icon2>", "<icon3>..."] }
```

#### Example

To enable the `plus` and `minus` icons use:

```toml
relm4-icons = { version = "0.6.0-beta.6", features = ["plus", "minus"] }
```

> The file ending `-symbolic.svg` isn't part of the icon name.

### 3. Load the icons 🛫

Add this somewhere in your initialization code (after initializing `RelmApp` or GTK).

```rust
relm4_icons::initialize_icons();
```

### 4. Use the icons 🎉

Use `set_icon_name` and similar methods to use your icons, for example with
[`ButtonExt`](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/prelude/trait.ButtonExt.html#tymethod.set_icon_name),
[`StackPage`](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/struct.StackPage.html#method.set_icon_name),
[`MenuButton`](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/struct.MenuButton.html#method.set_icon_name) or
[`Image`](https://gtk-rs.org/gtk4-rs/git/docs/gtk4/struct.Image.html#method.set_icon_name).

#### Example

```rust
let button = gtk::Button::default();
button.set_icon_name("plus");
```

You can also use the `icon_name` module for extra compile-time checking of icon names.

```rust
use relm4_icons::icon_name;

let button = gtk::Button::default();
button.set_icon_name(icon_name::PLUS);
```

## How it works

### Codegen

1. Find all icons in the `icons` folder
2. Generate a feature flag and a conditional constant for each icon

### Crate

1. Include only the selected icons in a gresource file
2. Include the gresource file in the compiled binary
3. On initialization, add the gresource file to the default icon theme

### Add new icons

To add new icons, move them into the `icons` folder and make sure their file name ends with `-symbolic.svg`.
Then run the following commands:

```sh
cd update_icons
cargo run
```

## Credit

+ GNOME contributors for providing so many outstanding icons
+ [gvdb-rs](https://github.com/felinira/gvdb-rs) for providing a great crate for interacting with gresources in pure Rust
+ [gtk-rs](https://gtk-rs.org) for outstanding Rust bindings for GTK4

## Legal

The source code of Relm4 icons is licensed under the terms of both the MIT license and the Apache License (Version 2.0).

### Icons

+ The icons in the `icons/icon-development-kit` folder are licensed under the terms of the [CC0 license](https://creativecommons.org/share-your-work/public-domain/cc0/) and therefore public domain.
+ The icons in the `icons/fluentui-system-icons` folder are licensed under the terms of the [MIT license](https://opensource.org/license/MIT/).

Both licenses should work for both open source and proprietary applications (without warranty).
