use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn changing_column_type(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for stmt in file.stmts() {
        let actions = match stmt {
            ast::Stmt::AlterTable(table) => table.actions(),
            ast::Stmt::AlterForeignTable(table) => table.actions(),
            ast::Stmt::AlterType(ty) => {
                if let Some(ast::AlterTypeAction::AlterTypeAttributeActionList(list)) = ty.action()
                {
                    for action in list.actions() {
                        if let ast::AlterTypeAttributeAction::AlterAttribute(node) = action {
                            ctx.report(Violation::for_node(
                                Rule::ChangingColumnType,
                                "changing an attribute type may break existing clients".into(),
                                node.syntax(),
                            ));
                        }
                    }
                }
                continue;
            }
            _ => continue,
        };
        for action in actions {
            if let ast::AlterTableAction::AlterColumn(alter_column) = action
                && let Some(ast::AlterColumnOption::SetType(set_type)) = alter_column.option()
            {
                ctx.report(Violation::for_node(
                        Rule::ChangingColumnType,
                        "changing a column type rewrites the table under an `access exclusive` lock".into(),
                        set_type.syntax(),
                    )
                    .note("the lock blocks reads and writes while the table is rewritten, and the new type may break other clients reading from the table"));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::Rule;
    use crate::test_utils::lint_errors;

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
        assert_snapshot!(lint_errors(sql, Rule::ChangingColumnType));
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
        assert_snapshot!(lint_errors(sql, Rule::ChangingColumnType));
    }

    #[test]
    fn other_column_types() {
        let sql = "ALTER TYPE composite ALTER ATTRIBUTE a TYPE text; ALTER FOREIGN TABLE ft ALTER COLUMN a TYPE text;";
        let errors = lint_errors(sql, Rule::ChangingColumnType);
        assert_eq!(errors.matches("warning[changing-column-type]").count(), 2);
    }
}
