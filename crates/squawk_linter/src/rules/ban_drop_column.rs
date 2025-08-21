use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_drop_column(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for stmt in file.stmts() {
        if let ast::Stmt::AlterTable(alter_table) = stmt {
            for action in alter_table.actions() {
                if let ast::AlterTableAction::DropColumn(drop_column) = action {
                    ctx.report(Violation::for_node(
                        Rule::BanDropColumn,
                        "Dropping a column may break existing clients.".into(),
                        drop_column.syntax(),
                    ));
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
ALTER TABLE "bar_tbl" DROP COLUMN "foo_col" CASCADE;
        "#;
        let errors = lint(sql, Rule::BanDropColumn);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }
}
