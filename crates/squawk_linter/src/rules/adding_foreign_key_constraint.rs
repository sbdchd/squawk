use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
    identifier::Identifier,
};

use crate::{
    Linter, Rule, Violation, rules::constraint_missing_not_valid::tables_created_in_transaction,
};

pub(crate) fn adding_foreign_key_constraint(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let message = "Adding a foreign key constraint requires a table scan and a `SHARE ROW EXCLUSIVE` lock on both tables, which blocks writes to each table.";
    let help = "Add `NOT VALID` to the constraint in one transaction and then VALIDATE the constraint in a separate transaction.";
    let file = parse.tree();
    let tables_created = tables_created_in_transaction(ctx.settings.assume_in_transaction, &file);
    // TODO: use match_ast! like in #api_walkthrough
    for stmt in file.stmts() {
        if let ast::Stmt::AlterTable(alter_table) = stmt {
            if let Some(table_name) = alter_table
                .relation_name()
                .and_then(|x| x.path())
                .and_then(|x| x.segment())
                .and_then(|x| x.name_ref())
            {
                for action in alter_table.actions() {
                    match action {
                        ast::AlterTableAction::AddConstraint(add_constraint) => {
                            if add_constraint.not_valid().is_some()
                                || tables_created.contains(&Identifier::new(&table_name.text()))
                            {
                                // Adding foreign key is okay when:
                                // - NOT VALID is specified.
                                // - The table is created in the same transaction
                                continue;
                            }
                            if let Some(constraint) = add_constraint.constraint() {
                                if matches!(
                                    constraint,
                                    ast::Constraint::ForeignKeyConstraint(_)
                                        | ast::Constraint::ReferencesConstraint(_)
                                ) {
                                    ctx.report(
                                        Violation::for_node(
                                            Rule::AddingForeignKeyConstraint,
                                            message.into(),
                                            constraint.syntax(),
                                        )
                                        .help(help),
                                    )
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
                                    ctx.report(
                                        Violation::for_node(
                                            Rule::AddingForeignKeyConstraint,
                                            message.into(),
                                            constraint.syntax(),
                                        )
                                        .help(help),
                                    )
                                }
                            }
                        }
                        _ => continue,
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Rule;
    use crate::test_utils::{lint, lint_with_assume_in_transaction};

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

        let errors = lint(sql, Rule::AddingForeignKeyConstraint);
        assert!(errors.is_empty());
    }

    #[test]
    fn alter_table_foreign_key_assume_transaction() {
        let sql = r#"
CREATE TABLE "emails" ("id" UUID NOT NULL, "user_id" UUID NOT NULL);
ALTER TABLE "emails" ADD CONSTRAINT "fk_user" FOREIGN KEY ("user_id") REFERENCES "users" ("id");
        "#;

        let errors = lint_with_assume_in_transaction(sql, Rule::AddingForeignKeyConstraint);
        assert!(errors.is_empty());
    }

    #[test]
    fn alter_table_foreign_key_in_transaction() {
        let sql = r#"
BEGIN;
CREATE TABLE "emails" ("id" UUID NOT NULL, "user_id" UUID NOT NULL);
ALTER TABLE "emails" ADD CONSTRAINT "fk_user" FOREIGN KEY ("user_id") REFERENCES "users" ("id");
COMMIT;
        "#;

        let errors = lint(sql, Rule::AddingForeignKeyConstraint);
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

        let errors = lint(sql, Rule::AddingForeignKeyConstraint);
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

        let errors = lint(sql, Rule::AddingForeignKeyConstraint);
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

        let errors = lint(sql, Rule::AddingForeignKeyConstraint);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, Rule::AddingForeignKeyConstraint);
    }
}
