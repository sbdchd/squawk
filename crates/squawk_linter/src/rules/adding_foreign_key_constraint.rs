use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation, transactions::TablesCreatedInTransaction};

pub(crate) fn adding_foreign_key_constraint(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let message = "Adding a foreign key constraint requires a table scan and a `SHARE ROW EXCLUSIVE` lock on both tables, which blocks writes to each table.";
    let help = "Add `NOT VALID` to the constraint in one transaction and then VALIDATE the constraint in a separate transaction.";
    let file = parse.tree();
    let mut tables_created = TablesCreatedInTransaction::new(ctx.settings.assume_in_transaction);
    // TODO: use match_ast! like in #api_walkthrough
    for stmt in file.stmts() {
        tables_created.update(&stmt);
        match stmt {
            ast::Stmt::AlterTable(alter_table) => {
                if let Some(table_name) = alter_table
                    .table_relation_name()
                    .and_then(|relation| relation.table_name_ref())
                    .and_then(|table| table.path_ref())
                {
                    for action in alter_table.actions() {
                        match action {
                            ast::AlterTableAction::AddConstraint(add_constraint) => {
                                let Some(constraint) = add_constraint.constraint() else {
                                    continue;
                                };
                                if constraint.is_not_valid() || tables_created.contains(&table_name)
                                {
                                    // Adding foreign key is okay when:
                                    // - NOT VALID is specified.
                                    // - The table is created in the same transaction
                                    continue;
                                }
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
                                    );
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
                                        );
                                    }
                                }
                            }
                            _ => continue,
                        }
                    }
                }
            }
            ast::Stmt::CreateTable(_) => {
                // NOTE: we don't consider foreign key constraints in create
                // table stmts as there are no rows in the current table.
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::test_utils::{lint_errors, lint_ok};

    #[test]
    fn created_table_after_commit_err() {
        let sql = "begin;
create table t (id int);
commit;
alter table t add foreign key (id) references other_t (id);";
        assert_snapshot!(lint_errors(sql, Rule::AddingForeignKeyConstraint), @"
        warning[adding-foreign-key-constraint]: Adding a foreign key constraint requires a table scan and a `SHARE ROW EXCLUSIVE` lock on both tables, which blocks writes to each table.
          ╭▸ 
        4 │ alter table t add foreign key (id) references other_t (id);
          │                   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ╰ help: Add `NOT VALID` to the constraint in one transaction and then VALIDATE the constraint in a separate transaction.
        ");
    }

    #[test]
    fn created_table_in_another_schema_err() {
        let sql = "begin;
create table new.t as select 1 as id;
alter table existing.t add foreign key (id) references other_t (id);
commit;";
        assert_snapshot!(lint_errors(sql, Rule::AddingForeignKeyConstraint), @"
        warning[adding-foreign-key-constraint]: Adding a foreign key constraint requires a table scan and a `SHARE ROW EXCLUSIVE` lock on both tables, which blocks writes to each table.
          ╭▸ 
        3 │ alter table existing.t add foreign key (id) references other_t (id);
          │                            ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ╰ help: Add `NOT VALID` to the constraint in one transaction and then VALIDATE the constraint in a separate transaction.
        ");
    }

    #[test]
    fn created_table_qualified_names_ok() {
        let sql = r#"begin;
create table new.t as select 1 as id;
alter table NEW . "t" add foreign key (id) references other_t (id);
commit;"#;
        lint_ok(sql, Rule::AddingForeignKeyConstraint);
    }
    use crate::{LinterSettings, Rule};

    fn lint_ok_with(sql: &str, settings: LinterSettings) {
        crate::test_utils::lint_ok_with(sql, settings, Rule::AddingForeignKeyConstraint);
    }

    #[test]
    fn create_table_with_foreign_key_constraint() {
        // Okay as there are no rows in the current table. We'll take a share
        // row exclusive lock on the related table but won't hold it for long.
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

        lint_ok(sql, Rule::AddingForeignKeyConstraint);
    }

    #[test]
    fn create_table_with_column_references() {
        // Okay as there are no rows in the current table. We'll take a share
        // row exclusive lock on the related table but won't hold it for long.
        let sql = r#"
CREATE TABLE table_name (
    id BIGSERIAL NOT NULL,
    my_fk_id INTEGER NOT NULL REFERENCES other_table (id) ON DELETE CASCADE
);
        "#;

        lint_ok(sql, Rule::AddingForeignKeyConstraint);
    }

    #[test]
    fn alter_table_foreign_key_assume_transaction() {
        let sql = r#"
CREATE TABLE "emails" ("id" UUID NOT NULL, "user_id" UUID NOT NULL);
ALTER TABLE "emails" ADD CONSTRAINT "fk_user" FOREIGN KEY ("user_id") REFERENCES "users" ("id");
        "#;

        lint_ok_with(
            sql,
            LinterSettings {
                assume_in_transaction: true,
                ..Default::default()
            },
        );
    }

    #[test]
    fn alter_table_foreign_key_in_transaction() {
        let sql = r#"
BEGIN;
CREATE TABLE "emails" ("id" UUID NOT NULL, "user_id" UUID NOT NULL);
ALTER TABLE "emails" ADD CONSTRAINT "fk_user" FOREIGN KEY ("user_id") REFERENCES "users" ("id");
COMMIT;
        "#;

        lint_ok(sql, Rule::AddingForeignKeyConstraint);
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

        lint_ok(sql, Rule::AddingForeignKeyConstraint);
    }

    #[test]
    fn add_foreign_key_constraint_lock() {
        let sql = r#"
BEGIN;
ALTER TABLE "email" ADD COLUMN "user_id" INT;
ALTER TABLE "email" ADD CONSTRAINT "fk_user" FOREIGN KEY ("user_id") REFERENCES "user" ("id");
COMMIT;
        "#;

        assert_snapshot!(lint_errors(sql, Rule::AddingForeignKeyConstraint));
    }

    #[test]
    fn add_column_references_lock() {
        let sql = r#"
BEGIN;
ALTER TABLE "emails" ADD COLUMN "user_id" INT REFERENCES "user" ("id");
COMMIT;
        "#;

        assert_snapshot!(lint_errors(sql, Rule::AddingForeignKeyConstraint));
    }
}
