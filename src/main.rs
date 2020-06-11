mod ast;
mod error;
mod parse;
mod reporter;
mod rules;
#[macro_use]
extern crate lazy_static;
use crate::reporter::{
    check_files, dump_ast_for_paths, explain_rule, list_rules, DumpAstOption, Reporter,
};
use std::io;
use std::process;
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
}

fn main() {
    let opts = Opt::from_args();
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    if !opts.paths.is_empty() {
        if let Some(dump_ast_kind) = opts.dump_ast {
            handle_exit_err(dump_ast_for_paths(&mut handle, &opts.paths, dump_ast_kind));
        } else {
            let reporter = opts.reporter.unwrap_or(Reporter::Tty);
            match check_files(&mut handle, &opts.paths, reporter, opts.exclude) {
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
    }

    if opts.list_rules {
        handle_exit_err(list_rules(&mut handle));
    }

    if let Some(rule_name) = opts.explain {
        handle_exit_err(explain_rule(&mut handle, &rule_name));
    }
}
