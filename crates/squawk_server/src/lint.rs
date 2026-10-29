use gen_lsp_types::{
    Code, CodeDescription, Diagnostic, DiagnosticSeverity, Message, Position, Range, TextEdit,
};
use salsa::Database as Db;
use squawk_diagnostics::Edit;
use squawk_ide::db::{File, line_index as file_line_index, parse};
use squawk_ide::diagnostics::{Suggestion, diagnostics as semantic_diagnostics};
use squawk_line_index::{LineIndex, TextRange, find_newline};
use squawk_linter::Linter;
use url::Url;

use crate::{
    diagnostic::{AssociatedDiagnosticData, DIAGNOSTIC_NAME},
    ignore::{ignore_file_edit, ignore_line_edit},
};

fn to_text_edit(edit: Edit, line_index: &LineIndex) -> Option<TextEdit> {
    let start_line = line_index.try_line_col(edit.text_range.start())?;
    let end_line = line_index.try_line_col(edit.text_range.end())?;
    let range = Range::new(
        Position::new(start_line.line, start_line.col),
        Position::new(end_line.line, end_line.col),
    );
    Some(TextEdit::new(
        range,
        edit.text.unwrap_or_default().into_owned(),
    ))
}

fn diagnostic_range(line_index: &LineIndex, range: TextRange) -> Range {
    let start = line_index.line_col(range.start());
    let mut end = line_index.line_col(range.end());

    if start == end {
        end.col += 1;
    }

    Range::new(
        Position::new(start.line, start.col),
        Position::new(end.line, end.col),
    )
}

#[salsa::tracked]
pub(crate) fn lint(db: &dyn Db, file: File) -> Vec<Diagnostic> {
    let parse = parse(db, file);
    let content = file.content(db);
    let parse_errors = parse.errors();
    let mut linter = Linter::with_default_rules();
    let violations = linter.lint(&parse, content);
    let line_index = file_line_index(db, file);
    let line_ending = find_newline(content)
        .map(|(_, ending)| ending)
        .unwrap_or_default();

    let mut diagnostics = Vec::with_capacity(violations.len() + parse_errors.len());

    for error in parse_errors {
        let message = match error.hint() {
            Some(hint) => format!("{}\nhint: {hint}", error.message()),
            None => error.message().to_string(),
        };
        let diagnostic = Diagnostic {
            range: diagnostic_range(line_index, error.range()),
            severity: Some(DiagnosticSeverity::Error),
            code: Some(Code::String("syntax-error".to_string())),
            code_description: Some(CodeDescription {
                href: Url::parse("https://squawkhq.com/docs/syntax-error").unwrap(),
            }),
            source: Some(DIAGNOSTIC_NAME.to_string()),
            message: Message::String(message),
            ..Default::default()
        };
        diagnostics.push(diagnostic);
    }

    for error in semantic_diagnostics(db, file) {
        let message = match error.hint {
            Some(hint) => format!("{}\nhint: {hint}", error.message),
            None => error.message.into_owned(),
        };

        let data = error
            .suggestion
            .map(|Suggestion { fix, .. }| AssociatedDiagnosticData {
                title: fix.title.into_owned(),
                edits: fix
                    .edits
                    .into_iter()
                    .filter_map(|edit| to_text_edit(edit, line_index))
                    .collect(),
                ignore_line_edit: None,
                ignore_file_edit: None,
            });

        let diagnostic = Diagnostic {
            range: diagnostic_range(line_index, error.range.value),
            severity: Some(DiagnosticSeverity::Error),
            code: Some(Code::String("semantic-error".to_string())),
            code_description: Some(CodeDescription {
                href: Url::parse("https://squawkhq.com/docs/semantic-error").unwrap(),
            }),
            source: Some(DIAGNOSTIC_NAME.to_string()),
            message: Message::String(message),
            data: data.map(|data| serde_json::to_value(data).unwrap()),
            ..Default::default()
        };
        diagnostics.push(diagnostic);
    }

    for violation in violations {
        let ignore_line_edit = ignore_line_edit(&violation, line_index, &parse, line_ending)
            .and_then(|e| to_text_edit(e, line_index));
        let ignore_file_edit = ignore_file_edit(&violation, line_index, &parse, line_ending)
            .and_then(|e| to_text_edit(e, line_index));

        let (title, fix_edits) = if let Some(fix) = violation.fix {
            (fix.title.into_owned(), fix.edits)
        } else {
            (String::new(), vec![])
        };

        let edits = fix_edits
            .into_iter()
            .filter_map(|x| to_text_edit(x, line_index))
            .collect();

        let mut message = violation.message.into_owned();
        if let Some(note) = violation.note {
            message.push_str("\nnote: ");
            message.push_str(&note);
        }
        if let Some(help) = violation.help {
            message.push_str("\nhelp: ");
            message.push_str(&help);
        }

        let data = AssociatedDiagnosticData {
            title,
            edits,
            ignore_line_edit,
            ignore_file_edit,
        };

        let diagnostic = Diagnostic {
            range: diagnostic_range(line_index, violation.text_range),
            severity: Some(DiagnosticSeverity::Warning),
            code: Some(Code::String(violation.code.to_string())),
            code_description: Some(CodeDescription {
                href: Url::parse(&format!("https://squawkhq.com/docs/{}", violation.code)).unwrap(),
            }),
            source: Some(DIAGNOSTIC_NAME.to_string()),
            message: Message::String(message),
            data: Some(serde_json::to_value(data).unwrap()),
            ..Default::default()
        };
        diagnostics.push(diagnostic);
    }
    diagnostics
}

#[cfg(test)]
mod test {
    use gen_lsp_types::{Code, Message};
    use insta::assert_snapshot;
    use squawk_ide::db::{Database, File};

    use super::lint;
    use crate::diagnostic::AssociatedDiagnosticData;

    fn check(sql: &str) -> String {
        let db = Database::default();
        let file = File::new(&db, sql.into());
        lint(&db, file)
            .into_iter()
            .map(|d| {
                let code = match d.code {
                    Some(Code::String(code)) => code,
                    Some(Code::Int(code)) => code.to_string(),
                    None => "-".to_string(),
                };
                let Message::String(message) = d.message else {
                    unreachable!()
                };
                let line = format!(
                    "{}:{} {:?} {code}: {message}",
                    d.range.start.line,
                    d.range.start.character,
                    d.severity.unwrap(),
                );
                if code != "semantic-error" {
                    return line;
                }
                let Some(data) = d.data else {
                    return line;
                };
                let data: AssociatedDiagnosticData = serde_json::from_value(data).unwrap();
                format!("{line}\nfix: {}", data.title)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn syntax_error() {
        assert_snapshot!(check("select from t where;"), @"
        0:19 Error syntax-error: expected an expression, found `;`
        0:12 Error semantic-error: relation `t` does not exist
        ");
    }

    #[test]
    fn semantic_error() {
        let sql = "
create table t(a int);
create table t(b int);
";
        assert_snapshot!(check(sql), @"
        2:13 Error semantic-error: relation `t` already exists
        1:0 Warning prefer-robust-stmts: missing `if not exists`, the migration can't be rerun if it fails part way through
        1:17 Warning prefer-bigint-over-int: using 32-bit integer fields can result in hitting the max `int` limit
        help: use 64-bit integer values instead to prevent hitting this limit
        2:0 Warning prefer-robust-stmts: missing `if not exists`, the migration can't be rerun if it fails part way through
        2:17 Warning prefer-bigint-over-int: using 32-bit integer fields can result in hitting the max `int` limit
        help: use 64-bit integer values instead to prevent hitting this limit
        ");
    }

    #[test]
    fn unresolved_cte_column() {
        let sql = "
with t as (select 1 a)
select a, b from t;
";
        assert_snapshot!(check(sql), @"
        2:10 Error semantic-error: column `b` does not exist
        fix: Replace with `a`
        ");
    }

    #[test]
    fn lint_violation() {
        let sql = "
create table t(a varchar(10));
";
        assert_snapshot!(check(sql), @"
        1:0 Warning prefer-robust-stmts: missing `if not exists`, the migration can't be rerun if it fails part way through
        1:17 Warning prefer-text-field: changing the size of a `varchar` field requires an `access exclusive` lock, that will prevent all reads and writes to the table
        help: use a `text` field with a `check` constraint
        ");
    }
}
