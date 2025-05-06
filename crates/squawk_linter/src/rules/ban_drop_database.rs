use squawk_syntax::{
    ast::{self, AstNode, HasModuleItem},
    Parse, SourceFile,
};

use crate::{Linter, Rule, Violation};

/// Brad's Rule aka ban dropping database statements.
pub(crate) fn ban_drop_database(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for item in file.items() {
        if let ast::Item::DropDatabase(drop_database) = item {
            ctx.report(Violation::new(
                Rule::BanDropDatabase,
                "Dropping a database may break existing clients.".into(),
                drop_database.syntax().text_range(),
                None,
            ));
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::{Linter, Rule};

    #[test]
    fn ban_drop_database() {
        let sql = r#"
        DROP DATABASE "table_name";
        DROP DATABASE IF EXISTS "table_name";
        DROP DATABASE IF EXISTS "table_name"
                "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanDropDatabase]);
        let errors = linter.lint(file, sql);
        assert!(!errors.is_empty());
        assert_debug_snapshot!(errors);
    }
}
