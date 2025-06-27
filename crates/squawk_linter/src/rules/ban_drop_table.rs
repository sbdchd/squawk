use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_drop_table(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for stmt in file.stmts() {
        if let ast::Stmt::DropTable(drop_table) = stmt {
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
