use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_drop_type(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::DropOperator(node) = stmt {
            ctx.report(Violation::for_node(Rule::BanDropType, "Dropping an operator may break existing clients.".into(), node.syntax()));
        } else if let ast::Stmt::DropOperatorClass(node) = stmt {
            ctx.report(Violation::for_node(Rule::BanDropType, "Dropping an operator class may break existing clients.".into(), node.syntax()));
        } else if let ast::Stmt::DropOperatorFamily(node) = stmt {
            ctx.report(Violation::for_node(Rule::BanDropType, "Dropping an operator family may break existing clients.".into(), node.syntax()));
        } else if let ast::Stmt::DropCast(node) = stmt {
            ctx.report(Violation::for_node(
                Rule::BanDropType,
                "Dropping a cast may break existing clients.".into(),
                node.syntax(),
            ));
        } else if let ast::Stmt::DropType(node) = stmt {
            ctx.report(Violation::for_node(
                Rule::BanDropType,
                "Dropping a type may break existing clients.".into(),
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
DROP TYPE t;
DROP TYPE IF EXISTS t CASCADE;
"#;
        assert_snapshot!(lint_errors(sql, Rule::BanDropType));
    }

    #[test]
    fn ok() {
        lint_ok(
            "CREATE TYPE t AS ENUM ('a'); DROP INDEX i;",
            Rule::BanDropType,
        );
    }
}
