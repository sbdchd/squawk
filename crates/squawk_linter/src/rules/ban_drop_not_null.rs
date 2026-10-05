use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_drop_not_null(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for stmt in file.stmts() {
        if let ast::Stmt::AlterDomain(domain) = &stmt {
            if let Some(ast::AlterDomainAction::DropNotNull(node)) = domain.action() {
                ctx.report(Violation::for_node(
                    Rule::BanDropNotNull,
                    "Dropping a `NOT NULL` constraint may break existing clients.".into(),
                    node.syntax(),
                ));
            }
        }
        let actions = match stmt {
            ast::Stmt::AlterTable(table) => table.actions(),
            ast::Stmt::AlterForeignTable(table) => table.actions(),
            _ => continue,
        };
        for action in actions {
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

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::Rule;
    use crate::test_utils::lint_errors;

    #[test]
    fn err() {
        let sql = r#"
ALTER TABLE "bar_tbl" ALTER COLUMN "foo_col" DROP NOT NULL;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::BanDropNotNull));
    }
}
