use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{AlterTableCmds, RootStmt, Stmt, TransactionStmtKind};

/// If a migration is running in a transaction, then we skip the statements
/// because if it fails part way through, it will revert.
/// For the cases where statements aren't running in a transaction, for instance,
/// when we CREATE INDEX CONCURRENTLY, we should try and make those migrations
/// more robust by using guards like `IF NOT EXISTS`. So if the migration fails
/// halfway through, it can be rerun without human intervention.
pub fn prefer_robust_stmts(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    let mut inside_transaction = false;
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::TransactionStmt(stmt) => match stmt.kind {
                TransactionStmtKind::Begin => inside_transaction = true,
                TransactionStmtKind::Commit => inside_transaction = false,
                _ => continue,
            },
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    if cmd.missing_ok || inside_transaction {
                        continue;
                    }
                    errs.push(RuleViolation::new(
                        RuleViolationKind::PreferRobustStmts,
                        raw_stmt,
                        None,
                    ));
                }
            }
            Stmt::IndexStmt(stmt) if !stmt.if_not_exists && !inside_transaction => {
                errs.push(RuleViolation::new(
                    RuleViolationKind::PreferRobustStmts,
                    raw_stmt,
                    None,
                ));
            }
            Stmt::CreateStmt(stmt) if !stmt.if_not_exists && !inside_transaction => {
                errs.push(RuleViolation::new(
                    RuleViolationKind::PreferRobustStmts,
                    raw_stmt,
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
    use crate::check_sql;
    use insta::assert_debug_snapshot;
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
                            "Consider wrapping in a transaction or adding a IF NOT EXISTS clause.",
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
                            "Consider wrapping in a transaction or adding a IF NOT EXISTS clause.",
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
                            "Consider wrapping in a transaction or adding a IF NOT EXISTS clause.",
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
                            "Consider wrapping in a transaction or adding a IF NOT EXISTS clause.",
                        ),
                    ],
                },
            ],
        )
        "###);
    }
}
