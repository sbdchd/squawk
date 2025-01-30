use crate::versions::Version;
use crate::violations::{RuleViolation, RuleViolationKind};
use crate::ViolationMessage;

use squawk_parser::ast::{RawStmt, Stmt, TransactionStmtKind};

#[must_use]
pub fn transaction_nesting(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];
    let mut in_explicit_transaction = false;
    let assume_in_transaction_help = ViolationMessage::Help(
        "Put migration statements in separate files to have them be in separate transactions or \
        don't use the assume-in-transaction setting."
            .into(),
    );

    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::TransactionStmt(stmt) => match stmt.kind {
                TransactionStmtKind::Begin | TransactionStmtKind::Start => {
                    if assume_in_transaction {
                        errs.push(RuleViolation::new(
                                RuleViolationKind::TransactionNesting,
                                raw_stmt.into(),
                                Some(vec![
                                    ViolationMessage::Note(
                                        "There is an existing transaction already in progress, managed by your migration tool.".into()
                                    ),
                                    assume_in_transaction_help.clone(),
                                ])
                            ));
                    } else if in_explicit_transaction {
                        errs.push(RuleViolation::new(
                            RuleViolationKind::TransactionNesting,
                            raw_stmt.into(),
                            None,
                        ));
                    }
                    in_explicit_transaction = true;
                }
                TransactionStmtKind::Commit | TransactionStmtKind::Rollback => {
                    if assume_in_transaction {
                        errs.push(RuleViolation::new(
                                RuleViolationKind::TransactionNesting,
                                raw_stmt.into(),
                                Some(vec![
                                    ViolationMessage::Note(
                                        "Attempting to end the transaction that is managed by your migration tool.".into()
                                    ),
                                    assume_in_transaction_help.clone(),
                                ])
                            ));
                    } else if !in_explicit_transaction {
                        errs.push(RuleViolation::new(
                                RuleViolationKind::TransactionNesting,
                                raw_stmt.into(),
                                Some(vec![
                                    ViolationMessage::Note(
                                        "There is no transaction to COMMIT or ROLLBACK.".into()
                                    ),
                                    ViolationMessage::Help(
                                        "BEGIN a transaction at an earlier point in the migration or remove this statement.".into()
                                    ),
                                ])
                            ));
                    }
                    in_explicit_transaction = false;
                }
                _ => continue,
            },
            _ => continue,
        }
    }
    errs
}

#[cfg(test)]
mod test_rules {
    use crate::{
        check_sql_with_rule,
        violations::{RuleViolation, RuleViolationKind},
    };
    use insta::assert_debug_snapshot;

    fn lint_sql(sql: &str) -> Vec<RuleViolation> {
        check_sql_with_rule(sql, &RuleViolationKind::TransactionNesting, None, false).unwrap()
    }
    fn lint_sql_assuming_in_transaction(sql: &str) -> Vec<RuleViolation> {
        check_sql_with_rule(sql, &RuleViolationKind::TransactionNesting, None, true).unwrap()
    }

    #[test]
    fn no_nesting() {
        let ok_sql = r"
BEGIN;
SELECT 1;
COMMIT;
  ";
        assert_eq!(lint_sql(ok_sql), vec![]);
    }

    #[test]
    fn no_nesting_repeated() {
        let ok_sql = r"
BEGIN;
SELECT 1;
COMMIT;
-- This probably shouldn't be done in a migration. However, Squawk may be linting several
-- migrations that are concatentated, so don't raise a warning here.
BEGIN;
SELECT 2;
COMMIT;
  ";
        assert_eq!(lint_sql(ok_sql), vec![]);
    }

    #[test]
    fn no_nesting_with_assume_in_transaction() {
        let ok_sql = r"
SELECT 1;
  ";
        assert_eq!(lint_sql_assuming_in_transaction(ok_sql), vec![]);
    }

    #[test]
    fn begin_repeated() {
        let bad_sql = r"
BEGIN;
BEGIN;
SELECT 1;
COMMIT;
  ";
        assert_debug_snapshot!(lint_sql(bad_sql));
    }

    #[test]
    fn begin_with_assume_in_transaction() {
        let bad_sql = r"
BEGIN;
SELECT 1;
  ";
        assert_debug_snapshot!(lint_sql_assuming_in_transaction(bad_sql));
    }

    #[test]
    fn commit_repeated() {
        let bad_sql = r"
BEGIN;
SELECT 1;
COMMIT;
COMMIT;
  ";
        assert_debug_snapshot!(lint_sql(bad_sql));
    }

    #[test]
    fn commit_with_assume_in_transaction() {
        let bad_sql = r"
SELECT 1;
COMMIT;
  ";
        assert_debug_snapshot!(lint_sql_assuming_in_transaction(bad_sql));
    }

    #[test]
    fn rollback_with_assume_in_transaction() {
        let bad_sql = r"
SELECT 1;
-- Not sure why rollback would be used in a migration, but test for completeness
ROLLBACK;
  ";
        assert_debug_snapshot!(lint_sql_assuming_in_transaction(bad_sql));
    }
}
