use std::collections::HashSet;

use crate::violations::{RuleViolation, RuleViolationKind, ViolationMessage};
use crate::{rules::utils::tables_created_in_transaction, violations::Span};
use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, AlterTableType, RawStmt, Stmt, TransactionStmtKind,
};

/// Return list of spans for offending transactions. From the start of BEGIN to
/// the end of COMMIT.
fn not_valid_validate_in_transaction(tree: &[RawStmt]) -> Vec<Span> {
    let mut not_valid_names = HashSet::new();
    let mut in_transaction = false;
    let mut in_bad_index = false;
    let mut begin_span_start = 0;
    let mut bad_spans = vec![];
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::TransactionStmt(stmt) => {
                if stmt.kind == TransactionStmtKind::Begin && !in_transaction {
                    in_transaction = true;
                    in_bad_index = false;
                    begin_span_start = raw_stmt.stmt_location;
                }
                if stmt.kind == TransactionStmtKind::Commit {
                    if in_bad_index && in_transaction {
                        bad_spans.push(Span {
                            start: begin_span_start,
                            len: Some(
                                raw_stmt.stmt_location + raw_stmt.stmt_len.unwrap_or_default(),
                            ),
                        });
                    }
                    in_transaction = false;
                }
            }
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    if cmd.subtype == AlterTableType::ValidateConstraint {
                        if let Some(constraint_name) = &cmd.name {
                            if not_valid_names.get(constraint_name).is_some() {
                                in_bad_index = true;
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
pub fn constraint_missing_not_valid(tree: &[RawStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    let tables_created = tables_created_in_transaction(tree);
    for span in not_valid_validate_in_transaction(tree) {
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
    use crate::{check_sql, violations::RuleViolationKind};
    use insta::assert_debug_snapshot;

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
        let res = check_sql(sql, &[]).unwrap();
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
    fn test_adding_foreign_key() {
        let bad_sql = r#"
-- instead of
ALTER TABLE distributors ADD CONSTRAINT distfk FOREIGN KEY (address) REFERENCES addresses (address);
   "#;

        assert_debug_snapshot!(check_sql(bad_sql, &["prefer-robust-stmts".into()]));

        let ok_sql = r#"
-- use `NOT VALID`
ALTER TABLE distributors ADD CONSTRAINT distfk FOREIGN KEY (address) REFERENCES addresses (address) NOT VALID;
ALTER TABLE distributors VALIDATE CONSTRAINT distfk;
   "#;
        assert_debug_snapshot!(check_sql(ok_sql, &["prefer-robust-stmts".into()]));
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
    fn test_adding_check_constraint() {
        let bad_sql = r#"
-- instead of
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0);
   "#;

        let ok_sql = r#"
-- use `NOT VALID`
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0) NOT VALID;
ALTER TABLE accounts VALIDATE CONSTRAINT positive_balance;
   "#;

        assert_debug_snapshot!(check_sql(bad_sql, &["prefer-robust-stmts".into()]));

        assert_debug_snapshot!(check_sql(ok_sql, &["prefer-robust-stmts".into()]));
    }
}
