use squawk_syntax::ast::AstNode;
use squawk_syntax::{Parse, SourceFile, ast};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_truncate_cascade(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for stmt in file.stmts() {
        if let ast::Stmt::Truncate(truncate) = stmt
            && let Some(ast::DropBehavior::Cascade(cascade)) = truncate.drop_behavior()
        {
            // TODO: if we had knowledge about the entire schema, we
            // could be more precise here and actually navigate the
            // foreign keys.
            ctx.report(Violation::for_range(
                    Rule::BanTruncateCascade,
                    "`cascade` recursively truncates every table with a foreign key to the truncated tables".into(),
                    cascade.syntax().text_range(),
                )
                .note("with foreign keys set up as `a <- b <- c`, truncating `a` also truncates `b` and `c`")
                .help("remove the `cascade` and list exactly which tables to truncate"));
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::Rule;
    use crate::test_utils::{lint_errors, lint_ok};

    #[test]
    fn err() {
        let sql = r#"
        truncate a, b, c cascade;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::BanTruncateCascade));
    }

    #[test]
    fn ok() {
        let sql = r#"
        truncate a, b, c;
        truncate a;
        "#;
        lint_ok(sql, Rule::BanTruncateCascade);
    }
}
