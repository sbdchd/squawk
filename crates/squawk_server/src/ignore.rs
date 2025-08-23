use line_index::{LineIndex, TextSize};
use squawk_linter::{
    Edit, Rule, Violation,
    ignore::{IGNORE_FILE_TEXT, IGNORE_LINE_TEXT},
};
use squawk_syntax::{Parse, SourceFile, SyntaxKind, SyntaxToken, ast::AstNode};

const UNSUPPORTED_RULES: &[Rule] = &[Rule::UnusedIgnore];

pub(crate) fn ignore_line_edit(
    violation: &Violation,
    line_index: &LineIndex,
    parse: &Parse<SourceFile>,
) -> Option<Edit> {
    if UNSUPPORTED_RULES.contains(&violation.code) {
        return None;
    }
    let tree = parse.tree();
    let rule_name = violation.code.to_string();

    let violation_line = line_index.line_col(violation.text_range.start());
    let previous_line = violation_line.line.checked_sub(1)?;
    let previous_line_offset = line_index.line(previous_line)?.start();
    let previous_line_token = tree
        .syntax()
        .token_at_offset(previous_line_offset)
        .right_biased()?;

    match previous_line_token.kind() {
        SyntaxKind::COMMENT if is_ignore_comment(&previous_line_token) => {
            let (_str, ignore_comment_range, _ignore_kind) =
                squawk_linter::ignore::ignore_rule_info(&previous_line_token)?;
            Some(Edit::insert(
                format!(" {rule_name},"),
                ignore_comment_range.start(),
            ))
        }

        // TODO: we need to handle indenting correctly
        _ => Some(Edit::insert(
            format!("-- {IGNORE_LINE_TEXT} {rule_name}\n"),
            line_index.line(violation_line.line)?.start(),
        )),
    }
}

pub(crate) fn ignore_file_edit(
    violation: &Violation,
    _line_index: &LineIndex,
    _parse: &Parse<SourceFile>,
) -> Option<Edit> {
    if UNSUPPORTED_RULES.contains(&violation.code) {
        return None;
    }
    let rule_name = violation.code.to_string();
    Some(Edit::insert(
        format!("-- {IGNORE_FILE_TEXT} {rule_name}\n"),
        TextSize::new(0),
    ))
}

fn is_ignore_comment(token: &SyntaxToken) -> bool {
    assert_eq!(token.kind(), SyntaxKind::COMMENT);
    squawk_linter::ignore::ignore_rule_info(&token).is_some()
}

#[cfg(test)]
mod test {
    use crate::{diagnostic::AssociatedDiagnosticData, lint::lint};

    #[test]
    fn ignore_line() {
        let sql = "
create table a (
  a int
);

-- an existing comment that shouldn't get in the way of us adding a new ignore
create table b (
  b int
);

-- squawk-ignore prefer-text-field
create table c (
  b int
);
";
        let ignore_line_edits = lint(sql)
            .into_iter()
            .flat_map(|x| {
                let data = x.data?;
                let associated_data: AssociatedDiagnosticData =
                    serde_json::from_value(data).unwrap();
                associated_data.ignore_line_edit
            })
            .collect::<Vec<_>>();
        insta::assert_snapshot!(apply_text_edits(sql, ignore_line_edits), @r"
        -- squawk-ignore prefer-robust-stmts
        create table a (
        -- squawk-ignore prefer-bigint-over-int
          a int
        );

        -- an existing comment that shouldn't get in the way of us adding a new ignore
        -- squawk-ignore prefer-robust-stmts
        create table b (
        -- squawk-ignore prefer-bigint-over-int
          b int
        );

        -- squawk-ignore prefer-robust-stmts, prefer-text-field
        create table c (
        -- squawk-ignore prefer-bigint-over-int
          b int
        );
        ");
    }

    #[test]
    fn ignore_file() {
        let sql = "
-- some existing comment
create table a (
  a int
);

create table b (
  b int
);

create table c (
  b int
);
";
        let ignore_line_edits = lint(sql)
            .into_iter()
            .flat_map(|x| {
                let data = x.data?;
                let associated_data: AssociatedDiagnosticData =
                    serde_json::from_value(data).unwrap();
                associated_data.ignore_file_edit
            })
            .collect::<Vec<_>>();
        insta::assert_snapshot!(apply_text_edits(sql, ignore_line_edits), @r"
        -- squawk-ignore-file prefer-bigint-over-int
        -- squawk-ignore-file prefer-robust-stmts
        -- squawk-ignore-file prefer-bigint-over-int
        -- squawk-ignore-file prefer-robust-stmts
        -- squawk-ignore-file prefer-bigint-over-int
        -- squawk-ignore-file prefer-robust-stmts

        -- some existing comment
        create table a (
          a int
        );

        create table b (
          b int
        );

        create table c (
          b int
        );
        ");
    }

    fn apply_text_edits(sql: &str, mut edits: Vec<lsp_types::TextEdit>) -> String {
        use line_index::{LineCol, LineIndex};

        // Sort edits by position (reverse order to apply from end to start)
        edits.sort_by(|a, b| {
            b.range
                .start
                .line
                .cmp(&a.range.start.line)
                .then_with(|| b.range.start.character.cmp(&a.range.start.character))
        });

        let line_index = LineIndex::new(sql);
        let mut result = sql.to_string();

        for edit in edits {
            // Convert LSP positions to byte offsets
            let start_offset = line_index.offset(LineCol {
                line: edit.range.start.line,
                col: edit.range.start.character,
            });
            let end_offset = line_index.offset(LineCol {
                line: edit.range.end.line,
                col: edit.range.end.character,
            });

            let start_byte: usize = start_offset.unwrap_or_default().into();
            let end_byte: usize = end_offset.unwrap_or_default().into();

            result.replace_range(start_byte..end_byte, &edit.new_text);
        }

        result
    }
}
