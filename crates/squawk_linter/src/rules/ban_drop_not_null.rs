use squawk_syntax::{
    ast::{self, AstNode, HasModuleItem},
    Parse, SourceFile,
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_drop_not_null(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for item in file.items() {
        if let ast::Item::AlterTable(alter_table) = item {
            for action in alter_table.actions() {
                if let ast::AlterTableAction::AlterColumn(alter_column) = action {
                    if let Some(ast::AlterColumnOption::DropNotNull(drop_not_null)) =
                        alter_column.option()
                    {
                        ctx.report(Violation::new(
                            Rule::BanDropNotNull,
                            "Dropping a `NOT NULL` constraint may break existing clients.".into(),
                            drop_not_null.syntax().text_range(),
                            None,
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

    use crate::{Linter, Rule};

    #[test]
    fn err() {
        let sql = r#"
ALTER TABLE "bar_tbl" ALTER COLUMN "foo_col" DROP NOT NULL;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanDropNotNull]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }
}
