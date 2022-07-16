use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{ColumnDef, QualifiedName, RawStmt};

use crate::rules::utils::columns_create_or_modified;

/// It's easier to update the check constraint on a text field than a varchar()
/// size since the check constraint can use NOT VALID with a separate VALIDATE
/// call.
#[must_use]
pub fn prefer_text_field(tree: &[RawStmt], _pg_version: Option<Version>) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        for column in columns_create_or_modified(&raw_stmt.stmt) {
            check_column_def(&mut errs, raw_stmt, column);
        }
    }
    errs
}

fn check_column_def(errs: &mut Vec<RuleViolation>, raw_stmt: &RawStmt, column_def: &ColumnDef) {
    let type_name = &column_def.type_name;
    for QualifiedName::String(field_type_name) in &type_name.names {
        if field_type_name.str == "varchar" && !type_name.typmods.is_empty() {
            errs.push(RuleViolation::new(
                RuleViolationKind::PreferTextField,
                raw_stmt.into(),
                None,
            ));
        }
    }
}

#[cfg(test)]
mod test_rules {
    use crate::check_sql;
    use crate::violations::RuleViolationKind;
    use insta::assert_debug_snapshot;
    #[test]
    fn test_ensure_ignored_when_new_table() {
        let sql = r#"
BEGIN;
CREATE TABLE "core_foo" (
  "id" serial NOT NULL PRIMARY KEY, 
  "created" timestamp with time zone NOT NULL, 
  "modified" timestamp with time zone NOT NULL, 
  "mongo_id" varchar(255) NOT NULL UNIQUE, 
  "description" text NOT NULL, 
  "metadata" jsonb NOT NULL, 
  "kind" varchar(255) NOT NULL, 
  "age" integer NOT NULL, 
  "tenant_id" integer NULL
);
CREATE INDEX "age_index" ON "core_foo" ("age");
ALTER TABLE "core_foo" ADD CONSTRAINT "age_restriction" CHECK ("age" >= 25);
ALTER TABLE "core_foo" ADD CONSTRAINT "core_foo_tenant_id_4d397ef9_fk_core_myuser_id" 
    FOREIGN KEY ("tenant_id") REFERENCES "core_myuser" ("id") 
    DEFERRABLE INITIALLY DEFERRED;
CREATE INDEX "core_foo_mongo_id_1c1a7e39_like" ON "core_foo" ("mongo_id" varchar_pattern_ops);
CREATE INDEX "core_foo_tenant_id_4d397ef9" ON "core_foo" ("tenant_id");
COMMIT;
        "#;

        assert_debug_snapshot!(check_sql(sql, &[RuleViolationKind::PreferTextField], None));
    }

    /// Changing a column of varchar(255) to varchar(1000) requires an ACCESS
    /// EXCLUSIVE lock
    #[test]
    fn test_increasing_varchar_size() {
        let sql = r#"
BEGIN;
--
-- Alter field kind on foo
--
ALTER TABLE "core_foo" ALTER COLUMN "kind" TYPE varchar(1000) USING "kind"::varchar(1000);
COMMIT;
"#;
        assert_debug_snapshot!(check_sql(sql, &[], None), @r###"
        Ok(
            [
                RuleViolation {
                    kind: ChangingColumnType,
                    span: Span {
                        start: 7,
                        len: Some(
                            123,
                        ),
                    },
                    messages: [
                        Note(
                            "Requires an ACCESS EXCLUSIVE lock on the table which blocks reads.",
                        ),
                        Note(
                            "Changing the type may break existing clients.",
                        ),
                    ],
                },
                RuleViolation {
                    kind: PreferTextField,
                    span: Span {
                        start: 7,
                        len: Some(
                            123,
                        ),
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
            ],
        )
        "###);
    }

    #[test]
    fn test_prefer_text_field() {
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
        assert_debug_snapshot!(check_sql(bad_sql, &[], None), @r###"
        Ok(
            [
                RuleViolation {
                    kind: PreferTextField,
                    span: Span {
                        start: 7,
                        len: Some(
                            127,
                        ),
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
            ],
        )
        "###);

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
        assert_debug_snapshot!(check_sql(ok_sql, &[], None), @r###"
        Ok(
            [],
        )
        "###);
    }

    #[test]
    fn test_adding_column_non_text() {
        let bad_sql = r#"
BEGIN;
ALTER TABLE "foo_table" ADD COLUMN "foo_column" varchar(256) NULL;
COMMIT;
"#;

        let res = check_sql(bad_sql, &[], None);
        assert!(res.is_ok());
        let data = res.unwrap_or_default();
        assert!(!data.is_empty());
        assert_debug_snapshot!(data);
    }

    #[test]
    fn allow_varchar_without_specified_limit() {
        let ok_sql = r#"
    CREATE TABLE IF NOT EXISTS foo_table(bar_col varchar);
    "#;
        let res = check_sql(ok_sql, &[], None);
        assert_eq!(res, Ok(vec![]));
    }
}
