use crate::error::PGQueryError;
use crate::parse::{parse_sql_query, parse_sql_query_json};
use crate::rules::{
    check_sql, CheckSQLError, RuleViolation, RuleViolationKind, Span, SquawkRule, ViolationMessage,
    RULES,
};
use console::{strip_ansi_codes, style};
use serde::Serialize;
use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use structopt::clap::arg_enum;
use structopt::StructOpt;

fn get_sql_from_path(path: &str) -> Option<String> {
    if path == "-" {
        return get_sql_from_stdin().ok();
    }
    if let Ok(mut file) = File::open(path) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).ok().map(|_| contents)
    } else {
        None
    }
}

arg_enum! {
    #[derive(Debug, StructOpt)]
    pub enum DumpAstOption {
        Raw,
        Parsed,
    }
}

#[derive(Debug)]
pub enum DumpAstError {
    PGQueryError(PGQueryError),
    IoError(std::io::Error),
    JsonError(serde_json::error::Error),
}

impl std::convert::From<PGQueryError> for DumpAstError {
    fn from(e: PGQueryError) -> Self {
        Self::PGQueryError(e)
    }
}

impl std::convert::From<std::io::Error> for DumpAstError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl std::convert::From<serde_json::error::Error> for DumpAstError {
    fn from(e: serde_json::error::Error) -> Self {
        Self::JsonError(e)
    }
}

pub fn dump_ast_for_paths<W: io::Write>(
    f: &mut W,
    paths: &[String],
    dump_ast: DumpAstOption,
) -> Result<(), DumpAstError> {
    for path in paths {
        if let Some(sql) = get_sql_from_path(path) {
            match dump_ast {
                DumpAstOption::Raw => {
                    let json_ast = parse_sql_query_json(&sql)?;
                    let json_str = serde_json::to_string(&json_ast)?;
                    writeln!(f, "{}", json_str)?;
                }
                DumpAstOption::Parsed => {
                    let ast = parse_sql_query(&sql)?;
                    let ast_str = serde_json::to_string(&ast)?;
                    writeln!(f, "{}", ast_str)?;
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub enum CheckFilesError {
    CheckSQL(CheckSQLError),
    IoError(std::io::Error),
}

impl std::convert::From<std::io::Error> for CheckFilesError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl std::convert::From<CheckSQLError> for CheckFilesError {
    fn from(e: CheckSQLError) -> Self {
        Self::CheckSQL(e)
    }
}

pub fn check_files<W: io::Write>(
    f: &mut W,
    paths: &[String],
    reporter: Reporter,
    excluded_rules: Option<Vec<String>>,
) -> Result<bool, CheckFilesError> {
    let mut found_errors = false;
    let excluded_rules = excluded_rules.unwrap_or_else(|| vec![]);
    for path in paths {
        if let Some(sql) = get_sql_from_path(path) {
            let violations = check_sql(&sql, &excluded_rules)?;
            if !violations.is_empty() {
                found_errors = true
            }
            let filename = if path == "-" { "stdin" } else { &path };
            print_violations(f, &pretty_violations(violations, &sql, filename), &reporter)?;
        }
    }
    Ok(found_errors)
}

fn get_sql_from_stdin() -> Result<String, io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

arg_enum! {
    #[derive(Debug, StructOpt)]
    pub enum Reporter {
        Tty,
        Gcc,
        Json,
    }
}

#[derive(Debug, Serialize)]
pub enum ViolationLevel {
    Error,
    Warning,
}

impl std::fmt::Display for ViolationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Self::Error => "error",
            Self::Warning => "warning",
        };
        write!(f, "{}", val)
    }
}

#[derive(Debug, Serialize)]
pub struct ReportViolation {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub level: ViolationLevel,
    pub messages: Vec<ViolationMessage>,
    pub rule_name: RuleViolationKind,
    #[serde(skip_serializing)]
    pub sql: String,
}

fn fmt_gcc<W: io::Write>(
    f: &mut W,
    violations: &[ReportViolation],
) -> std::result::Result<(), std::io::Error> {
    for violation in violations {
        let message = violation
            .messages
            .iter()
            .map(|v| {
                match v {
                    ViolationMessage::Note(s) => s,
                    ViolationMessage::Help(s) => s,
                }
                .to_string()
            })
            .collect::<Vec<String>>()
            .join(" ");
        writeln!(
            f,
            "{}:{}:{}: {}: {} {}",
            violation.file,
            violation.line,
            violation.column,
            violation.level,
            violation.rule_name,
            message
        )?;
    }
    Ok(())
}

fn fmt_tty<W: io::Write>(
    f: &mut W,
    violations: &[ReportViolation],
) -> std::result::Result<(), std::io::Error> {
    for violation in violations {
        writeln!(
            f,
            "{}:{}:{}: {}: {}",
            violation.file,
            violation.line,
            violation.column,
            style(format!("{}", violation.level)).yellow(),
            violation.rule_name
        )?;

        writeln!(f)?;
        for (i, line) in violation.sql.lines().enumerate() {
            // TODO(sbdchd): handle the transition from 2 digits to 3
            writeln!(f, "  {: >2} | {}", violation.line + i, line)?;
        }
        writeln!(f)?;
        for msg in &violation.messages {
            match msg {
                ViolationMessage::Note(note) => {
                    writeln!(f, "  {}: {}", style("note").bold(), note)?;
                }
                ViolationMessage::Help(help) => {
                    writeln!(f, "  {}: {}", style("help").bold(), help)?;
                }
            }
        }
        writeln!(f)?;
    }
    Ok(())
}

fn fmt_json<W: io::Write>(
    f: &mut W,
    violations: &[ReportViolation],
) -> std::result::Result<(), std::io::Error> {
    let json_str = serde_json::to_string(&violations)?;
    writeln!(f, "{}", json_str)
}

pub fn pretty_violations(
    violations: Vec<RuleViolation>,
    sql: &str,
    filename: &str,
) -> Vec<ReportViolation> {
    violations
        .into_iter()
        .map(|violation| {
            // NOTE: the span information from postgres includes the preceeding
            // whitespace for nodes.
            let Span { start, len } = violation.span;

            let start = start as usize;

            let len = len.unwrap_or(0) as usize;

            // 1-indexed
            let lineno = sql[..start].lines().count() + 1;

            let content = &sql[start..start + len + 1];

            // TODO(sbdchd): could remove the leading whitespace and comments to
            // get cleaner reports

            let col = content.find(|c: char| c != '\n').unwrap_or(0);

            let problem_sql = &sql[start + 1..start + len + 1];

            ReportViolation {
                file: filename.into(),
                line: lineno,
                column: col,
                level: ViolationLevel::Warning,
                messages: violation.messages,
                rule_name: violation.kind,
                sql: problem_sql.into(),
            }
        })
        .collect()
}

pub fn print_violations<W: io::Write>(
    writer: &mut W,
    violations: &[ReportViolation],
    reporter: &Reporter,
) -> Result<(), std::io::Error> {
    match reporter {
        Reporter::Gcc => fmt_gcc(writer, violations),
        Reporter::Json => fmt_json(writer, violations),
        Reporter::Tty => fmt_tty(writer, violations),
    }
}

pub fn list_rules<W: io::Write>(writer: &mut W) -> Result<(), std::io::Error> {
    for r in RULES.iter() {
        output_rule_info(writer, r)?;
    }
    Ok(())
}

fn output_rule_info<W: io::Write>(writer: &mut W, rule: &SquawkRule) -> Result<(), std::io::Error> {
    writeln!(writer, "{}", rule.name)?;
    for msg in &rule.messages {
        let msg_content = match msg {
            ViolationMessage::Note(s) => format!("note: {}", s),
            ViolationMessage::Help(s) => format!("help: {}", s),
        };
        writeln!(writer, "    {}", msg_content)?;
    }
    Ok(())
}

pub fn explain_rule<W: io::Write>(writer: &mut W, name: &str) -> Result<(), std::io::Error> {
    if let Ok(name) = RuleViolationKind::try_from(name) {
        if let Some(r) = RULES.iter().find(|r| r.name == name) {
            output_rule_info(writer, r)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod test_reporter {
    use super::*;

    use crate::rules::check_sql;
    use insta::{assert_debug_snapshot, assert_display_snapshot};

    #[test]
    fn test_display_violations_gcc() {
        let sql = r#" 
   ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
ALTER TABLE "core_foo" ADD COLUMN "bar" integer NOT NULL;
SELECT 1;
"#;
        let violations = check_sql(&sql, &[]).expect("valid sql should parse");

        let filename = "main.sql";

        let mut buff = Vec::new();

        let res = print_violations(
            &mut buff,
            &pretty_violations(violations, &sql, filename),
            &Reporter::Gcc,
        );
        assert!(res.is_ok());

        assert_display_snapshot!(String::from_utf8_lossy(&buff), @r###"
        main.sql:1:0: warning: adding-not-nullable-field Adding a NOT NULL field requires exclusive locks and table rewrites. Make the field nullable.
        main.sql:3:1: warning: adding-not-nullable-field Adding a NOT NULL field requires exclusive locks and table rewrites. Make the field nullable.
        "###);
    }

    #[test]
    fn test_display_violations_tty() {
        let sql = r#" 
   ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
ALTER TABLE "core_foo" ADD COLUMN "bar" integer NOT NULL;
SELECT 1;
"#;
        let violations = check_sql(&sql, &[]).expect("valid sql should parse");
        let filename = "main.sql";
        let mut buff = Vec::new();

        let res = print_violations(
            &mut buff,
            &pretty_violations(violations, &sql, filename),
            &Reporter::Tty,
        );

        assert!(res.is_ok());
        // remove the color codes so tests behave in CI as they do locally
        assert_display_snapshot!(strip_ansi_codes(&String::from_utf8_lossy(&buff)));
    }
    #[test]
    fn test_display_violations_json() {
        let sql = r#" 
   ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
ALTER TABLE "core_foo" ADD COLUMN "bar" integer NOT NULL;
SELECT 1;
"#;
        let violations = check_sql(&sql, &[]).expect("valid sql should parse");
        let filename = "main.sql";
        let mut buff = Vec::new();

        let res = print_violations(
            &mut buff,
            &pretty_violations(violations, &sql, filename),
            &Reporter::Json,
        );

        assert!(res.is_ok());
        assert_display_snapshot!(String::from_utf8_lossy(&buff), @r###"[{"file":"main.sql","line":1,"column":0,"level":"Warning","messages":[{"Note":"Adding a NOT NULL field requires exclusive locks and table rewrites."},{"Help":"Make the field nullable."}],"rule_name":"AddingNotNullableField"},{"file":"main.sql","line":3,"column":1,"level":"Warning","messages":[{"Note":"Adding a NOT NULL field requires exclusive locks and table rewrites."},{"Help":"Make the field nullable."}],"rule_name":"AddingNotNullableField"}]
"###);
    }

    #[test]
    fn test_span_offsets() {
        let sql = r#"

   ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
ALTER TABLE "core_foo" ADD COLUMN "bar" integer NOT NULL;
SELECT 1;
"#;
        let violations = check_sql(&sql, &[]).expect("valid sql should parse");

        let filename = "main.sql";
        assert_debug_snapshot!(pretty_violations(violations, &sql, filename));
    }
}
