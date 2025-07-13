# Relm4 icons - Icons for your gtk-rs and Relm4 applications!

[![CI](https://github.com/Relm4/icons/actions/workflows/rust.yml/badge.svg)](https://github.com/Relm4/icons/actions/workflows/rust.yml)
[![Matrix](https://img.shields.io/matrix/relm4:matrix.org?label=matrix%20chat)](https://matrix.to/#/#relm4:matrix.org)
[![Relm4 icons on crates.io](https://img.shields.io/crates/v/relm4-icons.svg)](https://crates.io/crates/relm4-icons)
[![Relm4 icons docs](https://img.shields.io/badge/rust-documentation-blue)](https://docs.rs/relm4_icons/)
![Minimum Rust version 1.75](https://img.shields.io/badge/rustc-1.65+-06a096.svg)
[![dependency status](https://deps.rs/repo/github/Relm4/icons/status.svg)](https://deps.rs/repo/github/Relm4/icons)

More than 3000 icons, ready for use in your app!

## Installing librsvg

The `relm4-icons` crate relies on `librsvg` for rendering SVG icons. Ensure that `librsvg` is installed on your system before using this crate. The installation process varies depending on your operating system.

```bash
# Linux (Debian/Ubuntu)
sudo apt update
sudo apt install librsvg2-dev

# Macos
brew install librsvg

# Windows
gvsbuild build librsvg
```

## Usage

### 1. Find your icons 🔍

You can either use the of the over 3000 shipped icons or your own icons.
You can browse the shipped icons using one of the following methods:

For icons from the GNOME [icon-development-kit](https://gitlab.gnome.org/Teams/Design/icon-development-kit):
+ Install [Icon library](https://flathub.org/apps/details/org.gnome.design.IconLibrary)
+ Browse the [GNOME icon resource](https://teams.pages.gitlab.gnome.org/Design/icon-development-kit-www/)

For icons from [Fluent UI System Icons](https://github.com/microsoft/fluentui-system-icons):
+ Browse the [fluent icon library catalog](https://aka.ms/fluentui-system-icons)

For browsing all shipped icons:
+ Use the icon previews provided by in this repo: [16x16](icons16.md), [32x32](icons32.md) and [64x64](icons64.md)

> Sometimes, icons-development-kit and Fluent UI System Icons have overlapping icon names, so the postfix "-alt" is added.

### 2. Add Relm4 icons ✍

```toml
relm4-icons = "0.10.0-beta.1"

[build-dependencies]
relm4-icons-build = "0.10.0-beta.1"
```

### 3. Add the icons to your project 📦

Add the following to your `build.rs`:

```rust
fn main() {
    relm4_icons_build::bundle_icons(
        // Name of the file that will be generated at `OUT_DIR`
        "icon_names.rs",
        // Optional app ID
        Some("com.example.myapp"),
        // Custom base resource path:
        // * defaults to `/com/example/myapp` in this case if not specified explicitly
        // * or `/org/relm4` if app ID was not specified either
        None::<&str>,
        // Directory with custom icons (if any)
        None::<&str>,
        // List of icons to include
        [
            "ssd",
            "size-horizontally",
            "cross",
        ],
        // prefix for the bundled icons' identifiers
        None::<&str>,
    );
}
```

### 4. Load the icons 🛫

Add this to your initialization code:

```rust
mod icon_names {
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}

fn main() {
    ///...///
    relm4_icons::initialize_icons(icon_names::GRESOURCE_BYTES, icon_names::RESOURCE_PREFIX);
}
```

### 5. Use the icons 🎉

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

You can also use the `icon_names` module for extra compile-time generated icon names.

```rust
use crate::icon_names;

let button = gtk::Button::default();
button.set_icon_name(icon_names::PLUS);
```

## How it works

### Crate

1. Collect all icons specified in the config file
2. Build a gresource bundle containing *only the selected icons*
3. Include the gresource file in the compiled binary
4. On initialization load the gresource file

### Add new icons

To add new icons, move them into the `icons` folder and make sure their file name ends with `-symbolic.svg`.
Then run the following commands:

```sh
cd update_icons
cargo run
```

Existing icon sets can be updated as submodules with following commands:
```sh
git submodule update --remote --checkout
rm -rf build_icons/icons/fluentui-system-icons
rm -rf build_icons/icons/icon-development-kit
cd update_icons
cargo run --bin fetch_icons
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
