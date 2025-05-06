use squawk_syntax::{
    ast::{self, AstNode, HasModuleItem},
    Parse, SourceFile,
};

use crate::{Linter, Rule, Violation};

pub(crate) fn changing_column_type(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for item in file.items() {
        if let ast::Item::AlterTable(alter_table) = item {
            for action in alter_table.actions() {
                if let ast::AlterTableAction::AlterColumn(alter_column) = action {
                    if let Some(ast::AlterColumnOption::SetType(set_type)) = alter_column.option() {
                        ctx.report(Violation::new(
                            Rule::ChangingColumnType,
                            "Changing a column type requires an `ACCESS EXCLUSIVE` lock on the table which blocks reads and writes while the table is rewritten. Changing the type of the column may also break other clients reading from the table.".into(),
                            set_type.syntax().text_range(),
                            None,
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

    use crate::{Linter, Rule};

    #[test]
    fn err() {
        let sql = r#"
BEGIN;
--
-- Alter field edits on recipe
--
ALTER TABLE "core_recipe" ALTER COLUMN "edits" TYPE text USING "edits"::text;
COMMIT;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::ChangingColumnType]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn another_err() {
        let sql = r#"
BEGIN;
--
-- Alter field foo on recipe
--
ALTER TABLE "core_recipe" ALTER COLUMN "foo" TYPE varchar(255) USING "foo"::varchar(255);
ALTER TABLE "core_recipe" ALTER COLUMN "foo" TYPE text USING "foo"::text;
COMMIT;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::ChangingColumnType]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }
}
