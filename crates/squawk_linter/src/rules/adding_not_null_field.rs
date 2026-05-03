use rustc_hash::{FxHashMap, FxHashSet};

use squawk_syntax::{
    Parse, SourceFile, SyntaxNode,
    ast::{self, AstNode},
    identifier::Identifier,
};

use crate::{Linter, Rule, Version, Violation};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TableColumn {
    table: Identifier,
    column: Identifier,
}

#[derive(Default)]
struct NotNullValidation {
    not_null_constraints: FxHashMap<Identifier, TableColumn>,
    validated_columns: FxHashSet<TableColumn>,
    external_validates: FxHashSet<(Identifier, Identifier)>,
    dropped_constraints: FxHashSet<(Identifier, Identifier)>,
}

impl NotNullValidation {
    fn record_not_null_constraint(&mut self, name: Identifier, table_column: TableColumn) {
        self.not_null_constraints.insert(name, table_column);
    }

    fn record_validate(&mut self, table: Identifier, constraint: Identifier) {
        if let Some(table_column) = self.not_null_constraints.get(&constraint)
            && table_column.table == table
        {
            self.validated_columns.insert(table_column.clone());
        } else {
            self.external_validates.insert((table, constraint));
        }
    }

    fn record_drop(&mut self, table: Identifier, constraint: Identifier) {
        self.dropped_constraints.insert((table, constraint));
    }

    fn is_column_validated(&self, table_column: &TableColumn) -> bool {
        self.validated_columns.contains(table_column)
    }

    fn has_external_validate_for(&self, table: &Identifier) -> bool {
        self.external_validates.iter().any(|(t, _)| t == table)
    }

    // External validate+drop pairs suppress SET NOT NULL warnings. When the
    // constraint name follows the documented `<column>_not_null` /
    // `<table>_<column>_not_null` convention we infer the exact column;
    // otherwise we fall back to a per-table count and pair by source order.
    fn resolved_pairs(&self) -> ResolvedPairs {
        let mut pairs = ResolvedPairs::default();
        for (table, constraint) in &self.external_validates {
            if !self
                .dropped_constraints
                .contains(&(table.clone(), constraint.clone()))
            {
                continue;
            }
            match infer_column_from_constraint_name(table, constraint) {
                Some(column) => {
                    pairs.precise.insert(TableColumn {
                        table: table.clone(),
                        column,
                    });
                }
                None => {
                    *pairs.generic_per_table.entry(table.clone()).or_default() += 1;
                }
            }
        }
        pairs
    }
}

#[derive(Default)]
struct ResolvedPairs {
    precise: FxHashSet<TableColumn>,
    generic_per_table: FxHashMap<Identifier, usize>,
}

// Infer the column covered by a NOT NULL helper constraint from its name.
// Recognized forms (case-folded by Identifier): `<column>_not_null` and
// `<table>_<column>_not_null`. Returns None for names that don't fit, in
// which case the caller falls back to a generic per-table count.
fn infer_column_from_constraint_name(
    table: &Identifier,
    constraint: &Identifier,
) -> Option<Identifier> {
    let stem = constraint.as_str().strip_suffix("_not_null")?;
    if stem.is_empty() {
        return None;
    }
    let table_prefix = format!("{}_", table.as_str());
    if let Some(column) = stem.strip_prefix(&table_prefix) {
        if column.is_empty() {
            return None;
        }
        return Some(Identifier::new(column));
    }
    // Bare `<stem>_not_null`: ambiguous when stem == table name (could be a
    // table-level helper rather than a column-level one), so fall back.
    if stem == table.as_str() {
        return None;
    }
    Some(Identifier::new(stem))
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

    let mut validation = NotNullValidation::default();
    let mut deferred_violations: Vec<(TableColumn, SyntaxNode)> = Vec::new();

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
                            validation.record_not_null_constraint(
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
                        {
                            validation.record_validate(table.clone(), constraint_name);
                        }
                    }
                    // Track DROP CONSTRAINT for cross-migration pairing
                    ast::AlterTableAction::DropConstraint(drop_constraint) if is_pg12_plus => {
                        if let Some(constraint_name) = drop_constraint
                            .name_ref()
                            .map(|x| Identifier::new(&x.text()))
                        {
                            validation.record_drop(table.clone(), constraint_name);
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
                            if validation.is_column_validated(&table_column) {
                                continue;
                            }
                            // Defer if there are external validates on this table —
                            // we need to see the full file before deciding.
                            if validation.has_external_validate_for(&table) {
                                deferred_violations.push((table_column, option.syntax().clone()));
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
                    _ => (),
                }
            }
        }
    }

    let resolved = validation.resolved_pairs();
    let mut generic_per_table = resolved.generic_per_table;
    for (table_column, node) in &deferred_violations {
        if resolved.precise.contains(table_column) {
            continue;
        }
        if let Some(count) = generic_per_table.get_mut(&table_column.table)
            && *count > 0
        {
            *count -= 1;
            continue;
        }
        ctx.report(
            Violation::for_node(
                Rule::AddingNotNullableField,
                "Setting a column `NOT NULL` blocks reads while the table is scanned.".into(),
                node,
            )
            .help("Make the field nullable and use a `CHECK` constraint instead."),
        );
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

    // Cross-migration pattern: ADD CONSTRAINT was in a previous migration file,
    // so only VALIDATE CONSTRAINT + SET NOT NULL + DROP CONSTRAINT appear in this file.
    // The validate+drop pairing signals it was a NOT NULL helper constraint.
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

    // Validating an unrelated constraint should NOT suppress SET NOT NULL warnings.
    #[test]
    fn pg12_cross_migration_unrelated_validate_err() {
        let sql = r#"
ALTER TABLE foo VALIDATE CONSTRAINT some_other_check;
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

    // When the constraint name follows the `<column>_not_null` convention, the
    // pair only suppresses the matching column — other SET NOT NULL statements
    // on the same table still warn regardless of source order.
    #[test]
    fn pg12_cross_migration_named_constraint_matches_column_only_err() {
        let sql = r#"
ALTER TABLE foo VALIDATE CONSTRAINT bar_not_null;
ALTER TABLE foo ALTER COLUMN baz SET NOT NULL;
ALTER TABLE foo ALTER COLUMN bar SET NOT NULL;
ALTER TABLE foo DROP CONSTRAINT bar_not_null;
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

    // `<table>_<column>_not_null` is also recognized by the inference.
    #[test]
    fn pg12_cross_migration_table_prefixed_constraint_ok() {
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

    // Validate without a corresponding DROP is not the helper pattern.
    #[test]
    fn pg12_cross_migration_validate_no_drop_err() {
        let sql = r#"
ALTER TABLE foo VALIDATE CONSTRAINT bar_not_null;
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
}
