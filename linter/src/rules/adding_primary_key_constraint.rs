use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, AlterTableType, ColumnDefConstraint, ConstrType, RawStmt, Stmt,
};

#[must_use]
pub fn adding_primary_key_constraint(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    match (&cmd.def, &cmd.subtype) {
                        (
                            Some(AlterTableDef::Constraint(constraint)),
                            AlterTableType::AddConstraint,
                        ) => {
                            if constraint.contype == ConstrType::Primary
                                && constraint.indexname.is_none()
                            {
                                errs.push(RuleViolation::new(
                                    RuleViolationKind::AddingSerialPrimaryKeyField,
                                    raw_stmt.into(),
                                    None,
                                ));
                            }
                        }
                        (Some(AlterTableDef::ColumnDef(def)), _) => {
                            for ColumnDefConstraint::Constraint(constraint) in &def.constraints {
                                if constraint.contype == ConstrType::Primary
                                    && constraint.indexname.is_none()
                                {
                                    errs.push(RuleViolation::new(
                                        RuleViolationKind::AddingSerialPrimaryKeyField,
                                        raw_stmt.into(),
                                        None,
                                    ));
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
        check_sql_with_rule(
            sql,
            &RuleViolationKind::AddingSerialPrimaryKeyField,
            None,
            false,
        )
        .unwrap()
    }

    #[test]
    fn serial_primary_key() {
        let bad_sql = r"
ALTER TABLE a ADD COLUMN b SERIAL PRIMARY KEY;
";

        let expected_bad_res = lint_sql(bad_sql);
        assert_ne!(expected_bad_res, vec![]);
        assert_debug_snapshot!(expected_bad_res);
    }

    #[test]
    fn plain_primary_key() {
        let bad_sql = r"
ALTER TABLE items ADD PRIMARY KEY (id);
";

        let expected_bad_res = lint_sql(bad_sql);
        assert_ne!(expected_bad_res, vec![]);
        assert_debug_snapshot!(expected_bad_res);

        let ok_sql = r"
ALTER TABLE items ADD CONSTRAINT items_pk PRIMARY KEY USING INDEX items_pk;
";
        assert_debug_snapshot!(lint_sql(ok_sql,));
    }
}
