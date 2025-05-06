mod config;
mod debug;
mod file;
mod file_finding;
mod github;
mod reporter;
use anyhow::{Context, Result};
use debug::debug;
use reporter::check_and_dump_files;
use squawk_linter::{Rule, Version};
use structopt::clap::arg_enum;

use crate::file_finding::find_paths;
use atty::Stream;
use config::Config;
use log::info;
use simplelog::CombinedLogger;
use std::io;
use std::path::PathBuf;
use std::process::{self, ExitCode};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Comment on a PR with Squawk's results.
    UploadToGithub {
        /// Paths to search
        paths: Vec<String>,
        /// Exits with an error if violations are found
        #[structopt(long)]
        fail_on_violations: bool,
        #[structopt(long, env = "SQUAWK_GITHUB_PRIVATE_KEY")]
        github_private_key: Option<String>,
        #[structopt(long, env = "SQUAWK_GITHUB_PRIVATE_KEY_BASE64")]
        github_private_key_base64: Option<String>,
        #[structopt(long, env = "SQUAWK_GITHUB_TOKEN")]
        github_token: Option<String>,
        /// GitHub App Id.
        #[structopt(long, env = "SQUAWK_GITHUB_APP_ID")]
        github_app_id: Option<i64>,
        /// GitHub Install Id. The installation that squawk is acting on.
        #[structopt(long, env = "SQUAWK_GITHUB_INSTALL_ID")]
        github_install_id: Option<i64>,
        /// GitHub Repo Owner
        /// github.com/sbdchd/squawk, sbdchd is the owner
        #[structopt(long, env = "SQUAWK_GITHUB_REPO_OWNER")]
        github_repo_owner: String,
        /// GitHub Repo Name
        /// github.com/sbdchd/squawk, squawk is the name
        #[structopt(long, env = "SQUAWK_GITHUB_REPO_NAME")]
        github_repo_name: String,
        /// GitHub Pull Request Number
        /// github.com/sbdchd/squawk/pull/10, 10 is the PR number
        #[structopt(long, env = "SQUAWK_GITHUB_PR_NUMBER")]
        github_pr_number: i64,
    },
}

arg_enum! {
    #[derive(Debug, StructOpt)]
    pub enum DebugOption {
        Lex,
        Parse
    }
}

arg_enum! {
    #[derive(Debug, StructOpt)]
    pub enum Reporter {
        Tty,
        Gcc,
        Json,
    }
}

/// Find problems in your SQL
#[allow(clippy::struct_excessive_bools)]
#[derive(StructOpt, Debug)]
struct Opt {
    /// Paths or patterns to search
    #[structopt(value_name = "path")]
    path_patterns: Vec<String>,
    /// Paths to exclude
    ///
    /// For example:
    ///
    /// `--exclude-path=005_user_ids.sql --exclude-path=009_account_emails.sql`
    ///
    /// `--exclude-path='*user_ids.sql'`
    #[structopt(long = "exclude-path", global = true)]
    excluded_path: Option<Vec<String>>,
    /// Exclude specific warnings
    ///
    /// For example:
    /// --exclude=require-concurrent-index-creation,ban-drop-database
    #[structopt(
        short = "e",
        long = "exclude",
        value_name = "rule",
        use_delimiter = true,
        global = true
    )]
    excluded_rules: Option<Vec<Rule>>,
    /// Specify postgres version
    ///
    /// For example:
    /// --pg-version=13.0
    #[structopt(long, global = true)]
    pg_version: Option<Version>,
    /// Output debug format
    #[structopt(long,value_name ="format", possible_values = &DebugOption::variants(), case_insensitive = true)]
    debug: Option<DebugOption>,
    /// Style of error reporting
    #[structopt(long, possible_values = &Reporter::variants(), case_insensitive = true)]
    reporter: Option<Reporter>,
    #[structopt(long, value_name = "filepath")]
    /// Path to use in reporting for stdin
    stdin_filepath: Option<String>,
    #[structopt(subcommand)]
    cmd: Option<Command>,
    /// Enable debug logging output
    #[structopt(long, global = true)]
    verbose: bool,
    /// Path to the squawk config file (.squawk.toml)
    #[structopt(short = "c", long = "config", global = true)]
    config_path: Option<PathBuf>,
    /// Assume that a transaction will wrap each SQL file when run by a migration tool
    ///
    /// Use --no-assume-in-transaction to override any config file that sets this
    #[structopt(long, global = true)]
    assume_in_transaction: bool,
    #[structopt(
        long,
        hidden = true,
        conflicts_with = "assume-in-transaction",
        global = true
    )]
    no_assume_in_transaction: bool,
}

#[allow(clippy::too_many_lines)]
fn main() -> Result<ExitCode> {
    let opts = Opt::from_args();

    if opts.verbose {
        CombinedLogger::init(vec![simplelog::TermLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            simplelog::TerminalMode::Stderr,
            simplelog::ColorChoice::Auto,
        )])
        .expect("problem creating logger");
    }

    let conf = Config::parse(opts.config_path)
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

    info!("pg version: {:?}", pg_version);
    info!("excluded rules: {:?}", &excluded_rules);
    info!("excluded paths: {:?}", &excluded_paths);
    info!("assume in a transaction: {:?}", assume_in_transaction);

    let mut clap_app = Opt::clap();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let is_stdin = !atty::is(Stream::Stdin);

    let found_paths = find_paths(&opts.path_patterns, &excluded_paths).unwrap_or_else(|e| {
        eprintln!("Failed to find files: {e}");
        process::exit(1);
    });
    if found_paths.is_empty() && !opts.path_patterns.is_empty() {
        eprintln!(
            "Failed to find files for provided patterns: {:?}",
            opts.path_patterns
        );
        process::exit(1);
    }
    if let Some(subcommand) = opts.cmd {
        github::check_and_comment_on_pr(
            subcommand,
            &conf,
            is_stdin,
            opts.stdin_filepath,
            &excluded_rules,
            &excluded_paths,
            pg_version,
            assume_in_transaction,
        )
        .context("Upload to GitHub failed")?;
    } else if !found_paths.is_empty() || is_stdin {
        let read_stdin = found_paths.is_empty() && is_stdin;
        if let Some(kind) = opts.debug {
            debug(&mut handle, &found_paths, read_stdin, &kind, opts.verbose)?;
        } else {
            let reporter = opts.reporter.unwrap_or(Reporter::Tty);
            let exit_code = check_and_dump_files(
                &mut handle,
                &found_paths,
                read_stdin,
                opts.stdin_filepath,
                &excluded_rules,
                pg_version,
                assume_in_transaction,
                &reporter,
            )?;
            return Ok(exit_code);
        }
    } else {
        clap_app.print_long_help()?;
        println!();
    }
    Ok(ExitCode::SUCCESS)
}
