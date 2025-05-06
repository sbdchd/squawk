use squawk_syntax::{
    ast::{self, AstNode, HasModuleItem},
    Parse, SourceFile,
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_drop_column(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for item in file.items() {
        if let ast::Item::AlterTable(alter_table) = item {
            for action in alter_table.actions() {
                if let ast::AlterTableAction::DropColumn(drop_column) = action {
                    ctx.report(Violation::new(
                        Rule::BanDropColumn,
                        "Dropping a column may break existing clients.".into(),
                        drop_column.syntax().text_range(),
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
ALTER TABLE "bar_tbl" DROP COLUMN "foo_col" CASCADE;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanDropColumn]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }
}
