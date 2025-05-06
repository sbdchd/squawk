use anyhow::{Context, Result};
use log::info;
use serde::Deserialize;
use squawk_linter::{Rule, Version};
use std::{env, path::Path, path::PathBuf};

const FILE_NAME: &str = ".squawk.toml";

#[derive(Debug, Default, Deserialize)]
pub struct UploadToGitHubConfig {
    #[serde(default)]
    pub fail_on_violations: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub excluded_paths: Vec<String>,
    #[serde(default)]
    pub excluded_rules: Vec<Rule>,
    #[serde(default)]
    pub pg_version: Option<Version>,
    #[serde(default)]
    pub assume_in_transaction: Option<bool>,
    #[serde(default)]
    pub upload_to_github: UploadToGitHubConfig,
}

impl Config {
    pub fn parse(custom_path: Option<PathBuf>) -> Result<Option<Self>> {
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

fn recurse_directory(directory: &Path, file_name: &str) -> Result<Option<PathBuf>, std::io::Error> {
    for entry in directory.read_dir()? {
        let entry = entry?;
        if entry.file_name() == file_name {
            return Ok(Some(entry.path()));
        }
    }
    if let Some(parent) = directory.parent() {
        recurse_directory(parent, file_name)
    } else {
        Ok(None)
    }
}

fn find_by_traversing_back() -> Result<Option<PathBuf>> {
    recurse_directory(&env::current_dir()?, FILE_NAME)
        .context("Error when finding configuration file")
}

#[cfg(test)]
mod test_config {
    use std::fs;
    use tempfile::NamedTempFile;

    use insta::assert_debug_snapshot;

    use super::*;

    #[test]
    fn load_cfg_full() {
        let squawk_toml = NamedTempFile::new().expect("generate tempFile");
        let file = r#"
pg_version = "19.1"
excluded_paths = ["example.sql"]
excluded_rules = ["require-concurrent-index-creation"]
assume_in_transaction = true
        
        "#;
        fs::write(&squawk_toml, file).expect("Unable to write file");
        assert_debug_snapshot!(Config::parse(Some(squawk_toml.path().to_path_buf())));
    }
    #[test]
    fn load_pg_version() {
        let squawk_toml = NamedTempFile::new().expect("generate tempFile");
        let file = r#"
pg_version = "19.1"
        
        "#;
        fs::write(&squawk_toml, file).expect("Unable to write file");
        assert_debug_snapshot!(Config::parse(Some(squawk_toml.path().to_path_buf())));
    }
    #[test]
    fn load_excluded_rules() {
        let squawk_toml = NamedTempFile::new().expect("generate tempFile");
        let file = r#"
excluded_rules = ["require-concurrent-index-creation"]
        
        "#;
        fs::write(&squawk_toml, file).expect("Unable to write file");
        assert_debug_snapshot!(Config::parse(Some(squawk_toml.path().to_path_buf())));
    }
    #[test]
    fn load_excluded_paths() {
        let squawk_toml = NamedTempFile::new().expect("generate tempFile");
        let file = r#"
excluded_paths = ["example.sql"]
        
        "#;
        fs::write(&squawk_toml, file).expect("Unable to write file");
        assert_debug_snapshot!(Config::parse(Some(squawk_toml.path().to_path_buf())));
    }
    #[test]
    fn load_assume_in_transaction() {
        let squawk_toml = NamedTempFile::new().expect("generate tempFile");
        let file = r"
assume_in_transaction = false
        
        ";
        fs::write(&squawk_toml, file).expect("Unable to write file");
        assert_debug_snapshot!(Config::parse(Some(squawk_toml.path().to_path_buf())));
    }
    #[test]
    fn load_fail_on_violations() {
        let squawk_toml = NamedTempFile::new().expect("generate tempFile");
        let file = r"
[upload_to_github]
fail_on_violations = true        
        ";
        fs::write(&squawk_toml, file).expect("Unable to write file");
        assert_debug_snapshot!(Config::parse(Some(squawk_toml.path().to_path_buf())));
    }
}
