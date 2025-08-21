use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn adding_not_null_field(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for stmt in file.stmts() {
        if let ast::Stmt::AlterTable(alter_table) = stmt {
            for action in alter_table.actions() {
                if let ast::AlterTableAction::AlterColumn(alter_column) = action {
                    let Some(option) = alter_column.option() else {
                        continue;
                    };

                    if matches!(option, ast::AlterColumnOption::SetNotNull(_)) {
                        ctx.report(Violation::for_node(
                            Rule::AddingNotNullableField,
                            "Setting a column `NOT NULL` blocks reads while the table is scanned."
                                .into(),
                            option.syntax(),
                        ).help("Make the field nullable and use a `CHECK` constraint instead."));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::Rule;
    use crate::test_utils::lint;

    #[test]
    fn set_not_null() {
        let sql = r#"
ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET NOT NULL;
        "#;
        let errors = lint(sql, Rule::AddingNotNullableField);
        assert!(!errors.is_empty());
        assert_debug_snapshot!(errors);
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
        let errors = lint(sql, Rule::AddingNotNullableField);
        assert!(errors.is_empty());
    }

    #[test]
    fn adding_field_that_is_not_nullable_without_default() {
        let sql = r#"
-- This won't work if the table is populated, but that error is caught by adding-required-field.
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
        "#;
        let errors = lint(sql, Rule::AddingNotNullableField);
        assert!(errors.is_empty());
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
        let errors = lint(sql, Rule::AddingNotNullableField);
        assert!(errors.is_empty());
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
        let errors = lint(sql, Rule::AddingNotNullableField);
        assert!(!errors.is_empty());
        assert_debug_snapshot!(errors);
    }
}
