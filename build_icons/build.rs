const CONSTANTS_FILE: &str = "shipped_icons.txt";

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    std::fs::write(
        std::path::Path::new(&out_dir).join(CONSTANTS_FILE),
        std::path::Path::new(&manifest_path)
            .join("icons")
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap(),
    )
    .unwrap();
}
