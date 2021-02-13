use console::strip_ansi_codes;
use console::style;
use log::info;
use serde::Serialize;
use squawk_linter::errors::CheckSQLError;
use squawk_linter::violations::{RuleViolation, RuleViolationKind, Span, ViolationMessage};
use squawk_linter::{check_sql, SquawkRule, RULES};
use squawk_parser::error::PGQueryError;
use squawk_parser::parse::{parse_sql_query, parse_sql_query_json};
use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use structopt::clap::arg_enum;
use structopt::StructOpt;

fn get_sql_from_path(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map(|_| contents)
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
    PGQuery(PGQueryError),
    Io(std::io::Error),
    Json(serde_json::error::Error),
}

impl std::convert::From<PGQueryError> for DumpAstError {
    fn from(e: PGQueryError) -> Self {
        Self::PGQuery(e)
    }
}

impl std::convert::From<std::io::Error> for DumpAstError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl std::convert::From<serde_json::error::Error> for DumpAstError {
    fn from(e: serde_json::error::Error) -> Self {
        Self::Json(e)
    }
}

pub fn dump_ast_for_paths<W: io::Write>(
    f: &mut W,
    paths: &[String],
    is_stdin: bool,
    dump_ast: &DumpAstOption,
) -> Result<(), DumpAstError> {
    let mut process_dump_ast = |sql: &str| -> Result<(), DumpAstError> {
        match dump_ast {
            DumpAstOption::Raw => {
                let json_ast = parse_sql_query_json(sql)?;
                let json_str = serde_json::to_string(&json_ast)?;
                writeln!(f, "{}", json_str)?;
            }
            DumpAstOption::Parsed => {
                let ast = parse_sql_query(sql)?;
                let ast_str = serde_json::to_string(&ast)?;
                writeln!(f, "{}", ast_str)?;
            }
        }
        Ok(())
    };
    if is_stdin {
        let sql = get_sql_from_stdin()?;
        process_dump_ast(&sql)?;
        return Ok(());
    }

    for path in paths {
        let sql = get_sql_from_path(path)?;
        process_dump_ast(&sql)?;
    }
    Ok(())
}

#[derive(Debug)]
pub enum CheckFilesError {
    CheckSQL(CheckSQLError),
    IoError(std::io::Error),
}

impl std::fmt::Display for CheckFilesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::CheckSQL(ref err) => {
                write!(f, "{}", format!("Problem linting SQL files: {}", err))
            }
            Self::IoError(ref err) => err.fmt(f),
        }
    }
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

pub fn check_files(
    paths: &[String],
    is_stdin: bool,
    stdin_path: Option<String>,
    excluded_rules: Option<Vec<String>>,
) -> Result<Vec<ViolationContent>, CheckFilesError> {
    let excluded_rules = excluded_rules.unwrap_or_else(Vec::new);

    let mut output_violations = vec![];

    let mut process_violations = |sql: &str, path: &str| -> Result<(), CheckFilesError> {
        let violations = check_sql(sql, &excluded_rules)?;
        output_violations.push(pretty_violations(violations, sql, path));
        Ok(())
    };

    if is_stdin {
        info!("reading content from stdin");
        let sql = get_sql_from_stdin()?;
        let path = stdin_path.unwrap_or_else(|| "stdin".into());
        process_violations(&sql, &path)?;
    }

    for path in paths {
        info!("checking file path: {}", path);
        let sql = get_sql_from_path(path)?;
        process_violations(&sql, path)?;
    }
    Ok(output_violations)
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
    // don't output in JSON format
    #[serde(skip_serializing)]
    pub sql: String,
}

fn fmt_gcc<W: io::Write>(
    f: &mut W,
    files: &[ViolationContent],
) -> std::result::Result<(), std::io::Error> {
    for file in files {
        for violation in &file.violations {
            let message = violation
                .messages
                .iter()
                .map(|v| {
                    match v {
                        ViolationMessage::Note(s) | ViolationMessage::Help(s) => s,
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
    }
    Ok(())
}

pub fn fmt_tty_violation<W: io::Write>(
    f: &mut W,
    violation: &ReportViolation,
) -> Result<(), std::io::Error> {
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
    writeln!(f)
}

pub fn fmt_tty<W: io::Write>(f: &mut W, files: &[ViolationContent]) -> Result<(), std::io::Error> {
    for file in files {
        for violation in &file.violations {
            fmt_tty_violation(f, violation)?;
        }
    }
    let total_violations = files.iter().map(|f| f.violations.len()).sum::<usize>();
    let files_checked = files.len();
    if total_violations == 0 {
        writeln!(
            f,
            "Found 0 issues in {files_checked} {files} 🎉",
            files_checked = files_checked,
            files = if files_checked == 1 { "file" } else { "files" }
        )?;
    }
    Ok(())
}

fn fmt_json<W: io::Write>(
    f: &mut W,
    files: Vec<ViolationContent>,
) -> std::result::Result<(), std::io::Error> {
    let violations = files
        .into_iter()
        .flat_map(|x| x.violations)
        .collect::<Vec<_>>();
    let json_str = serde_json::to_string(&violations)?;
    writeln!(f, "{}", json_str)
}

#[derive(Debug)]
pub struct ViolationContent {
    pub filename: String,
    pub sql: String,
    pub violations: Vec<ReportViolation>,
}

pub fn pretty_violations(
    violations: Vec<RuleViolation>,
    sql: &str,
    filename: &str,
) -> ViolationContent {
    ViolationContent {
        filename: filename.into(),
        sql: sql.into(),
        violations: violations
            .into_iter()
            .map(|violation| {
                // NOTE: the span information from postgres includes the preceeding
                // whitespace for nodes.
                let Span { start, len } = violation.span;

                let start = start as usize;

                let len = len.unwrap_or(0) as usize;

                // 1-indexed
                let lineno = sql[..start].lines().count() + 1;

                let content = &sql[start..=start + len];

                // TODO(sbdchd): could remove the leading whitespace and comments to
                // get cleaner reports

                let col = content.find(|c: char| c != '\n').unwrap_or(0);

                // slice off the beginning new lines
                let problem_sql = &content[col..];

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
            .collect(),
    }
}

pub fn print_violations<W: io::Write>(
    writer: &mut W,
    file_reports: Vec<ViolationContent>,
    reporter: &Reporter,
) -> Result<(), std::io::Error> {
    match reporter {
        Reporter::Gcc => fmt_gcc(writer, &file_reports),
        Reporter::Json => fmt_json(writer, file_reports),
        Reporter::Tty => fmt_tty(writer, &file_reports),
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

const fn get_violations_emoji(count: usize) -> &'static str {
    if count > 0 {
        "🚒"
    } else {
        "✅"
    }
}

fn get_sql_file_content(violation: ViolationContent) -> Result<String, std::io::Error> {
    let sql = violation.sql;
    let mut buff = Vec::new();
    let violation_count = violation.violations.len();
    for v in violation.violations {
        fmt_tty_violation(&mut buff, &v)?;
    }
    let violations_text_raw = &String::from_utf8_lossy(&buff);
    let violations_text = strip_ansi_codes(violations_text_raw);

    let violation_content = if violation_count > 0 {
        format!(
            r#"
```
{}
```"#,
            violations_text.trim_matches('\n')
        )
    } else {
        "No violations found.".to_string()
    };

    let violations_emoji = get_violations_emoji(violation_count);

    Ok(format!(
        r#"
<h3><code>{filename}</code></h3>

```sql
{sql}
```

<h4>{violations_emoji} Rule Violations ({violation_count})</h4>

{violation_content}
    
---
    "#,
        violations_emoji = violations_emoji,
        filename = violation.filename,
        sql = sql,
        violation_count = violation_count,
        violation_content = violation_content
    ))
}

pub fn get_comment_body(files: Vec<ViolationContent>) -> String {
    let violations_count: usize = files.iter().map(|x| x.violations.len()).sum();

    let violations_emoji = get_violations_emoji(violations_count);

    format!(
        r#"
# Squawk Report

### **{violations_emoji} {violation_count}** violations across **{file_count}** file(s)

---
{sql_file_content}

[📚 More info on rules](https://github.com/sbdchd/squawk#rules)

⚡️ Powered by [`Squawk`](https://github.com/sbdchd/squawk), a linter for PostgreSQL, focused on migrations
"#,
        violations_emoji = violations_emoji,
        violation_count = violations_count,
        file_count = files.len(),
        sql_file_content = files
            .into_iter()
            .flat_map(|x| get_sql_file_content(x).ok())
            .collect::<Vec<String>>()
            .join("\n")
    )
    .trim_matches('\n')
    .into()
}

#[cfg(test)]
mod test_github_comment {
    use super::*;

    use insta::assert_display_snapshot;

    /// Most cases, hopefully, will be a single migration for a given PR, but
    /// let's check the case of multiple migrations
    #[test]
    fn test_generating_comment_multiple_files() {
        let violations = vec![ViolationContent {
            filename: "alpha.sql".into(),
            sql: r#"
SELECT 1;
                "#
            .into(),
            violations: vec![ReportViolation {
                file: "alpha.sql".into(),
                line: 1,
                column: 0,
                level: ViolationLevel::Warning,
                messages: vec![
                    ViolationMessage::Note(
                        "Adding a NOT NULL field requires exclusive locks and table rewrites."
                            .into(),
                    ),
                    ViolationMessage::Help("Make the field nullable.".into()),
                ],
                rule_name: RuleViolationKind::AddingNotNullableField,
                sql: "ALTER TABLE \"core_recipe\" ADD COLUMN \"foo\" integer NOT NULL;".into(),
            }],
        }];

        let body = get_comment_body(violations);

        assert_display_snapshot!(body, @r###"
# Squawk Report

### **🚒 1** violations across **1** file(s)

---

<h3><code>alpha.sql</code></h3>

```sql

SELECT 1;
                
```

<h4>🚒 Rule Violations (1)</h4>


```
alpha.sql:1:0: warning: adding-not-nullable-field

   1 | ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;

  note: Adding a NOT NULL field requires exclusive locks and table rewrites.
  help: Make the field nullable.
```
    
---
    

[📚 More info on rules](https://github.com/sbdchd/squawk#rules)

⚡️ Powered by [`Squawk`](https://github.com/sbdchd/squawk), a linter for PostgreSQL, focused on migrations
"###);
    }

    /// Even when we don't have violations we still want to output the SQL for
    /// easy human reading.
    #[test]
    fn test_generating_comment_no_violations() {
        let violations = vec![
            ViolationContent {
                filename: "alpha.sql".into(),
                sql: r#"
BEGIN;
--
-- Create model Bar
--
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "alpha" varchar(100) NOT NULL
);
                "#
                .into(),
                violations: vec![],
            },
            ViolationContent {
                filename: "bravo.sql".into(),
                sql: r#"
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
                "#
                .into(),
                violations: vec![],
            },
        ];

        let body = get_comment_body(violations);

        assert_display_snapshot!(body, @r###"
# Squawk Report

### **✅ 0** violations across **2** file(s)

---

<h3><code>alpha.sql</code></h3>

```sql

BEGIN;
--
-- Create model Bar
--
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "alpha" varchar(100) NOT NULL
);
                
```

<h4>✅ Rule Violations (0)</h4>

No violations found.
    
---
    

<h3><code>bravo.sql</code></h3>

```sql

ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
                
```

<h4>✅ Rule Violations (0)</h4>

No violations found.
    
---
    

[📚 More info on rules](https://github.com/sbdchd/squawk#rules)

⚡️ Powered by [`Squawk`](https://github.com/sbdchd/squawk), a linter for PostgreSQL, focused on migrations
"###);
    }

    /// Ideally the logic won't leave a comment when there are no migrations but
    /// better safe than sorry
    #[test]
    fn test_generating_no_violations_no_files() {
        let violations = vec![];

        let body = get_comment_body(violations);

        assert_display_snapshot!(body, @r###"
# Squawk Report

### **✅ 0** violations across **0** file(s)

---


[📚 More info on rules](https://github.com/sbdchd/squawk#rules)

⚡️ Powered by [`Squawk`](https://github.com/sbdchd/squawk), a linter for PostgreSQL, focused on migrations
"###);
    }
}

#[cfg(test)]
mod test_reporter {
    use super::*;

    use insta::{assert_debug_snapshot, assert_display_snapshot};

    #[test]
    fn test_display_violations_gcc() {
        let sql = r#" 
   ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
ALTER TABLE "core_foo" ADD COLUMN "bar" integer NOT NULL;
SELECT 1;
"#;
        let violations =
            check_sql(sql, &["prefer-robust-stmts".into()]).expect("valid sql should parse");

        let filename = "main.sql";

        let mut buff = Vec::new();

        let res = print_violations(
            &mut buff,
            vec![pretty_violations(violations, sql, filename)],
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
        let violations =
            check_sql(sql, &["prefer-robust-stmts".into()]).expect("valid sql should parse");
        let filename = "main.sql";
        let mut buff = Vec::new();

        let res = print_violations(
            &mut buff,
            vec![pretty_violations(violations, sql, filename)],
            &Reporter::Tty,
        );

        assert!(res.is_ok());
        // remove the color codes so tests behave in CI as they do locally
        assert_display_snapshot!(strip_ansi_codes(&String::from_utf8_lossy(&buff)));
    }
    #[test]
    fn test_display_no_violations_tty() {
        let mut buff = Vec::new();

        let res = print_violations(
            &mut buff,
            vec![pretty_violations(vec![], "", "main.sql")],
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
        let violations =
            check_sql(sql, &["prefer-robust-stmts".into()]).expect("valid sql should parse");
        let filename = "main.sql";
        let mut buff = Vec::new();

        let res = print_violations(
            &mut buff,
            vec![pretty_violations(violations, sql, filename)],
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
        let violations =
            check_sql(sql, &["prefer-robust-stmts".into()]).expect("valid sql should parse");

        let filename = "main.sql";
        assert_debug_snapshot!(pretty_violations(violations, sql, filename));
    }

    /// `pretty_violations` was slicing the SQL improperly, trimming off the first
    /// letter.
    #[test]
    fn test_trimming_sql_newlines() {
        let sql = r#"ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;"#;
        let violations =
            check_sql(sql, &["prefer-robust-stmts".into()]).expect("valid sql should parse");

        assert_debug_snapshot!(violations, @r###"
        [
            RuleViolation {
                kind: AddingNotNullableField,
                span: Span {
                    start: 0,
                    len: Some(
                        59,
                    ),
                },
                messages: [
                    Note(
                        "Adding a NOT NULL field requires exclusive locks and table rewrites.",
                    ),
                    Help(
                        "Make the field nullable.",
                    ),
                ],
            },
        ]
        "###);

        let filename = "main.sql";
        assert_debug_snapshot!(pretty_violations(violations, sql, filename), @r###"
        ViolationContent {
            filename: "main.sql",
            sql: "ALTER TABLE \"core_recipe\" ADD COLUMN \"foo\" integer NOT NULL;",
            violations: [
                ReportViolation {
                    file: "main.sql",
                    line: 1,
                    column: 0,
                    level: Warning,
                    messages: [
                        Note(
                            "Adding a NOT NULL field requires exclusive locks and table rewrites.",
                        ),
                        Help(
                            "Make the field nullable.",
                        ),
                    ],
                    rule_name: AddingNotNullableField,
                    sql: "ALTER TABLE \"core_recipe\" ADD COLUMN \"foo\" integer NOT NULL;",
                },
            ],
        }
        "###);
    }
}
