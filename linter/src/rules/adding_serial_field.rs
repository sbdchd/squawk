use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, ColumnDefConstraint, ConstrType, RootStmt, Stmt,
};

#[must_use]
pub fn adding_serial_primary_key_field(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    match &cmd.def {
                        Some(AlterTableDef::ColumnDef(def)) => {
                            for ColumnDefConstraint::Constraint(constraint) in &def.constraints {
                                if constraint.contype == ConstrType::Primary {
                                    errs.push(RuleViolation::new(
                                        RuleViolationKind::AddingSerialPrimaryKeyField,
                                        raw_stmt,
                                        None,
                                    ));
                                }
                            }
                        }
                        Some(AlterTableDef::Constraint(constraint)) => {
                            if constraint.contype == ConstrType::Primary {
                                errs.push(RuleViolation::new(
                                    RuleViolationKind::AddingSerialPrimaryKeyField,
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

    ///
    /// ```sql
    /// ALTER TABLE a ADD COLUMN IF NOT EXISTS b SERIAL PRIMARY KEY;
    /// ```
    #[test]
    fn test_serial_primary_key() {
        let bad_sql = r#"
ALTER TABLE a ADD COLUMN b SERIAL PRIMARY KEY;
"#;

        let expected_bad_res =
            check_sql(bad_sql, &["prefer-robust-stmts".into()]).unwrap_or_default();
        assert_ne!(expected_bad_res, vec![]);
        assert_debug_snapshot!(expected_bad_res);
    }

    #[test]
    fn test_plain_primary_key() {
        let bad_sql = r#"
ALTER TABLE items ADD PRIMARY KEY (id);
"#;

        let expected_bad_res =
            check_sql(bad_sql, &["prefer-robust-stmts".into()]).unwrap_or_default();
        assert_ne!(expected_bad_res, vec![]);
        assert_debug_snapshot!(expected_bad_res);

        let ok_sql = r#"
ALTER TABLE items ADD CONSTRAINT items_pk PRIMARY KEY USING INDEX items_pk;
"#;
        assert_debug_snapshot!(check_sql(ok_sql, &["prefer-robust-stmts".into()]));
    }
}
