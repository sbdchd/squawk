use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation, transactions::TablesCreatedInTransaction};

pub(crate) fn disallow_unique_constraint(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let message = "Adding a `UNIQUE` constraint requires an `ACCESS EXCLUSIVE` lock which blocks reads and writes to the table while the index is built.";
    let help = "Create an index `CONCURRENTLY` and create the constraint using the index.";
    let file = parse.tree();
    let mut tables_created = TablesCreatedInTransaction::new(ctx.settings.assume_in_transaction);
    for stmt in file.stmts() {
        tables_created.update(&stmt);
        if let ast::Stmt::AlterTable(alter_table) = stmt {
            let Some(table_name) = alter_table
                .table_relation_name()
                .and_then(|relation| relation.table_name_ref())
                .and_then(|table| table.path_ref())
            else {
                continue;
            };
            for action in alter_table.actions() {
                match action {
                    ast::AlterTableAction::AddConstraint(add_constraint) => {
                        if let Some(ast::Constraint::UniqueConstraint(unique_constraint)) =
                            add_constraint.constraint()
                            && unique_constraint.using_index().is_none()
                            && !tables_created.contains(&table_name)
                        {
                            ctx.report(
                                Violation::for_node(
                                    Rule::DisallowedUniqueConstraint,
                                    message.into(),
                                    unique_constraint.syntax(),
                                )
                                .help(help),
                            );
                        }
                    }
                    ast::AlterTableAction::AddColumn(add_column) => {
                        for constraint in add_column.constraints() {
                            if let ast::Constraint::UniqueConstraint(unique_constraint) = constraint
                            {
                                ctx.report(
                                    Violation::for_node(
                                        Rule::DisallowedUniqueConstraint,
                                        message.into(),
                                        unique_constraint.syntax(),
                                    )
                                    .help(help),
                                );
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
    use insta::assert_snapshot;

    use crate::test_utils::{lint_errors, lint_ok};

    #[test]
    fn created_table_after_commit_err() {
        let sql = "begin;
create table t as select 1 as id;
commit;
alter table t add unique (id);";
        assert_snapshot!(lint_errors(sql, Rule::DisallowedUniqueConstraint), @"
        warning[disallowed-unique-constraint]: Adding a `UNIQUE` constraint requires an `ACCESS EXCLUSIVE` lock which blocks reads and writes to the table while the index is built.
          ╭▸ 
        4 │ alter table t add unique (id);
          │                   ━━━━━━━━━━━
          │
          ╰ help: Create an index `CONCURRENTLY` and create the constraint using the index.
        ");
    }

    #[test]
    fn created_table_in_another_schema_err() {
        let sql = "begin;
select 1 as id into new.t;
alter table existing.t add unique (id);
commit;";
        assert_snapshot!(lint_errors(sql, Rule::DisallowedUniqueConstraint), @"
        warning[disallowed-unique-constraint]: Adding a `UNIQUE` constraint requires an `ACCESS EXCLUSIVE` lock which blocks reads and writes to the table while the index is built.
          ╭▸ 
        3 │ alter table existing.t add unique (id);
          │                            ━━━━━━━━━━━
          │
          ╰ help: Create an index `CONCURRENTLY` and create the constraint using the index.
        ");
    }

    #[test]
    fn created_table_qualified_names_ok() {
        let sql = r#"begin;
select 1 as id into new.t;
alter table NEW . "t" add unique (id);
commit;"#;
        lint_ok(sql, Rule::DisallowedUniqueConstraint);
    }
    use crate::{LinterSettings, Rule};

    fn lint_ok_with(sql: &str, settings: LinterSettings) {
        crate::test_utils::lint_ok_with(sql, settings, Rule::DisallowedUniqueConstraint);
    }

    #[test]
    fn adding_unique_constraint_err() {
        let sql = r#"
ALTER TABLE table_name ADD CONSTRAINT field_name_constraint UNIQUE (field_name);
        "#;
        assert_snapshot!(lint_errors(sql, Rule::DisallowedUniqueConstraint));
    }

    #[test]
    fn ok() {
        let sql = r#"
ALTER TABLE table_name DROP CONSTRAINT field_name_constraint;
        "#;
        lint_ok(sql, Rule::DisallowedUniqueConstraint);
    }

    #[test]
    fn also_ok() {
        let sql = r#"
CREATE UNIQUE INDEX CONCURRENTLY dist_id_temp_idx ON distributors (dist_id);
ALTER TABLE distributors DROP CONSTRAINT distributors_pkey,
ADD CONSTRAINT distributors_pkey PRIMARY KEY USING INDEX dist_id_temp_idx;
        "#;
        lint_ok(sql, Rule::DisallowedUniqueConstraint);
    }

    #[test]
    fn unique_constraint_ok() {
        let sql = r#"
CREATE UNIQUE INDEX CONCURRENTLY "legacy_questiongrouppg_mongo_id_1f8f47d9_uniq_idx"
    ON "legacy_questiongrouppg" ("mongo_id");
ALTER TABLE "legacy_questiongrouppg" ADD CONSTRAINT "legacy_questiongrouppg_mongo_id_1f8f47d9_uniq" UNIQUE USING INDEX "legacy_questiongrouppg_mongo_id_1f8f47d9_uniq_idx";
        "#;
        lint_ok(sql, Rule::DisallowedUniqueConstraint);
    }

    #[test]
    fn unique_constraint_after_create_table() {
        let sql = r#"
BEGIN;
CREATE TABLE products (
    id bigint generated by default as identity primary key,
    sku text not null
);
ALTER TABLE products ADD CONSTRAINT sku_constraint UNIQUE (sku);
COMMIT;
        "#;
        lint_ok(sql, Rule::DisallowedUniqueConstraint);
    }

    #[test]
    fn unique_constraint_after_create_table_with_assume_in_transaction() {
        let sql = r#"
CREATE TABLE products (
    id bigint generated by default as identity primary key,
    sku text not null
);
ALTER TABLE products ADD CONSTRAINT sku_constraint UNIQUE (sku);
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
    fn unique_constraint_inline_add_column_err() {
        let sql = r#"
ALTER TABLE foo ADD COLUMN bar text CONSTRAINT foo_bar_unique UNIQUE;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::DisallowedUniqueConstraint));
    }

    #[test]
    fn unique_constraint_inline_add_column_unique_err() {
        let sql = r#"
ALTER TABLE foo ADD COLUMN bar text UNIQUE;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::DisallowedUniqueConstraint));
    }
}
