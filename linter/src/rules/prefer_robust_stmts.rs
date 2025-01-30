use std::collections::HashMap;

use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind, ViolationMessage},
};
use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, AlterTableType, RawStmt, Stmt, TransactionStmtKind,
};
#[derive(PartialEq)]
enum Constraint {
    Dropped,
    Added,
}

/// If a migration is running in a transaction, then we skip the statements
/// because if it fails part way through, it will revert.
/// For the cases where statements aren't running in a transaction, for instance,
/// when we CREATE INDEX CONCURRENTLY, we should try and make those migrations
/// more robust by using guards like `IF NOT EXISTS`. So if the migration fails
/// halfway through, it can be rerun without human intervention.
#[must_use]
pub fn prefer_robust_stmts(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];
    let mut inside_transaction = assume_in_transaction;
    let mut constraint_names: HashMap<String, Constraint> = HashMap::new();
    // if we only have one statement in our file, Postgres will run that
    // statement in an implicit transaction, so we don't need to worry about
    // wrapping with `BEGIN;COMMIT;`.
    if tree.len() == 1 {
        return errs;
    }
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::TransactionStmt(stmt) => match stmt.kind {
                TransactionStmtKind::Begin | TransactionStmtKind::Start => {
                    inside_transaction = true;
                }
                TransactionStmtKind::Commit => inside_transaction = false,
                _ => continue,
            },
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    if let Some(constraint_name) = &cmd.name {
                        if cmd.subtype == AlterTableType::DropConstraint {
                            constraint_names.insert(constraint_name.clone(), Constraint::Dropped);
                        }
                        if (cmd.subtype == AlterTableType::AddConstraint
                            || cmd.subtype == AlterTableType::ValidateConstraint)
                            && constraint_names.contains_key(constraint_name)
                        {
                            continue;
                        }
                    }

                    if cmd.subtype == AlterTableType::EnableRowSecurity
                        || cmd.subtype == AlterTableType::DisableRowSecurity
                    {
                        continue;
                    }

                    if let Some(AlterTableDef::Constraint(constraint)) = &cmd.def {
                        if let Some(constraint_name) = &constraint.conname {
                            if let Some(constraint) = constraint_names.get_mut(constraint_name) {
                                if *constraint == Constraint::Dropped {
                                    *constraint = Constraint::Added;
                                    continue;
                                }
                            }
                        }
                    }
                    if cmd.missing_ok || inside_transaction {
                        continue;
                    }
                    errs.push(RuleViolation::new(
                        RuleViolationKind::PreferRobustStmts,
                        raw_stmt.into(),
                        None,
                    ));
                }
            }
            // bad: CREATE INDEX CONCURRENTLY ON ..
            // good: CREATE INDEX CONCURRENTLY somename ON ..
            Stmt::IndexStmt(stmt) if stmt.concurrent && stmt.idxname.is_none() => {
                errs.push(RuleViolation::new(
                    RuleViolationKind::PreferRobustStmts,
                    raw_stmt.into(),
                    Some(vec![ViolationMessage::Help(
                        "Use an explicit name for a concurrently created index".into(),
                    )]),
                ));
            }
            Stmt::IndexStmt(stmt)
                if !stmt.if_not_exists && (stmt.concurrent || !inside_transaction) =>
            {
                errs.push(RuleViolation::new(
                    RuleViolationKind::PreferRobustStmts,
                    raw_stmt.into(),
                    None,
                ));
            }
            Stmt::CreateStmt(stmt) if !stmt.if_not_exists && !inside_transaction => {
                errs.push(RuleViolation::new(
                    RuleViolationKind::PreferRobustStmts,
                    raw_stmt.into(),
                    None,
                ));
            }
            Stmt::DropStmt(stmt) if !stmt.missing_ok && !inside_transaction => {
                errs.push(RuleViolation::new(
                    RuleViolationKind::PreferRobustStmts,
                    raw_stmt.into(),
                    None,
                ));
            }
            _ => continue,
        }
    }
    errs
}

#[cfg(test)]
mod test_rules {
    use crate::{
        check_sql_with_rule,
        errors::CheckSqlError,
        violations::{RuleViolation, RuleViolationKind},
    };
    use insta::assert_debug_snapshot;
    use squawk_parser::parse::parse_sql_query;

    use super::prefer_robust_stmts;

    fn lint_sql(sql: &str) -> Result<Vec<RuleViolation>, CheckSqlError> {
        let mut sql = sql.to_owned();
        // our check ignores single statement queries, so we add an extra statement to ensure we check that case
        sql.push_str(";\nSELECT 1;");
        let tree = parse_sql_query(&sql)?;
        Ok(prefer_robust_stmts(&tree, None, false))
    }

    fn lint_sql_assuming_in_transaction(sql: &str) -> Result<Vec<RuleViolation>, CheckSqlError> {
        let mut sql = sql.to_owned();
        // our check ignores single statement queries, so we add an extra statement to ensure we check that case
        sql.push_str(";\nSELECT 1;");
        let tree = parse_sql_query(&sql)?;
        Ok(prefer_robust_stmts(&tree, None, true))
    }

    #[test]
    /// If we drop the constraint before adding it, we don't need the IF EXISTS or a transaction.
    fn drop_before_add() {
        let sql = r#"
ALTER TABLE "app_email" DROP CONSTRAINT IF EXISTS "email_uniq";
ALTER TABLE "app_email" ADD CONSTRAINT "email_uniq" UNIQUE USING INDEX "email_idx";
"#;
        assert_eq!(lint_sql(sql), Ok(vec![]));
    }
    #[test]
    fn drop_index() {
        let sql = r#"
DROP INDEX CONCURRENTLY "email_idx";
"#;
        let res = lint_sql(sql).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].kind, RuleViolationKind::PreferRobustStmts);
    }
    #[test]
    fn drop_index_if_exists() {
        let sql = r#"
DROP INDEX CONCURRENTLY IF EXISTS "email_idx";
"#;
        assert_eq!(lint_sql(sql), Ok(vec![]));
    }
    #[test]
    /// DROP CONSTRAINT and then ADD CONSTRAINT is safe. We can also safely run VALIDATE CONSTRAINT.
    fn drop_before_add_foreign_key() {
        let sql = r#"
ALTER TABLE "app_email" DROP CONSTRAINT IF EXISTS "fk_user";
ALTER TABLE "app_email" ADD CONSTRAINT "fk_user" FOREIGN KEY ("user_id") REFERENCES "app_user" ("id") DEFERRABLE INITIALLY DEFERRED NOT VALID;
ALTER TABLE "app_email" VALIDATE CONSTRAINT "fk_user";
"#;
        assert_eq!(lint_sql(sql), Ok(vec![]));
    }
    #[test]
    /// We can only use the dropped constraint in one ADD CONSTRAINT statement.
    fn double_add_after_drop() {
        let sql = r#"
ALTER TABLE "app_email" DROP CONSTRAINT IF EXISTS "email_uniq";
ALTER TABLE "app_email" ADD CONSTRAINT "email_uniq" UNIQUE USING INDEX "email_idx";
-- this second add constraint should error because it's not robust
ALTER TABLE "app_email" ADD CONSTRAINT "email_uniq" UNIQUE USING INDEX "email_idx";
        "#;
        let res = lint_sql(sql).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].kind, RuleViolationKind::PreferRobustStmts);
    }

    /// If the statement is in a transaction, or it has a guard like IF NOT
    /// EXISTS, then it is considered valid by the `prefer-robust-stmt` rule.
    #[test]
    fn prefer_robust_stmt_okay_cases() {
        let sql = r#"
BEGIN;
ALTER TABLE "core_foo" ADD COLUMN "answer_id" integer NULL;
COMMIT;
"#;
        assert_eq!(lint_sql(sql), Ok(vec![]));

        let sql = r#"
ALTER TABLE "core_foo" ADD COLUMN IF NOT EXISTS "answer_id" integer NULL;
"#;
        assert_eq!(lint_sql(sql), Ok(vec![]));

        let sql = r#"
CREATE INDEX CONCURRENTLY IF NOT EXISTS "core_foo_idx" ON "core_foo" ("answer_id");
"#;
        assert_eq!(lint_sql(sql), Ok(vec![]));

        let sql = r#"
BEGIN;
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "bravo" text NOT NULL
);
COMMIT;
"#;
        assert_eq!(lint_sql(sql), Ok(vec![]));

        let sql = r#"
CREATE TABLE IF NOT EXISTS "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "bravo" text NOT NULL
);
"#;
        assert_eq!(lint_sql(sql), Ok(vec![]));

        // If done in a transaction, most forms of drop are fine
        let sql = r#"
BEGIN;
DROP INDEX "core_bar_foo_id_idx";
DROP TABLE "core_bar";
DROP TYPE foo;
COMMIT;
"#;
        assert_eq!(lint_sql(sql), Ok(vec![]));

        // select is fine, we're only interested in modifications to the tables
        let sql = r"
SELECT 1;
";
        assert_eq!(lint_sql(sql), Ok(vec![]));

        // inserts are also okay
        let sql = r"
INSERT INTO tbl VALUES (a);
";
        assert_eq!(lint_sql(sql), Ok(vec![]));

        let sql = r#"
ALTER TABLE "core_foo" DROP CONSTRAINT IF EXISTS "core_foo_idx";
        "#;
        assert_eq!(lint_sql(sql), Ok(vec![]));
    }

    /// If the statement is in a transaction, or it has a guard like IF NOT
    /// EXISTS, then it is considered valid by the `prefer-robust-stmt` rule.
    #[test]
    fn prefer_robust_stmt_okay_cases_with_assume_in_transaction() {
        let sql = r#"
ALTER TABLE "core_foo" ADD COLUMN "answer_id" integer NULL;
"#;
        assert_eq!(lint_sql_assuming_in_transaction(sql), Ok(vec![]));

        let sql = r#"
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "bravo" text NOT NULL
);
"#;
        assert_eq!(lint_sql_assuming_in_transaction(sql), Ok(vec![]));

        let sql = r#"
DROP INDEX "core_bar_foo_id_idx";
DROP TABLE "core_bar";
DROP TYPE foo;
"#;
        assert_eq!(lint_sql_assuming_in_transaction(sql), Ok(vec![]));
    }

    #[test]
    fn create_index_concurrently_unnamed() {
        let bad_sql = r#"
  CREATE INDEX CONCURRENTLY ON "table_name" ("field_name");
  "#;

        assert_debug_snapshot!(lint_sql(bad_sql));
    }

    #[test]
    fn enable_row_level_security() {
        let bad_sql = r"
CREATE TABLE IF NOT EXISTS test();
ALTER TABLE IF EXISTS test ENABLE ROW LEVEL SECURITY;
  ";

        assert_debug_snapshot!(lint_sql(bad_sql));
    }

    #[test]
    fn enable_row_level_security_without_exists_check() {
        let bad_sql = r"
CREATE TABLE IF NOT EXISTS test();
ALTER TABLE test ENABLE ROW LEVEL SECURITY;
  ";

        assert_debug_snapshot!(lint_sql(bad_sql));
    }

    #[test]
    fn disable_row_level_security() {
        let bad_sql = r"
CREATE TABLE IF NOT EXISTS test();
ALTER TABLE IF EXISTS test DISABLE ROW LEVEL SECURITY;
  ";

        assert_debug_snapshot!(lint_sql(bad_sql));
    }

    fn violations(
        res: Result<Vec<RuleViolation>, CheckSqlError>,
    ) -> Result<Vec<RuleViolationKind>, CheckSqlError> {
        match res {
            Ok(res) => Ok(res.into_iter().map(|x| x.kind).collect()),
            Err(err) => Err(err),
        }
    }

    #[test]
    fn ignore_single_stmts() {
        let bad_sql = r#"
  CREATE INDEX CONCURRENTLY ON "table_name" ("field_name");
  "#;
        assert_eq!(
            check_sql_with_rule(bad_sql, &RuleViolationKind::PreferRobustStmts, None, false),
            Ok(vec![])
        );
        let bad_sql = r#"
  CREATE INDEX CONCURRENTLY ON "table_name" ("field_name");
  CREATE INDEX CONCURRENTLY ON "table_name" ("field_name");
  "#;

        assert_eq!(
            violations(check_sql_with_rule(
                bad_sql,
                &RuleViolationKind::PreferRobustStmts,
                None,
                false,
            )),
            Ok(vec![
                RuleViolationKind::PreferRobustStmts,
                RuleViolationKind::PreferRobustStmts
            ])
        );
    }

    #[test]
    fn start_transaction() {
        let sql = r#"
START TRANSACTION;

ALTER TABLE "A" DROP CONSTRAINT "UQ_c4fb579a038211909ee524ccf29";

ALTER TABLE "B" DROP CONSTRAINT "UQ_791c01fe9438d66a94490d0da28";

ALTER TABLE "C" DROP CONSTRAINT "UQ_23fbf20e8ab4e806941359f4f79";

ALTER TABLE "D" DROP CONSTRAINT "UQ_468cad3743146a81c94b0b114ac";

COMMIT;"#;
        assert_eq!(lint_sql(sql), Ok(vec![]));
    }

    #[test]
    fn prefer_robust_stmt_failure_cases() {
        let sql = r#"
ALTER TABLE "core_foo" ADD COLUMN "answer_id" integer NULL;
"#;
        assert_debug_snapshot!(lint_sql(sql), @r#"
        Ok(
            [
                RuleViolation {
                    kind: PreferRobustStmts,
                    span: Span {
                        start: 0,
                        len: Some(
                            59,
                        ),
                    },
                    messages: [
                        Help(
                            "Consider wrapping in a transaction or adding a IF NOT EXISTS clause if the statement supports it.",
                        ),
                    ],
                },
            ],
        )
        "#);

        let sql = r#"
CREATE INDEX CONCURRENTLY "core_foo_idx" ON "core_foo" ("answer_id");
"#;
        assert_debug_snapshot!(lint_sql(sql), @r#"
        Ok(
            [
                RuleViolation {
                    kind: PreferRobustStmts,
                    span: Span {
                        start: 0,
                        len: Some(
                            69,
                        ),
                    },
                    messages: [
                        Help(
                            "Consider wrapping in a transaction or adding a IF NOT EXISTS clause if the statement supports it.",
                        ),
                    ],
                },
            ],
        )
        "#);

        let sql = r#"
CREATE TABLE "core_bar" ( "id" serial NOT NULL PRIMARY KEY, "bravo" text NOT NULL);
"#;
        assert_debug_snapshot!(lint_sql(sql), @r#"
        Ok(
            [
                RuleViolation {
                    kind: PreferRobustStmts,
                    span: Span {
                        start: 0,
                        len: Some(
                            83,
                        ),
                    },
                    messages: [
                        Help(
                            "Consider wrapping in a transaction or adding a IF NOT EXISTS clause if the statement supports it.",
                        ),
                    ],
                },
            ],
        )
        "#);

        let sql = r#"
ALTER TABLE "core_foo" DROP CONSTRAINT "core_foo_idx";
        "#;
        assert_debug_snapshot!(lint_sql(sql), @r#"
        Ok(
            [
                RuleViolation {
                    kind: PreferRobustStmts,
                    span: Span {
                        start: 0,
                        len: Some(
                            54,
                        ),
                    },
                    messages: [
                        Help(
                            "Consider wrapping in a transaction or adding a IF NOT EXISTS clause if the statement supports it.",
                        ),
                    ],
                },
            ],
        )
        "#);
    }
}
