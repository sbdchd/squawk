use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_drop_generated_expression(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::AlterTable(table) = stmt {
            for action in table.actions() {
                if let ast::AlterTableAction::AlterColumn(column) = action
                    && let Some(ast::AlterColumnOption::DropExpression(node)) = column.option()
                {
                    ctx.report(Violation::for_node(
                        Rule::BanDropGeneratedExpression,
                        "Dropping a generated expression changes the values returned to existing clients."
                            .into(),
                        node.syntax(),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        Rule,
        test_utils::{lint_errors, lint_ok},
    };

    #[test]
    fn drop_expression() {
        let errors = lint_errors(
            "ALTER TABLE t ALTER COLUMN c DROP EXPRESSION;",
            Rule::BanDropGeneratedExpression,
        );
        assert!(errors.contains("ban-drop-generated-expression"), "{errors}");
    }

    #[test]
    fn other_generated_operations() {
        lint_ok(
            "ALTER TABLE t ALTER COLUMN c SET EXPRESSION AS (a + b);",
            Rule::BanDropGeneratedExpression,
        );
    }
}
