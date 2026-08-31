use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
use anyhow::Result;
use clap::Parser;
use squawk_fmt::token_compare::assert_no_dropped_tokens;
use squawk_syntax::SourceFile;

#[derive(Parser)]
#[command(name = "squawk-fmt")]
struct Cli {
    /// File to format; reads from stdin if omitted
    file: Option<PathBuf>,
}

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();

    let (input, path) = match cli.file {
        Some(path) => {
            let input = std::fs::read_to_string(&path)?;
            (input, path.display().to_string())
        }
        None => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            (buf, "stdin".to_string())
        }
    };

    let parse = SourceFile::parse(&input);
    let errors = parse.errors();
    if !errors.is_empty() {
        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        let stderr = io::stderr();
        let mut stderr = stderr.lock();

        for error in errors {
            let snippet = Snippet::source(&input)
                .path(&path)
                .fold(true)
                .annotation(AnnotationKind::Primary.span(error.range().into()));
            let group = Level::ERROR
                .primary_title(error.message())
                .id("syntax-error")
                .element(snippet);
            writeln!(stderr, "{}", renderer.render(&[group]))?;
        }

        return Ok(ExitCode::FAILURE);
    }

    let formatted = squawk_fmt::fmt_str(&input)?;
    assert_no_dropped_tokens(&input, &formatted);

    let reparse = SourceFile::parse(&formatted);
    assert!(
        reparse.errors().is_empty(),
        "formatted output has syntax errors:\n{}\n\nformatted output:\n{formatted}",
        reparse
            .errors()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );

    write!(io::stdout().lock(), "{formatted}")?;
    Ok(ExitCode::SUCCESS)
}
