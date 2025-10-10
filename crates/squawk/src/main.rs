mod cmd;
mod config;
mod debug;
mod file;
mod file_finding;
mod github;
mod reporter;
use crate::cmd::Cmd;
use crate::reporter::LintArgs;
use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use debug::debug;
use reporter::lint_and_report;
use simplelog::CombinedLogger;
use squawk_linter::{Rule, Version};
use std::io;
use std::panic;
use std::path::PathBuf;
use std::process::ExitCode;

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

#[derive(Debug, ValueEnum, Clone, Default)]
pub enum Reporter {
    #[default]
    Tty,
    Gcc,
    Json,
    Gitlab,
}

/// Find problems in your SQL
#[allow(clippy::struct_excessive_bools)]
#[derive(Parser, Debug)]
#[command(version)]
struct Opts {
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

    let opts = Opts::parse();

    if opts.verbose {
        // ANSI codes don't render properly in the VSCode output pane
        let color_choice = if matches!(opts.cmd, Some(Command::Server)) {
            simplelog::ColorChoice::Never
        } else {
            simplelog::ColorChoice::Auto
        };
        CombinedLogger::init(vec![simplelog::TermLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            simplelog::TerminalMode::Stderr,
            color_choice,
        )])
        .expect("problem creating logger");
    }

    match Cmd::from(opts) {
        Cmd::Server => {
            squawk_server::run().context("language server failed")?;
        }
        Cmd::UploadToGithub(config) => {
            github::check_and_comment_on_pr(*config).context("Upload to GitHub failed")?;
        }
        Cmd::Debug(debug_args) => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            debug(&mut handle, debug_args)?;
        }
        Cmd::Lint(lint_args) => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            return lint_and_report(&mut handle, lint_args);
        }
        Cmd::Help => {
            Opts::command().print_long_help()?;
            println!();
        }
        Cmd::None => (),
    }

    Ok(ExitCode::SUCCESS)
}
