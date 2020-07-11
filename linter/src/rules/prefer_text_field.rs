use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{ColumnDefTypeName, QualifiedName, RootStmt, Stmt, TableElt};

/// It's easier to update the check constraint on a text field than a varchar()
/// size since the check constraint can use NOT VALID with a separate VALIDATE
/// call.
pub fn prefer_text_field(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::CreateStmt(stmt) => {
                for TableElt::ColumnDef(column_def) in &stmt.table_elts {
                    let ColumnDefTypeName::TypeName(type_name) = &column_def.type_name;
                    for QualifiedName::String(field_type_name) in &type_name.names {
                        if field_type_name.str == "varchar" {
                            errs.push(RuleViolation::new(
                                RuleViolationKind::PreferTextField,
                                raw_stmt,
                                None,
                            ));
                        }
                    }
                }
            }
            _ => continue,
        }
    }
    errs
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

        assert_debug_snapshot!(check_sql(
            sql,
            &[RuleViolationKind::PreferTextField.to_string()]
        ));
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
        assert_debug_snapshot!(check_sql(sql, &[]), @r###"
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
        assert_debug_snapshot!(check_sql(bad_sql, &[]), @r###"
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
        assert_debug_snapshot!(check_sql(ok_sql, &[]), @r###"
        Ok(
            [],
        )
        "###);
    }
}
