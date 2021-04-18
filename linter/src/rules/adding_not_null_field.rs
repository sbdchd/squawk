use crate::violations::{RuleViolation, RuleViolationKind};
use crate::ViolationMessage;
use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, AlterTableType, ColumnDefConstraint, ConstrType, RootStmt, Stmt,
};

#[must_use]
pub fn adding_not_nullable_field(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    if cmd.subtype == AlterTableType::SetNotNull {
                        errs.push(RuleViolation::new(
                            RuleViolationKind::AddingNotNullableField,
                            raw_stmt.into(),
                            Some(vec![
                                ViolationMessage::Note("Setting a column NOT NULL blocks reads while the table is scanned.".into()),
                                ViolationMessage::Help("Use a check constraint instead.".into())
                            ]),
                        ))
                    }
                    if cmd.subtype == AlterTableType::AddColumn {
                        if let Some(AlterTableDef::ColumnDef(column_def)) = &cmd.def {
                            for ColumnDefConstraint::Constraint(constraint) in
                                &column_def.constraints
                            {
                                if constraint.contype == ConstrType::NotNull {
                                    errs.push(RuleViolation::new(
                                        RuleViolationKind::AddingNotNullableField,
                                        raw_stmt.into(),
                                        None,
                                    ));
                                }
                            }
                        }
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
    use crate::{check_sql, violations::RuleViolationKind, ViolationMessage};
    use insta::assert_debug_snapshot;

    #[test]
    fn set_null() {
        let sql = r#"
ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET NOT NULL;
        "#;
        let res = check_sql(sql, &["prefer-robust-stmts".into()]).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].kind, RuleViolationKind::AddingNotNullableField);
        assert_eq!(
            res[0].messages,
            vec![
                ViolationMessage::Note(
                    "Setting a column NOT NULL blocks reads while the table is scanned.".into()
                ),
                ViolationMessage::Help("Use a check constraint instead.".into())
            ]
        )
    }

    #[test]
    fn test_adding_field_that_is_not_nullable() {
        let bad_sql = r#"
BEGIN;
--
-- Add field foo to recipe
--
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10 NOT NULL;
ALTER TABLE "core_recipe" ALTER COLUMN "foo" DROP DEFAULT;
COMMIT;
        "#;

        assert_debug_snapshot!(check_sql(bad_sql, &["prefer-robust-stmts".into()]));

        let bad_sql = r#"
-- not sure how this would ever work, but might as well test it
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
        "#;

        assert_debug_snapshot!(check_sql(bad_sql, &["prefer-robust-stmts".into()]));
    }
}
