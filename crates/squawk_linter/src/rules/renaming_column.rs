use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn renaming_column(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for stmt in file.stmts() {
        match stmt {
            ast::Stmt::AlterType(ty) => {
                if let Some(ast::AlterTypeAction::RenameAttribute(node)) = ty.action() {
                    ctx.report(Violation::for_node(
                        Rule::RenamingColumn,
                        "Renaming an attribute may break existing clients.".into(),
                        node.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterTable(table) => {
                for action in table.actions() {
                    if let ast::AlterTableAction::RenameColumn(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::RenamingColumn,
                            "Renaming a column may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterForeignTable(table) => {
                for action in table.actions() {
                    if let ast::AlterTableAction::RenameColumn(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::RenamingColumn,
                            "Renaming a column may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            ast::Stmt::AlterView(view) => {
                if let Some(ast::AlterViewAction::RenameColumn(node)) = view.action() {
                    ctx.report(Violation::for_node(
                        Rule::RenamingColumn,
                        "Renaming a column may break existing clients.".into(),
                        node.syntax(),
                    ));
                }
            }
            ast::Stmt::AlterMaterializedView(view) => {
                for action in view.action() {
                    if let ast::AlterMaterializedViewAction::RenameColumn(node) = action {
                        ctx.report(Violation::for_node(
                            Rule::RenamingColumn,
                            "Renaming a column may break existing clients.".into(),
                            node.syntax(),
                        ));
                    }
                }
            }
            _ => (),
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
ALTER TABLE "table_name" RENAME COLUMN "column_name" TO "new_column_name";
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RenamingColumn));
    }
}
