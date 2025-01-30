use std::collections::HashSet;

use crate::versions::Version;
use crate::violations::{RuleViolation, RuleViolationKind, ViolationMessage};
use crate::{rules::utils::tables_created_in_transaction, violations::Span};
use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, AlterTableType, RawStmt, Stmt, TransactionStmtKind,
};

/// Return list of spans for offending transactions. From the start of BEGIN to
/// the end of COMMIT.
fn not_valid_validate_in_transaction(tree: &[RawStmt], assume_in_transaction: bool) -> Vec<Span> {
    let mut not_valid_names = HashSet::new();
    let mut in_transaction = assume_in_transaction;
    let mut bad_spans = vec![];
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::TransactionStmt(stmt) => {
                if (stmt.kind == TransactionStmtKind::Begin
                    || stmt.kind == TransactionStmtKind::Start)
                    && !in_transaction
                {
                    in_transaction = true;
                    not_valid_names.clear();
                }
                if stmt.kind == TransactionStmtKind::Commit {
                    in_transaction = false;
                }
            }
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    if cmd.subtype == AlterTableType::ValidateConstraint {
                        if let Some(constraint_name) = &cmd.name {
                            if in_transaction && not_valid_names.contains(constraint_name) {
                                bad_spans.push(raw_stmt.into());
                            }
                        }
                    }
                    match &cmd.def {
                        Some(AlterTableDef::Constraint(constraint)) => {
                            if !constraint.initially_valid {
                                if let Some(constraint_name) = &constraint.conname {
                                    not_valid_names.insert(constraint_name);
                                }
                            }
                        }
                        _ => continue,
                    }
                }
            }
            _ => continue,
        }
    }
    bad_spans
}

#[must_use]
pub fn constraint_missing_not_valid(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];
    let tables_created = tables_created_in_transaction(tree, assume_in_transaction);
    for span in not_valid_validate_in_transaction(tree, assume_in_transaction) {
        errs.push(RuleViolation::new(
                RuleViolationKind::ConstraintMissingNotValid,
                span,
                Some(vec![
                    ViolationMessage::Note("Using NOT VALID and VALIDATE CONSTRAINT in the same transaction will block all reads while the constraint is validated.".into()), ViolationMessage::Help("Add constraint as NOT VALID in one transaction and VALIDATE CONSTRAINT in a separate transaction.".into())
                ]),
            ));
    }
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                let range = &stmt.relation;
                let tbl_name = &range.relname;
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    match &cmd.def {
                        Some(AlterTableDef::Constraint(constraint)) => {
                            if !tables_created.contains(tbl_name) && constraint.initially_valid {
                                errs.push(RuleViolation::new(
                                    RuleViolationKind::ConstraintMissingNotValid,
                                    raw_stmt.into(),
                                    None,
                                ));
                            }
                        }
                        _ => continue,
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
    use insta::assert_debug_snapshot;

    use crate::{
        check_sql_with_rule,
        violations::{RuleViolation, RuleViolationKind},
    };

    fn lint_sql(sql: &str) -> Vec<RuleViolation> {
        check_sql_with_rule(
            sql,
            &RuleViolationKind::ConstraintMissingNotValid,
            None,
            false,
        )
        .unwrap()
    }

    fn lint_sql_assuming_in_transaction(sql: &str) -> Vec<RuleViolation> {
        check_sql_with_rule(
            sql,
            &RuleViolationKind::ConstraintMissingNotValid,
            None,
            true,
        )
        .unwrap()
    }

    #[test]
    fn ensure_ignored_when_new_table() {
        let sql = r#"
BEGIN;
CREATE TABLE "core_foo" (
"id" serial NOT NULL PRIMARY KEY,
"age" integer NOT NULL
);
ALTER TABLE "core_foo" ADD CONSTRAINT "age_restriction" CHECK ("age" >= 25);
COMMIT;
    "#;

        assert_debug_snapshot!(lint_sql(sql));
    }

    #[test]
    fn ensure_ignored_when_new_table_with_assume_in_transaction() {
        let sql = r#"
CREATE TABLE "core_foo" (
"id" serial NOT NULL PRIMARY KEY,
"age" integer NOT NULL
);
ALTER TABLE "core_foo" ADD CONSTRAINT "age_restriction" CHECK ("age" >= 25);
    "#;

        assert_debug_snapshot!(lint_sql_assuming_in_transaction(sql));
    }

    /// Using NOT VALID and VALIDATE in a single transaction is equivalent to
    /// adding a constraint without NOT VALID. It will block!
    #[test]
    fn not_valid_validate_in_transaction() {
        let sql = r#"
BEGIN;
ALTER TABLE "app_email" ADD CONSTRAINT "fk_user" FOREIGN KEY (user_id) REFERENCES "app_user" (id) NOT VALID;
ALTER TABLE "app_email" VALIDATE CONSTRAINT "fk_user";
COMMIT;
"#;
        let res = lint_sql(sql);
        assert_eq!(
            res.len(),
            1,
            "it's unsafe to run NOT VALID with VALIDATE in a transaction."
        );
        assert_eq!(res[0].kind, RuleViolationKind::ConstraintMissingNotValid);
        // We have a custom error message for this case.
        assert_debug_snapshot!(res[0].messages);
    }

    /// Using NOT VALID and VALIDATE in a single transaction is equivalent to
    /// adding a constraint without NOT VALID. It will block!
    #[test]
    fn not_valid_validate_with_assume_in_transaction() {
        let sql = r#"
ALTER TABLE "app_email" ADD CONSTRAINT "fk_user" FOREIGN KEY (user_id) REFERENCES "app_user" (id) NOT VALID;
ALTER TABLE "app_email" VALIDATE CONSTRAINT "fk_user";
"#;
        let res = lint_sql_assuming_in_transaction(sql);
        assert_eq!(
            res.len(),
            1,
            "it's unsafe to run NOT VALID with VALIDATE in a transaction."
        );
        assert_eq!(res[0].kind, RuleViolationKind::ConstraintMissingNotValid);
        // We have a custom error message for this case.
        assert_debug_snapshot!(res[0].messages);
    }

    /// This builds off of the previous test to see that the error is correctly
    /// attributed when using the "assume in transaction" option and an
    /// explicit COMMIT.
    #[test]
    fn not_valid_validate_with_assume_in_transaction_with_explicit_commit() {
        let sql = r#"
ALTER TABLE "app_email" ADD CONSTRAINT "fk_user" FOREIGN KEY (user_id) REFERENCES "app_user" (id) NOT VALID;
ALTER TABLE "app_email" VALIDATE CONSTRAINT "fk_user";
COMMIT;
"#;
        let res = lint_sql_assuming_in_transaction(sql);
        assert_eq!(
            res.len(),
            1,
            "it's unsafe to run NOT VALID with VALIDATE in a transaction."
        );
        assert_eq!(res[0].kind, RuleViolationKind::ConstraintMissingNotValid);
        // We have a custom error message for this case.
        assert_debug_snapshot!(res[0].messages);
    }

    /// ```sql
    /// -- instead of
    /// ALTER TABLE distributors ADD CONSTRAINT distfk FOREIGN KEY (address) REFERENCES addresses (address);
    /// -- use `NOT VALID`
    /// ALTER TABLE distributors ADD CONSTRAINT distfk FOREIGN KEY (address) REFERENCES addresses (address) NOT VALID;
    /// ALTER TABLE distributors VALIDATE CONSTRAINT distfk;
    /// ```
    #[test]
    fn adding_foreign_key() {
        let bad_sql = r"
-- instead of
ALTER TABLE distributors ADD CONSTRAINT distfk FOREIGN KEY (address) REFERENCES addresses (address);
   ";

        assert_debug_snapshot!(lint_sql(bad_sql));

        let ok_sql = r"
-- use `NOT VALID`
ALTER TABLE distributors ADD CONSTRAINT distfk FOREIGN KEY (address) REFERENCES addresses (address) NOT VALID;
ALTER TABLE distributors VALIDATE CONSTRAINT distfk;
   ";
        assert_debug_snapshot!(lint_sql(ok_sql));
    }

    ///
    /// ```sql
    /// -- instead of
    /// ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0);
    ///
    /// -- use `NOT VALID`
    /// ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0) NOT VALID;
    /// ALTER TABLE accounts VALIDATE CONSTRAINT positive_balance;
    /// ```
    #[test]
    fn adding_check_constraint() {
        let bad_sql = r#"
-- instead of
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0);
   "#;

        let ok_sql = r#"
-- use `NOT VALID`
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0) NOT VALID;
ALTER TABLE accounts VALIDATE CONSTRAINT positive_balance;
   "#;

        assert_debug_snapshot!(lint_sql(bad_sql));

        assert_debug_snapshot!(lint_sql(ok_sql));
    }

    #[test]
    fn regression_with_indexing_2() {
        let sql = r#"
BEGIN;
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
ALTER TABLE "core_recipe" ADD CONSTRAINT foo_not_null
    CHECK ("foo" IS NOT NULL) NOT VALID;
COMMIT;
BEGIN;

"#;
        assert_debug_snapshot!(lint_sql(sql));
    }
}
