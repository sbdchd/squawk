use squawk_syntax::{
    ast::{self, AstNode, HasModuleItem},
    Parse, SourceFile,
};

use crate::{Linter, Rule, Violation};

pub(crate) fn adding_primary_key_constraint(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let message = "Adding a primary key constraint requires an `ACCESS EXCLUSIVE` lock that will block all reads and writes to the table while the primary key index is built.";
    let help = "Add the `PRIMARY KEY` constraint `USING` an index.";
    let file = parse.tree();
    for item in file.items() {
        if let ast::Item::AlterTable(alter_table) = item {
            for action in alter_table.actions() {
                match action {
                    ast::AlterTableAction::AddConstraint(add_constraint) => {
                        if let Some(ast::Constraint::PrimaryKeyConstraint(primary_key_constraint)) =
                            add_constraint.constraint()
                        {
                            if primary_key_constraint.using_index().is_none() {
                                ctx.report(Violation::new(
                                    Rule::AddingSerialPrimaryKeyField,
                                    message.to_string(),
                                    primary_key_constraint.syntax().text_range(),
                                    help.to_string(),
                                ));
                            }
                        }
                    }
                    ast::AlterTableAction::AddColumn(add_column) => {
                        for constraint in add_column.constraints() {
                            if let ast::Constraint::PrimaryKeyConstraint(primary_key_constraint) =
                                constraint
                            {
                                if primary_key_constraint.using_index().is_none() {
                                    ctx.report(Violation::new(
                                        Rule::AddingSerialPrimaryKeyField,
                                        message.to_string(),
                                        primary_key_constraint.syntax().text_range(),
                                        help.to_string(),
                                    ));
                                }
                            }
                        }
                    }
                    _ => (),
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
    fn serial_primary_key() {
        let sql = r#"
        ALTER TABLE a ADD COLUMN b SERIAL PRIMARY KEY;
    "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::AddingSerialPrimaryKeyField]);
        let errors = linter.lint(file, sql);
        assert!(!errors.is_empty());
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn plain_primary_key() {
        let sql = r#"
ALTER TABLE items ADD PRIMARY KEY (id);
    "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::AddingSerialPrimaryKeyField]);
        let errors = linter.lint(file, sql);
        assert!(!errors.is_empty());
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn okay_add_constraint() {
        let sql = r#"
ALTER TABLE items ADD CONSTRAINT items_pk PRIMARY KEY USING INDEX items_pk;
        "#;

        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::AddingSerialPrimaryKeyField]);
        let errors = linter.lint(file, sql);
        assert!(errors.is_empty());
    }
}
