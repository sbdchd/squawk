use squawk_syntax::ast::{self, AstNode};

use crate::{Linter, Rule, Violation};

/// Brad's Rule aka ban dropping database statements.
pub(crate) fn ban_drop_database(ctx: &mut Linter, file: &ast::SourceFile) {
    for stmt in file.stmts() {
        if let ast::Stmt::DropDatabase(drop_database) = stmt {
            ctx.report(Violation::for_node(
                Rule::BanDropDatabase,
                "dropping a database may break existing clients".into(),
                drop_database.syntax(),
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
    fn ban_drop_database() {
        let sql = r#"
        DROP DATABASE "table_name";
        DROP DATABASE IF EXISTS "table_name";
        DROP DATABASE IF EXISTS "table_name"
                "#;
        assert_snapshot!(lint_errors(sql, Rule::BanDropDatabase));
    }
}
