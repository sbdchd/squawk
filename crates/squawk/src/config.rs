use anyhow::{Context, Result};
use log::info;
use serde::Deserialize;
use squawk_linter::{Rule, Version};
use std::{
    env,
    io::{self, IsTerminal},
    path::{Path, PathBuf},
    process,
};

use crate::{Command, DebugOption, Opts, Reporter, UploadToGithubArgs};

const FILE_NAME: &str = ".squawk.toml";

#[derive(Debug, Default, Deserialize)]
pub struct UploadToGitHubConfig {
    #[serde(default)]
    pub fail_on_violations: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
pub struct ConfigFile {
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

impl ConfigFile {
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

pub struct Config {
    pub excluded_paths: Vec<String>,
    pub excluded_rules: Vec<Rule>,
    pub pg_version: Option<Version>,
    pub assume_in_transaction: bool,
    pub upload_to_github: UploadToGitHubConfig,
    pub upload_to_github_args: Option<UploadToGithubArgs>,
    pub no_error_on_unmatched_pattern: bool,
    pub is_stdin: bool,
    pub stdin_filepath: Option<String>,
    pub github_annotations: bool,
    pub reporter: Reporter,
    pub verbose: bool,
    pub debug: Option<DebugOption>,
    pub path_patterns: Vec<String>,
}

impl Config {
    pub fn from(opts: Opts) -> Config {
        let conf = ConfigFile::parse(opts.config_path)
            .unwrap_or_else(|e| {
                eprintln!("Configuration error: {e}");
                process::exit(1);
            })
            .unwrap_or_default();

        // the --exclude flag completely overrides the configuration file.
        let excluded_rules = if let Some(excluded_rules) = opts.excluded_rules {
            excluded_rules
        } else {
            conf.excluded_rules.clone()
        };

        // the --exclude-path flag completely overrides the configuration file.
        let excluded_paths = if let Some(excluded_paths) = opts.excluded_path {
            excluded_paths
        } else {
            conf.excluded_paths.clone()
        };
        let pg_version = if let Some(pg_version) = opts.pg_version {
            Some(pg_version)
        } else {
            conf.pg_version
        };

        let assume_in_transaction = if opts.assume_in_transaction {
            opts.assume_in_transaction
        } else if opts.no_assume_in_transaction {
            !opts.no_assume_in_transaction
        } else {
            conf.assume_in_transaction.unwrap_or_default()
        };

        let no_error_on_unmatched_pattern = if opts.no_error_on_unmatched_pattern {
            opts.no_error_on_unmatched_pattern
        } else {
            // TODO: we should have config support for these
            false
        };

        info!("pg version: {pg_version:?}");
        info!("excluded rules: {:?}", &excluded_rules);
        info!("excluded paths: {:?}", &excluded_paths);
        info!("assume in a transaction: {assume_in_transaction:?}");
        info!("no error on unmatched pattern: {no_error_on_unmatched_pattern:?}");

        let is_stdin = !io::stdin().is_terminal();
        let github_annotations = std::env::var("GITHUB_ACTIONS").is_ok()
            && std::env::var("SQUAWK_DISABLE_GITHUB_ANNOTATIONS").is_err();

        // TODO: we should support all of these in the config file as well
        let debug = opts.debug;
        let verbose = opts.verbose;
        let path_patterns = opts.path_patterns;
        let reporter = opts.reporter.unwrap_or_default();
        let stdin_filepath = opts.stdin_filepath;
        let upload_to_github = conf.upload_to_github;
        let upload_to_github_args = match opts.cmd {
            Some(Command::UploadToGithub(args)) => Some(*args),
            _ => None,
        };

        Config {
            excluded_paths,
            excluded_rules,
            pg_version,
            assume_in_transaction,
            upload_to_github,
            upload_to_github_args,
            no_error_on_unmatched_pattern,
            is_stdin,
            stdin_filepath,
            github_annotations,
            reporter,
            verbose,
            debug,
            path_patterns,
        }
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
        assert_debug_snapshot!(ConfigFile::parse(Some(squawk_toml.path().to_path_buf())));
    }
    #[test]
    fn load_pg_version() {
        let squawk_toml = NamedTempFile::new().expect("generate tempFile");
        let file = r#"
pg_version = "19.1"
        
        "#;
        fs::write(&squawk_toml, file).expect("Unable to write file");
        assert_debug_snapshot!(ConfigFile::parse(Some(squawk_toml.path().to_path_buf())));
    }
    #[test]
    fn load_excluded_rules() {
        let squawk_toml = NamedTempFile::new().expect("generate tempFile");
        let file = r#"
excluded_rules = ["require-concurrent-index-creation"]
        
        "#;
        fs::write(&squawk_toml, file).expect("Unable to write file");
        assert_debug_snapshot!(ConfigFile::parse(Some(squawk_toml.path().to_path_buf())));
    }
    #[test]
    fn load_excluded_paths() {
        let squawk_toml = NamedTempFile::new().expect("generate tempFile");
        let file = r#"
excluded_paths = ["example.sql"]
        
        "#;
        fs::write(&squawk_toml, file).expect("Unable to write file");
        assert_debug_snapshot!(ConfigFile::parse(Some(squawk_toml.path().to_path_buf())));
    }
    #[test]
    fn load_assume_in_transaction() {
        let squawk_toml = NamedTempFile::new().expect("generate tempFile");
        let file = r"
assume_in_transaction = false
        
        ";
        fs::write(&squawk_toml, file).expect("Unable to write file");
        assert_debug_snapshot!(ConfigFile::parse(Some(squawk_toml.path().to_path_buf())));
    }
    #[test]
    fn load_fail_on_violations() {
        let squawk_toml = NamedTempFile::new().expect("generate tempFile");
        let file = r"
[upload_to_github]
fail_on_violations = true        
        ";
        fs::write(&squawk_toml, file).expect("Unable to write file");
        assert_debug_snapshot!(ConfigFile::parse(Some(squawk_toml.path().to_path_buf())));
    }
    #[test]
    fn load_excluded_rules_with_alias() {
        let squawk_toml = NamedTempFile::new().expect("generate tempFile");
        let file = r#"
excluded_rules = ["prefer-timestamp-tz", "prefer-timestamptz"]

        "#;
        fs::write(&squawk_toml, file).expect("Unable to write file");
        assert_debug_snapshot!(ConfigFile::parse(Some(squawk_toml.path().to_path_buf())));
    }
}
