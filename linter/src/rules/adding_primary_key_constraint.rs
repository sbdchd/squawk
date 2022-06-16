use crate::violations::{RuleViolation, RuleViolationKind};
use ::semver::Version;
use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, AlterTableType, ColumnDefConstraint, ConstrType, RawStmt, Stmt,
};

#[must_use]
pub fn adding_primary_key_constraint(
    tree: &[RawStmt],
    _pg_version: &Version,
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
        check_sql,
        violations::{default_pg_version, RuleViolationKind},
    };
    use insta::assert_debug_snapshot;

    #[test]
    fn test_serial_primary_key() {
        let bad_sql = r#"
ALTER TABLE a ADD COLUMN b SERIAL PRIMARY KEY;
"#;

        let expected_bad_res = check_sql(
            bad_sql,
            &[RuleViolationKind::PreferRobustStmts],
            &default_pg_version(),
        )
        .unwrap_or_default();
        assert_ne!(expected_bad_res, vec![]);
        assert_debug_snapshot!(expected_bad_res);
    }

    #[test]
    fn test_plain_primary_key() {
        let bad_sql = r#"
ALTER TABLE items ADD PRIMARY KEY (id);
"#;

        let expected_bad_res = check_sql(
            bad_sql,
            &[RuleViolationKind::PreferRobustStmts],
            &default_pg_version(),
        )
        .unwrap_or_default();
        assert_ne!(expected_bad_res, vec![]);
        assert_debug_snapshot!(expected_bad_res);

        let ok_sql = r#"
ALTER TABLE items ADD CONSTRAINT items_pk PRIMARY KEY USING INDEX items_pk;
"#;
        assert_debug_snapshot!(check_sql(
            ok_sql,
            &[RuleViolationKind::PreferRobustStmts],
            &default_pg_version()
        ));
    }
}
