use crate::rules::utils::tables_created_in_transaction;
use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{AlterTableCmds, AlterTableDef, RelationKind, RootStmt, Stmt};

#[must_use]
pub fn constraint_missing_not_valid(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    let tables_created = tables_created_in_transaction(tree);
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
    use crate::check_sql;
    use insta::assert_debug_snapshot;

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
