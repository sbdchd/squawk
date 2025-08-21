use line_index::LineIndex;
use lsp_types::{CodeDescription, Diagnostic, DiagnosticSeverity, Position, Range, TextEdit, Url};
use squawk_linter::{Edit, Linter};
use squawk_syntax::{Parse, SourceFile};

use crate::{
    DIAGNOSTIC_NAME,
    diagnostic::AssociatedDiagnosticData,
    ignore::{ignore_file_edit, ignore_line_edit},
};

fn to_text_edit(edit: Edit, line_index: &LineIndex) -> Option<TextEdit> {
    let start_line = line_index.try_line_col(edit.text_range.start())?;
    let end_line = line_index.try_line_col(edit.text_range.end())?;
    let range = Range::new(
        Position::new(start_line.line, start_line.col),
        Position::new(end_line.line, end_line.col),
    );
    Some(TextEdit::new(range, edit.text.unwrap_or_default()))
}

pub(crate) fn lint(content: &str) -> Vec<Diagnostic> {
    let parse: Parse<SourceFile> = SourceFile::parse(content);
    let parse_errors = parse.errors();
    let mut linter = Linter::with_all_rules();
    let violations = linter.lint(&parse, content);
    let line_index = LineIndex::new(content);

    let mut diagnostics = Vec::with_capacity(violations.len() + parse_errors.len());

    for error in parse_errors {
        let range_start = error.range().start();
        let range_end = error.range().end();
        let start_line_col = line_index.line_col(range_start);
        let mut end_line_col = line_index.line_col(range_end);

        if start_line_col.line == end_line_col.line && start_line_col.col == end_line_col.col {
            end_line_col.col += 1;
        }

        let diagnostic = Diagnostic {
            range: Range::new(
                Position::new(start_line_col.line, start_line_col.col),
                Position::new(end_line_col.line, end_line_col.col),
            ),
            severity: Some(DiagnosticSeverity::ERROR),
            code: Some(lsp_types::NumberOrString::String(
                "syntax-error".to_string(),
            )),
            code_description: Some(CodeDescription {
                href: Url::parse("https://squawkhq.com/docs/syntax-error").unwrap(),
            }),
            source: Some(DIAGNOSTIC_NAME.to_string()),
            message: error.message().to_string(),
            ..Default::default()
        };
        diagnostics.push(diagnostic);
    }

    for violation in violations {
        let range_start = violation.text_range.start();
        let range_end = violation.text_range.end();
        let start_line_col = line_index.line_col(range_start);
        let mut end_line_col = line_index.line_col(range_end);

        if start_line_col.line == end_line_col.line && start_line_col.col == end_line_col.col {
            end_line_col.col += 1;
        }

        let ignore_line_edit = ignore_line_edit(&violation, &line_index, &parse)
            .and_then(|e| to_text_edit(e, &line_index));
        let ignore_file_edit = ignore_file_edit(&violation, &line_index, &parse)
            .and_then(|e| to_text_edit(e, &line_index));

        let (title, fix_edits) = if let Some(fix) = violation.fix {
            (fix.title, fix.edits)
        } else {
            ("".to_string(), vec![])
        };

        let edits = fix_edits
            .into_iter()
            .filter_map(|x| to_text_edit(x, &line_index))
            .collect();

        let data = AssociatedDiagnosticData {
            title,
            edits,
            ignore_line_edit,
            ignore_file_edit,
        };

        let diagnostic = Diagnostic {
            range: Range::new(
                Position::new(start_line_col.line, start_line_col.col),
                Position::new(end_line_col.line, end_line_col.col),
            ),
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(lsp_types::NumberOrString::String(
                violation.code.to_string(),
            )),
            code_description: Some(CodeDescription {
                href: Url::parse(&format!("https://squawkhq.com/docs/{}", violation.code)).unwrap(),
            }),
            source: Some(DIAGNOSTIC_NAME.to_string()),
            message: violation.message,
            data: Some(serde_json::to_value(data).unwrap()),
            ..Default::default()
        };
        diagnostics.push(diagnostic);
    }
    diagnostics
}
