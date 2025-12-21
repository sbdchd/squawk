use squawk_syntax::{Parse, SourceFile, ast};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_truncate_cascade(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for stmt in file.stmts() {
        if let ast::Stmt::Truncate(truncate) = stmt {
            if let Some(cascade) = truncate.cascade_token() {
                // TODO: if we had knowledge about the entire schema, we
                // could be more precise here and actually navigate the
                // foreign keys.
                ctx.report(Violation::for_range(
                    Rule::BanTruncateCascade,
                    "Using `CASCADE` will recursively truncate any tables that foreign key to the referenced tables! So if you had foreign keys setup as `a <- b <- c` and truncated `a`, then `b` & `c` would also be truncated!".to_string(),
                    cascade.text_range(),
                ).help("Remove the `CASCADE` and specify exactly which tables you want to truncate."));
            }
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
