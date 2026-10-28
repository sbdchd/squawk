use rustc_hash::FxHashSet;

use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode, NameLike},
};

use crate::{Linter, Rule, Violation, transactions::TablesCreatedInTransaction};

fn not_valid_validate_in_transaction(
    ctx: &mut Linter,
    assume_in_transaction: bool,
    file: &ast::SourceFile,
) {
    let mut inside_transaction = assume_in_transaction;
    let mut not_valid_names: FxHashSet<String> = FxHashSet::default();
    for stmt in file.stmts() {
        match stmt {
            ast::Stmt::AlterTable(alter_table) => {
                for action in alter_table.actions() {
                    match action {
                        ast::AlterTableAction::ValidateConstraint(validate_constraint) => {
                            if let Some(constraint_name) = validate_constraint
                                .constraint_name_ref()
                                .and_then(|constraint| constraint.path_ref())
                                .and_then(|path| path.segment())
                                .map(|name| name.text())
                                && inside_transaction
                                && not_valid_names.contains(&constraint_name)
                            {
                                ctx.report(
                                        Violation::for_node(
                                        Rule::ConstraintMissingNotValid,
                                        "using `not valid` and `validate constraint` in the same transaction will block all reads while the constraint is validated".into(),
                                        validate_constraint.syntax(),
                                    ).help("add constraint as `not valid` in one transaction and `validate constraint` in a separate transaction"));
                            }
                        }
                        ast::AlterTableAction::AddConstraint(add_constraint) => {
                            if let Some(constraint) = add_constraint.constraint()
                                && constraint.is_not_valid()
                                && let Some(constraint_name) = constraint.constraint_name()
                            {
                                not_valid_names.insert(constraint_name.text());
                            }
                        }
                        _ => (),
                    }
                }
            }
            ast::Stmt::Begin(_) => {
                if !inside_transaction {
                    not_valid_names.clear();
                }
                inside_transaction = true;
            }
            ast::Stmt::Commit(_) => {
                inside_transaction = false;
            }
            _ => (),
        }
    }
}

pub(crate) fn constraint_missing_not_valid(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();

    let assume_in_transaction = ctx.settings.assume_in_transaction;

    not_valid_validate_in_transaction(ctx, assume_in_transaction, &file);

    let mut tables_created = TablesCreatedInTransaction::new(assume_in_transaction);

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
                if let ast::AlterTableAction::AddConstraint(add_constraint) = action
                    && !tables_created.contains(&table_name)
                    && let Some(constraint) = add_constraint.constraint()
                    && !constraint.is_not_valid()
                {
                    if let ast::Constraint::UniqueConstraint(uc) = &constraint
                        && uc.using_index().is_some()
                    {
                        continue;
                    }
                    if let ast::Constraint::PrimaryKeyConstraint(pk) = &constraint
                        && pk.using_index().is_some()
                    {
                        continue;
                    }

                    ctx.report(Violation::for_node(
                            Rule::ConstraintMissingNotValid,
                            "by default new constraints require a table scan and block writes to the table while that scan occurs".into(),
                            add_constraint.syntax(),
                        ).help("use `not valid` with a later `validate constraint` call"));
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::test_utils::{lint_errors, lint_ok};
    use crate::{LinterSettings, Rule};

    fn lint_ok_with(sql: &str, settings: LinterSettings) {
        crate::test_utils::lint_ok_with(sql, settings, Rule::ConstraintMissingNotValid);
    }

    #[must_use]
    fn lint_errors_with(sql: &str, settings: LinterSettings) -> String {
        crate::test_utils::lint_errors_with(sql, settings, Rule::ConstraintMissingNotValid)
    }

    #[test]
    fn created_table_after_commit_err() {
        let sql = "begin;
create table t (id int);
commit;
alter table t add check (id > 0);";
        assert_snapshot!(lint_errors(sql, Rule::ConstraintMissingNotValid), @"
        warning[constraint-missing-not-valid]: by default new constraints require a table scan and block writes to the table while that scan occurs
          ╭▸ 
        4 │ alter table t add check (id > 0);
          │               ━━━━━━━━━━━━━━━━━━
          │
          ╰ help: use `not valid` with a later `validate constraint` call
        ");
    }

    #[test]
    fn created_table_in_another_schema_err() {
        let sql = "begin;
create table new.t as select 1 as id;
alter table existing.t add check (id > 0);
alter table t add check (id > 0);
commit;";
        assert_snapshot!(lint_errors(sql, Rule::ConstraintMissingNotValid), @"
        warning[constraint-missing-not-valid]: by default new constraints require a table scan and block writes to the table while that scan occurs
          ╭▸ 
        3 │ alter table existing.t add check (id > 0);
          │                        ━━━━━━━━━━━━━━━━━━
          │
          ╰ help: use `not valid` with a later `validate constraint` call
        warning[constraint-missing-not-valid]: by default new constraints require a table scan and block writes to the table while that scan occurs
          ╭▸ 
        4 │ alter table t add check (id > 0);
          │               ━━━━━━━━━━━━━━━━━━
          │
          ╰ help: use `not valid` with a later `validate constraint` call
        ");
    }

    #[test]
    fn created_table_qualified_names_ok() {
        let sql = r#"begin;
create table new.t (id int);
alter table NEW . "t" add check (id > 0);
create table new.u as select 1 as id;
alter table new.u add check (id > 0);
select 1 as id into new.v;
alter table new.v add check (id > 0);
commit;"#;
        lint_ok(sql, Rule::ConstraintMissingNotValid);
    }

    #[test]
    fn created_table_after_rollback_err() {
        let sql = "begin;
create table t as select 1 as id;
rollback;
alter table t add check (id > 0);";
        assert_snapshot!(lint_errors(sql, Rule::ConstraintMissingNotValid), @"
        warning[constraint-missing-not-valid]: by default new constraints require a table scan and block writes to the table while that scan occurs
          ╭▸ 
        4 │ alter table t add check (id > 0);
          │               ━━━━━━━━━━━━━━━━━━
          │
          ╰ help: use `not valid` with a later `validate constraint` call
        ");
    }

    #[test]
    fn created_table_in_previous_transaction_err() {
        let sql = "begin;
select 1 as id into t;
commit;
begin;
alter table t add check (id > 0);
commit;";
        assert_snapshot!(lint_errors(sql, Rule::ConstraintMissingNotValid), @"
        warning[constraint-missing-not-valid]: by default new constraints require a table scan and block writes to the table while that scan occurs
          ╭▸ 
        5 │ alter table t add check (id > 0);
          │               ━━━━━━━━━━━━━━━━━━
          │
          ╰ help: use `not valid` with a later `validate constraint` call
        ");
    }

    #[test]
    fn table_created_later_err() {
        let sql = "begin;
alter table t add check (id > 0);
create table t (id int);
commit;";
        assert_snapshot!(lint_errors(sql, Rule::ConstraintMissingNotValid), @"
        warning[constraint-missing-not-valid]: by default new constraints require a table scan and block writes to the table while that scan occurs
          ╭▸ 
        2 │ alter table t add check (id > 0);
          │               ━━━━━━━━━━━━━━━━━━
          │
          ╰ help: use `not valid` with a later `validate constraint` call
        ");
    }

    #[test]
    fn assumed_transaction_after_commit_err() {
        let sql = "create table t (id int);
commit;
alter table t add check (id > 0);";
        assert_snapshot!(lint_errors_with(sql, LinterSettings {
            assume_in_transaction: true,
            ..Default::default()
        }), @"
        warning[constraint-missing-not-valid]: by default new constraints require a table scan and block writes to the table while that scan occurs
          ╭▸ 
        3 │ alter table t add check (id > 0);
          │               ━━━━━━━━━━━━━━━━━━
          │
          ╰ help: use `not valid` with a later `validate constraint` call
        ");
    }

    #[test]
    fn chained_transactions_clear_created_tables_err() {
        let sql = "begin;
create table t (id int);
commit and chain;
alter table t add check (id > 0);
create table u (id int);
rollback and chain;
alter table u add check (id > 0);
commit;";
        assert_snapshot!(lint_errors(sql, Rule::ConstraintMissingNotValid), @"
        warning[constraint-missing-not-valid]: by default new constraints require a table scan and block writes to the table while that scan occurs
          ╭▸ 
        4 │ alter table t add check (id > 0);
          │               ━━━━━━━━━━━━━━━━━━
          │
          ╰ help: use `not valid` with a later `validate constraint` call
        warning[constraint-missing-not-valid]: by default new constraints require a table scan and block writes to the table while that scan occurs
          ╭▸ 
        7 │ alter table u add check (id > 0);
          │               ━━━━━━━━━━━━━━━━━━
          │
          ╰ help: use `not valid` with a later `validate constraint` call
        ");
    }

    #[test]
    fn chained_transactions_track_new_tables_ok() {
        let sql = "begin;
commit and chain;
create table t (id int);
alter table t add check (id > 0);
rollback and chain;
create table u (id int);
alter table u add check (id > 0);
commit;";
        lint_ok(sql, Rule::ConstraintMissingNotValid);
    }

    #[test]
    fn new_tables_in_explicit_transaction_ok() {
        let sql = "begin;
create table t as
  select 1 as id;
alter table t add primary key (id);
select 1 as id into u;
alter table u add primary key (id);
commit;";
        lint_ok(sql, Rule::ConstraintMissingNotValid);
    }

    #[test]
    fn new_tables_in_assumed_transaction_ok() {
        let sql = "create table t as
  select 1 as id;
alter table t add primary key (id);
select 1 as id into u;
alter table u add primary key (id);";
        lint_ok_with(
            sql,
            LinterSettings {
                assume_in_transaction: true,
                ..Default::default()
            },
        );
    }

    #[test]
    fn new_tables_without_transaction_err() {
        let sql = "create table t as
  select 1 as id;
alter table t add primary key (id);
select 1 as id into u;
alter table u add primary key (id);";
        assert_snapshot!(lint_errors(sql, Rule::ConstraintMissingNotValid), @"
        warning[constraint-missing-not-valid]: by default new constraints require a table scan and block writes to the table while that scan occurs
          ╭▸ 
        3 │ alter table t add primary key (id);
          │               ━━━━━━━━━━━━━━━━━━━━
          │
          ╰ help: use `not valid` with a later `validate constraint` call
        warning[constraint-missing-not-valid]: by default new constraints require a table scan and block writes to the table while that scan occurs
          ╭▸ 
        5 │ alter table u add primary key (id);
          │               ━━━━━━━━━━━━━━━━━━━━
          │
          ╰ help: use `not valid` with a later `validate constraint` call
        ");
    }

    #[test]
    fn not_valid_validate_transaction_err() {
        let sql = r#"
BEGIN;
ALTER TABLE "app_email" ADD CONSTRAINT "fk_user" FOREIGN KEY (user_id) REFERENCES "app_user" (id) NOT VALID;
ALTER TABLE "app_email" VALIDATE CONSTRAINT "fk_user";
COMMIT;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::ConstraintMissingNotValid));
    }

    #[test]
    fn not_valid_validate_assume_transaction_err() {
        let sql = r#"
ALTER TABLE "app_email" ADD CONSTRAINT "fk_user" FOREIGN KEY (user_id) REFERENCES "app_user" (id) NOT VALID;
ALTER TABLE "app_email" VALIDATE CONSTRAINT "fk_user";
        "#;
        assert_snapshot!(lint_errors_with(
            sql,
            LinterSettings {
                assume_in_transaction: true,
                ..Default::default()
            },
        ));
    }

    #[test]
    fn not_valid_validate_with_assume_in_transaction_with_explicit_commit_err() {
        let sql = r#"
ALTER TABLE "app_email" ADD CONSTRAINT "fk_user" FOREIGN KEY (user_id) REFERENCES "app_user" (id) NOT VALID;
ALTER TABLE "app_email" VALIDATE CONSTRAINT "fk_user";
COMMIT;
        "#;
        assert_snapshot!(lint_errors_with(
            sql,
            LinterSettings {
                assume_in_transaction: true,
                ..Default::default()
            },
        ));
    }

    #[test]
    fn adding_fk_err() {
        let sql = r#"
-- instead of
ALTER TABLE distributors ADD CONSTRAINT distfk FOREIGN KEY (address) REFERENCES addresses (address);
        "#;
        assert_snapshot!(lint_errors(sql, Rule::ConstraintMissingNotValid));
    }

    #[test]
    fn adding_fk_not_valid_ok() {
        let sql = r#"
-- use `NOT VALID`
ALTER TABLE distributors ADD CONSTRAINT distfk FOREIGN KEY (address) REFERENCES addresses (address) NOT VALID;
ALTER TABLE distributors VALIDATE CONSTRAINT distfk;
        "#;
        lint_ok(sql, Rule::ConstraintMissingNotValid);
    }

    #[test]
    fn adding_using_index_ok() {
        let sql = r#"
ALTER TABLE account ADD CONSTRAINT account_pk PRIMARY KEY USING INDEX account_pk_idx;
        "#;
        lint_ok(sql, Rule::ConstraintMissingNotValid);
    }

    #[test]
    fn adding_check_constraint_err() {
        let sql = r#"
-- instead of
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0);
        "#;
        assert_snapshot!(lint_errors_with(
            sql,
            LinterSettings {
                assume_in_transaction: true,
                ..Default::default()
            },
        ));
    }

    #[test]
    fn adding_check_constraint_ok() {
        let sql = r#"
-- use `NOT VALID`
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0) NOT VALID;
ALTER TABLE accounts VALIDATE CONSTRAINT positive_balance;
        "#;
        lint_ok(sql, Rule::ConstraintMissingNotValid);
    }

    #[test]
    fn new_table_with_transaction_ok() {
        let sql = r#"
BEGIN;
CREATE TABLE "core_foo" (
"id" serial NOT NULL PRIMARY KEY,
"age" integer NOT NULL
);
ALTER TABLE "core_foo" ADD CONSTRAINT "age_restriction" CHECK ("age" >= 25);
COMMIT;
        "#;
        lint_ok(sql, Rule::ConstraintMissingNotValid);
    }

    #[test]
    fn new_table_assume_transaction_ok() {
        let sql = r#"
CREATE TABLE "core_foo" (
"id" serial NOT NULL PRIMARY KEY,
"age" integer NOT NULL
);
ALTER TABLE "core_foo" ADD CONSTRAINT "age_restriction" CHECK ("age" >= 25);
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
    fn regression_with_indexing_ok() {
        let sql = r#"
CREATE TABLE "core_foo" (
"id" serial NOT NULL PRIMARY KEY,
"age" integer NOT NULL
);
ALTER TABLE "core_foo" ADD CONSTRAINT "age_restriction" CHECK ("age" >= 25);
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
    fn using_unique_index_ok() {
        let sql = r#"
ALTER TABLE "app_email" ADD CONSTRAINT "email_uniq" UNIQUE USING INDEX "email_idx";
        "#;
        lint_ok(sql, Rule::ConstraintMissingNotValid);
    }
}
