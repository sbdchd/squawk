use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_drop_default(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::AlterDomain(domain) = &stmt {
            if let Some(ast::AlterDomainAction::DropDefault(node)) = domain.action() {
                ctx.report(Violation::for_node(
                    Rule::BanDropDefault,
                    "Dropping a column default may break existing clients.".into(),
                    node.syntax(),
                ));
            }
        }
        if let ast::Stmt::AlterView(view) = &stmt {
            if let Some(ast::AlterViewAction::AlterViewColumn(column)) = view.action() {
                if let Some(ast::AlterViewColumnAction::DropDefault(node)) =
                    column.alter_view_column_action()
                {
                    ctx.report(Violation::for_node(
                        Rule::BanDropDefault,
                        "Dropping a column default may break existing clients.".into(),
                        node.syntax(),
                    ));
                }
            }
        }
        let actions = match stmt {
            ast::Stmt::AlterTable(table) => table.actions(),
            ast::Stmt::AlterForeignTable(table) => table.actions(),
            _ => continue,
        };
        for action in actions {
            if let ast::AlterTableAction::AlterColumn(alter_column) = action {
                if let Some(ast::AlterColumnOption::DropDefault(drop_default)) =
                    alter_column.option()
                {
                    ctx.report(Violation::for_node(
                        Rule::BanDropDefault,
                        "Dropping a column default may break existing clients.".into(),
                        drop_default.syntax(),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::{
        Rule,
        test_utils::{lint_errors, lint_ok},
    };

    #[test]
    fn err() {
        let sql = r#"
ALTER TABLE tbl ALTER COLUMN c DROP DEFAULT;
ALTER TABLE IF EXISTS tbl ALTER COLUMN c DROP DEFAULT;
ALTER TABLE tbl ALTER COLUMN c DROP DEFAULT, ALTER COLUMN d DROP DEFAULT;
"#;
        assert_snapshot!(lint_errors(sql, Rule::BanDropDefault));
    }

    #[test]
    fn ok() {
        lint_ok(
            "ALTER TABLE tbl ALTER COLUMN c SET DEFAULT 1; ALTER TABLE tbl ALTER COLUMN c DROP NOT NULL; DROP DOMAIN d; DROP INDEX i;",
            Rule::BanDropDefault,
        );
    }
}
