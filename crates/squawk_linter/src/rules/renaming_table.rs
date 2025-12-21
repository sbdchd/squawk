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
                if let ast::AlterTableAction::RenameTo(rename_table) = action {
                    ctx.report(Violation::for_node(
                        Rule::RenamingTable,
                        "Renaming a table may break existing clients.".into(),
                        rename_table.syntax(),
                    ));
                }
            }
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
ALTER TABLE "table_name" RENAME TO "new_table_name";
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RenamingTable));
    }
}
