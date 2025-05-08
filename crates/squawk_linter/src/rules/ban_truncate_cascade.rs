use squawk_syntax::{
    ast::{self, HasModuleItem},
    Parse, SourceFile,
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_truncate_cascade(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for item in file.items() {
        match item {
            ast::Item::Truncate(truncate) => {
                if let Some(cascade) = truncate.cascade_token() {
                    // TODO: if we had knowledge about the entire schema, we
                    // could be more precise here and actually navigate the
                    // foreign keys.
                    ctx.report(Violation::new(
                        Rule::BanTruncateCascade,
                        format!("Using `CASCADE` will recursively truncate any tables that foreign key to the referenced tables! So if you had foreign keys setup as `a <- b <- c` and truncated `a`, then `b` & `c` would also be truncated!"),
                        cascade.text_range(),
                        "Remove the `CASCADE` and specify exactly which tables you want to truncate.".to_string(),
                    ));
                }
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::{Linter, Rule};
    use squawk_syntax::SourceFile;

    #[test]
    fn err() {
        let sql = r#"
        truncate a, b, c cascade;
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanTruncateCascade]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn ok() {
        let sql = r#"
        truncate a, b, c;
        truncate a;
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanTruncateCascade]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }
}
