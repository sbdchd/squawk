use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast;

use crate::db::{Database, File};
use crate::test_utils::fixture;

use super::{ActionKind, CodeAction};

pub(super) fn apply_code_action(
    f: impl Fn(&dyn Db, File, &mut Vec<CodeAction>, TextSize) -> Option<()>,
    sql: &str,
) -> String {
    let (mut offset, sql) = fixture(sql);
    let db = Database::default();
    let file = File::new(&db, sql.clone().into());
    let parse_result = crate::db::parse(&db, file);

    offset = offset.checked_sub(1.into()).unwrap_or_default();

    let mut actions = vec![];
    f(&db, file, &mut actions, offset);

    assert!(
        !actions.is_empty(),
        "We should always have actions for `apply_code_action`. If you want to ensure there are no actions, use `code_action_not_applicable` instead."
    );

    let action = &actions[0];

    match action.kind {
        ActionKind::QuickFix => {
            // Quickfixes can fix syntax errors so we don't assert
        }
        ActionKind::RefactorRewrite => {
            assert_eq!(parse_result.errors(), vec![]);
        }
    }

    let mut result = sql.clone();

    let mut edits = action.edits.clone();
    edits.sort_by_key(|e| e.text_range.start());
    check_overlap(&edits);
    edits.reverse();

    for edit in edits {
        let start: usize = edit.text_range.start().into();
        let end: usize = edit.text_range.end().into();
        let replacement = edit.text.as_deref().unwrap_or("");
        result.replace_range(start..end, replacement);
    }

    let reparse = ast::SourceFile::parse(&result);

    match action.kind {
        ActionKind::QuickFix => {
            // Quickfixes can fix syntax errors so we don't assert
        }
        ActionKind::RefactorRewrite => {
            assert_eq!(
                reparse.errors(),
                vec![],
                "Code actions shouldn't cause syntax errors"
            );
        }
    }

    result
}

// There's an invariant where the edits can't overlap.
// For example, if we have an edit that deletes the full `else clause` and
// another edit that deletes the `else` keyword and they overlap, then
// vscode doesn't surface the code action.
fn check_overlap(edits: &[Edit]) {
    for (edit_i, edit_j) in edits.iter().zip(edits.iter().skip(1)) {
        if let Some(intersection) = edit_i.text_range.intersect(edit_j.text_range) {
            assert!(
                intersection.is_empty(),
                "Edit ranges must not overlap: {:?} and {:?} intersect at {:?}",
                edit_i.text_range,
                edit_j.text_range,
                intersection
            );
        }
    }
}

fn code_action_not_applicable_(
    f: impl Fn(&dyn Db, File, &mut Vec<CodeAction>, TextSize) -> Option<()>,
    sql: &str,
    allow_errors: bool,
) -> bool {
    let (offset, sql) = fixture(sql);
    let db = Database::default();
    let file = File::new(&db, sql.clone().into());
    let parse_result = crate::db::parse(&db, file);
    if !allow_errors {
        assert_eq!(parse_result.errors(), vec![]);
    }

    let mut actions = vec![];
    f(&db, file, &mut actions, offset);
    actions.is_empty()
}

pub(super) fn code_action_not_applicable(
    f: impl Fn(&dyn Db, File, &mut Vec<CodeAction>, TextSize) -> Option<()>,
    sql: &str,
) -> bool {
    code_action_not_applicable_(f, sql, false)
}

pub(super) fn code_action_not_applicable_with_errors(
    f: impl Fn(&dyn Db, File, &mut Vec<CodeAction>, TextSize) -> Option<()>,
    sql: &str,
) -> bool {
    code_action_not_applicable_(f, sql, true)
}
