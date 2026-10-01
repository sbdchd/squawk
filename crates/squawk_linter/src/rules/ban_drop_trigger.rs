use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_drop_trigger(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::DropTrigger(node) = stmt {
            ctx.report(Violation::for_node(
                Rule::BanDropTrigger,
                "Dropping a trigger may silently change behaviour for existing clients.".into(),
                node.syntax(),
            ));
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
DROP TRIGGER trg ON tbl;
DROP TRIGGER IF EXISTS trg ON tbl CASCADE;
"#;
        assert_snapshot!(lint_errors(sql, Rule::BanDropTrigger));
    }

    #[test]
    fn ok() {
        lint_ok(
            "DROP INDEX i; CREATE VIEW v AS SELECT 1;",
            Rule::BanDropTrigger,
        );
    }
}
