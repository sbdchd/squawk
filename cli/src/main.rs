#![allow(clippy::match_wildcard_for_single_variants)]
#[allow(clippy::non_ascii_literal)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::enum_variant_names)]
#[allow(clippy::module_name_repetitions)]
mod config;
mod reporter;
mod subcommand;

use crate::reporter::{
    check_files, dump_ast_for_paths, explain_rule, list_rules, print_violations, DumpAstOption,
    Reporter,
};
use crate::subcommand::{check_and_comment_on_pr, Command};
use ::semver::Version;
use atty::Stream;
use config::Config;
use log::info;
use simplelog::CombinedLogger;
use squawk_linter::violations::{RuleViolationKind, default_pg_version};
use std::io;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

fn exit<E: std::fmt::Display, T>(res: Result<T, E>, msg: &str) -> ! {
    match res {
        Ok(_) => process::exit(0),
        Err(err) => {
            eprintln!("{}: {}", msg, err);
            process::exit(1)
        }
    }
}

/// Find problems in your SQL
#[derive(StructOpt, Debug)]
struct Opt {
    /// Paths to search
    #[structopt(value_name = "path")]
    paths: Vec<String>,
    /// Exclude specific warnings
    ///
    /// For example:
    /// --exclude=require-concurrent-index-creation,ban-drop-database
<<<<<<< HEAD
    #[structopt(
        short = "e",
        long = "exclude",
        value_name = "rule",
        use_delimiter = true
    )]
    excluded_rules: Option<Vec<RuleViolationKind>>,
=======
    #[structopt(short, long, use_delimiter = true)]
    exclude: Option<Vec<String>>,
    /// Specify postgres version
    ///
    /// For example:
    /// --pg_version=13.0
    #[structopt(long)]
    pg_version: Option<String>,
>>>>>>> fbaafc8 (Add version param and pass default version)
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
    #[structopt(long)]
    verbose: bool,
    /// Path to the squawk config file (.squawk.toml)
    #[structopt(short = "c", long = "config")]
    config_path: Option<PathBuf>,
}

fn main() {
    let opts = Opt::from_args();
<<<<<<< HEAD

=======
    let given_version = opts.pg_version;
    let pg_version: Version;
    if given_version.is_none() {
        pg_version = default_pg_version();
    } else {
        pg_version = Version::parse(given_version.as_deref().unwrap_or("")).unwrap();
    }
>>>>>>> fbaafc8 (Add version param and pass default version)
    if opts.verbose {
        CombinedLogger::init(vec![simplelog::TermLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        )])
        .expect("problem creating logger");
    }

    let conf = Config::parse(opts.config_path);
    // the --exclude flag completely overrides the configuration file.
    let excluded_rules = if let Some(excluded_rules) = opts.excluded_rules {
        excluded_rules
    } else {
        conf.unwrap_or_else(|e| {
            eprintln!("Configuration error: {}", e);
            process::exit(1);
        })
        .unwrap_or_default()
        .excluded_rules
    };
    info!("excluded rules: {:?}", &excluded_rules);

    let mut clap_app = Opt::clap();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let is_stdin = !atty::is(Stream::Stdin);
    if let Some(subcommand) = opts.cmd {
        exit(
<<<<<<< HEAD
            check_and_comment_on_pr(subcommand, is_stdin, opts.stdin_filepath, &excluded_rules),
=======
            check_and_comment_on_pr(
                subcommand,
                is_stdin,
                opts.stdin_filepath,
                &opts.exclude.unwrap_or_else(Vec::new),
                pg_version,
            ),
>>>>>>> fbaafc8 (Add version param and pass default version)
            "Upload to GitHub failed",
        );
    } else if !opts.paths.is_empty() || is_stdin {
        if let Some(dump_ast_kind) = opts.dump_ast {
            exit(
                dump_ast_for_paths(&mut handle, &opts.paths, is_stdin, &dump_ast_kind),
                "Failed to dump AST",
            );
        } else {
<<<<<<< HEAD
            match check_files(&opts.paths, is_stdin, opts.stdin_filepath, &excluded_rules) {
=======
            match check_files(
                &opts.paths,
                is_stdin,
                opts.stdin_filepath,
                &opts.exclude.unwrap_or_else(Vec::new),
                pg_version,
            ) {
>>>>>>> fbaafc8 (Add version param and pass default version)
                Ok(file_reports) => {
                    let reporter = opts.reporter.unwrap_or(Reporter::Tty);
                    let total_violations = file_reports
                        .iter()
                        .map(|f| f.violations.len())
                        .sum::<usize>();
                    match print_violations(&mut handle, file_reports, &reporter) {
                        Ok(_) => {
                            let exit_code = if total_violations > 0 { 1 } else { 0 };
                            process::exit(exit_code);
                        }
                        Err(e) => {
                            eprintln!("Problem reporting violations: {}", e);
                            process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Problem linting SQL files: {}", e);
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
