const CONSTANTS_FILE: &str = "constants.rs";

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    // Create file that contains the icon names as constants
    let constants = format!(
        "pub const SHIPPED_ICONS_PATH: &str = \"{}/icons\";",
        manifest_path
    );

    std::fs::write(
        std::path::Path::new(&out_dir).join(CONSTANTS_FILE),
        constants,
    )
    .unwrap();
}
