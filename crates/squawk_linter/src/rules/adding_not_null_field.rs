use squawk_syntax::{
    ast::{self, AstNode, HasModuleItem},
    Parse, SourceFile,
};

use crate::{ErrorCode, Linter, Version, Violation};

pub(crate) fn adding_not_null_field(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    if ctx.settings.pg_version >= Version::new(11, 0, 0) {
        return;
    }
    let file = parse.tree();
    for item in file.items() {
        if let ast::Item::AlterTable(alter_table) = item {
            for action in alter_table.actions() {
                if let ast::AlterTableAction::AlterColumn(alter_column) = action {
                    let Some(option) = alter_column.option() else {
                        continue;
                    };

                    if matches!(option, ast::AlterColumnOption::SetNotNull(_)) {
                        ctx.report(Violation::new(
                            ErrorCode::AddingNotNullableField,
                            "Setting a column NOT NULL blocks reads while the table is scanned."
                                .into(),
                            option.syntax().text_range(),
                            vec!["Use a check constraint instead.".into()],
                        ));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::{Linter, Rule, Version};

    #[test]
    fn set_not_null() {
        let sql = r#"
ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET NOT NULL;
        "#;
        let file = syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::AddingNotNullField]);
        linter.settings.pg_version = Version::new(10, 0, 0);
        let errors = linter.lint(file, sql);
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
        let file = syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::AddingNotNullField]);
        let errors = linter.lint(file, sql);
        assert!(errors.is_empty());
    }

    #[test]
    fn adding_field_that_is_not_nullable_without_default() {
        let sql = r#"
-- This won't work if the table is populated, but that error is caught by adding-required-field.
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer NOT NULL;
        "#;
        let file = syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::AddingNotNullField]);
        let errors = linter.lint(file, sql);
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
        let file = syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::AddingNotNullField]);
        let errors = linter.lint(file, sql);
        assert!(errors.is_empty());
    }
}
