use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_set_default(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::AlterTable(table) = stmt {
            for action in table.actions() {
                if let ast::AlterTableAction::AlterColumn(column) = action {
                    if let Some(ast::AlterColumnOption::SetDefault(node)) = column.option() {
                        ctx.report(Violation::for_node(Rule::BanSetDefault, "Setting a column default may silently change values written by existing clients.".into(), node.syntax()));
                    }
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
        let sql = "ALTER TABLE t ALTER COLUMN c SET DEFAULT 1;";
        assert_snapshot!(lint_errors(sql, Rule::BanSetDefault));
    }
    #[test]
    fn ok() {
        lint_ok(
            "ALTER TABLE t ALTER COLUMN c DROP DEFAULT;",
            Rule::BanSetDefault,
        );
    }
}
