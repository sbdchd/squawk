use std::fmt;
use std::io::{self, BufWriter, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
use anyhow::Result;
use console::style;
use log::info;
use rayon::prelude::*;
use similar::{ChangeTag, TextDiff};
use squawk_fmt::FormatOptions;
use squawk_lexer::BOM;
use squawk_line_index::find_newline;
use squawk_syntax::SourceFile;
use squawk_syntax::syntax_error::SyntaxError;

use crate::cmd::Input;
use crate::file::{sql_from_path, sql_from_stdin};

const ERROR_EXIT_CODE: u8 = 2;

#[derive(Debug, Clone, Copy)]
pub(crate) enum FormatMode {
    Write,
    Check,
    Diff,
}

pub(crate) struct FormatConfig {
    pub(crate) input: Input,
    pub(crate) mode: FormatMode,
    pub(crate) options: FormatOptions,
    pub(crate) exit_non_zero_on_format: bool,
}

enum Formatted {
    Unchanged,
    Changed(String),
}

enum FormatError {
    Read(anyhow::Error),
    Parse {
        sql: String,
        errors: Vec<SyntaxError>,
    },
    Format(anyhow::Error),
    Write(io::Error),
}

enum FileOutcome {
    Unchanged,
    Changed,
    Diff(String),
}

struct FileResult<'a> {
    path: &'a Path,
    outcome: Result<FileOutcome, FormatError>,
}

pub(crate) fn format(config: &FormatConfig) -> Result<ExitCode> {
    match &config.input {
        Input::Stdin(stdin) => format_stdin(config, stdin.path.as_deref()),
        Input::Paths(paths) => format_paths(config, paths),
    }
}

fn format_source(source: &str, options: FormatOptions) -> Result<Formatted, FormatError> {
    let parse = SourceFile::parse(source);
    let errors = parse.errors();
    if !errors.is_empty() {
        return Err(FormatError::Parse {
            sql: source.to_owned(),
            errors,
        });
    }
    let line_ending = find_newline(source)
        .map(|(_, ending)| ending)
        .unwrap_or_default();
    let formatted =
        squawk_fmt::fmt(&parse.tree(), line_ending, options).map_err(FormatError::Format)?;
    if formatted == source {
        return Ok(Formatted::Unchanged);
    }
    Ok(Formatted::Changed(formatted))
}

fn format_stdin(config: &FormatConfig, path: Option<&str>) -> Result<ExitCode> {
    info!("reading content from stdin");
    let display_path = path.unwrap_or("stdin");
    let source = match sql_from_stdin() {
        Ok(source) => source,
        Err(err) => {
            report_error(display_path, &FormatError::Read(err))?;
            return Ok(ExitCode::from(ERROR_EXIT_CODE));
        }
    };
    let formatted = match format_source(&source, config.options) {
        Ok(formatted) => formatted,
        Err(err) => {
            report_error(display_path, &err)?;
            return Ok(ExitCode::from(ERROR_EXIT_CODE));
        }
    };
    let mut stdout = BufWriter::new(io::stdout().lock());
    let exit_code = match (formatted, config.mode) {
        (Formatted::Unchanged, FormatMode::Write) => {
            stdout.write_all(source.as_bytes())?;
            ExitCode::SUCCESS
        }
        (Formatted::Unchanged, FormatMode::Check | FormatMode::Diff) => ExitCode::SUCCESS,
        (Formatted::Changed(formatted), FormatMode::Write) => {
            stdout.write_all(formatted.as_bytes())?;
            ExitCode::SUCCESS
        }
        (Formatted::Changed(_), FormatMode::Check) => ExitCode::FAILURE,
        (Formatted::Changed(formatted), FormatMode::Diff) => {
            stdout.write_all(render_diff(&source, &formatted, path).as_bytes())?;
            ExitCode::FAILURE
        }
    };
    stdout.flush()?;
    Ok(exit_code)
}

fn format_path(
    path: &PathBuf,
    mode: FormatMode,
    options: FormatOptions,
) -> Result<FileOutcome, FormatError> {
    info!("formatting file path: {}", path.display());
    let source = sql_from_path(path).map_err(FormatError::Read)?;
    match format_source(&source, options)? {
        Formatted::Unchanged => Ok(FileOutcome::Unchanged),
        Formatted::Changed(formatted) => match mode {
            FormatMode::Write => {
                std::fs::write(path, formatted).map_err(FormatError::Write)?;
                Ok(FileOutcome::Changed)
            }
            FormatMode::Check => Ok(FileOutcome::Changed),
            FormatMode::Diff => Ok(FileOutcome::Diff(render_diff(
                &source,
                &formatted,
                Some(&path.to_string_lossy()),
            ))),
        },
    }
}

fn format_paths(config: &FormatConfig, paths: &[PathBuf]) -> Result<ExitCode> {
    let results: Vec<FileResult> = paths
        .par_iter()
        .map(|path| FileResult {
            path,
            outcome: format_path(path, config.mode, config.options),
        })
        .collect();

    let mut stdout = BufWriter::new(io::stdout().lock());
    let mut summary = Summary {
        mode: config.mode,
        changed: 0,
        unchanged: 0,
    };
    let mut has_errors = false;
    for result in &results {
        let display_path = result.path.to_string_lossy();
        match &result.outcome {
            Ok(FileOutcome::Unchanged) => summary.unchanged += 1,
            Ok(FileOutcome::Changed) => {
                summary.changed += 1;
                if let FormatMode::Check = config.mode {
                    writeln!(stdout, "Would reformat: {display_path}")?;
                }
            }
            Ok(FileOutcome::Diff(diff)) => {
                summary.changed += 1;
                stdout.write_all(diff.as_bytes())?;
            }
            Err(err) => {
                has_errors = true;
                report_error(&display_path, err)?;
            }
        }
    }

    match config.mode {
        FormatMode::Diff => {
            stdout.flush()?;
            summary.write(&mut io::stderr().lock())?;
        }
        FormatMode::Write | FormatMode::Check => summary.write(&mut stdout)?,
    }
    stdout.flush()?;

    if has_errors {
        return Ok(ExitCode::from(ERROR_EXIT_CODE));
    }
    let fail_on_changes = match config.mode {
        FormatMode::Write => config.exit_non_zero_on_format,
        FormatMode::Check | FormatMode::Diff => true,
    };
    if fail_on_changes && summary.changed > 0 {
        return Ok(ExitCode::FAILURE);
    }
    Ok(ExitCode::SUCCESS)
}

struct Summary {
    mode: FormatMode,
    changed: usize,
    unchanged: usize,
}

impl Summary {
    fn write(&self, f: &mut impl Write) -> io::Result<()> {
        let (changed_label, unchanged_label) = match self.mode {
            FormatMode::Write => ("reformatted", "left unchanged"),
            FormatMode::Check | FormatMode::Diff => ("would be reformatted", "already formatted"),
        };
        let changed = format_count(self.changed, changed_label);
        let unchanged = format_count(self.unchanged, unchanged_label);
        match (self.changed, self.unchanged) {
            (0, 0) => Ok(()),
            (_, 0) => writeln!(f, "{changed}"),
            (0, _) => writeln!(f, "{unchanged}"),
            (_, _) => writeln!(f, "{changed}, {unchanged}"),
        }
    }
}

fn format_count(count: usize, label: &str) -> String {
    let plural = if count == 1 { "" } else { "s" };
    format!("{count} file{plural} {label}")
}

fn render_diff(source: &str, formatted: &str, path: Option<&str>) -> String {
    let mut out = String::with_capacity(source.len() + formatted.len());
    write_diff(&mut out, source, formatted, path).expect("writing to a String can't fail");
    out
}

fn write_diff(
    f: &mut impl fmt::Write,
    source: &str,
    formatted: &str,
    path: Option<&str>,
) -> fmt::Result {
    let diff = TextDiff::from_lines(source, formatted);
    if let Some(path) = path {
        writeln!(f, "{}", style(format_args!("--- {path}")).red())?;
        writeln!(f, "{}", style(format_args!("+++ {path}")).green())?;
    }
    for hunk in diff.unified_diff().iter_hunks() {
        writeln!(f, "{}", hunk.header())?;
        for change in hunk.iter_changes() {
            let value = change.value();
            match change.tag() {
                ChangeTag::Equal => write!(f, " {value}")?,
                ChangeTag::Delete => write!(f, "{}{}", style("-").red(), style(value).red())?,
                ChangeTag::Insert => {
                    write!(f, "{}{}", style("+").green(), style(value).green())?;
                }
            }
            if change.missing_newline() {
                writeln!(f)?;
                writeln!(f, "\\ No newline at end of file")?;
            }
        }
    }
    Ok(())
}

fn report_error(path: &str, err: &FormatError) -> io::Result<()> {
    let mut stderr = io::stderr().lock();
    let (action, message) = match err {
        FormatError::Read(err) => ("read", err.to_string()),
        FormatError::Format(err) => ("format", err.to_string()),
        FormatError::Write(err) => ("write", err.to_string()),
        FormatError::Parse { sql, errors } => {
            let renderer = if console::colors_enabled_stderr() {
                Renderer::styled()
            } else {
                Renderer::plain()
            }
            .decor_style(DecorStyle::Unicode);
            let (offset, sql) = match sql.strip_prefix(BOM) {
                Some(sql) => (BOM.len(), sql),
                None => (0, sql.as_str()),
            };
            for error in errors {
                let range: Range<usize> = error.range().into();
                let span = range.start.saturating_sub(offset)..range.end.saturating_sub(offset);
                let snippet = Snippet::source(sql)
                    .path(path)
                    .fold(true)
                    .annotation(AnnotationKind::Primary.span(span));
                let mut group = Level::ERROR
                    .primary_title(error.message())
                    .id("syntax-error")
                    .element(snippet);
                if let Some(hint) = error.hint() {
                    group = group.element(Level::HELP.message(hint));
                }
                writeln!(stderr, "{}", renderer.render(&[group]))?;
            }
            return Ok(());
        }
    };
    writeln!(
        stderr,
        "{}: Failed to {action} {path}: {message}",
        style("error").red().bold().for_stderr()
    )
}
