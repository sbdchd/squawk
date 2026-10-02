use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_drop_function(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        match stmt {
            ast::Stmt::DropFunction(node) => ctx.report(Violation::for_node(
                Rule::BanDropFunction,
                "Dropping a function may break existing clients.".into(),
                node.syntax(),
            )),
            ast::Stmt::DropProcedure(node) => ctx.report(Violation::for_node(
                Rule::BanDropFunction,
                "Dropping a function may break existing clients.".into(),
                node.syntax(),
            )),
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::{
        Rule,
        test_utils::{lint_errors, lint_ok},
    };

    #[test]
    fn err() {
        let sql = r#"
DROP FUNCTION f(int);
DROP FUNCTION IF EXISTS f(int) CASCADE;
DROP PROCEDURE p(int);
DROP PROCEDURE IF EXISTS p(int) CASCADE;
"#;
        assert_snapshot!(lint_errors(sql, Rule::BanDropFunction));
    }

    #[test]
    fn ok() {
        lint_ok(
            "DROP INDEX i; CREATE VIEW v AS SELECT 1;",
            Rule::BanDropFunction,
        );
    }
}
