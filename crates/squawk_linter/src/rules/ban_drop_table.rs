use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_drop_table(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for stmt in file.stmts() {
        if let ast::Stmt::DropTable(drop_table) = stmt {
            ctx.report(Violation::for_node(
                Rule::BanDropTable,
                "Dropping a table may break existing clients.".into(),
                drop_table.syntax(),
            ));
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::Rule;
    use crate::test_utils::lint_errors;

    #[test]
    fn err() {
        let sql = r#"
DROP TABLE "table_name";
DROP TABLE IF EXISTS "table_name";
DROP TABLE IF EXISTS "table_name"
        "#;
        assert_snapshot!(lint_errors(sql, Rule::BanDropTable));
    }
}
