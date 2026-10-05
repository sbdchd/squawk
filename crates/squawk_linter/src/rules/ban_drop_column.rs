use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_drop_column(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for stmt in file.stmts() {
        let actions: Vec<_> = match stmt {
            ast::Stmt::AlterTable(table) => table.actions().collect(),
            ast::Stmt::AlterForeignTable(table) => table.actions().collect(),
            ast::Stmt::AlterType(ty) => {
                if let Some(ast::AlterTypeAction::AlterTypeAttributeActionList(list)) = ty.action() {
                    for action in list.actions() {
                        if let ast::AlterTypeAttributeAction::DropAttribute(node) = action {
                            ctx.report(Violation::for_node(Rule::BanDropColumn, "Dropping an attribute may break existing clients.".into(), node.syntax()));
                        }
                    }
                }
                Vec::new()
            }
            _ => Vec::new(),
        };
        for action in actions {
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

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::Rule;
    use crate::test_utils::lint_errors;

    #[test]
    fn err() {
        let sql = r#"
ALTER TABLE "bar_tbl" DROP COLUMN "foo_col" CASCADE;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::BanDropColumn));
    }
}
