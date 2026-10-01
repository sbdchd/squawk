use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_drop_view(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        match stmt {
            ast::Stmt::DropView(node) => ctx.report(Violation::for_node(
                Rule::BanDropView,
                "Dropping a view may break existing clients.".into(),
                node.syntax(),
            )),
            ast::Stmt::DropMaterializedView(node) => ctx.report(Violation::for_node(
                Rule::BanDropView,
                "Dropping a view may break existing clients.".into(),
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
DROP VIEW v;
DROP VIEW IF EXISTS v CASCADE;
DROP MATERIALIZED VIEW mv;
DROP MATERIALIZED VIEW IF EXISTS mv CASCADE;
"#;
        assert_snapshot!(lint_errors(sql, Rule::BanDropView));
    }

    #[test]
    fn ok() {
        lint_ok(
            "CREATE VIEW v AS SELECT 1; DROP INDEX i;",
            Rule::BanDropView,
        );
    }
}
