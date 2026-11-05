use crate::{Linter, Rule, Violation};
use squawk_syntax::ast::{self, AstNode};

pub(crate) fn ban_drop_sequence(ctx: &mut Linter, file: &ast::SourceFile) {
    for stmt in file.stmts() {
        if let ast::Stmt::DropSequence(node) = stmt {
            ctx.report(Violation::for_node(
                Rule::BanDropSequence,
                "dropping a sequence may break existing clients".into(),
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
        let sql = "DROP SEQUENCE IF EXISTS s CASCADE;";
        assert_snapshot!(lint_errors(sql, Rule::BanDropSequence));
    }
    #[test]
    fn ok() {
        lint_ok("CREATE SEQUENCE s;", Rule::BanDropSequence);
    }
}
