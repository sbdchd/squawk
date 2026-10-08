use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_drop_policy(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        match stmt {
            ast::Stmt::DropPolicy(node) => {
                ctx.report(Violation::for_node(
                    Rule::BanDropPolicy,
                    "Dropping a policy or rule may silently change behaviour for existing clients."
                        .into(),
                    node.syntax(),
                ));
            }
            ast::Stmt::DropRule(node) => {
                ctx.report(Violation::for_node(
                    Rule::BanDropPolicy,
                    "Dropping a policy or rule may silently change behaviour for existing clients."
                        .into(),
                    node.syntax(),
                ));
            }
            _ => (),
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
        let sql = "DROP POLICY IF EXISTS p ON t; DROP RULE IF EXISTS r ON t;";
        assert_snapshot!(lint_errors(sql, Rule::BanDropPolicy));
    }
    #[test]
    fn ok() {
        lint_ok("CREATE POLICY p ON t USING (true);", Rule::BanDropPolicy);
        lint_ok("ALTER POLICY p ON t USING (true);", Rule::BanDropPolicy);
    }
}
