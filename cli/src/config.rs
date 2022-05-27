use log::info;
use serde::Deserialize;
use std::{env, io, path::PathBuf};

const FILE_NAME: &str = ".squawk.toml";

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum ConfigError {
    LookupError(io::Error),
    ReadError(io::Error),
    ParseError(toml::de::Error),
}

impl std::convert::From<io::Error> for ConfigError {
    fn from(e: io::Error) -> Self {
        Self::ReadError(e)
    }
}

impl std::convert::From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        Self::ParseError(e)
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::LookupError(ref err) => {
                write!(f, "Error when finding configuration file: {}", err)
            }
            Self::ReadError(ref err) => write!(f, "Failed to read configuration file: {}", err),
            Self::ParseError(ref err) => write!(f, "Failed to parse configuration file: {}", err),
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub excluded_rules: Vec<String>,
}

impl Config {
    pub fn parse(custom_path: Option<PathBuf>) -> Result<Option<Self>, ConfigError> {
        let path = if let Some(path) = custom_path {
            Some(path)
        } else {
            find_by_traversing_back()?
        };

        if let Some(p) = path {
            info!("using config file path: {}", p.display());

            let file_content = std::fs::read_to_string(p)?;
            return Ok(Some(toml::from_str(&file_content)?));
        }

        info!("no config file found");
        Ok(None)
    }
}

fn recurse_directory(
    directory: PathBuf,
    file_name: &str,
) -> Result<Option<PathBuf>, std::io::Error> {
    for entry in directory.read_dir()? {
        let entry = entry?;
        if entry.file_name() == file_name {
            return Ok(Some(entry.path()));
        }
    }
    if let Some(parent) = directory.parent() {
        recurse_directory(parent.to_path_buf(), file_name)
    } else {
        Ok(None)
    }
}

fn find_by_traversing_back() -> Result<Option<PathBuf>, ConfigError> {
    recurse_directory(env::current_dir()?, FILE_NAME).map_err(|e| ConfigError::LookupError(e))
}
