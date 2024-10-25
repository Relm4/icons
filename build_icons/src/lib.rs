//! Utilities for build scripts using `relm4-icons`.

use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fmt::Display;
use std::path::{Path, PathBuf};

use gvdb::gresource::{GResourceBuilder, GResourceFileData, PreprocessOptions};

/// Constants file with paths to icons.
pub mod constants {
    pub const SHIPPED_ICONS_PATH: &str =
        include_str!(concat!(env!("OUT_DIR"), "/shipped_icons.txt"));
}

const GENERAL_PREFIX: &str = "/org/relm4/icons/scalable/actions/";
const TARGET_FILE: &str = "resources.gresource";

/// Convert file name to icon name
pub fn path_to_icon_name(string: &OsStr) -> String {
    match string.to_str() {
        Some(string) => {
            if string.ends_with(".svg") {
                string
                    .trim_end_matches("-symbolic.svg")
                    .trim_end_matches(".svg")
                    .to_owned()
            } else {
                panic!("Found non-icon file `{string}`");
            }
        }
        None => panic!("Failed to convert file name `{string:?}` to string"),
    }
}

/// Bundles icons into a GResource file and generates constants for icon names.
pub fn bundle_icons<P, I, S>(
    constants_file: P,
    app_id: Option<&str>,
    base_resource_path: Option<P>,
    icons_folder: Option<P>,
    icon_names: I,
) where
    P: AsRef<Path> + Display + Clone + Default,
    I: IntoIterator<Item = S>,
    S: Into<Cow<'static, str>> + Display + Clone,
{
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut icons: HashMap<String, PathBuf> = HashMap::new();

    if let Some(folder) = &icons_folder {
        println!("cargo:rerun-if-changed={folder}");
        let read_dir = std::fs::read_dir(folder)
            .expect("Couldn't open icon path specified in config (relative to the manifest)");
        for entry in read_dir {
            let entry = entry.unwrap();
            let icon = path_to_icon_name(&entry.file_name());
            if icons.insert(icon.clone(), entry.path()).is_some() {
                panic!("Icon with name `{icon}` exists twice")
            }
        }
    }

    let shipped_icons_folder = constants::SHIPPED_ICONS_PATH;

    let dirs =
        std::fs::read_dir(shipped_icons_folder).expect("Couldn't open folder of shipped icons");
    let dirs: Vec<_> = dirs
        .map(|entry| {
            let entry = entry.expect("Couldn't open directories in shipped icon folder");
            entry.path()
        })
        .collect();

    'outer: for icon in icon_names {
        for dir in &dirs {
            let icon_file_name = format!("{icon}-symbolic.svg");
            let icon_path = dir.join(icon_file_name);
            if icon_path.exists() {
                if icons.insert(icon.to_string(), icon_path).is_some() {
                    panic!("Icon with name `{icon}` exists twice")
                }
                continue 'outer;
            }
        }
        panic!("Icon {icon} not found in shipped icons");
    }

    let prefix = if let Some(base_resource_path) = &base_resource_path {
        format!("{}icons/scalable/actions/", base_resource_path)
    } else if let Some(app_id) = app_id {
        format!("/{}/icons/scalable/actions/", app_id.replace('.', "/"))
    } else {
        GENERAL_PREFIX.into()
    };

    // Generate resource bundle
    let resources = icons
        .iter()
        .map(|(icon, path)| {
            GResourceFileData::from_file(
                [&prefix, icon, "-symbolic.svg"].into_iter().collect(),
                path,
                true,
                &PreprocessOptions::xml_stripblanks(),
            )
            .unwrap()
        })
        .collect();

    let data = GResourceBuilder::from_file_data(resources)
        .build()
        .expect("Failed to build resource bundle");

    std::fs::write(Path::new(&out_dir).join(TARGET_FILE), data).unwrap();

    // Create file that contains the icon names as constants
    let constants: String = icons
        .iter()
        .map(|(icon, icon_path)| {
            let const_name = icon.to_uppercase().replace('-', "_");
            format!(
                "
            /// Icon name of the icon `{icon}`, found at `{icon_path:?}`.
            pub const {const_name}: &str = \"{icon}\";
            "
            )
        })
        .chain([format!("pub const RESOURCE_PREFIX: &str = \"{prefix}\";")])
        .collect();

    std::fs::write(Path::new(&out_dir).join(constants_file), constants).unwrap();
}
