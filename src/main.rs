mod ast;
mod error;
mod github;
mod parse;
mod reporter;
mod rules;
#[macro_use]
extern crate lazy_static;
use crate::reporter::{
    check_files, dump_ast_for_paths, explain_rule, list_rules, print_violations, DumpAstOption,
    Reporter,
};
use atty::Stream;
use std::io;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};
use structopt::StructOpt;

fn handle_exit_err<E: std::fmt::Debug>(res: Result<(), E>) -> ! {
    match res {
        Ok(_) => process::exit(0),
        Err(err) => {
            eprintln!("{:#?}", err);
            process::exit(1)
        }
    }
}

/// Find problems in your SQL
#[derive(StructOpt, Debug)]
struct Opt {
    /// Paths to search
    paths: Vec<String>,
    /// Exclude specific warnings
    ///
    /// For example:
    /// --exclude=require-concurrent-index-creation,ban-drop-database
    #[structopt(short, long, use_delimiter = true)]
    exclude: Option<Vec<String>>,
    /// List all available rules
    #[structopt(long)]
    list_rules: bool,
    /// Provide documentation on the given rule
    #[structopt(long)]
    explain: Option<String>,
    /// Output AST in JSON
    #[structopt(long, possible_values = &DumpAstOption::variants(), case_insensitive = true)]
    dump_ast: Option<DumpAstOption>,
    /// Style of error reporting
    #[structopt(long, possible_values = &Reporter::variants(), case_insensitive = true)]
    reporter: Option<Reporter>,
    /// Output SQL and Errors in a GitHub comment
    #[structopt(long)]
    upload_to_github: bool,
    /// GitHub Private Key.
    #[structopt(long, env = "GITHUB_PRIVATE_KEY")]
    github_private_key: Option<String>,
    /// GitHub App Id.
    #[structopt(long, env = "GITHUB_APP_ID")]
    github_app_id: Option<String>,
    /// GitHub Install Id. The installation that squawk is acting on.
    #[structopt(long, env = "GITHUB_INSTALL_ID")]
    github_install_id: Option<String>,
    /// GitHub Bot Id. The User id of the bot.
    #[structopt(long, env = "GITHUB_BOT_ID")]
    github_bot_id: Option<String>,
    /// GitHub Repo Owner
    /// github.com/sbdchd/squawk, sbdchd is the owner
    #[structopt(long, env = "GITHUB_REPO_OWNER")]
    github_repo_owner: Option<String>,
    /// GitHub Repo Name
    /// github.com/sbdchd/squawk, squawk is the name
    #[structopt(long, env = "GITHUB_REPO_NAME")]
    github_repo_name: Option<String>,
    /// GitHub Pull Request Number
    /// github.com/sbdchd/squawk/pull/10, 10 is the PR number
    #[structopt(long, env = "GITHUB_PR_NUMBER")]
    github_pr_number: Option<i64>,
}

fn main() {
    let opts = Opt::from_args();
    let mut clap_app = Opt::clap();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let is_stdin = !atty::is(Stream::Stdin);
    if !opts.paths.is_empty() || is_stdin {
        if let Some(dump_ast_kind) = opts.dump_ast {
            handle_exit_err(dump_ast_for_paths(
                &mut handle,
                &opts.paths,
                is_stdin,
                dump_ast_kind,
            ));
        } else {
            match check_files(&opts.paths, is_stdin, opts.exclude) {
                Ok(violations) => {
                    if opts.upload_to_github {
                        match (
                            opts.github_private_key,
                            opts.github_app_id,
                            opts.github_install_id,
                            opts.github_bot_id,
                            opts.github_repo_owner,
                            opts.github_repo_name,
                            opts.github_pr_number,
                        ) {
                            (
                                Some(private_key),
                                Some(app_id),
                                Some(install_id),
                                Some(bot_id),
                                Some(repo_owner),
                                Some(repo_name),
                                Some(pr_number),
                            ) => {
                                let now_unix_time = SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .expect("problem getting current time");
                                // TOOD(sbdchd): generate actually comment content

                                let comment_body = format!(
                                    r##"
# foo bar

testing 123
violations count {violations}

--- 

updated @ {timestamp}
    "##,
                                    timestamp = now_unix_time.as_secs(),
                                    violations = violations.len()
                                );
                                let res = github::comment_on_pr(
                                    &private_key,
                                    &app_id,
                                    &install_id,
                                    &bot_id,
                                    &repo_owner,
                                    &repo_name,
                                    pr_number,
                                    &comment_body,
                                );
                                println!("{:#?}", res);
                                process::exit(1)
                            }

                            values => {
                                eprintln!("missing github argument {:#?}", values);
                                process::exit(1);
                            }
                        }
                    }
                    let reporter = opts.reporter.unwrap_or(Reporter::Tty);
                    match print_violations(&mut handle, &violations, &reporter) {
                        Ok(_) => {
                            let exit_code = if !violations.is_empty() { 1 } else { 0 };
                            process::exit(exit_code);
                        }
                        Err(e) => {
                            eprintln!("{:#?}", e);
                            process::exit(1);
                        }
                    }
                }
                e => {
                    eprintln!("{:#?}", e);
                    process::exit(1)
                }
            }
        }
    } else if opts.list_rules {
        handle_exit_err(list_rules(&mut handle));
    } else if let Some(rule_name) = opts.explain {
        handle_exit_err(explain_rule(&mut handle, &rule_name));
    } else {
        clap_app.print_long_help().expect("problem printing help");
    }
}
