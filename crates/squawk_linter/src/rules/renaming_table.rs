use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn renaming_table(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for stmt in file.stmts() {
        if let ast::Stmt::AlterTable(alter_table) = stmt {
            for action in alter_table.actions() {
                if let ast::AlterTableAction::RenameTable(rename_table) = action {
                    ctx.report(Violation::new(
                        Rule::RenamingTable,
                        "Renaming a table may break existing clients.".into(),
                        rename_table.syntax().text_range(),
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
ALTER TABLE "table_name" RENAME TO "new_table_name";
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::RenamingTable]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }
}
