use squawk_syntax::{
    ast::{self, AstNode, HasModuleItem},
    Parse, SourceFile,
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_drop_table(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for item in file.items() {
        if let ast::Item::DropTable(drop_table) = item {
            ctx.report(Violation::new(
                Rule::BanDropTable,
                "Dropping a table may break existing clients.".into(),
                drop_table.syntax().text_range(),
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
    fn err() {
        let sql = r#"
DROP TABLE "table_name";
DROP TABLE IF EXISTS "table_name";
DROP TABLE IF EXISTS "table_name"
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanDropTable]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }
}
