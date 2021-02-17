use std::collections::HashMap;

use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, AlterTableType, RootStmt, Stmt, TransactionStmtKind,
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
pub fn prefer_robust_stmts(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    let mut inside_transaction = false;
    let mut constraint_names: HashMap<String, Constraint> = HashMap::new();
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::TransactionStmt(stmt) => match stmt.kind {
                TransactionStmtKind::Begin => inside_transaction = true,
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
            Stmt::IndexStmt(stmt) if !stmt.if_not_exists && !inside_transaction => {
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
            _ => continue,
        }
    }
    errs
}

#[cfg(test)]
mod test_rules {
    use crate::{check_sql, violations::RuleViolationKind};
    use insta::assert_debug_snapshot;

    #[test]
    /// If we drop the constraint before adding it, we don't need the IF EXISTS or a transaction.
    fn drop_before_add() {
        let sql = r#"
ALTER TABLE "app_email" DROP CONSTRAINT IF EXISTS "email_uniq";
ALTER TABLE "app_email" ADD CONSTRAINT "email_uniq" UNIQUE USING INDEX "email_idx";
"#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));
    }
    #[test]
    /// DROP CONSTRAINT and then ADD CONSTRAINT is safe. We can also safely run VALIDATE CONSTRAINT.
    fn drop_before_add_foreign_key() {
        let sql = r#"
ALTER TABLE "app_email" DROP CONSTRAINT IF EXISTS "fk_user";
ALTER TABLE "app_email" ADD CONSTRAINT "fk_user" FOREIGN KEY ("user_id") REFERENCES "app_user" ("id") DEFERRABLE INITIALLY DEFERRED NOT VALID;
ALTER TABLE "app_email" VALIDATE CONSTRAINT "fk_user";
"#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));
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
        let res = check_sql(sql, &[]).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].kind, RuleViolationKind::PreferRobustStmts);
    }

    /// If the statement is in a transaction, or it has a guard like IF NOT
    /// EXISTS, then it is considered valid by the `prefer-robust-stmt` rule.
    #[test]
    fn test_prefer_robust_stmt_okay_cases() {
        let sql = r#"
BEGIN;
ALTER TABLE "core_foo" ADD COLUMN "answer_id" integer NULL;
COMMIT;
"#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));

        let sql = r#"
ALTER TABLE "core_foo" ADD COLUMN IF NOT EXISTS "answer_id" integer NULL;
"#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));

        let sql = r#"
CREATE INDEX CONCURRENTLY IF NOT EXISTS "core_foo_idx" ON "core_foo" ("answer_id");
"#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));

        let sql = r#"
BEGIN;
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "bravo" text NOT NULL
);
COMMIT;
"#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));

        let sql = r#"
CREATE TABLE IF NOT EXISTS "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "bravo" text NOT NULL
);
"#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));

        // select is fine, we're only interested in modifications to the tables
        let sql = r#"
SELECT 1;
"#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));

        // inserts are also okay
        let sql = r#"
INSERT INTO tbl VALUES (a);
"#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));

        let sql = r#"
ALTER TABLE "core_foo" DROP CONSTRAINT IF EXISTS "core_foo_idx";
        "#;
        assert_eq!(check_sql(sql, &[]), Ok(vec![]));
    }

    #[test]
    fn test_prefer_robust_stmt_failure_cases() {
        let sql = r#"
ALTER TABLE "core_foo" ADD COLUMN "answer_id" integer NULL;
"#;
        assert_debug_snapshot!(check_sql(sql, &[]), @r###"
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
                            "Consider wrapping in a transaction or adding a IF NOT EXISTS clause if the statment supports it.",
                        ),
                    ],
                },
            ],
        )
        "###);

        let sql = r#"
CREATE INDEX CONCURRENTLY "core_foo_idx" ON "core_foo" ("answer_id");
"#;
        assert_debug_snapshot!(check_sql(sql, &[]), @r###"
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
                            "Consider wrapping in a transaction or adding a IF NOT EXISTS clause if the statment supports it.",
                        ),
                    ],
                },
            ],
        )
        "###);

        let sql = r#"
CREATE TABLE "core_bar" ( "id" serial NOT NULL PRIMARY KEY, "bravo" text NOT NULL);
"#;
        assert_debug_snapshot!(check_sql(sql, &[]), @r###"
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
                            "Consider wrapping in a transaction or adding a IF NOT EXISTS clause if the statment supports it.",
                        ),
                    ],
                },
            ],
        )
        "###);

        let sql = r#"
ALTER TABLE "core_foo" DROP CONSTRAINT "core_foo_idx";
        "#;
        assert_debug_snapshot!(check_sql(sql, &[]), @r###"
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
                            "Consider wrapping in a transaction or adding a IF NOT EXISTS clause if the statment supports it.",
                        ),
                    ],
                },
            ],
        )
        "###);
    }
}
