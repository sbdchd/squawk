use std::{path::PathBuf, process};

use crate::{
    Command, config::Config, debug::DebugArgs, file_finding::find_paths, reporter::LintArgs,
};

pub(crate) struct Stdin {
    pub(crate) path: Option<String>,
}

pub(crate) enum Input {
    Stdin(Stdin),
    Paths(Vec<PathBuf>),
}

pub(crate) enum Cmd {
    Debug(DebugArgs),
    Lint(LintArgs),
    Help,
    None,
    Server,
    UploadToGithub(Box<Config>),
}

impl Cmd {
    fn resolve_cli(conf: Config) -> Cmd {
        // TODO: do we need to do the same thing for the github command?
        let found_paths =
            find_paths(&conf.path_patterns, &conf.excluded_paths).unwrap_or_else(|e| {
                eprintln!("Failed to find files: {e}");
                process::exit(1);
            });
        if found_paths.is_empty() && !conf.path_patterns.is_empty() {
            eprintln!(
                "Failed to find files for provided patterns: {:?}",
                conf.path_patterns
            );
            if !conf.no_error_on_unmatched_pattern {
                process::exit(1);
            }
        }
        if !found_paths.is_empty() || conf.is_stdin {
            let read_stdin = found_paths.is_empty() && conf.is_stdin;
            let input = if read_stdin {
                Input::Stdin(Stdin {
                    path: conf.stdin_filepath,
                })
            } else {
                Input::Paths(found_paths)
            };
            if let Some(debug_option) = conf.debug {
                return Cmd::Debug(DebugArgs {
                    input,
                    debug_option,
                    verbose: conf.verbose,
                });
            } else {
                return Cmd::Lint(LintArgs {
                    input,
                    excluded_rules: conf.excluded_rules,
                    pg_version: conf.pg_version,
                    assume_in_transaction: conf.assume_in_transaction,
                    reporter: conf.reporter,
                    github_annotations: conf.github_annotations,
                });
            }
        } else if !conf.no_error_on_unmatched_pattern {
            return Cmd::Help;
        } else {
            return Cmd::None;
        }
    }

    pub(crate) fn from(opts: crate::Opts) -> Cmd {
        match opts.cmd {
            Some(Command::Server) => Cmd::Server,
            Some(Command::UploadToGithub(_)) => {
                let conf = Config::from(opts);
                Cmd::UploadToGithub(Box::new(conf))
            }
            None => {
                let conf = Config::from(opts);
                Cmd::resolve_cli(conf)
            }
        }
    }
}
