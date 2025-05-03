use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{
    AlterTableCmds, AlterTableDef, AlterTableType, ColumnDefConstraint, ConstrType, RawStmt, Stmt,
};

/// Adding a foreign key constraint requires a table scan and a
/// SHARE ROW EXCLUSIVE lock on both tables, which blocks writes.
///
/// Adding the constraint as NOT VALID in one transaction and then using
/// VALIDATE in another transaction will allow writes when adding the
/// constraint.
#[must_use]
pub fn adding_foreign_key_constraint(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        match &raw_stmt.stmt {
            Stmt::AlterTableStmt(stmt) => {
                for cmd in &stmt.cmds {
                    match cmd {
                        AlterTableCmds::AlterTableCmd(ref command) => {
                            if let AlterTableType::AddConstraint = command.subtype {
                                if let Some(AlterTableDef::Constraint(constraint)) = &command.def {
                                    // Adding foreign key is okay when NOT VALID is specified.
                                    if constraint.skip_validation {
                                        continue;
                                    }
                                    if constraint.contype == ConstrType::Foreign {
                                        errs.push(RuleViolation::new(
                                            RuleViolationKind::AddingForeignKeyConstraint,
                                            raw_stmt.into(),
                                            None,
                                        ));
                                    }
                                }
                            } else if AlterTableType::AddColumn == command.subtype {
                                if let Some(AlterTableDef::ColumnDef(column_def)) = &command.def {
                                    for ColumnDefConstraint::Constraint(constraint) in
                                        &column_def.constraints
                                    {
                                        if !constraint.skip_validation
                                            && constraint.contype == ConstrType::Foreign
                                        {
                                            errs.push(RuleViolation::new(
                                                RuleViolationKind::AddingForeignKeyConstraint,
                                                raw_stmt.into(),
                                                None,
                                            ));
                                        }
                                    }
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
    use crate::{
        check_sql_with_rule,
        violations::{RuleViolation, RuleViolationKind},
    };

    fn lint_sql(sql: &str) -> Vec<RuleViolation> {
        check_sql_with_rule(
            sql,
            &RuleViolationKind::AddingForeignKeyConstraint,
            None,
            false,
        )
        .unwrap()
    }

    #[test]
    fn create_table_with_foreign_key_constraint() {
        let sql = r#"
BEGIN;
CREATE TABLE email (
    id BIGINT GENERATED ALWAYS AS IDENTITY,
    user_id BIGINT,
    email TEXT,
    PRIMARY KEY(id),
    CONSTRAINT fk_user
        FOREIGN KEY ("user_id") 
        REFERENCES "user" ("id")
);
COMMIT;
        "#;

        let violations = lint_sql(sql);
        assert_eq!(violations.len(), 0);
    }
    #[test]
    fn add_foreign_key_constraint_not_valid_validate() {
        let sql = r#"
BEGIN;
ALTER TABLE "email" ADD COLUMN "user_id" INT;
ALTER TABLE "email" ADD CONSTRAINT "fk_user" FOREIGN KEY ("user_id") REFERENCES "user" ("id") NOT VALID;
ALTER TABLE "email" VALIDATE CONSTRAINT "fk_user";
COMMIT;
        "#;

        let violations = lint_sql(sql);
        assert_eq!(violations.len(), 0);
    }
    #[test]
    fn add_foreign_key_constraint_lock() {
        let sql = r#"
BEGIN;
ALTER TABLE "email" ADD COLUMN "user_id" INT;
ALTER TABLE "email" ADD CONSTRAINT "fk_user" FOREIGN KEY ("user_id") REFERENCES "user" ("id");
COMMIT;
        "#;

        let violations = lint_sql(sql);
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].kind,
            RuleViolationKind::AddingForeignKeyConstraint
        );
    }
    #[test]
    fn add_column_references_lock() {
        let sql = r#"
BEGIN;
ALTER TABLE "emails" ADD COLUMN "user_id" INT REFERENCES "user" ("id");
COMMIT;
        "#;

        let violations = lint_sql(sql);
        assert_eq!(violations.len(), 1);
        assert_eq!(
            violations[0].kind,
            RuleViolationKind::AddingForeignKeyConstraint
        );
    }
}
