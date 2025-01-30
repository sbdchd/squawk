use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{ColumnDef, RawStmt};

use crate::rules::utils::columns_create_or_modified;

/// It's easier to update the check constraint on a text field than a `varchar()`
/// size since the check constraint can use NOT VALID with a separate VALIDATE
/// call.
#[must_use]
pub fn prefer_text_field(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        for column in columns_create_or_modified(&raw_stmt.stmt) {
            check_column_def(&mut errs, column);
        }
    }
    errs
}

fn check_column_def(errs: &mut Vec<RuleViolation>, column_def: &ColumnDef) {
    let type_name = &column_def.type_name;
    for field_type_name in &type_name.names {
        if field_type_name.string.sval == "varchar" && !type_name.typmods.is_empty() {
            errs.push(RuleViolation::new(
                RuleViolationKind::PreferTextField,
                column_def.into(),
                None,
            ));
        }
    }
}

#[cfg(test)]
mod test_rules {

    use insta::assert_debug_snapshot;

    use crate::{
        check_sql_with_rule,
        violations::{RuleViolation, RuleViolationKind},
    };
    fn lint_sql(sql: &str) -> Vec<RuleViolation> {
        check_sql_with_rule(sql, &RuleViolationKind::PreferTextField, None, false).unwrap()
    }

    /// Changing a column of varchar(255) to varchar(1000) requires an ACCESS
    /// EXCLUSIVE lock
    #[test]
    fn increasing_varchar_size() {
        let sql = r#"
BEGIN;
--
-- Alter field kind on foo
--
ALTER TABLE "core_foo" ALTER COLUMN "kind" TYPE varchar(1000) USING "kind"::varchar(1000);
COMMIT;
"#;
        assert_debug_snapshot!(lint_sql(sql), @r#"
        [
            RuleViolation {
                kind: PreferTextField,
                span: Span {
                    start: 77,
                    len: None,
                },
                messages: [
                    Note(
                        "Changing the size of a varchar field requires an ACCESS EXCLUSIVE lock.",
                    ),
                    Help(
                        "Use a text field with a check constraint.",
                    ),
                ],
            },
        ]
        "#);
    }

    #[test]
    fn prefer_text_field() {
        let bad_sql = r#"
BEGIN;
--
-- Create model Bar
--
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY, 
    "alpha" varchar(100) NOT NULL
);
COMMIT;
"#;
        assert_debug_snapshot!(lint_sql(bad_sql), @r#"
        [
            RuleViolation {
                kind: PreferTextField,
                span: Span {
                    start: 103,
                    len: None,
                },
                messages: [
                    Note(
                        "Changing the size of a varchar field requires an ACCESS EXCLUSIVE lock.",
                    ),
                    Help(
                        "Use a text field with a check constraint.",
                    ),
                ],
            },
        ]
        "#);

        let ok_sql = r#"
BEGIN;
--
-- Create model Bar
--
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY, 
    "bravo" text NOT NULL
);
--
-- Create constraint text_size on model bar
--
ALTER TABLE "core_bar" ADD CONSTRAINT "text_size" CHECK (LENGTH("bravo") <= 100);
COMMIT;"#;
        assert_debug_snapshot!(lint_sql(ok_sql), @"[]");
    }

    #[test]
    fn adding_column_non_text() {
        let bad_sql = r#"
BEGIN;
ALTER TABLE "foo_table" ADD COLUMN "foo_column" varchar(256) NULL;
COMMIT;
"#;

        let res = lint_sql(bad_sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn allow_varchar_without_specified_limit() {
        let ok_sql = r"
    CREATE TABLE IF NOT EXISTS foo_table(bar_col varchar);
    ";
        let res = lint_sql(ok_sql);
        assert_eq!(res, vec![]);
    }
}
