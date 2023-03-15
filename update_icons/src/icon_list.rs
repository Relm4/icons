const ICON_PATH: &str = "../icons";
const FILE_ENDING: &str = "-symbolic.svg";

pub fn get_icons() -> Vec<String> {
    let read_dir = std::fs::read_dir(ICON_PATH).unwrap();

    read_dir
        .filter_map(|info| {
            let info = info.unwrap();
            if info.file_type().unwrap().is_file() {
                let file_name = info.file_name().to_str().unwrap().to_owned();
                assert!(
                    file_name.ends_with(FILE_ENDING),
                    "Expected only symbolic SVG files"
                );
                Some(file_name.trim_end_matches(FILE_ENDING).to_owned())
            } else {
                None
            }
        })
        .collect()
}
