use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_alter_generated_expression(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::AlterTable(table) = stmt {
            for action in table.actions() {
                match action {
                    ast::AlterTableAction::AlterColumn(column) => {
                        if let Some(ast::AlterColumnOption::SetExpression(node)) = column.option() {
                            ctx.report(Violation::for_node(Rule::BanAlterGeneratedExpression, "Changing a generated column expression may change values read by existing clients.".into(), node.syntax()));
                        }
                    }
                    ast::AlterTableAction::AddColumn(column) => {
                        for constraint in column.constraints() {
                            if let ast::Constraint::GeneratedConstraint(node) = constraint {
                                ctx.report(Violation::for_node(Rule::BanAlterGeneratedExpression, "Adding a generated column may break inserts from existing clients.".into(), node.syntax()));
                            }
                        }
                    }
                    _ => (),
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
    use insta::assert_snapshot;
    #[test]
    fn err() {
        let sql = "ALTER TABLE t ALTER COLUMN c SET EXPRESSION AS (id + 2); ALTER TABLE t ADD COLUMN c int GENERATED ALWAYS AS (id + 1) STORED;";
        assert_snapshot!(lint_errors(sql, Rule::BanAlterGeneratedExpression));
    }
    #[test]
    fn ok() {
        lint_ok(
            "ALTER TABLE t ADD COLUMN c int; ALTER TABLE t ALTER COLUMN c DROP EXPRESSION;",
            Rule::BanAlterGeneratedExpression,
        );
    }

    #[test]
    fn generated_expression_variant() {
        let sql = "ALTER TABLE t ALTER COLUMN c SET EXPRESSION AS (id + 1);";
        assert_eq!(
            lint_errors(sql, Rule::BanAlterGeneratedExpression)
                .matches("warning[ban-alter-generated-expression]")
                .count(),
            1
        );
        lint_ok(
            "ALTER TABLE t ALTER COLUMN c DROP EXPRESSION;",
            Rule::BanAlterGeneratedExpression,
        );
    }
}
