use lazy_static::lazy_static;
use log::info;
use serde::Deserialize;
use std::{fs, path::Path};

const FILE_NAME: &str = ".squawk.toml";
lazy_static! {
    static ref STATIC_SEARCH_PATHS: Vec<String> =
        vec![format!("./{}", FILE_NAME), format!("~/{}", FILE_NAME)];
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub excluded_rules: Option<Vec<String>>,
}

impl Config {
    pub fn parse(custom_path: Option<String>) -> Option<Self> {
        let path = custom_path.or_else(Self::find_path);

        if let Some(p) = path {
            info!("config file path: {}", &p);
            if Path::new(&p).exists() {
                if let Ok(file_content) = std::fs::read_to_string(p) {
                    return toml::from_str(&file_content).ok();
                }
            }
        }

        info!("no config file found");
        None
    }

    fn find_path() -> Option<String> {
        let static_path = STATIC_SEARCH_PATHS
            .iter()
            .find(|p| Path::new(p).exists())
            .map(String::to_string);

        // Config by traversing back takes priority after the local directory config
        match (&static_path, find_by_traversing_back()) {
            (Some(sp), Some(alt_path)) => {
                if sp == &STATIC_SEARCH_PATHS[0] {
                    static_path
                } else {
                    Some(alt_path)
                }
            }
            (None, Some(alt_path)) => Some(alt_path),
            _ => static_path,
        }
    }
}

fn find_by_traversing_back() -> Option<String> {
    for depth in 1..6 {
        let dir = "../".repeat(depth);
        let expected_file = format!("{}{}", dir, FILE_NAME);

        match fs::read_dir(dir) {
            Ok(dir_entries) => {
                for f in dir_entries.flatten() {
                    if f.path().display().to_string() == expected_file {
                        return Some(expected_file);
                    }
                }
            }
            _ => return None,
        }
    }

    None
}
