use std::collections::{HashMap, HashSet};

use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
    identifier::Identifier,
};

use crate::{Linter, Rule, Version, Violation};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TableColumn {
    table: Identifier,
    column: Identifier,
}

fn is_not_null_check(expr: &ast::Expr) -> Option<Identifier> {
    let ast::Expr::BinExpr(bin_expr) = expr else {
        return None;
    };
    let ast::BinOp::IsNot(_) = bin_expr.op()? else {
        return None;
    };

    let ast::Expr::Literal(lit) = bin_expr.rhs()? else {
        return None;
    };
    if !matches!(lit.kind(), Some(ast::LitKind::Null(_))) {
        return None;
    }

    match bin_expr.lhs()? {
        ast::Expr::NameRef(name_ref) => Some(Identifier::new(&name_ref.text())),
        _ => None,
    }
}

fn get_table_name(alter_table: &ast::AlterTable) -> Option<Identifier> {
    alter_table
        .relation_name()?
        .path()?
        .segment()?
        .name_ref()
        .map(|x| Identifier::new(&x.text()))
}

pub(crate) fn adding_not_null_field(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();

    let is_pg12_plus = ctx.settings.pg_version >= Version::new(12, None, None);

    let mut not_null_constraints: HashMap<Identifier, TableColumn> = HashMap::new();
    let mut validated_not_null_columns: HashSet<TableColumn> = HashSet::new();

    for stmt in file.stmts() {
        if let ast::Stmt::AlterTable(alter_table) = stmt {
            let Some(table) = get_table_name(&alter_table) else {
                continue;
            };

            for action in alter_table.actions() {
                match action {
                    // Step 1: Add constraint
                    ast::AlterTableAction::AddConstraint(add_constraint)
                        if is_pg12_plus && add_constraint.not_valid().is_some() =>
                    {
                        if let Some(ast::Constraint::CheckConstraint(check)) =
                            add_constraint.constraint()
                            && let Some(constraint_name) =
                                check.constraint_name().and_then(|c| c.name())
                            && let Some(expr) = check.expr()
                            && let Some(column) = is_not_null_check(&expr)
                        {
                            not_null_constraints.insert(
                                Identifier::new(&constraint_name.text()),
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
                            .name_ref()
                            .map(|x| Identifier::new(&x.text()))
                            && let Some(table_column) = not_null_constraints.get(&constraint_name)
                            && table_column.table == table
                        {
                            validated_not_null_columns.insert(table_column.clone());
                        }
                    }
                    // Step 3: Check that we're altering a validated constraint
                    ast::AlterTableAction::AlterColumn(alter_column) => {
                        let Some(ast::AlterColumnOption::SetNotNull(option)) =
                            alter_column.option()
                        else {
                            continue;
                        };

                        if is_pg12_plus
                            && let Some(column) =
                                alter_column.name_ref().map(|x| Identifier::new(&x.text()))
                        {
                            let table_column = TableColumn {
                                table: table.clone(),
                                column,
                            };
                            if validated_not_null_columns.contains(&table_column) {
                                continue;
                            }
                        }

                        ctx.report(
                            Violation::for_node(
                                Rule::AddingNotNullableField,
                                "Setting a column `NOT NULL` blocks reads while the table is scanned."
                                    .into(),
                                option.syntax(),
                            )
                            .help("Make the field nullable and use a `CHECK` constraint instead."),
                        );
                    }
                    _ => ()
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
}
