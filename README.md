# Relm4 icons - Icons for your Relm4 and gtk-rs applications!

More than 800 icons, ready for use in your app!

## Usage

### 1. Find your icons 🔍

Use one of the following methods:

+ Install [Icon library](https://flathub.org/apps/details/org.gnome.design.IconLibrary)
+ Browse the [GNOME icon resource](https://teams.pages.gitlab.gnome.org/Design/icon-development-kit-www/)
+ Use the icon previews provided by in this repo: [16x16](icons16.md), [32x32](icons32.md) and [64x64](icons64.md)

### 2. Get relm4-icons ✍

For each icon you want to use, you can add a feature flag.
Only the icons you specify will be compiled into your application.

```toml
relm4-icons = { version = "0.6.0-beta.1", features = ["<icon1>", "<icon2>", "<icon3>..."] }
```

#### Example

To enable the `plus` and `minus` icons use:

```toml
relm4-icons = { version = "0.6.0-beta.1", features = ["plus", "minus"] }
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