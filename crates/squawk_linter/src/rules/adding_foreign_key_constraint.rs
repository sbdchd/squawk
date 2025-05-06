use squawk_syntax::{
    ast::{self, AstNode, HasModuleItem},
    Parse, SourceFile,
};

use crate::{Linter, Rule, Violation};

pub(crate) fn adding_foreign_key_constraint(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let message = "Adding a foreign key constraint requires a table scan and a `SHARE ROW EXCLUSIVE` lock on both tables, which blocks writes to each table.";
    let help = "Add `NOT VALID` to the constraint in one transaction and then VALIDATE the constraint in a separate transaction.";
    let file = parse.tree();
    // TODO: use match_ast! like in #api_walkthrough
    for item in file.items() {
        if let ast::Item::AlterTable(alter_table) = item {
            for action in alter_table.actions() {
                match action {
                    ast::AlterTableAction::AddConstraint(add_constraint) => {
                        if add_constraint.not_valid().is_some() {
                            // Adding foreign key is okay when NOT VALID is specified.
                            continue;
                        }
                        if let Some(constraint) = add_constraint.constraint() {
                            if matches!(
                                constraint,
                                ast::Constraint::ForeignKeyConstraint(_)
                                    | ast::Constraint::ReferencesConstraint(_)
                            ) {
                                ctx.report(Violation::new(
                                    Rule::AddingForeignKeyConstraint,
                                    message.into(),
                                    constraint.syntax().text_range(),
                                    help.to_string(),
                                ))
                            }
                        }
                    }
                    ast::AlterTableAction::AddColumn(add_column) => {
                        for constraint in add_column.constraints() {
                            if matches!(
                                constraint,
                                ast::Constraint::ForeignKeyConstraint(_)
                                    | ast::Constraint::ReferencesConstraint(_)
                            ) {
                                ctx.report(Violation::new(
                                    Rule::AddingForeignKeyConstraint,
                                    message.into(),
                                    constraint.syntax().text_range(),
                                    help.to_string(),
                                ))
                            }
                        }
                    }
                    _ => continue,
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Linter, Rule};

    #[test]
    fn create_table_with_foreign_key_constraint() {
        let sql = r#"
        BEGIN;
        CREATE TABLE email (
            id BIGINT GENERATED ALWAYS AS IDENTITY,
            user_id BIGINT,
            email TEXT,
            PRIMARY KEY(id),
            CONSTRAINT fk_user
                FOREIGN KEY ("user_id") 
                REFERENCES "user" ("id")
        );
        COMMIT;
        "#;

        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::AddingForeignKeyConstraint]);
        let errors = linter.lint(file, sql);
        assert!(errors.is_empty());
    }

    #[test]
    fn add_foreign_key_constraint_not_valid_validate() {
        let sql = r#"
BEGIN;
ALTER TABLE "email" ADD COLUMN "user_id" INT;
ALTER TABLE "email" ADD CONSTRAINT "fk_user" FOREIGN KEY ("user_id") REFERENCES "user" ("id") NOT VALID;
ALTER TABLE "email" VALIDATE CONSTRAINT "fk_user";
COMMIT;
        "#;

        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::AddingForeignKeyConstraint]);
        let errors = linter.lint(file, sql);
        assert!(errors.is_empty());
    }

    #[test]
    fn add_foreign_key_constraint_lock() {
        let sql = r#"
BEGIN;
ALTER TABLE "email" ADD COLUMN "user_id" INT;
ALTER TABLE "email" ADD CONSTRAINT "fk_user" FOREIGN KEY ("user_id") REFERENCES "user" ("id");
COMMIT;
        "#;

        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::AddingForeignKeyConstraint]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, Rule::AddingForeignKeyConstraint);
    }

    #[test]
    fn add_column_references_lock() {
        let sql = r#"
BEGIN;
ALTER TABLE "emails" ADD COLUMN "user_id" INT REFERENCES "user" ("id");
COMMIT;
        "#;

        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::AddingForeignKeyConstraint]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, Rule::AddingForeignKeyConstraint);
    }
}
