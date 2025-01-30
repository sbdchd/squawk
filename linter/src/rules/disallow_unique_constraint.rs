use crate::rules::utils::tables_created_in_transaction;
use crate::versions::Version;
use crate::violations::{RuleViolation, RuleViolationKind};

use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, AlterTableType, ColumnDefConstraint, ConstrType, RawStmt, Stmt,
};

#[must_use]
pub fn disallow_unique_constraint(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let tables_created = tables_created_in_transaction(tree, assume_in_transaction);
    let mut errs = vec![];
    for raw_stmt in tree {
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
                        (Some(AlterTableDef::ColumnDef(col)), AlterTableType::AddColumn) => {
                            for ColumnDefConstraint::Constraint(constraint) in &col.constraints {
                                if constraint.contype == ConstrType::Unique {
                                    errs.push(RuleViolation::new(
                                        RuleViolationKind::DisallowedUniqueConstraint,
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
            &RuleViolationKind::DisallowedUniqueConstraint,
            None,
            false,
        )
        .unwrap()
    }

    fn lint_sql_assuming_in_transaction(sql: &str) -> Vec<RuleViolation> {
        check_sql_with_rule(
            sql,
            &RuleViolationKind::DisallowedUniqueConstraint,
            None,
            true,
        )
        .unwrap()
    }

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
    fn adding_unique_constraint() {
        let bad_sql = r"
ALTER TABLE table_name ADD CONSTRAINT field_name_constraint UNIQUE (field_name);
   ";

        let ignored_sql = r"
ALTER TABLE table_name DROP CONSTRAINT field_name_constraint;
   ";

        let ok_sql = r"
CREATE UNIQUE INDEX CONCURRENTLY dist_id_temp_idx ON distributors (dist_id);
ALTER TABLE distributors DROP CONSTRAINT distributors_pkey,
ADD CONSTRAINT distributors_pkey PRIMARY KEY USING INDEX dist_id_temp_idx;
   ";

        assert_debug_snapshot!(lint_sql(bad_sql));
        assert_debug_snapshot!(lint_sql(ignored_sql));
        assert_debug_snapshot!(lint_sql(ok_sql));
    }

    /// Creating a UNIQUE constraint from an existing index should be considered
    /// safe
    #[test]
    fn unique_constraint_ok() {
        let sql = r#"
CREATE UNIQUE INDEX CONCURRENTLY "legacy_questiongrouppg_mongo_id_1f8f47d9_uniq_idx"
    ON "legacy_questiongrouppg" ("mongo_id");
ALTER TABLE "legacy_questiongrouppg" ADD CONSTRAINT "legacy_questiongrouppg_mongo_id_1f8f47d9_uniq" UNIQUE USING INDEX "legacy_questiongrouppg_mongo_id_1f8f47d9_uniq_idx";
        "#;
        assert_eq!(lint_sql(sql), vec![]);
    }

    /// Creating a UNIQUE constraint in the same transaction as the table is create is OK.
    #[test]
    fn unique_constraint_after_create_table() {
        let sql = r"
BEGIN;
CREATE TABLE products (
    id bigint generated by default as identity primary key,
    sku text not null
);
ALTER TABLE products ADD CONSTRAINT sku_constraint UNIQUE (sku);
COMMIT;
        ";
        assert_eq!(lint_sql(sql), vec![]);
    }

    /// Creating a UNIQUE constraint in the same transaction as the table is create is OK.
    #[test]
    fn unique_constraint_after_create_table_with_assume_in_transaction() {
        let sql = r"
CREATE TABLE products (
    id bigint generated by default as identity primary key,
    sku text not null
);
ALTER TABLE products ADD CONSTRAINT sku_constraint UNIQUE (sku);
        ";
        assert_eq!(lint_sql_assuming_in_transaction(sql), vec![]);
    }
    #[test]
    fn unique_constraint_inline_add_column() {
        let sql = r"
ALTER TABLE foo ADD COLUMN bar text CONSTRAINT foo_bar_unique UNIQUE;
    ";
        assert_debug_snapshot!(lint_sql(sql));
    }
    #[test]
    fn unique_constraint_inline_add_column_unique() {
        let sql = r"
ALTER TABLE foo ADD COLUMN bar text UNIQUE;
";
        assert_debug_snapshot!(lint_sql(sql));
    }
}
