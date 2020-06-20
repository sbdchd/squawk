mod ast;
mod error;
mod github;
mod parse;
mod reporter;
mod rules;
#[macro_use]
extern crate lazy_static;
use crate::reporter::{
    check_files, dump_ast_for_paths, explain_rule, list_rules, DumpAstOption, Reporter,
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
    /// GitHub Private Key. Paired with `--upload-to-github`.
    #[structopt(long, env = "GITHUB_PRIVATE_KEY")]
    github_private_key: Option<String>,
    /// GitHub App Id. Paired with `--upload-to-github`.
    #[structopt(long, env = "GITHUB_APP_ID")]
    github_app_id: Option<String>,
    /// GitHub Install Id. The installation that squawk is acting on. Paired with
    /// `--upload-to-github`.
    #[structopt(long, env = "GITHUB_INSTALL_ID")]
    github_install_id: Option<String>,
    /// GitHub Bot Id. The User id of the bot.
    #[structopt(long, env = "GITHUB_BOT_ID")]
    github_bot_id: Option<String>,
}

fn main() {
    let opts = Opt::from_args();
    let mut clap_app = Opt::clap();
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let is_stdin = !atty::is(Stream::Stdin);
    if opts.upload_to_github {
        let now_unix_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("problem getting current time");
        let comment = format!(
            r##"
# foo bar

testing 123

updated @ {}

    
    "##,
            now_unix_time.as_secs()
        );

        match (
            opts.github_private_key,
            opts.github_app_id,
            opts.github_install_id,
            opts.github_bot_id,
        ) {
            (Some(private_key), Some(app_id), Some(install_id), Some(bot_id)) => {
                // let res = comment_on_pr();
                // println!("{:#?}", res);
                let jwt =
                    github::generate_jwt(private_key, app_id).expect("successfully generated jwt");
                match github::create_access_token(&jwt, &install_id) {
                    Ok(access_token) => {
                        let owner = "sbdchd";
                        let repo = "squawk";
                        let issue = 14i64;

                        match github::list_comments(owner, repo, issue, &access_token.token) {
                            Ok(comments) => {
                                match comments.iter().find(|x| x.user.id.to_string() == bot_id) {
                                    Some(prev_comment) => {
                                        println!("updating {:#?}", prev_comment);
                                        let res = github::update_comment(
                                            owner,
                                            repo,
                                            prev_comment.id,
                                            &comment,
                                            &access_token.token,
                                        );
                                        println!("res from update {:#?}", res);
                                    }
                                    None => {
                                        let res = github::create_comment(
                                            github::CommentArgs {
                                                owner: owner.into(),
                                                repo: repo.into(),
                                                issue: issue,
                                                body: comment.into(),
                                            },
                                            &access_token.token,
                                        );
                                        println!("creating new comment");
                                    }
                                }
                            }
                            err => {
                                eprintln!(
                                    "missing github private key or app_id or install_id {:#?}",
                                    err
                                );
                                process::exit(1)
                            }
                        }

                        // println!("{:#?}", res);
                    }
                    err => {
                        eprintln!(
                            "missing github private key or app_id or install_id {:#?}",
                            err
                        );
                        process::exit(1)
                    }
                }
                // let res = github::get_app_installs(&jwt);
                // github::create_comment(
                //     github::CommentArgs {
                //         owner: "sbdchd".into(),
                //         repo: "squawk".into(),
                //         issue: 10,
                //         body: comment.into(),
                //     },
                //     &jwt,
                // );
            }
            _ => {
                eprintln!("missing github private key or app_id or install_id");
                process::exit(1)
            }
        }
    } else if !opts.paths.is_empty() || is_stdin {
        if let Some(dump_ast_kind) = opts.dump_ast {
            handle_exit_err(dump_ast_for_paths(
                &mut handle,
                &opts.paths,
                is_stdin,
                dump_ast_kind,
            ));
        } else {
            let reporter = opts.reporter.unwrap_or(Reporter::Tty);
            match check_files(&mut handle, &opts.paths, is_stdin, reporter, opts.exclude) {
                Ok(found_errors) => {
                    if found_errors {
                        process::exit(1);
                    } else {
                        process::exit(0);
                    }
                }
                Err(e) => {
                    eprintln!("{:#?}", e);
                    process::exit(1);
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
