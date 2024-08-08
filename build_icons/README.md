## relm4-icons-build

This crate provides the build-time functionality to bundle icons into the binary. It is used in the `build.rs` file of the project that uses `relm4-icons`.

### Usage

Add this to your `Cargo.toml`:

```toml
[build-dependencies]
relm4-icons-build = { version = "0.8.0" }
```

And in your `build.rs` file, use `relm4-icons-build` to bundle the icons and include them in the compiled binary:

```rust
use relm4_icons_build::Config;

fn main() {
    let config = Config {
        icons: Some(
            vec![
                "ssd",
                "size-horizontally",
                "cross",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        ),
        app_id: Some("com.my.app".to_string()),
        base_resource_path: Some("icons".to_string()),
        icons_folder: None,
    };

    relm4_icons_build::bundle_icons(config);
}
```

And in your `main.rs` or `lib.rs` file, create a module named `icon_names`:

```rust
mod icon_names {
    include!(concat!(env!("OUT_DIR"), "/icon_names.rs"));
}
```
