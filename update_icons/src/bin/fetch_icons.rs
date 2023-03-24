use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

trait IconFilter {
    fn icon_name(path: &Path) -> Option<String>;
    fn alt_icon_name(name: &str) -> String {
        name.replace("-symbolic.svg", "-alt-symbolic.svg")
    }
    fn alt_icon_name2(name: &str) -> String;
    fn filter_dirs(_path: &Path) -> bool {
        true
    }
}

struct DevKitFilter;

impl IconFilter for DevKitFilter {
    fn icon_name(path: &Path) -> Option<String> {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        file_name
            .ends_with("-symbolic.svg")
            .then(|| file_name.into())
    }

    fn alt_icon_name2(name: &str) -> String {
        name.replace("-symbolic.svg", "-devkit-alt-symbolic.svg")
    }
}

struct FluentFilter;

impl IconFilter for FluentFilter {
    fn icon_name(path: &Path) -> Option<String> {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with("16_filled.svg") || file_name.ends_with("16_regular.svg") {
            let file_name = file_name.trim_start_matches("ic_fluent_");
            let file_name = file_name.replace("16_", "");
            let file_name = file_name.replace("_", "-");
            let file_name = file_name.replace(".svg", "-symbolic.svg");
            Some(file_name)
        } else {
            None
        }
    }

    fn alt_icon_name2(name: &str) -> String {
        name.replace("-symbolic.svg", "-fluent-alt-symbolic.svg")
    }

    fn filter_dirs(path: &Path) -> bool {
        let dir_name = path.file_name().unwrap().to_str().unwrap();
        const EXCLUDE_LIST: [&str; 4] = ["RTL", "LTR", "sr-cyrl", "sr-latn"];
        let is_lang_code = dir_name.len() == 2 && dir_name.chars().all(|c| c.is_ascii_lowercase());
        !is_lang_code && EXCLUDE_LIST.iter().all(|name| !dir_name.contains(name))
    }
}

fn main() {
    let mut list = HashMap::new();
    analyze_dir::<DevKitFilter>("../source/icon-development-kit/export", &mut list);
    copy_files("../icons/icon-development-kit", list);

    let mut list = HashMap::new();
    analyze_dir::<FluentFilter>("../source/fluentui-system-icons/assets", &mut list);
    copy_files("../icons/fluentui-system-icons", list);
}

fn copy_files(path: &str, list: HashMap<String, PathBuf>) {
    let path = Path::new(path);
    std::fs::create_dir(path).ok();
    for (key, value) in list.into_iter() {
        std::fs::copy(value, path.join(key)).unwrap();
    }
}

fn analyze_dir<F: IconFilter>(path: &str, list: &mut HashMap<String, PathBuf>) {
    let dir = std::fs::read_dir(path).unwrap();
    analyze_dir_recursively::<F>(dir, list);
}

fn analyze_dir_recursively<F: IconFilter>(dir: fs::ReadDir, list: &mut HashMap<String, PathBuf>) {
    for entry in dir {
        let entry = entry.unwrap();
        let file_type = entry.file_type().unwrap();
        let path = entry.path();

        if file_type.is_file() {
            if let Some(name) = F::icon_name(&path) {
                // Try normal name
                if list.contains_key(&name) {
                    // Try alternative name
                    let alt_name = F::alt_icon_name(&name);
                    if list.contains_key(&alt_name) {
                        // Try 2nd alternative name
                        let alt_name2 = F::alt_icon_name2(&name);
                        if list.contains_key(&alt_name) {
                            panic!(
                                "Triple key for icon: \n > normal: {name}, \n > alt:    {alt_name}, \n > alt2:   {alt_name2}"
                            );
                        } else {
                            list.insert(alt_name2, path);
                        }
                    } else {
                        list.insert(alt_name, path);
                    }
                } else {
                    list.insert(name, path);
                }
            }
        } else if file_type.is_dir() && F::filter_dirs(&path) {
            let dir = std::fs::read_dir(path).unwrap();
            analyze_dir_recursively::<F>(dir, list);
        }
    }
}
