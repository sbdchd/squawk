use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_replace_view_function(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        match stmt {
            ast::Stmt::CreateView(node) if node.or_replace().is_some() => {
                ctx.report(Violation::for_node(Rule::BanReplaceViewFunction, "Replacing a view, function, or procedure may silently change behaviour for existing clients.".into(), node.syntax()));
            }
            ast::Stmt::CreateFunction(node) if node.or_replace().is_some() => {
                ctx.report(Violation::for_node(Rule::BanReplaceViewFunction, "Replacing a view, function, or procedure may silently change behaviour for existing clients.".into(), node.syntax()));
            }
            ast::Stmt::CreateProcedure(node) if node.or_replace().is_some() => {
                ctx.report(Violation::for_node(Rule::BanReplaceViewFunction, "Replacing a view, function, or procedure may silently change behaviour for existing clients.".into(), node.syntax()));
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
        let sql = "CREATE OR REPLACE VIEW v AS SELECT 1 AS id; CREATE OR REPLACE FUNCTION f() RETURNS int LANGUAGE sql AS $$ SELECT 1 $$; CREATE OR REPLACE PROCEDURE p() LANGUAGE sql AS $$ SELECT 1 $$;";
        assert_snapshot!(lint_errors(sql, Rule::BanReplaceViewFunction));
    }
    #[test]
    fn ok() {
        lint_ok("CREATE VIEW v AS SELECT 1;", Rule::BanReplaceViewFunction);
    }
}
