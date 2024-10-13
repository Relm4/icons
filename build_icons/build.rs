const CONSTANTS_FILE: &str = "constants.rs";

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("Icons path: {}", manifest_path);

    // Create file that contains the icon names as constants
    let constants = format!(
        "
        /// Path to the shipped icons.
        pub const SHIPPED_ICONS_PATH: &str = \"{}\";",
        std::path::Path::new(&manifest_path)
            .join("icons")
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap()
    );

    std::fs::write(
        std::path::Path::new(&out_dir).join(CONSTANTS_FILE),
        constants,
    )
    .unwrap();
}
