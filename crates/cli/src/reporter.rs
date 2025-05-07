use annotate_snippets::Level;
use annotate_snippets::Renderer;
use annotate_snippets::Snippet;
use anyhow::Result;
use console::style;
use line_index::LineIndex;
use line_index::TextRange;
use log::info;
use serde::Serialize;
use squawk_linter::Linter;
use squawk_linter::Rule;
use squawk_linter::Version;
use squawk_syntax::SourceFile;
use std::io;
use std::path::PathBuf;
use std::process::ExitCode;

use crate::{
    file::{sql_from_path, sql_from_stdin},
    Reporter,
};

fn check_sql(
    sql: &str,
    path: &str,
    excluded_rules: &[Rule],
    pg_version: Option<Version>,
    assume_in_transaction: bool,
) -> CheckReport {
    let mut linter = Linter::without_rules(excluded_rules);
    if let Some(pg_version) = pg_version {
        linter.settings.pg_version = pg_version;
    }
    linter.settings.assume_in_transaction = assume_in_transaction;
    let parse = SourceFile::parse(sql);
    let parse_errors = parse.errors();
    let errors = linter.lint(parse, sql);
    let line_index = LineIndex::new(sql);

    let mut violations = Vec::with_capacity(parse_errors.len() + errors.len());

    for e in parse_errors {
        let range_start = e.range().start();
        let line_col = line_index.line_col(range_start);
        violations.push(ReportViolation {
            file: path.to_string(),
            line: line_col.line as usize,
            column: line_col.col as usize,
            level: ViolationLevel::Error,
            help: None,
            range: e.range(),
            message: e.message().to_string(),
            rule_name: "syntax-error".to_string(),
        })
    }
    for e in errors {
        let range_start = e.text_range.start();
        let line_col = line_index.line_col(range_start);
        violations.push(ReportViolation {
            file: path.to_string(),
            line: line_col.line as usize,
            column: line_col.col as usize,
            range: e.text_range,
            help: e.help,
            level: ViolationLevel::Warning,
            message: e.message,
            rule_name: e.code.to_string(),
        })
    }

    CheckReport {
        filename: path.into(),
        sql: sql.into(),
        violations,
    }
}

fn render_lint_error<W: std::io::Write>(
    f: &mut W,
    err: &ReportViolation,
    filename: &str,
    sql: &str,
) -> Result<()> {
    let renderer = Renderer::styled();
    let error_name = &err.rule_name;

    let title = &err.message;

    let mut message = Level::Warning.title(title).id(error_name).snippet(
        Snippet::source(sql)
            .origin(filename)
            .fold(true)
            .annotation(Level::Error.span(err.range.into())),
    );
    if let Some(help) = &err.help {
        message = message.footer(Level::Help.title(help));
    }

    writeln!(f, "{}", renderer.render(message))?;
    Ok(())
}

pub fn check_files(
    path_patterns: &[PathBuf],
    read_stdin: bool,
    stdin_path: Option<String>,
    excluded_rules: &[Rule],
    pg_version: Option<Version>,
    assume_in_transaction: bool,
) -> Result<Vec<CheckReport>> {
    let mut violations = vec![];
    if read_stdin {
        info!("reading content from stdin");
        let sql = sql_from_stdin()?;
        // ignore stdin if it's empty.
        if sql.trim().is_empty() {
            info!("ignoring empty stdin");
        } else {
            let path = stdin_path.unwrap_or_else(|| "stdin".into());
            let content = check_sql(
                &sql,
                &path,
                excluded_rules,
                pg_version,
                assume_in_transaction,
            );
            violations.push(content);
        }
    }

    for path in path_patterns {
        info!("checking file path: {}", path.display());
        let sql = sql_from_path(path)?;
        let content = check_sql(
            &sql,
            path.to_str().unwrap(),
            excluded_rules,
            pg_version,
            assume_in_transaction,
        );
        violations.push(content);
    }

    Ok(violations)
}

pub fn check_and_dump_files<W: io::Write>(
    f: &mut W,
    path_patterns: &[PathBuf],
    read_stdin: bool,
    stdin_path: Option<String>,
    excluded_rules: &[Rule],
    pg_version: Option<Version>,
    assume_in_transaction: bool,
    reporter: &Reporter,
) -> Result<ExitCode> {
    let violations = check_files(
        path_patterns,
        read_stdin,
        stdin_path,
        excluded_rules,
        pg_version,
        assume_in_transaction,
    )?;

    let ok = violations.iter().map(|x| x.violations.len()).sum::<usize>() == 0;

    print_violations(f, violations, reporter)?;

    Ok(if ok {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

struct Summary {
    total_violations: usize,
    files_checked: usize,
    files_with_violations: usize,
}

impl Summary {
    fn from(violations: &[CheckReport]) -> Summary {
        let total_violations: usize = violations.iter().map(|x| x.violations.len()).sum();
        let files_checked = violations.len();
        let files_with_violations = violations
            .iter()
            .filter(|x| !x.violations.is_empty())
            .count();
        Summary {
            total_violations,
            files_checked,
            files_with_violations,
        }
    }
}

fn print_summary<W: io::Write>(f: &mut W, summary: &Summary) -> Result<()> {
    if summary.total_violations == 0 {
        writeln!(
            f,
            "\nFound 0 issues in {files_checked} {files} 🎉",
            files_checked = summary.files_checked,
            files = if summary.files_checked == 1 {
                "file"
            } else {
                "files"
            }
        )?;
    } else {
        writeln!(
            f,
            "\nFind detailed examples and solutions for each rule at {}",
            style("https://squawkhq.com/docs/rules").underlined()
        )?;
        writeln!(
            f,
            "Found {total_violations} issue{plural} in {files_with_violations} file{files_plural} (checked {files_checked} {files_checked_plural})",
            total_violations = summary.total_violations,
            plural = if summary.total_violations == 1 { "" } else { "s" },
            files_with_violations = summary.files_with_violations,
            files_plural = if summary.files_with_violations == 1 { "" } else { "s" },
            files_checked = summary.files_checked,
            files_checked_plural = if summary.files_checked == 1 { "source file" } else { "source files" }
        )?;
    }
    Ok(())
}

#[derive(Debug, Serialize)]
pub enum ViolationLevel {
    Warning,
    Error,
}

impl std::fmt::Display for ViolationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Self::Warning => "warning",
            Self::Error => "error",
        };
        write!(f, "{val}")
    }
}

// TODO: don't use this for json dumps
#[derive(Debug, Serialize)]
pub struct ReportViolation {
    pub file: String,
    pub line: usize,
    pub column: usize,
    #[serde(skip_serializing)]
    pub range: TextRange,
    pub level: ViolationLevel,
    pub message: String,
    pub help: Option<String>,
    pub rule_name: String,
}

fn fmt_gcc<W: io::Write>(f: &mut W, reports: &[CheckReport]) -> Result<()> {
    for report in reports {
        for violation in &report.violations {
            writeln!(
                f,
                "{}:{}:{}: {}: {} {}",
                violation.file,
                violation.line,
                violation.column,
                violation.level,
                violation.rule_name,
                violation.message,
            )?;
        }
    }
    Ok(())
}

pub fn fmt_tty_violation<W: io::Write>(
    f: &mut W,
    violation: &ReportViolation,
    filename: &str,
    sql: &str,
) -> Result<()> {
    render_lint_error(f, violation, filename, sql)?;
    Ok(())
}

pub fn fmt_tty<W: io::Write>(f: &mut W, reports: &[CheckReport]) -> Result<()> {
    let summary = Summary::from(reports);
    for report in reports {
        for violation in &report.violations {
            fmt_tty_violation(f, violation, &report.filename, &report.sql)?;
        }
    }
    print_summary(f, &summary)?;
    Ok(())
}

fn fmt_json<W: io::Write>(f: &mut W, reports: Vec<CheckReport>) -> Result<()> {
    let violations = reports
        .into_iter()
        .flat_map(|x| x.violations)
        .collect::<Vec<_>>();
    let json_str = serde_json::to_string(&violations)?;
    writeln!(f, "{json_str}")?;
    Ok(())
}

#[derive(Debug)]
pub struct CheckReport {
    pub filename: String,
    pub sql: String,
    pub violations: Vec<ReportViolation>,
}

pub fn print_violations<W: io::Write>(
    writer: &mut W,
    reports: Vec<CheckReport>,
    reporter: &Reporter,
) -> Result<()> {
    match reporter {
        Reporter::Gcc => fmt_gcc(writer, &reports),
        Reporter::Json => fmt_json(writer, reports),
        Reporter::Tty => fmt_tty(writer, &reports),
    }
}

#[cfg(test)]
mod test_check_files {
    use super::check_sql;
    use crate::reporter::fmt_json;
    use insta::assert_snapshot;
    use serde_json::Value;

    #[test]
    fn check_files_invalid_syntax() {
        let sql = r"
select \;
        ";
        let mut buff = Vec::new();
        let res = check_sql(sql, "test.sql", &[], None, false);
        fmt_json(&mut buff, vec![res]).unwrap();

        let val: Value = serde_json::from_slice(&buff).unwrap();
        assert_snapshot!(val);
    }
}

#[cfg(test)]
mod test_reporter {
    use super::check_sql;
    use crate::reporter::{print_violations, Reporter};
    use console::strip_ansi_codes;
    use insta::{assert_debug_snapshot, assert_snapshot};

    #[test]
    fn display_violations_gcc() {
        let sql = r#" 
   ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
ALTER TABLE "core_foo" ADD COLUMN "bar" integer NOT NULL;
SELECT 1;
"#;

        let filename = "main.sql";

        let mut buff = Vec::new();

        let res = print_violations(
            &mut buff,
            vec![check_sql(sql, filename, &[], None, false)],
            &Reporter::Gcc,
        );
        assert!(res.is_ok());

        assert_snapshot!(String::from_utf8_lossy(&buff), @r###"
        main.sql:1:29: warning: adding-required-field Adding a new column that is `NOT NULL` and has no default value to an existing table effectively makes it required.
        main.sql:1:29: warning: prefer-robust-stmts Missing `IF NOT EXISTS`, the migration can't be rerun if it fails part way through.
        main.sql:1:46: warning: prefer-bigint-over-int Using 32-bit integer fields can result in hitting the max `int` limit.
        main.sql:2:23: warning: adding-required-field Adding a new column that is `NOT NULL` and has no default value to an existing table effectively makes it required.
        main.sql:2:23: warning: prefer-robust-stmts Missing `IF NOT EXISTS`, the migration can't be rerun if it fails part way through.
        main.sql:2:40: warning: prefer-bigint-over-int Using 32-bit integer fields can result in hitting the max `int` limit.
        "###);
    }

    #[test]
    fn display_violations_tty() {
        let sql = r#" 
   ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
ALTER TABLE "core_foo" ADD COLUMN "bar" integer NOT NULL;
SELECT 1;
"#;
        let filename = "main.sql";
        let mut buff = Vec::new();

        let res = print_violations(
            &mut buff,
            vec![check_sql(sql, filename, &[], None, false)],
            &Reporter::Tty,
        );

        assert!(res.is_ok());
        // remove the color codes so tests behave in CI as they do locally
        assert_snapshot!(strip_ansi_codes(&String::from_utf8_lossy(&buff)));
    }
    #[test]
    fn display_no_violations_tty() {
        let mut buff = Vec::new();
        let sql = "select 1;";

        let res = print_violations(
            &mut buff,
            vec![check_sql(sql, "main.sql", &[], None, false)],
            &Reporter::Tty,
        );

        assert!(res.is_ok());
        // remove the color codes so tests behave in CI as they do locally
        assert_snapshot!(strip_ansi_codes(&String::from_utf8_lossy(&buff)));
    }

    #[test]
    fn display_violations_json() {
        let sql = r#" 
   ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
ALTER TABLE "core_foo" ADD COLUMN "bar" integer NOT NULL;
SELECT 1;
"#;
        let filename = "main.sql";
        let mut buff = Vec::new();

        let res = print_violations(
            &mut buff,
            vec![check_sql(sql, filename, &[], None, false)],
            &Reporter::Json,
        );

        assert!(res.is_ok());
        assert_snapshot!(String::from_utf8_lossy(&buff), @r###"[{"file":"main.sql","line":1,"column":29,"level":"Warning","message":"Adding a new column that is `NOT NULL` and has no default value to an existing table effectively makes it required.","help":"Make the field nullable or add a non-VOLATILE DEFAULT","rule_name":"adding-required-field"},{"file":"main.sql","line":1,"column":29,"level":"Warning","message":"Missing `IF NOT EXISTS`, the migration can't be rerun if it fails part way through.","help":null,"rule_name":"prefer-robust-stmts"},{"file":"main.sql","line":1,"column":46,"level":"Warning","message":"Using 32-bit integer fields can result in hitting the max `int` limit.","help":"Use 64-bit integer values instead to prevent hitting this limit.","rule_name":"prefer-bigint-over-int"},{"file":"main.sql","line":2,"column":23,"level":"Warning","message":"Adding a new column that is `NOT NULL` and has no default value to an existing table effectively makes it required.","help":"Make the field nullable or add a non-VOLATILE DEFAULT","rule_name":"adding-required-field"},{"file":"main.sql","line":2,"column":23,"level":"Warning","message":"Missing `IF NOT EXISTS`, the migration can't be rerun if it fails part way through.","help":null,"rule_name":"prefer-robust-stmts"},{"file":"main.sql","line":2,"column":40,"level":"Warning","message":"Using 32-bit integer fields can result in hitting the max `int` limit.","help":"Use 64-bit integer values instead to prevent hitting this limit.","rule_name":"prefer-bigint-over-int"}]"###);
    }

    #[test]
    fn span_offsets() {
        let sql = r#"

   ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
ALTER TABLE "core_foo" ADD COLUMN "bar" integer NOT NULL;
SELECT 1;
"#;
        let filename = "main.sql";
        assert_debug_snapshot!(check_sql(sql, filename, &[], None, false));
    }
}
