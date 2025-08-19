//! Utilities for build scripts using `relm4-icons`.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::{Path, PathBuf};

use gvdb::gresource::{BundleBuilder, FileData, PreprocessOptions};
use walkdir::WalkDir;

/// Stores data for each icon:
struct IconData {
    /// actual location on disk
    path: PathBuf,
    /// whether the icon is part of the shipped set
    is_shipped: bool,
}

/// Constants file with paths to icons.
pub mod constants {
    pub const SHIPPED_ICONS_PATH: &str =
        include_str!(concat!(env!("OUT_DIR"), "/shipped_icons.txt"));
}

const GENERAL_PREFIX: &str = "/org/relm4/icons";

/// Parse a filename into icon name.
/// - Strips `.svg`
pub fn path_to_icon_alias(path: impl AsRef<Path>) -> Option<String> {
    match path.as_ref().to_str() {
        Some(path) => {
            if path.ends_with(".svg") {
                println!("{path}");
                Some(path.trim_end_matches(".svg").to_owned())
            } else {
                println!("Found non-icon file `{path}`, ignoring");
                None
            }
        }
        None => panic!(
            "Failed to convert file path `{:?}` to string",
            path.as_ref()
        ),
    }
}

/// Bundles icons into a `.gresource` file and generates Rust constants for icon names.
///
/// - Custom icons keep their original symbolic state based on the filename.
/// - Shipped icons are always treated as symbolic internally, but their constant names do **not** get `_SYMBOLIC`.
pub fn bundle_icons<P, I, S>(
    out_file_name: &str,
    app_id: Option<&str>,
    base_resource_path: Option<&str>,
    icons_folder: Option<P>,
    icon_names: I,
) where
    P: AsRef<Path>,
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let mut icons: HashMap<String, IconData> = HashMap::new();

    // Package custom icons
    if let Some(folder) = &icons_folder {
        println!("cargo:rerun-if-changed={}", folder.as_ref().display());

        let read_dir = WalkDir::new(folder);
        for entry in read_dir {
            let entry = entry
                .expect("Couldn't open icon path specified in config (relative to the manifest)");
            if let Some(icon) = path_to_icon_alias(&entry.path()) {
                if icons
                    .insert(
                        icon.replace("/", "-").clone(),
                        IconData {
                            path: entry.path().to_path_buf(),
                            is_shipped: false,
                        },
                    )
                    .is_some()
                {
                    panic!("Icon with name `{icon}` exists twice");
                }
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
        let icon = icon.as_ref();
        let icon_path = dirs
            .iter()
            .find_map(|dir| {
                let icon_file_name = format!("{icon}-symbolic.svg");
                let icon_path = dir.join(icon_file_name);
                icon_path.exists().then_some(icon_path)
            })
            .unwrap_or_else(|| panic!("Icon with name `{icon}` does not exist"));

        if icons
            .insert(
                icon.to_string(),
                IconData {
                    path: icon_path,
                    is_shipped: true,
                },
            )
            .is_some()
        {
            panic!("Icon with name `{icon}` exists twice");
        }
    }

    let prefix = if let Some(base_resource_path) = &base_resource_path {
        format!("{}/icons", base_resource_path)
    } else if let Some(app_id) = app_id {
        format!("/{}/icons", app_id.replace('.', "/"))
    } else {
        GENERAL_PREFIX.into()
    };
    let gresource_file_name = format!("{out_file_name}.gresource");

    // Generate resource bundle
    {
        let resources = icons
            .iter()
            .map(|(icon, IconData { path, is_shipped })| {
                FileData::from_file(
                    if *is_shipped {
                        format!("{prefix}/scalable/actions/{icon}-symbolic.svg")
                    } else {
                        format!("{prefix}/scalable/actions/{icon}.svg")
                    },
                    path,
                    true,
                    &PreprocessOptions::xml_stripblanks(),
                )
                .unwrap()
            })
            .collect();

        let data = BundleBuilder::from_file_data(resources)
            .build()
            .expect("Failed to build resource bundle");

        fs::write(out_dir.join(&gresource_file_name), data).unwrap();
    }

    // Create file that contains the icon names as constants
    {
        let mut out_file = BufWriter::new(File::create(out_dir.join(out_file_name)).unwrap());

        writeln!(out_file, "#![rustfmt::skip]").unwrap();
        writeln!(
            out_file,
            "pub mod shipped {{\n\
            //! module contains shipped icons\n"
        )
        .unwrap();
        for (icon, IconData { path, is_shipped }) in &icons {
            if *is_shipped {
                let const_name = icon.to_uppercase().replace('-', "_");
                let path = path.display();
                writeln!(
                    out_file,
                    "/// Icon name of the icon `{icon}`, found at `{path}`\n\
                    pub const {const_name}: &str = \"{icon}\";"
                )
                .unwrap();
            }
        }
        writeln!(out_file, "}}\n").unwrap();

        writeln!(
            out_file,
            "pub mod custom {{\n\
            //! module contains user's custom icons\n"
        )
        .unwrap();
        let mut modules: std::collections::BTreeMap<String, Vec<(String, String)>> =
            std::collections::BTreeMap::new();
        for (icon, IconData { path, is_shipped }) in &icons {
            if !*is_shipped {
                let path_vec = path
                    .strip_prefix(&icons_folder.as_ref().unwrap())
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .split('/')
                    .collect::<Vec<_>>();

                let file_name = path_vec.last().unwrap().trim_end_matches(".svg");
                let dir_components = &path_vec[..path_vec.len() - 1];

                let const_name = file_name.to_uppercase().replace('-', "_");
                modules
                    .entry(dir_components.join("-"))
                    .or_default()
                    .push((const_name, icon.to_string()));
            }
        }
        for (module_path, constants) in &modules {
            if module_path.is_empty() {
                for (const_name, const_value) in constants {
                    writeln!(
                        out_file,
                        "pub const {}: &str = \"{}\";",
                        const_name, const_value
                    )
                    .unwrap();
                }
            } else {
                let mod_parts: Vec<&str> = module_path.split('-').collect();
                for part in mod_parts.iter() {
                    writeln!(out_file, "pub mod {} {{", part.replace('-', "_")).unwrap();
                }
                for (const_name, const_value) in constants {
                    writeln!(
                        out_file,
                        "pub const {}: &str = \"{}\";",
                        const_name, const_value
                    )
                    .unwrap();
                }
                for _ in (0..mod_parts.len()).rev() {
                    writeln!(out_file, "}}").unwrap();
                }
            }
        }
        write!(out_file, "}}\n").unwrap();
        write!(
            out_file,
            "/// GResource file contents\n\
            pub const GRESOURCE_BYTES: &[u8] = include_bytes!(\"{gresource_file_name}\");\n\
            /// Resource prefix used in generated `.gresource` file\n\
            pub const RESOURCE_PREFIX: &str = \"{prefix}\";"
        )
        .unwrap();
    }
}
