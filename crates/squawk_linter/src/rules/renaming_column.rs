use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn renaming_column(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for stmt in file.stmts() {
        if let ast::Stmt::AlterTable(alter_table) = stmt {
            for action in alter_table.actions() {
                if let ast::AlterTableAction::RenameColumn(rename_column) = action {
                    ctx.report(Violation::for_node(
                        Rule::RenamingColumn,
                        "Renaming a column may break existing clients.".into(),
                        rename_column.syntax(),
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
ALTER TABLE "table_name" RENAME COLUMN "column_name" TO "new_column_name";
        "#;
        let errors = lint(sql, Rule::RenamingColumn);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }
}
