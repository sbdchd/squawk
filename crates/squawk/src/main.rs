mod config;
mod debug;
mod file;
mod file_finding;
mod github;
mod reporter;
use crate::file_finding::find_paths;
use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use config::Config;
use debug::debug;
use log::info;
use reporter::check_and_dump_files;
use simplelog::CombinedLogger;
use squawk_linter::{Rule, Version};
use std::io;
use std::io::IsTerminal;
use std::panic;
use std::path::PathBuf;
use std::process::{self, ExitCode};

#[derive(Parser, Debug)]
pub struct UploadToGithubArgs {
    /// Paths to search
    paths: Vec<String>,
    /// Exits with an error if violations are found
    #[arg(long)]
    fail_on_violations: bool,
    #[arg(long, env = "SQUAWK_GITHUB_PRIVATE_KEY")]
    github_private_key: Option<String>,
    #[arg(long, env = "SQUAWK_GITHUB_PRIVATE_KEY_BASE64")]
    github_private_key_base64: Option<String>,
    /// GitHub API url.
    #[arg(long, env = "SQUAWK_GITHUB_API_URL")]
    github_api_url: Option<String>,
    #[arg(long, env = "SQUAWK_GITHUB_TOKEN")]
    github_token: Option<String>,
    /// GitHub App Id.
    #[arg(long, env = "SQUAWK_GITHUB_APP_ID")]
    github_app_id: Option<i64>,
    /// GitHub Install Id. The installation that squawk is acting on.
    #[arg(long, env = "SQUAWK_GITHUB_INSTALL_ID")]
    github_install_id: Option<i64>,
    /// GitHub Repo Owner
    /// github.com/sbdchd/squawk, sbdchd is the owner
    #[arg(long, env = "SQUAWK_GITHUB_REPO_OWNER")]
    github_repo_owner: String,
    /// GitHub Repo Name
    /// github.com/sbdchd/squawk, squawk is the name
    #[arg(long, env = "SQUAWK_GITHUB_REPO_NAME")]
    github_repo_name: String,
    /// GitHub Pull Request Number
    /// github.com/sbdchd/squawk/pull/10, 10 is the PR number
    #[arg(long, env = "SQUAWK_GITHUB_PR_NUMBER")]
    github_pr_number: i64,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Run the language server
    Server,
    /// Comment on a PR with Squawk's results.
    UploadToGithub(Box<UploadToGithubArgs>),
}

#[derive(Debug, ValueEnum, Clone)]
pub enum DebugOption {
    Lex,
    Parse,
    Ast,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum Reporter {
    Tty,
    Gcc,
    Json,
    Gitlab,
}

/// Find problems in your SQL
#[allow(clippy::struct_excessive_bools)]
#[derive(Parser, Debug)]
#[command(version)]
struct Opt {
    /// Paths or patterns to search
    #[arg(value_name = "path")]
    path_patterns: Vec<String>,
    /// Paths to exclude
    ///
    /// For example:
    ///
    /// `--exclude-path=005_user_ids.sql --exclude-path=009_account_emails.sql`
    ///
    /// `--exclude-path='*user_ids.sql'`
    #[arg(long = "exclude-path", global = true)]
    excluded_path: Option<Vec<String>>,
    /// Exclude specific warnings
    ///
    /// For example:
    /// --exclude=require-concurrent-index-creation,ban-drop-database
    #[arg(
        short = 'e',
        long = "exclude",
        value_name = "rule",
        value_delimiter = ',',
        global = true
    )]
    excluded_rules: Option<Vec<Rule>>,
    /// Specify postgres version
    ///
    /// For example:
    /// --pg-version=13.0
    #[arg(long, global = true)]
    pg_version: Option<Version>,
    /// Output debug format
    #[arg(long, value_name = "format", ignore_case = true)]
    debug: Option<DebugOption>,
    /// Style of error reporting
    #[arg(long, ignore_case = true)]
    reporter: Option<Reporter>,
    #[arg(long, value_name = "filepath")]
    /// Path to use in reporting for stdin
    stdin_filepath: Option<String>,
    #[command(subcommand)]
    cmd: Option<Command>,
    /// Enable debug logging output
    #[arg(long, global = true)]
    verbose: bool,
    /// Path to the squawk config file (.squawk.toml)
    #[arg(short = 'c', long = "config", global = true)]
    config_path: Option<PathBuf>,
    /// Assume that a transaction will wrap each SQL file when run by a migration tool
    ///
    /// Use --no-assume-in-transaction to override any config file that sets this
    #[arg(long, global = true)]
    assume_in_transaction: bool,
    #[arg(
        long,
        hide = true,
        conflicts_with = "assume_in_transaction",
        global = true
    )]
    no_assume_in_transaction: bool,
    /// Do not exit with an error when provided path patterns do not match any files
    #[arg(long = "no-error-on-unmatched-pattern", global = true)]
    no_error_on_unmatched_pattern: bool,
}

#[allow(clippy::too_many_lines)]
fn main() -> Result<ExitCode> {
    let version = env!("CARGO_PKG_VERSION");

    // via: https://github.com/astral-sh/ruff/blob/40fd52dde0ddf0b95a3433163f51560b4827ab75/crates/ruff_server/src/server.rs#L140
    panic::set_hook(Box::new(move |panic_info| {
        use std::io::Write;
        let backtrace = std::backtrace::Backtrace::force_capture();
        // Don't use `eprintln` because `eprintln` itself may panic if the pipe is broken.
        let mut stderr = std::io::stderr().lock();
        let open_an_issue = format!(
            r#"An internal error has occured with Squawk v{version}!
Please open an issue at https://github.com/sbdchd/squawk/issues/new with the logs above!
"#
        );
        writeln!(stderr, "{panic_info}\n{backtrace}\n{open_an_issue}").ok();
    }));

    let opts = Opt::parse();

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

    let no_error_on_unmatched_pattern = if opts.no_error_on_unmatched_pattern {
        opts.no_error_on_unmatched_pattern
    } else {
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
    match opts.cmd {
        Some(Command::Server) => {
            squawk_server::run().context("language server failed")?;
        }
        Some(Command::UploadToGithub(args)) => {
            github::check_and_comment_on_pr(
                *args,
                &conf,
                is_stdin,
                opts.stdin_filepath,
                &excluded_rules,
                &excluded_paths,
                pg_version,
                assume_in_transaction,
                github_annotations,
            )
            .context("Upload to GitHub failed")?;
        }
        None => {
            let found_paths =
                find_paths(&opts.path_patterns, &excluded_paths).unwrap_or_else(|e| {
                    eprintln!("Failed to find files: {e}");
                    process::exit(1);
                });
            if found_paths.is_empty() && !opts.path_patterns.is_empty() {
                eprintln!(
                    "Failed to find files for provided patterns: {:?}",
                    opts.path_patterns
                );
                if !no_error_on_unmatched_pattern {
                    process::exit(1);
                }
            }
            if !found_paths.is_empty() || is_stdin {
                let stdout = io::stdout();
                let mut handle = stdout.lock();

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
                        github_annotations,
                    )?;
                    return Ok(exit_code);
                }
            } else if !no_error_on_unmatched_pattern {
                Opt::command().print_long_help()?;
                println!();
            }
        }
    }
    Ok(ExitCode::SUCCESS)
}
