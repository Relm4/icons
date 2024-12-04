use std::{collections::HashMap, path::Path};

use crate::IconInfo;

const BASE_ICON_PATH: &str = "../build_icons/icons";
const FILE_ENDING: &str = "-symbolic.svg";

pub fn get_icons(sources: &[&str]) -> Vec<IconInfo> {
    let mut list = HashMap::new();

    for source in sources {
        get_icon_list(source, &mut list);
    }

    let mut list: Vec<IconInfo> = list.into_values().collect();
    list.sort_unstable_by_key(|icon| icon.name.clone());
    list
}

fn get_icon_list(path: &str, list: &mut HashMap<String, IconInfo>) {
    let base_icon_path = Path::new(BASE_ICON_PATH);
    let read_dir = std::fs::read_dir(base_icon_path.join(path)).unwrap();

    for entry in read_dir {
        let entry = entry.unwrap();

        if entry.file_type().unwrap().is_file() {
            let file_name = entry.file_name().to_str().unwrap().to_owned();
            assert!(
                file_name.ends_with(FILE_ENDING),
                "Expected only symbolic SVG files, but found {file_name}"
            );

            let mut file_name = file_name.trim_end_matches(FILE_ENDING).to_owned();
            if list.contains_key(&file_name) {
                eprintln!("Name `{file_name}` already exists in icon list.");
                file_name.push_str("-alt");
                eprintln!("Using `{file_name}` instead.");
                eprintln!();
            }
            list.insert(
                file_name.clone(),
                IconInfo {
                    name: file_name,
                    path: entry.path(),
                    source: path.to_owned(),
                },
            );
        }
    }
}
