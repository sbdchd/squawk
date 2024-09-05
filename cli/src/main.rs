#![allow(clippy::match_wildcard_for_single_variants)]
#[allow(clippy::non_ascii_literal)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::enum_variant_names)]
#[allow(clippy::module_name_repetitions)]
mod config;
mod file_finding;
mod reporter;
mod subcommand;

use crate::file_finding::find_paths;
use crate::reporter::{
    check_files, dump_ast_for_paths, explain_rule, list_rules, print_violations, DumpAstOption,
    Reporter,
};
use crate::subcommand::{check_and_comment_on_pr, Command};
use atty::Stream;
use config::Config;
use log::info;
use simplelog::CombinedLogger;
use squawk_linter::versions::Version;
use squawk_linter::violations::RuleViolationKind;
use std::io;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

fn exit<E: std::fmt::Display, T>(res: Result<T, E>, msg: &str) -> ! {
    match res {
        Ok(_) => process::exit(0),
        Err(err) => {
            eprintln!("{msg}: {err}");
            process::exit(1)
        }
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
    excluded_rules: Option<Vec<RuleViolationKind>>,
    /// Specify postgres version
    ///
    /// For example:
    /// --pg-version=13.0
    #[structopt(long, global = true)]
    pg_version: Option<Version>,
    /// List all available rules
    #[structopt(long)]
    list_rules: bool,
    /// Provide documentation on the given rule
    #[structopt(long, value_name = "rule")]
    explain: Option<String>,
    /// Output AST in JSON
    #[structopt(long,value_name ="ast-format", possible_values = &DumpAstOption::variants(), case_insensitive = true)]
    dump_ast: Option<DumpAstOption>,
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
fn main() {
    let opts = Opt::from_args();

    if opts.verbose {
        CombinedLogger::init(vec![simplelog::TermLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
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
        exit(
            check_and_comment_on_pr(
                subcommand,
                &conf,
                is_stdin,
                opts.stdin_filepath,
                &excluded_rules,
                &excluded_paths,
                pg_version,
                assume_in_transaction,
            ),
            "Upload to GitHub failed",
        );
    } else if !found_paths.is_empty() || is_stdin {
        let read_stdin = found_paths.is_empty() && is_stdin;
        if let Some(dump_ast_kind) = opts.dump_ast {
            exit(
                dump_ast_for_paths(&mut handle, &found_paths, read_stdin, &dump_ast_kind),
                "Failed to dump AST",
            );
        } else {
            match check_files(
                &found_paths,
                read_stdin,
                opts.stdin_filepath,
                &excluded_rules,
                pg_version,
                assume_in_transaction,
            ) {
                Ok(file_reports) => {
                    let reporter = opts.reporter.unwrap_or(Reporter::Tty);
                    let total_violations = file_reports
                        .iter()
                        .map(|f| f.violations.len())
                        .sum::<usize>();
                    match print_violations(&mut handle, file_reports, &reporter) {
                        Ok(()) => {
                            let exit_code = i32::from(total_violations > 0);
                            process::exit(exit_code);
                        }
                        Err(e) => {
                            eprintln!("Problem reporting violations: {e}");
                            process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Problem linting SQL files: {e}");
                    process::exit(1)
                }
            }
        }
    } else if opts.list_rules {
        exit(list_rules(&mut handle), "Could not list rules");
    } else if let Some(rule_name) = opts.explain {
        exit(
            explain_rule(&mut handle, &rule_name),
            "Could not explain rules",
        );
    } else {
        clap_app.print_long_help().expect("problem printing help");
        println!();
    }
}
