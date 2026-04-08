use log::info;
use squawk_linter::config::{ConfigFile, UploadToGitHubConfig};
use squawk_linter::{Rule, Version};
use std::{
    io::{self, IsTerminal},
    process,
};

use crate::{Command, DebugOption, Opts, Reporter, UploadToGithubArgs};

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
