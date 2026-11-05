use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;

use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode, NameLike},
};

use crate::name::Name;
use crate::transactions::{TablesCreatedInTransaction, table_name_ref};
use crate::{Linter, Rule, Version, Violation};

type TableName = SmallVec<[Name; 2]>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TableColumn {
    table: TableName,
    column: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct TableConstraint {
    table: TableName,
    constraint: String,
}

fn is_not_null_check(expr: &ast::Expr) -> Option<String> {
    let ast::Expr::BinExpr(bin_expr) = expr.unparen() else {
        return None;
    };
    let ast::BinOp::IsNot(_) = bin_expr.op()? else {
        return None;
    };

    let lit = bin_expr.rhs()?.as_literal()?;
    if !matches!(lit.kind(), Some(ast::LitKind::Null(_))) {
        return None;
    }

    match bin_expr.lhs()?.unparen() {
        ast::Expr::NameRef(name_ref) => Some(name_ref.text().into_owned()),
        _ => None,
    }
}

fn get_table_path(alter_table: &ast::AlterTable) -> Option<ast::PathRef> {
    alter_table
        .table_relation_name()?
        .table_name_ref()?
        .path_ref()
}

pub(crate) fn adding_not_null_field(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();

    let is_pg12_plus = ctx.settings.pg_version >= Version::new(12, None, None);

    let mut not_null_constraints: FxHashMap<String, TableColumn> = FxHashMap::default();
    let mut validated_not_null_columns: FxHashSet<TableColumn> = FxHashSet::default();
    // VALIDATE CONSTRAINT seen without a matching ADD CONSTRAINT in the same
    // file (cross-migration pattern).
    let mut external_validated_constraints: FxHashSet<TableConstraint> = FxHashSet::default();

    let mut tables_created = TablesCreatedInTransaction::new(ctx.settings.assume_in_transaction);

    for stmt in file.stmts() {
        tables_created.update(&stmt);
        if let ast::Stmt::AlterTable(alter_table) = stmt {
            let Some(path) = get_table_path(&alter_table) else {
                continue;
            };
            let created_in_transaction = tables_created.contains(&path);
            let table = table_name_ref(path);

            for action in alter_table.actions() {
                match action {
                    // Step 1: Add constraint
                    ast::AlterTableAction::AddConstraint(add_constraint) => {
                        if !is_pg12_plus {
                            continue;
                        }
                        let Some(constraint) = add_constraint.constraint() else {
                            continue;
                        };
                        if !constraint.is_not_valid() {
                            continue;
                        }
                        if let ast::Constraint::CheckConstraint(check) = constraint
                            && let Some(constraint_name) = check
                                .constraint_name_clause()
                                .and_then(|clause| clause.constraint_name())
                            && let Some(expr) = check.expr()
                            && let Some(column) = is_not_null_check(&expr)
                        {
                            not_null_constraints.insert(
                                constraint_name.text().into_owned(),
                                TableColumn {
                                    table: table.clone(),
                                    column,
                                },
                            );
                        }
                    }
                    // Step 2: Validate constraint
                    ast::AlterTableAction::ValidateConstraint(validate_constraint)
                        if is_pg12_plus =>
                    {
                        if let Some(constraint_name) = validate_constraint
                            .constraint_name_ref()
                            .and_then(|constraint| constraint.path_ref())
                            .and_then(|path| path.segment())
                        {
                            if let Some(table_column) =
                                not_null_constraints.get(constraint_name.text().as_ref())
                                && table_column.table == table
                            {
                                validated_not_null_columns.insert(table_column.clone());
                            } else {
                                // Cross-migration pattern: the ADD CONSTRAINT ... NOT VALID
                                // was in a previous migration file. Track the constraint so
                                // that a subsequent SET NOT NULL on a column it's named after
                                // is considered safe.
                                external_validated_constraints.insert(TableConstraint {
                                    table: table.clone(),
                                    constraint: constraint_name.text().into_owned(),
                                });
                            }
                        }
                    }
                    // Step 3: Check that we're altering a validated constraint
                    ast::AlterTableAction::AlterColumn(alter_column) => {
                        let Some(ast::AlterColumnOption::SetNotNull(option)) =
                            alter_column.option()
                        else {
                            continue;
                        };
                        if created_in_transaction {
                            continue;
                        }

                        if is_pg12_plus
                            && let Some(column) = alter_column
                                .column_name_ref()
                                .map(|column| column.text().into_owned())
                        {
                            let validated_externally =
                                external_validated_constraints.iter().any(|validated| {
                                    validated.table == table
                                        && validated.constraint.contains(column.as_str())
                                });
                            let table_column = TableColumn {
                                table: table.clone(),
                                column,
                            };
                            if validated_externally
                                || validated_not_null_columns.contains(&table_column)
                            {
                                continue;
                            }
                        }

                        ctx.report(
                            Violation::for_node(
                                Rule::AddingNotNullableField,
                                "setting a column `not null` blocks reads while the table is scanned"
                                    .into(),
                                option.syntax(),
                            )
                            .help("make the field nullable and use a `check` constraint instead"),
                        );
                    }
                    _ => (),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::test_utils::{lint_errors, lint_errors_with, lint_ok, lint_ok_with};
    use crate::{LinterSettings, Rule};

    #[test]
    fn set_not_null() {
        let sql = r#"
ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET NOT NULL;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::AddingNotNullableField));
    }

    #[test]
    fn domain_and_foreign_table_set_not_null_are_not_locking_warnings() {
        lint_ok(
            "ALTER DOMAIN d SET NOT NULL; ALTER FOREIGN TABLE ft ALTER COLUMN c SET NOT NULL;",
            Rule::AddingNotNullableField,
        );
    }

    #[test]
    fn adding_field_that_is_not_nullable() {
        let sql = r#"
BEGIN;
-- This will cause a table rewrite for Postgres versions before 11, but that is handled by
-- adding-field-with-default.
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10 NOT NULL;
ALTER TABLE "core_recipe" ALTER COLUMN "foo" DROP DEFAULT;
COMMIT;
        "#;
        lint_ok(sql, Rule::AddingNotNullableField);
    }

    #[test]
    fn adding_field_that_is_not_nullable_without_default() {
        let sql = r#"
-- This won't work if the table is populated, but that error is caught by adding-required-field.
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
        "#;
        lint_ok(sql, Rule::AddingNotNullableField);
    }

    #[test]
    fn adding_field_that_is_not_nullable_in_version_11() {
        let sql = r#"
BEGIN;
--
-- Add field foo to recipe
--
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL DEFAULT 10;
COMMIT;
        "#;
        lint_ok(sql, Rule::AddingNotNullableField);
    }

    #[test]
    fn regression_gh_issue_519() {
        let sql = r#"
BEGIN;
-- Running upgrade a -> b
ALTER TABLE my_table ALTER COLUMN my_column SET NOT NULL;
UPDATE alembic_version SET version_num='b' WHERE alembic_version.version_num = 'a';
COMMIT;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::AddingNotNullableField));
    }

    // GitHub issue #628: SET NOT NULL should be safe on PostgreSQL 12+ when
    // there's a validated CHECK constraint for the column.
    #[test]
    fn regression_gh_issue_628_pg16_with_validated_check_ok() {
        let sql = r#"
BEGIN;
ALTER TABLE foo ADD COLUMN bar BIGINT;
ALTER TABLE foo ADD CONSTRAINT bar_not_null CHECK (bar IS NOT NULL) NOT VALID;
COMMIT;

BEGIN;
ALTER TABLE foo VALIDATE CONSTRAINT bar_not_null;
ALTER TABLE foo ALTER COLUMN bar SET NOT NULL;
ALTER TABLE foo DROP CONSTRAINT bar_not_null;
COMMIT;
        "#;

        lint_ok_with(
            sql,
            LinterSettings {
                pg_version: "16".parse().expect("Invalid PostgreSQL version"),
                ..Default::default()
            },
            Rule::AddingNotNullableField,
        );
    }

    #[test]
    fn regression_gh_issue_628_pg12_with_validated_check_ok() {
        let sql = r#"
BEGIN;
ALTER TABLE foo ADD COLUMN bar BIGINT;
ALTER TABLE foo ADD CONSTRAINT bar_not_null CHECK (bar IS NOT NULL) NOT VALID;
COMMIT;

BEGIN;
ALTER TABLE foo VALIDATE CONSTRAINT bar_not_null;
ALTER TABLE foo ALTER COLUMN bar SET NOT NULL;
ALTER TABLE foo DROP CONSTRAINT bar_not_null;
COMMIT;
        "#;
        lint_ok_with(
            sql,
            LinterSettings {
                pg_version: "12".parse().expect("Invalid PostgreSQL version"),
                ..Default::default()
            },
            Rule::AddingNotNullableField,
        );
    }

    #[test]
    fn regression_gh_issue_628_pg11_with_validated_check_err() {
        // PostgreSQL 11 doesn't support using CHECK constraint to skip table scan
        let sql = r#"
BEGIN;
ALTER TABLE foo ADD COLUMN bar BIGINT;
ALTER TABLE foo ADD CONSTRAINT bar_not_null CHECK (bar IS NOT NULL) NOT VALID;
COMMIT;

BEGIN;
ALTER TABLE foo VALIDATE CONSTRAINT bar_not_null;
ALTER TABLE foo ALTER COLUMN bar SET NOT NULL;
COMMIT;
        "#;
        assert_snapshot!(lint_errors_with(
            sql,
            LinterSettings {
                pg_version: "11".parse().expect("Invalid PostgreSQL version"),
                ..Default::default()
            },
            Rule::AddingNotNullableField
        ));
    }

    #[test]
    fn pg12_without_validated_check_err() {
        // Without a validated CHECK constraint, SET NOT NULL is still unsafe
        let sql = r#"
ALTER TABLE foo ALTER COLUMN bar SET NOT NULL;
        "#;
        assert_snapshot!(lint_errors_with(
            sql,
            LinterSettings {
                pg_version: "12".parse().expect("Invalid PostgreSQL version"),
                ..Default::default()
            },
            Rule::AddingNotNullableField
        ));
    }

    #[test]
    fn pg12_with_check_but_not_validated_err() {
        // CHECK constraint exists but not validated yet
        let sql = r#"
ALTER TABLE foo ADD CONSTRAINT bar_not_null CHECK (bar IS NOT NULL) NOT VALID;
ALTER TABLE foo ALTER COLUMN bar SET NOT NULL;
        "#;
        assert_snapshot!(lint_errors_with(
            sql,
            LinterSettings {
                pg_version: "12".parse().expect("Invalid PostgreSQL version"),
                ..Default::default()
            },
            Rule::AddingNotNullableField
        ));
    }

    // Cross-migration pattern: ADD CONSTRAINT was in a previous migration file,
    // so only VALIDATE CONSTRAINT + SET NOT NULL appear in this file.
    #[test]
    fn pg16_cross_migration_validate_then_set_not_null_ok() {
        let sql = r#"
ALTER TABLE foo VALIDATE CONSTRAINT foo_bar_not_null;

ALTER TABLE foo
ALTER COLUMN bar
SET NOT NULL;

ALTER TABLE foo
DROP CONSTRAINT foo_bar_not_null;
        "#;
        lint_ok_with(
            sql,
            LinterSettings {
                pg_version: "16".parse().expect("Invalid PostgreSQL version"),
                ..Default::default()
            },
            Rule::AddingNotNullableField,
        );
    }

    #[test]
    fn pg12_cross_migration_validate_then_set_not_null_ok() {
        let sql = r#"
ALTER TABLE foo VALIDATE CONSTRAINT foo_bar_not_null;
ALTER TABLE foo ALTER COLUMN bar SET NOT NULL;
ALTER TABLE foo DROP CONSTRAINT foo_bar_not_null;
        "#;
        lint_ok_with(
            sql,
            LinterSettings {
                pg_version: "12".parse().expect("Invalid PostgreSQL version"),
                ..Default::default()
            },
            Rule::AddingNotNullableField,
        );
    }

    #[test]
    fn pg11_cross_migration_validate_then_set_not_null_err() {
        // PostgreSQL 11 doesn't support using CHECK constraint to skip table scan
        let sql = r#"
ALTER TABLE foo VALIDATE CONSTRAINT foo_bar_not_null;
ALTER TABLE foo ALTER COLUMN bar SET NOT NULL;
ALTER TABLE foo DROP CONSTRAINT foo_bar_not_null;
        "#;
        assert_snapshot!(lint_errors_with(
            sql,
            LinterSettings {
                pg_version: "11".parse().expect("Invalid PostgreSQL version"),
                ..Default::default()
            },
            Rule::AddingNotNullableField
        ));
    }

    #[test]
    fn pg12_with_different_column_validated_err() {
        // Validated CHECK exists for a different column
        let sql = r#"
ALTER TABLE foo ADD CONSTRAINT baz_not_null CHECK (baz IS NOT NULL) NOT VALID;
ALTER TABLE foo VALIDATE CONSTRAINT baz_not_null;
ALTER TABLE foo ALTER COLUMN bar SET NOT NULL;
        "#;
        assert_snapshot!(lint_errors_with(
            sql,
            LinterSettings {
                pg_version: "12".parse().expect("Invalid PostgreSQL version"),
                ..Default::default()
            },
            Rule::AddingNotNullableField
        ));
    }

    #[test]
    fn table_created_in_transaction_ok() {
        let sql = r#"
begin;
create table t (id bigint);
alter table t alter column id set not null;
commit;
        "#;
        lint_ok(sql, Rule::AddingNotNullableField);
    }

    #[test]
    fn pg16_cross_migration_validate_unrelated_constraint_err() {
        let sql = r#"
alter table t validate constraint some_fk;
alter table t alter column x set not null;
        "#;
        assert_snapshot!(lint_errors_with(
            sql,
            LinterSettings {
                pg_version: "16".parse().expect("Invalid PostgreSQL version"),
                ..Default::default()
            },
            Rule::AddingNotNullableField
        ));
    }

    #[test]
    fn pg16_validated_check_in_other_schema_err() {
        let sql = r#"
alter table a.users add constraint email_not_null check (email is not null) not valid;
alter table a.users validate constraint email_not_null;
alter table b.users alter column email set not null;
        "#;
        assert_snapshot!(lint_errors_with(
            sql,
            LinterSettings {
                pg_version: "16".parse().expect("Invalid PostgreSQL version"),
                ..Default::default()
            },
            Rule::AddingNotNullableField
        ));
    }

    #[test]
    fn pg16_validated_parenthesized_check_ok() {
        let sql = r#"
alter table t add constraint chk check ((email is not null)) not valid;
alter table t validate constraint chk;
alter table t alter column email set not null;
        "#;
        lint_ok_with(
            sql,
            LinterSettings {
                pg_version: "16".parse().expect("Invalid PostgreSQL version"),
                ..Default::default()
            },
            Rule::AddingNotNullableField,
        );
    }
}
