use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_drop_constraint(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::AlterTable(table) = stmt {
            for action in table.actions() {
                if let ast::AlterTableAction::DropConstraint(node) = action {
                    ctx.report(Violation::for_node(Rule::BanDropConstraint, "Dropping a constraint may remove a guarantee that existing clients assume.".into(), node.syntax()));
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
        let sql = "ALTER TABLE t DROP CONSTRAINT IF EXISTS c;";
        assert_snapshot!(lint_errors(sql, Rule::BanDropConstraint));
    }
    #[test]
    fn ok() {
        lint_ok(
            "ALTER TABLE t ADD CONSTRAINT c CHECK (id > 0);",
            Rule::BanDropConstraint,
        );
    }
}
