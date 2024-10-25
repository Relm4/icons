//! Utilities for build scripts using `relm4-icons`.

use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
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
    out_file_name: &str,
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
    let out_dir = Path::new(&out_dir);
    let mut icons: HashMap<String, PathBuf> = HashMap::new();

    if let Some(folder) = &icons_folder {
        println!("cargo:rerun-if-changed={folder}");
        let read_dir = fs::read_dir(folder)
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

    let dirs = fs::read_dir(shipped_icons_folder)
        .expect("Couldn't open folder of shipped icons")
        .map(|entry| {
            entry
                .expect("Couldn't open directories in shipped icon folder")
                .path()
        })
        .collect::<Vec<_>>();

    for icon in icon_names {
        let icon_path = dirs
            .iter()
            .find_map(|dir| {
                let icon_file_name = format!("{icon}-symbolic.svg");
                let icon_path = dir.join(icon_file_name);
                icon_path.exists().then_some(icon_path)
            })
            .unwrap_or_else(|| panic!("Icon with name `{icon}` exists twice"));

        if icons.insert(icon.to_string(), icon_path).is_some() {
            panic!("Icon with name `{icon}` exists twice");
        }
    }

    let prefix = if let Some(base_resource_path) = &base_resource_path {
        format!("{}icons/scalable/actions/", base_resource_path)
    } else if let Some(app_id) = app_id {
        format!("/{}/icons/scalable/actions/", app_id.replace('.', "/"))
    } else {
        GENERAL_PREFIX.into()
    };

    // Generate resource bundle
    {
        let resources = icons
            .iter()
            .map(|(icon, path)| {
                GResourceFileData::from_file(
                    format!("{prefix}{icon}-symbolic.svg"),
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

        fs::write(out_dir.join(TARGET_FILE), data).unwrap();
    }

    // Create file that contains the icon names as constants
    {
        let mut out_file = BufWriter::new(File::create(out_dir.join(out_file_name)).unwrap());

        for (icon, icon_path) in icons {
            let const_name = icon.to_uppercase().replace('-', "_");
            let path = icon_path.display();
            write!(
                out_file,
                "/// Icon name of the icon `{icon}`, found at `{path}`\n\
                pub const {const_name}: &str = \"{icon}\";\n"
            )
            .unwrap();
        }

        write!(
            out_file,
            "/// Resource prefix used in generated `.gresource` file\n\
            pub const RESOURCE_PREFIX: &str = \"{prefix}\";"
        )
        .unwrap();
    }
}
