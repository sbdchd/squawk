use rowan::TextSize;
use squawk_diagnostics::Edit;
use squawk_ide_db::{Database, Db, original_file, parse};
use squawk_sema::file::{FileIndex, InFile};
use squawk_syntax::ast;

use crate::{ActionKind, CodeAction};

const MARKER: &str = "$0";

pub(crate) struct Fixture {
    marker_offset: TextSize,
    sql: String,
    db: Database,
    file: FileIndex,
}

pub(crate) struct Marker {
    file: FileIndex,
    offset_before: TextSize,
}

impl Fixture {
    #[track_caller]
    pub(crate) fn new(sql: &str) -> Self {
        let fixture = Self::new_allow_errors(sql);
        assert_eq!(parse(&fixture.db, fixture.file.into()).errors(), vec![]);
        fixture
    }

    #[track_caller]
    pub(crate) fn new_allow_errors(sql: &str) -> Self {
        let Some(pos) = sql.find(MARKER) else {
            panic!("No marker found in SQL. Did you forget to add a marker `$0`?");
        };
        let sql = sql.replace(MARKER, "");
        let mut db = Database::default();
        let file = db.add_file(sql.as_str().into());
        Self {
            marker_offset: TextSize::try_from(pos).unwrap(),
            sql,
            db,
            file,
        }
    }

    pub(crate) fn marker(&self) -> Marker {
        let marker_offset: usize = self.marker_offset.into();
        let offset_before = self.sql[..marker_offset]
            .char_indices()
            .last()
            .map_or(0, |(offset, _)| offset);
        Marker {
            file: self.file,
            offset_before: TextSize::try_from(offset_before).unwrap(),
        }
    }

    pub(crate) fn db(&self) -> &Database {
        &self.db
    }
}

impl Marker {
    pub(crate) fn offset_before(&self) -> InFile<TextSize> {
        InFile::new(self.file, self.offset_before)
    }
}

#[must_use]
pub(crate) fn apply_code_action(
    f: impl Fn(&dyn Db, InFile<TextSize>, &mut Vec<CodeAction>) -> Option<()>,
    sql: &str,
) -> String {
    apply_code_action_(f, sql, false)
}

#[must_use]
pub(crate) fn apply_code_action_with_errors(
    f: impl Fn(&dyn Db, InFile<TextSize>, &mut Vec<CodeAction>) -> Option<()>,
    sql: &str,
) -> String {
    apply_code_action_(f, sql, true)
}

fn apply_code_action_(
    f: impl Fn(&dyn Db, InFile<TextSize>, &mut Vec<CodeAction>) -> Option<()>,
    sql: &str,
    allow_errors: bool,
) -> String {
    let fixture = if allow_errors {
        Fixture::new_allow_errors(sql)
    } else {
        Fixture::new(sql)
    };
    let offset = fixture.marker().offset_before();
    let db = fixture.db();
    let file = offset.file_id;

    let mut actions = vec![];
    f(db, offset, &mut actions);

    assert!(
        !actions.is_empty(),
        "We should always have actions for `apply_code_action`. If you want to ensure there are no actions, use `code_action_not_applicable` instead."
    );

    let action = &actions[0];

    let mut result = original_file(db, file).content(db).to_string();

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
                "Code actions shouldn't cause syntax errors.\nParsed:\n{result}"
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
    f: impl Fn(&dyn Db, InFile<TextSize>, &mut Vec<CodeAction>) -> Option<()>,
    sql: &str,
    allow_errors: bool,
) -> bool {
    let fixture = if allow_errors {
        Fixture::new_allow_errors(sql)
    } else {
        Fixture::new(sql)
    };
    let offset = fixture.marker().offset_before();
    let db = fixture.db();

    let mut actions = vec![];
    f(db, offset, &mut actions);
    actions.is_empty()
}

#[must_use]
pub(crate) fn code_action_not_applicable(
    f: impl Fn(&dyn Db, InFile<TextSize>, &mut Vec<CodeAction>) -> Option<()>,
    sql: &str,
) -> bool {
    code_action_not_applicable_(f, sql, false)
}

#[must_use]
pub(crate) fn code_action_not_applicable_with_errors(
    f: impl Fn(&dyn Db, InFile<TextSize>, &mut Vec<CodeAction>) -> Option<()>,
    sql: &str,
) -> bool {
    code_action_not_applicable_(f, sql, true)
}
