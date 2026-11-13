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
    fn read_input(conf: &mut Config) -> Option<Input> {
        // TODO: do we need to do the same thing for the github command?
        let found_paths =
            find_paths(&conf.path_patterns, &conf.excluded_paths).unwrap_or_else(|e| {
                eprintln!("Failed to find files: {e}");
                process::exit(1);
            });
        if !found_paths.is_empty() {
            return Some(Input::Paths(found_paths));
        }
        if !conf.path_patterns.is_empty() {
            eprintln!(
                "Failed to find files for provided patterns: {:?}",
                conf.path_patterns
            );
            if !conf.no_error_on_unmatched_pattern {
                process::exit(1);
            }
        }
        if conf.is_stdin {
            return Some(Input::Stdin(Stdin {
                path: conf.stdin_filepath.take(),
            }));
        }
        None
    }

    fn check(mut conf: Config) -> Cmd {
        let Some(input) = Cmd::read_input(&mut conf) else {
            if conf.no_error_on_unmatched_pattern {
                return Cmd::None;
            }
            return Cmd::Help;
        };

        match conf.debug {
            Some(debug_option) => Cmd::Debug(DebugArgs {
                input,
                debug_option,
                verbose: conf.verbose,
            }),
            None => Cmd::Lint(LintArgs {
                input,
                excluded_rules: conf.excluded_rules,
                included_rules: conf.included_rules,
                pg_version: conf.pg_version,
                assume_in_transaction: conf.assume_in_transaction,
                reporter: conf.reporter,
                github_annotations: conf.github_annotations,
            }),
        }
    }

    pub(crate) fn from(opts: crate::Opts) -> Cmd {
        match opts.cmd {
            Some(Command::Check(_)) | None => Cmd::check(Config::from(opts)),
            Some(Command::Server) => Cmd::Server,
            Some(Command::UploadToGithub(_)) => Cmd::UploadToGithub(Box::new(Config::from(opts))),
        }
    }
}
