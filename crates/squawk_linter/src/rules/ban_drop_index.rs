use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_drop_index(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::DropIndex(node) = stmt {
            ctx.report(Violation::for_node(
                Rule::BanDropIndex,
                "Dropping an index may remove a guarantee or change query plans for existing clients.".into(),
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
        let sql = "DROP INDEX CONCURRENTLY IF EXISTS i; DROP INDEX i;";
        let errors = lint_errors(sql, Rule::BanDropIndex);
        assert_eq!(errors.matches("warning[ban-drop-index]").count(), 2);
        assert_snapshot!(errors);
    }
    #[test]
    fn ok() {
        lint_ok("CREATE INDEX i ON t (id);", Rule::BanDropIndex);
    }
}
