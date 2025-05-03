use std::{
    io::{self, Read},
    process::ExitCode,
};

use annotate_snippets::{Level, Renderer, Snippet};
use anyhow::Result;
use atty::Stream;
use clap::{error::ErrorKind, CommandFactory, Parser, ValueEnum};
use squawk_linter::Violation;
use squawk_syntax::syntax_error::SyntaxError;
use std::{fs, path::PathBuf};

#[derive(ValueEnum, Clone, Debug)]
enum Mode {
    Parse,
    Lex,
    Lint,
}

/// Dump Parse/Lex Data
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// SQL to dump
    #[arg(short, long, conflicts_with = "file")]
    sql: Option<String>,

    /// Path to read SQL
    #[arg(short, long, conflicts_with = "sql")]
    file: Option<PathBuf>,

    /// Either Parser debug output or Lexer debug output.
    #[arg(short, long, default_value = "parse")]
    mode: Mode,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Assume the SQL is being run within a transaction. No explicit begin,
    /// commit required.
    #[arg(short, long)]
    assume_in_transaction: bool,
}

fn read_stdin() -> Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn read_sql(arg_sql: Option<String>, file: &Option<PathBuf>) -> Result<String> {
    let is_stdin = !atty::is(Stream::Stdin);
    if is_stdin {
        read_stdin()
    } else if let Some(path) = &file {
        Ok(fs::read_to_string(path)?)
    } else if let Some(sql) = arg_sql {
        Ok(sql)
    } else {
        let err = Args::command().error(
            ErrorKind::ArgumentConflict,
            "--sql, --file, or stdin must be provided.",
        );
        err.exit()
    }
}

fn main() -> Result<ExitCode> {
    let args = Args::parse();
    let sql = read_sql(args.sql, &args.file)?;
    let filename = args
        .file
        .map(|x| x.display().to_string())
        .unwrap_or("stdin".to_string());
    match args.mode {
        Mode::Lex => {
            let tokens = squawk_lexer::tokenize(&sql);
            let mut start = 0;
            for token in tokens {
                if args.verbose {
                    let content = &sql[start as usize..(start + token.len) as usize];
                    start += token.len;
                    println!("{:?} @ {:?}", content, token.kind);
                } else {
                    println!("{:?}", token);
                }
            }
            Ok(ExitCode::SUCCESS)
        }
        Mode::Parse => {
            let parse = squawk_syntax::SourceFile::parse(&sql);
            if args.verbose {
                println!("{}\n---", parse.syntax_node());
            }
            print!("{:#?}", parse.syntax_node());
            let errors = parse.errors();
            if !errors.is_empty() {
                let mut snap = "---".to_string();
                for syntax_error in &errors {
                    let range = syntax_error.range();
                    let text = syntax_error.message();
                    // split into there own lines so that we can just grep
                    // for error without hitting this part
                    snap += "\n";
                    snap += "ERROR";
                    if range.start() == range.end() {
                        snap += &format!("@{:?} {:?}", range.start(), text);
                    } else {
                        snap += &format!("@{:?}:{:?} {:?}", range.start(), range.end(), text);
                    }
                }
                println!("{}", snap);

                render_syntax_errors(&errors, &filename, &sql);

                return Ok(ExitCode::FAILURE);
            }
            Ok(ExitCode::SUCCESS)
        }
        Mode::Lint => {
            let mut linter = squawk_linter::squawk_linter::with_all_rules();
            linter.settings.assume_in_transaction = args.assume_in_transaction;
            let parse = squawk_syntax::SourceFile::parse(&sql);

            if args.verbose {
                println!("{}\n---", parse.syntax_node());
                // print!("{:#?}\n---", parse.syntax_node());
            }

            let errors = linter.lint(parse, &sql);

            if errors.is_empty() {
                Ok(ExitCode::SUCCESS)
            } else {
                render_lint_errors(&errors, &filename, &sql);
                println!();
                println!("Find detailed examples and solutions for each rule at https://squawkhq.com/docs/rules");
                println!(
                    "Found {} issue in 1 file (checked 1 source file)",
                    errors.len()
                );
                Ok(ExitCode::FAILURE)
            }
        }
    }
}

fn render_syntax_errors(errors: &[SyntaxError], filename: &str, sql: &str) {
    let renderer = Renderer::styled();
    for err in errors {
        let text = err.message();
        let span = err.range().into();
        let message = Level::Warning.title(text).id("syntax-error").snippet(
            Snippet::source(sql)
                .origin(filename)
                .fold(true)
                .annotation(Level::Error.span(span)),
        );
        println!("{}", renderer.render(message));
    }
}

fn render_lint_errors(errors: &Vec<&Violation>, filename: &str, sql: &str) {
    let renderer = Renderer::styled();
    for err in errors {
        let meta = err.code.meta();
        let footers = err.messages.iter().map(|e| Level::Help.title(e));
        // TODO: we need to figure out error messages, they shouldn't be in two places
        let prebuilt_footers = meta.messages.into_iter().map(|x| match x {
            squawk_linter::ViolationMessage::Note(x) => Level::Note.title(x),
            squawk_linter::ViolationMessage::Help(x) => Level::Help.title(x),
        });
        let error_name = err.code.to_string();
        let message = Level::Warning
            .title(&meta.title)
            .id(&error_name)
            .snippet(
                Snippet::source(sql)
                    .origin(filename)
                    .fold(true)
                    .annotation(Level::Error.span(err.text_range.into())),
            )
            .footers(footers)
            .footers(prebuilt_footers);

        println!("{}", renderer.render(message));
    }
}
