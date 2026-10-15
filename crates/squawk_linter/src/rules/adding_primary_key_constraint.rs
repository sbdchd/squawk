use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation, transactions::TablesCreatedInTransaction};

pub(crate) fn adding_primary_key_constraint(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let message = "Adding a primary key constraint requires an `ACCESS EXCLUSIVE` lock that will block all reads and writes to the table while the primary key index is built.";
    let help = "Add the `PRIMARY KEY` constraint `USING` an index.";
    let file = parse.tree();
    let mut tables_created = TablesCreatedInTransaction::new(ctx.settings.assume_in_transaction);
    for stmt in file.stmts() {
        tables_created.update(&stmt);
        if let ast::Stmt::AlterTable(alter_table) = stmt {
            if let Some(table_name) = alter_table
                .table_relation_name()
                .and_then(|relation| relation.table_name_ref())
                .and_then(|table| table.path_ref())
                && tables_created.contains(&table_name)
            {
                continue;
            }
            for action in alter_table.actions() {
                match action {
                    ast::AlterTableAction::AddConstraint(add_constraint) => {
                        if let Some(ast::Constraint::PrimaryKeyConstraint(primary_key_constraint)) =
                            add_constraint.constraint()
                            && primary_key_constraint.using_index().is_none()
                        {
                            ctx.report(
                                Violation::for_node(
                                    Rule::AddingSerialPrimaryKeyField,
                                    message.to_string(),
                                    primary_key_constraint.syntax(),
                                )
                                .help(help),
                            );
                        }
                    }
                    ast::AlterTableAction::AddColumn(add_column) => {
                        for constraint in add_column.constraints() {
                            if let ast::Constraint::PrimaryKeyConstraint(primary_key_constraint) =
                                constraint
                                && primary_key_constraint.using_index().is_none()
                            {
                                ctx.report(
                                    Violation::for_node(
                                        Rule::AddingSerialPrimaryKeyField,
                                        message.to_string(),
                                        primary_key_constraint.syntax(),
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

    use crate::Rule;
    use crate::test_utils::{lint_errors, lint_ok};

    #[test]
    fn created_table_after_commit_err() {
        let sql = "begin;
create table t (id int);
create table u as select 1 as id;
commit;
alter table t add primary key (id);
alter table u add column b serial primary key;";
        assert_snapshot!(lint_errors(sql, Rule::AddingSerialPrimaryKeyField), @"
        warning[adding-serial-primary-key-field]: Adding a primary key constraint requires an `ACCESS EXCLUSIVE` lock that will block all reads and writes to the table while the primary key index is built.
          ╭▸ 
        5 │ alter table t add primary key (id);
          │                   ━━━━━━━━━━━━━━━━
          │
          ╰ help: Add the `PRIMARY KEY` constraint `USING` an index.
        warning[adding-serial-primary-key-field]: Adding a primary key constraint requires an `ACCESS EXCLUSIVE` lock that will block all reads and writes to the table while the primary key index is built.
          ╭▸ 
        6 │ alter table u add column b serial primary key;
          │                                   ━━━━━━━━━━━
          │
          ╰ help: Add the `PRIMARY KEY` constraint `USING` an index.
        ");
    }

    #[test]
    fn created_table_in_another_schema_err() {
        let sql = "begin;
create table new.t (id int);
select 1 as id into new.u;
alter table existing.t add primary key (id);
alter table existing.u add column b serial primary key;
commit;";
        assert_snapshot!(lint_errors(sql, Rule::AddingSerialPrimaryKeyField), @"
        warning[adding-serial-primary-key-field]: Adding a primary key constraint requires an `ACCESS EXCLUSIVE` lock that will block all reads and writes to the table while the primary key index is built.
          ╭▸ 
        4 │ alter table existing.t add primary key (id);
          │                            ━━━━━━━━━━━━━━━━
          │
          ╰ help: Add the `PRIMARY KEY` constraint `USING` an index.
        warning[adding-serial-primary-key-field]: Adding a primary key constraint requires an `ACCESS EXCLUSIVE` lock that will block all reads and writes to the table while the primary key index is built.
          ╭▸ 
        5 │ alter table existing.u add column b serial primary key;
          │                                            ━━━━━━━━━━━
          │
          ╰ help: Add the `PRIMARY KEY` constraint `USING` an index.
        ");
    }

    #[test]
    fn created_table_qualified_names_ok() {
        let sql = r#"begin;
create table new.t (id int);
select 1 as id into new.u;
alter table NEW . "t" add primary key (id);
alter table new.u add column b serial primary key;
commit;"#;
        lint_ok(sql, Rule::AddingSerialPrimaryKeyField);
    }

    #[test]
    fn new_tables_in_explicit_transaction_ok() {
        let sql = "begin;
create table t (id int);
alter table t add primary key (id);
create table u as
  select 1 as id;
alter table u add primary key (id);
select 1 as id into v;
alter table v add primary key (id);
create table x (id int);
alter table x add column b serial primary key;
create table y as
  select 1 as id;
alter table y add column b serial primary key;
select 1 as id into z;
alter table z add column b serial primary key;
commit;";
        lint_ok(sql, Rule::AddingSerialPrimaryKeyField);
    }

    #[test]
    fn new_tables_in_assumed_transaction_ok() {
        let sql = "create table t (id int);
alter table t add primary key (id);
create table u as
  select 1 as id;
alter table u add primary key (id);
select 1 as id into v;
alter table v add primary key (id);
create table x (id int);
alter table x add column b serial primary key;
create table y as
  select 1 as id;
alter table y add column b serial primary key;
select 1 as id into z;
alter table z add column b serial primary key;";
        crate::test_utils::lint_ok_with(
            sql,
            crate::LinterSettings {
                assume_in_transaction: true,
                ..Default::default()
            },
            Rule::AddingSerialPrimaryKeyField,
        );
    }

    #[test]
    fn new_tables_without_transaction_err() {
        let sql = "create table t as
  select 1 as id;
alter table t add primary key (id);
select 1 as id into u;
alter table u add column b serial primary key;";
        assert_snapshot!(lint_errors(sql, Rule::AddingSerialPrimaryKeyField), @"
        warning[adding-serial-primary-key-field]: Adding a primary key constraint requires an `ACCESS EXCLUSIVE` lock that will block all reads and writes to the table while the primary key index is built.
          ╭▸ 
        3 │ alter table t add primary key (id);
          │                   ━━━━━━━━━━━━━━━━
          │
          ╰ help: Add the `PRIMARY KEY` constraint `USING` an index.
        warning[adding-serial-primary-key-field]: Adding a primary key constraint requires an `ACCESS EXCLUSIVE` lock that will block all reads and writes to the table while the primary key index is built.
          ╭▸ 
        5 │ alter table u add column b serial primary key;
          │                                   ━━━━━━━━━━━
          │
          ╰ help: Add the `PRIMARY KEY` constraint `USING` an index.
        ");
    }

    #[test]
    fn serial_primary_key() {
        let sql = r#"
        ALTER TABLE a ADD COLUMN b SERIAL PRIMARY KEY;
    "#;
        assert_snapshot!(lint_errors(sql, Rule::AddingSerialPrimaryKeyField));
    }

    #[test]
    fn plain_primary_key() {
        let sql = r#"
ALTER TABLE items ADD PRIMARY KEY (id);
    "#;
        assert_snapshot!(lint_errors(sql, Rule::AddingSerialPrimaryKeyField));
    }

    #[test]
    fn okay_add_constraint() {
        let sql = r#"
ALTER TABLE items ADD CONSTRAINT items_pk PRIMARY KEY USING INDEX items_pk;
        "#;

        lint_ok(sql, Rule::AddingSerialPrimaryKeyField);
    }
}
