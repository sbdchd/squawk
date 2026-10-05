use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_drop_domain(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::DropDomain(node) = stmt {
            ctx.report(Violation::for_node(
                Rule::BanDropDomain,
                "Dropping a domain may break existing clients.".into(),
                node.syntax(),
            ));
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
        let sql = "DROP DOMAIN IF EXISTS d CASCADE;";
        assert_snapshot!(lint_errors(sql, Rule::BanDropDomain));
    }
    #[test]
    fn ok() {
        lint_ok("CREATE DOMAIN d AS integer;", Rule::BanDropDomain);
    }
}
