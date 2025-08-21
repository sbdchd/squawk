use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_drop_not_null(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for stmt in file.stmts() {
        if let ast::Stmt::AlterTable(alter_table) = stmt {
            for action in alter_table.actions() {
                if let ast::AlterTableAction::AlterColumn(alter_column) = action {
                    if let Some(ast::AlterColumnOption::DropNotNull(drop_not_null)) =
                        alter_column.option()
                    {
                        ctx.report(Violation::for_node(
                            Rule::BanDropNotNull,
                            "Dropping a `NOT NULL` constraint may break existing clients.".into(),
                            drop_not_null.syntax(),
                        ));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::Rule;
    use crate::test_utils::lint;

    #[test]
    fn err() {
        let sql = r#"
ALTER TABLE "bar_tbl" ALTER COLUMN "foo_col" DROP NOT NULL;
        "#;
        let errors = lint(sql, Rule::BanDropNotNull);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }
}
