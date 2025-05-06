use std::{io, path::PathBuf};

use annotate_snippets::{Level, Message, Renderer, Snippet};
use anyhow::Result;
use squawk_syntax::syntax_error::SyntaxError;

use crate::{
    file::{sql_from_path, sql_from_stdin},
    DebugOption,
};

pub(crate) fn debug<W: io::Write>(
    f: &mut W,
    paths: &[PathBuf],
    read_stdin: bool,
    dump_ast: &DebugOption,
    verbose: bool,
) -> Result<()> {
    let process_dump_ast = |sql: &str, filename: &str, f: &mut W| -> Result<()> {
        match dump_ast {
            DebugOption::Lex => {
                let tokens = squawk_lexer::tokenize(sql);
                let mut start = 0;
                for token in tokens {
                    if verbose {
                        let content = &sql[start as usize..(start + token.len) as usize];
                        start += token.len;
                        writeln!(f, "{:?} @ {:?}", content, token.kind)?;
                    } else {
                        writeln!(f, "{:?}", token)?;
                    }
                }
            }
            DebugOption::Parse => {
                let parse = squawk_syntax::SourceFile::parse(sql);
                if verbose {
                    writeln!(f, "{}\n---", parse.syntax_node())?;
                }
                writeln!(f, "{:#?}", parse.syntax_node())?;
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
                    writeln!(f, "{}", snap)?;
                    let renderer = Renderer::styled();
                    render_syntax_errors(&errors, filename, sql, |message| {
                        writeln!(f, "{}", renderer.render(message))?;
                        Ok(())
                    })?;
                }
            }
        }
        Ok(())
    };
    if read_stdin {
        let sql = sql_from_stdin()?;
        process_dump_ast(&sql, "stdin", f)?;
        return Ok(());
    }

    for path in paths {
        let sql = sql_from_path(path)?;
        process_dump_ast(&sql, &path.to_string_lossy(), f)?;
    }
    Ok(())
}

fn render_syntax_errors(
    errors: &[SyntaxError],
    filename: &str,
    sql: &str,
    mut render: impl FnMut(Message<'_>) -> Result<()>,
) -> Result<()> {
    for err in errors {
        let text = err.message();
        let span = err.range().into();
        let message = Level::Warning.title(text).id("syntax-error").snippet(
            Snippet::source(sql)
                .origin(filename)
                .fold(true)
                .annotation(Level::Error.span(span)),
        );
        render(message)?;
    }
    Ok(())
}
