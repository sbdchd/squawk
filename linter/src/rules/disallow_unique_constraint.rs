use crate::rules::utils::tables_created_in_transaction;
use crate::violations::{RuleViolation, RuleViolationKind};
use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, AlterTableType, ConstrType, RootStmt, Stmt,
};

#[must_use]
pub fn disallow_unique_constraint(tree: &[RootStmt]) -> Vec<RuleViolation> {
    let tables_created = tables_created_in_transaction(tree);
    let mut errs = vec![];
    for RootStmt::RawStmt(raw_stmt) in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                let range = &stmt.relation;
                let tbl_name = &range.relname;
                for AlterTableCmds::AlterTableCmd(cmd) in &stmt.cmds {
                    match (&cmd.def, &cmd.subtype) {
                        (
                            Some(AlterTableDef::Constraint(constraint)),
                            AlterTableType::AddConstraint,
                        ) => {
                            if !tables_created.contains(tbl_name)
                                && constraint.contype == ConstrType::Unique
                                && constraint.indexname.is_none()
                            {
                                errs.push(RuleViolation::new(
                                    RuleViolationKind::DisallowedUniqueConstraint,
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
    use crate::check_sql;
    use insta::assert_debug_snapshot;

    /// ```sql
    /// -- instead of
    /// ALTER TABLE table_name ADD CONSTRAINT field_name_constraint UNIQUE (field_name);
    /// -- also works with PRIMARY KEY
    /// -- use:
    /// -- To recreate a primary key constraint, without blocking updates while the index is rebuilt:
    /// CREATE UNIQUE INDEX CONCURRENTLY dist_id_temp_idx ON distributors (dist_id);
    /// ALTER TABLE distributors DROP CONSTRAINT distributors_pkey,
    /// ADD CONSTRAINT distributors_pkey PRIMARY KEY USING INDEX dist_id_temp_idx;
    /// ```
    #[test]
    fn test_adding_unique_constraint() {
        let bad_sql = r#"
ALTER TABLE table_name ADD CONSTRAINT field_name_constraint UNIQUE (field_name);
   "#;

        let ignored_sql = r#"
ALTER TABLE table_name DROP CONSTRAINT field_name_constraint;
   "#;

        let ok_sql = r#"
CREATE UNIQUE INDEX CONCURRENTLY dist_id_temp_idx ON distributors (dist_id);
ALTER TABLE distributors DROP CONSTRAINT distributors_pkey,
ADD CONSTRAINT distributors_pkey PRIMARY KEY USING INDEX dist_id_temp_idx;
   "#;

        assert_debug_snapshot!(check_sql(bad_sql, &["prefer-robust-stmts".into()]));
        assert_debug_snapshot!(check_sql(ignored_sql, &["prefer-robust-stmts".into()]));
        assert_debug_snapshot!(check_sql(ok_sql, &["prefer-robust-stmts".into()]));
    }

    /// Creating a UNIQUE constraint from an existing index should be considered
    /// safe
    #[test]
    fn test_unique_constraint_ok() {
        let sql = r#"
CREATE UNIQUE INDEX CONCURRENTLY "legacy_questiongrouppg_mongo_id_1f8f47d9_uniq_idx"
    ON "legacy_questiongrouppg" ("mongo_id");
ALTER TABLE "legacy_questiongrouppg" ADD CONSTRAINT "legacy_questiongrouppg_mongo_id_1f8f47d9_uniq" UNIQUE USING INDEX "legacy_questiongrouppg_mongo_id_1f8f47d9_uniq_idx";
        "#;
        assert_eq!(check_sql(sql, &["prefer-robust-stmts".into()]), Ok(vec![]));
    }
}
