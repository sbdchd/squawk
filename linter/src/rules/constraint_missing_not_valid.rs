use std::collections::HashSet;

use crate::rules::utils::tables_created_in_transaction;
use crate::violations::{RuleViolation, RuleViolationKind};
use serde_json::Value;
use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, AlterTableType, RawStmt, RelationKind, RootStmt, Stmt,
    TransactionStmtKind,
};

fn not_valid_validate_in_transaction(tree: &[RootStmt]) -> Vec<String> {
    let mut not_valid_names = HashSet::new();
    let mut bad_indexes: Vec<String> = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::TransactionStmt(stmt) => {
                // if stmt.kind == TransactionStmtKind::Begin {
                //     in_transaction = true;
                // }
                if stmt.kind == TransactionStmtKind::Commit {
                    not_valid_names.clear();
                }
            }
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    if cmd.subtype == AlterTableType::ValidateConstraint {
                        if let Some(constraint_name) = &cmd.name {
                            if not_valid_names.get(constraint_name).is_some() {
                                bad_indexes.push(constraint_name.clone());
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
    bad_indexes
}

#[must_use]
pub fn constraint_missing_not_valid(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    let tables_created = tables_created_in_transaction(tree);
    not_valid_validate_in_transaction(tree)
        .iter()
        .for_each(|index| {
            println!("{}", index);
            errs.push(RuleViolation::new(
                RuleViolationKind::ConstraintMissingNotValid,
                &RawStmt {
                    stmt: Stmt::DropStmt(Value::Null),
                    stmt_location: 123,
                    stmt_len: None,
                },
                None,
            ));
        });
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                let RelationKind::RangeVar(range) = &stmt.relation;
                let tbl_name = &range.relname;
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    match &cmd.def {
                        Some(AlterTableDef::Constraint(constraint)) => {
                            if !tables_created.contains(tbl_name) && constraint.initially_valid {
                                errs.push(RuleViolation::new(
                                    RuleViolationKind::ConstraintMissingNotValid,
                                    raw_stmt,
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
