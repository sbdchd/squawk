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
                    ctx.report(Violation::new(
                        Rule::RenamingColumn,
                        "Renaming a column may break existing clients.".into(),
                        rename_column.syntax().text_range(),
                        None,
                    ));
                }
            }
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
ALTER TABLE "table_name" RENAME COLUMN "column_name" TO "new_column_name";
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::RenamingColumn]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }
}
