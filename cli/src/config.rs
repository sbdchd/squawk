use lazy_static::lazy_static;
use log::info;
use serde::Deserialize;
use std::{char, path::Path, process::Command};

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

        // Config in git root takes priority after the local directory config
        match (&static_path, get_git_root_config_path()) {
            (Some(sp), Some(git_root_config_path)) => {
                if sp == &STATIC_SEARCH_PATHS[0] {
                    static_path
                } else {
                    Some(git_root_config_path)
                }
            }
            (None, Some(git_root_config_path)) => Some(git_root_config_path),
            _ => static_path,
        }
    }
}

fn get_git_root_config_path() -> Option<String> {
    let git_root_path = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim_end_matches(char::is_control).to_string());

    git_root_path
        .map(|grp| format!("{}/{}", grp, FILE_NAME))
        .filter(|git_root_config_path| Path::new(git_root_config_path).exists())
}
