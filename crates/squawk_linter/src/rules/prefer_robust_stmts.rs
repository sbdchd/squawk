use std::collections::HashMap;

use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Edit, Fix, Linter, Rule, Violation, identifier::Identifier};

#[derive(PartialEq)]
enum Constraint {
    Dropped,
    Added,
}

pub(crate) fn prefer_robust_stmts(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    let mut inside_transaction = ctx.settings.assume_in_transaction;
    let mut constraint_names: HashMap<Identifier, Constraint> = HashMap::new();

    let mut total_stmts = 0;
    for _ in file.stmts() {
        total_stmts += 1;
        if total_stmts > 1 {
            break;
        }
    }
    if total_stmts <= 1 {
        // single stmts are fine
        return;
    }

    enum ActionErrorMessage {
        IfExists,
        IfNotExists,
        None,
    }

    for stmt in file.stmts() {
        match stmt {
            ast::Stmt::Begin(_) => {
                inside_transaction = true;
            }
            ast::Stmt::Commit(_) => {
                inside_transaction = false;
            }
            ast::Stmt::AlterTable(alter_table) => {
                for action in alter_table.actions() {
                    let (message_type, fix) = match &action {
                        ast::AlterTableAction::DropConstraint(drop_constraint) => {
                            if let Some(constraint_name) = drop_constraint.name_ref() {
                                constraint_names.insert(
                                    Identifier::new(constraint_name.text().as_str()),
                                    Constraint::Dropped,
                                );
                            }
                            if drop_constraint.if_exists().is_some() {
                                continue;
                            }

                            let fix = drop_constraint.constraint_token().map(|constraint_token| {
                                let at = constraint_token.text_range().end();
                                let edit = Edit::insert(" if exists", at);
                                Fix::new("Insert `if exists`", vec![edit])
                            });

                            (ActionErrorMessage::IfExists, fix)
                        }
                        ast::AlterTableAction::AddColumn(add_column) => {
                            if add_column.if_not_exists().is_some() {
                                continue;
                            }

                            let fix = add_column.column_token().map(|column_token| {
                                let at = column_token.text_range().end();
                                let edit = Edit::insert(" if not exists", at);
                                Fix::new("Insert `if not exists`", vec![edit])
                            });
                            (ActionErrorMessage::IfNotExists, fix)
                        }
                        ast::AlterTableAction::ValidateConstraint(validate_constraint) => {
                            if let Some(constraint_name) = validate_constraint.name_ref() {
                                if constraint_names
                                    .contains_key(&Identifier::new(constraint_name.text().as_str()))
                                {
                                    continue;
                                }
                            }
                            (ActionErrorMessage::None, None)
                        }
                        ast::AlterTableAction::AddConstraint(add_constraint) => {
                            let constraint = add_constraint.constraint();
                            if let Some(constraint_name) = constraint.and_then(|x| x.name()) {
                                let name_text = constraint_name.text();
                                let name = Identifier::new(name_text.as_str());
                                if let Some(constraint) = constraint_names.get_mut(&name) {
                                    if *constraint == Constraint::Dropped {
                                        *constraint = Constraint::Added;
                                        continue;
                                    }
                                }
                            }
                            (ActionErrorMessage::None, None)
                        }
                        ast::AlterTableAction::DropColumn(drop_column) => {
                            if drop_column.if_exists().is_some() {
                                continue;
                            }

                            let fix = drop_column.column_token().map(|column_token| {
                                let at = column_token.text_range().end();
                                let edit = Edit::insert(" if exists", at);
                                Fix::new("Insert `if exists`", vec![edit])
                            });
                            (ActionErrorMessage::IfExists, fix)
                        }
                        _ => (ActionErrorMessage::None, None),
                    };

                    if inside_transaction {
                        continue;
                    }

                    let message =  match message_type {
                        ActionErrorMessage::IfExists => {
                            "Missing `IF EXISTS`, the migration can't be rerun if it fails part way through.".to_string()
                        },
                        ActionErrorMessage::IfNotExists => {
                            "Missing `IF NOT EXISTS`, the migration can't be rerun if it fails part way through.".to_string()
                        },
                        ActionErrorMessage::None => {
                            "Missing transaction, the migration can't be rerun if it fails part way through.".to_string()
                        },
                    };

                    ctx.report(
                        Violation::for_node(Rule::PreferRobustStmts, message, action.syntax())
                            .fix(fix),
                    );
                }
            }
            ast::Stmt::CreateIndex(create_index)
                if create_index.if_not_exists().is_none()
                    && create_index.name().is_some()
                    && (create_index.concurrently_token().is_some() || !inside_transaction) =>
            {
                let fix = create_index.name().map(|name| {
                    let at = name.syntax().text_range().start();
                    let edit = Edit::insert("if not exists ", at);
                    Fix::new("Insert `if not exists`", vec![edit])
                });
                ctx.report(Violation::for_node(
                    Rule::PreferRobustStmts,
                    "Missing `IF NOT EXISTS`, the migration can't be rerun if it fails part way through.".into(),
                    create_index.syntax(),
                ).help("Use an explicit name for a concurrently created index").fix(fix));
            }
            ast::Stmt::CreateTable(create_table)
                if create_table.if_not_exists().is_none() && !inside_transaction =>
            {
                let fix = create_table.table_token().map(|table_token| {
                    let at = table_token.text_range().end();
                    let edit = Edit::insert(" if not exists", at);
                    Fix::new("Insert `if not exists`", vec![edit])
                });

                ctx.report(Violation::for_node(
                    Rule::PreferRobustStmts,
                    "Missing `IF NOT EXISTS`, the migration can't be rerun if it fails part way through.".into(),
                    create_table.syntax(),
                ).fix(fix));
            }
            ast::Stmt::DropIndex(drop_index)
                if drop_index.if_exists().is_none() && !inside_transaction =>
            {
                let fix = drop_index.paths().next().map(|first_index| {
                    let at = first_index.syntax().text_range().start();
                    let edit = Edit::insert("if exists ", at);
                    Fix::new("Insert `if exists`", vec![edit])
                });

                ctx.report(Violation::for_node(
                    Rule::PreferRobustStmts,
                    "Missing `IF EXISTS`, the migration can't be rerun if it fails part way through.".into(),
                    drop_index.syntax(),
                ).fix(fix));
            }
            ast::Stmt::DropTable(drop_table)
                if drop_table.if_exists().is_none() && !inside_transaction =>
            {
                let fix = drop_table.table_token().map(|table_token| {
                    let at = table_token.text_range().end();
                    let edit = Edit::insert(" if exists", at);
                    Fix::new("Insert `if exists`", vec![edit])
                });
                ctx.report(Violation::for_node(
                    Rule::PreferRobustStmts,
                    "Missing `IF EXISTS`, the migration can't be rerun if it fails part way through.".into(),
                    drop_table.syntax(),
                ).fix(fix));
            }
            ast::Stmt::DropType(drop_type)
                if drop_type.if_exists().is_none() && !inside_transaction =>
            {
                let fix = drop_type.type_token().map(|type_token| {
                    let at = type_token.text_range().end();
                    let edit = Edit::insert(" if exists", at);
                    Fix::new("Insert `if exists`", vec![edit])
                });

                ctx.report(Violation::for_node(
                    Rule::PreferRobustStmts,
                    "Missing `IF EXISTS`, the migration can't be rerun if it fails part way through.".into(),
                    drop_type.syntax(),
                ).fix(fix));
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use insta::{assert_debug_snapshot, assert_snapshot};

    use crate::{
        Rule,
        test_utils::{fix_sql, lint, lint_with_assume_in_transaction},
    };

    fn fix(sql: &str) -> String {
        fix_sql(sql, Rule::PreferRobustStmts)
    }

    #[test]
    fn fix_drop_type_if_exists() {
        assert_snapshot!(fix("
drop type t;
DROP TYPE f;
"), @r"
        drop type if exists t;
        DROP TYPE if exists f;
        ");
    }

    #[test]
    fn fix_drop_index_if_exists() {
        assert_snapshot!(fix("
drop index i;
DROP INDEX CONCURRENTLY idx;
"), @r"
        drop index if exists i;
        DROP INDEX CONCURRENTLY if exists idx;
        ");
    }

    #[test]
    fn fix_drop_table_if_exists() {
        assert_snapshot!(fix("
drop table t;
DROP TABLE users;
"), @r"
        drop table if exists t;
        DROP TABLE if exists users;
        ");
    }

    #[test]
    fn fix_create_index_if_not_exists() {
        assert_snapshot!(fix("
create index idx on table (col);
CREATE INDEX CONCURRENTLY idx2 ON users (email);
"), @r"
        create index if not exists idx on table (col);
        CREATE INDEX CONCURRENTLY if not exists idx2 ON users (email);
        ");
    }

    #[test]
    fn fix_create_table_if_not_exists() {
        assert_snapshot!(fix("
create table t (id int);
CREATE TABLE users (id serial, name text);
"), @r"
        create table if not exists t (id int);
        CREATE TABLE if not exists users (id serial, name text);
        ");
    }

    #[test]
    fn fix_alter_table_add_column_if_not_exists() {
        assert_snapshot!(fix("
alter table t add column c text;
ALTER TABLE users ADD COLUMN email text;
"), @r"
        alter table t add column if not exists c text;
        ALTER TABLE users ADD COLUMN if not exists email text;
        ");
    }

    #[test]
    fn fix_alter_table_drop_column_if_exists() {
        assert_snapshot!(fix("
alter table t drop column c;
ALTER TABLE users DROP COLUMN email;
"), @r"
        alter table t drop column if exists c;
        ALTER TABLE users DROP COLUMN if exists email;
        ");
    }

    #[test]
    fn fix_alter_table_drop_constraint_if_exists() {
        assert_snapshot!(fix("
alter table t drop constraint c;
ALTER TABLE users DROP CONSTRAINT pk_users;
"), @r"
        alter table t drop constraint if exists c;
        ALTER TABLE users DROP CONSTRAINT if exists pk_users;
        ");
    }

    #[test]
    fn drop_before_end_ok() {
        let sql = r#"
ALTER TABLE "app_email" DROP CONSTRAINT IF EXISTS "email_uniq";
ALTER TABLE "app_email" ADD CONSTRAINT "email_uniq" UNIQUE USING INDEX "email_idx";
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn drop_index_if_exists_ok() {
        let sql = r#"
select 1; -- so we don't skip checking
DROP INDEX CONCURRENTLY IF EXISTS "email_idx";
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn drop_before_add_foreign_key_ok() {
        let sql = r#"
ALTER TABLE "app_email" DROP CONSTRAINT IF EXISTS "fk_user";
ALTER TABLE "app_email" ADD CONSTRAINT "fk_user" FOREIGN KEY ("user_id") REFERENCES "app_user" ("id") DEFERRABLE INITIALLY DEFERRED NOT VALID;
ALTER TABLE "app_email" VALIDATE CONSTRAINT "fk_user";
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn prefer_robust_stmt_ok() {
        let sql = r#"
BEGIN;
ALTER TABLE "core_foo" ADD COLUMN "answer_id" integer NULL;
COMMIT;
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn prefer_robust_stmt_part_2_ok() {
        let sql = r#"
select 1; -- so we don't skip checking
ALTER TABLE "core_foo" ADD COLUMN IF NOT EXISTS "answer_id" integer NULL;
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn prefer_robust_stmt_part_3_ok() {
        let sql = r#"
select 1; -- so we don't skip checking
CREATE INDEX CONCURRENTLY IF NOT EXISTS "core_foo_idx" ON "core_foo" ("answer_id");
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn prefer_robust_stmt_part_4_ok() {
        let sql = r#"
BEGIN;
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "bravo" text NOT NULL
);
COMMIT;
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn prefer_robust_stmt_part_5_ok() {
        let sql = r#"
CREATE TABLE IF NOT EXISTS "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "bravo" text NOT NULL
);
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn prefer_robust_stmt_part_6_ok() {
        // If done in a transaction, most forms of drop are fine
        let sql = r#"
BEGIN;
DROP INDEX "core_bar_foo_id_idx";
DROP TABLE "core_bar";
DROP TYPE foo;
COMMIT;
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn select_ok() {
        // select is fine, we're only interested in modifications to the tables
        let sql = r#"
select 1; -- so we don't skip checking
SELECT 1;
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn insert_ok() {
        // select is fine, we're only interested in modifications to the tables
        let sql = r#"
select 1; -- so we don't skip checking
INSERT INTO tbl VALUES (a);
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn alter_table_ok() {
        // select is fine, we're only interested in modifications to the tables
        let sql = r#"
select 1; -- so we don't skip checking
ALTER TABLE "core_foo" DROP CONSTRAINT IF EXISTS "core_foo_idx";
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn assume_in_transaction_add_column_ok() {
        let sql = r#"
select 1; -- so we don't skip checking
ALTER TABLE "core_foo" ADD COLUMN "answer_id" integer NULL;
        "#;
        let errors = lint_with_assume_in_transaction(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn assume_in_transaction_drop_table_ok() {
        let sql = r#"
DROP INDEX "core_bar_foo_id_idx";
DROP TABLE "core_bar";
DROP TYPE foo;
        "#;
        let errors = lint_with_assume_in_transaction(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn assume_in_transaction_create_table_ok() {
        let sql = r#"
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "bravo" text NOT NULL
);
        "#;
        let errors = lint_with_assume_in_transaction(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn ignore_single_stmts_ok() {
        // we don't include a placeholder stmt because we're actually checking
        // for the single stmt behavior here
        let sql = r#"
CREATE INDEX CONCURRENTLY ON "table_name" ("field_name");
        "#;
        let errors = lint_with_assume_in_transaction(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn create_index_concurrently_without_name_ok() {
        let sql = r#"
CREATE INDEX CONCURRENTLY ON "table_name" ("field_name");
CREATE INDEX CONCURRENTLY ON "table_name" ("field_name");
        "#;
        let errors = lint_with_assume_in_transaction(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn start_transaction_ok() {
        let sql = r#"
START TRANSACTION;

ALTER TABLE "A" DROP CONSTRAINT "UQ_c4fb579a038211909ee524ccf29";

ALTER TABLE "B" DROP CONSTRAINT "UQ_791c01fe9438d66a94490d0da28";

ALTER TABLE "C" DROP CONSTRAINT "UQ_23fbf20e8ab4e806941359f4f79";

ALTER TABLE "D" DROP CONSTRAINT "UQ_468cad3743146a81c94b0b114ac";

COMMIT;
        "#;
        let errors = lint_with_assume_in_transaction(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn alter_table_err() {
        let sql = r#"
select 1; -- so we don't skip checking
ALTER TABLE "core_foo" ADD COLUMN "answer_id" integer NULL;
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn create_index_concurrently_err() {
        let sql = r#"
select 1; -- so we don't skip checking
CREATE INDEX CONCURRENTLY "core_foo_idx" ON "core_foo" ("answer_id");
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn alter_table_drop_column_err() {
        let sql = r#"
select 1; -- so we don't skip checking
alter table t drop column c cascade;
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn alter_table_drop_column_ok() {
        let sql = r#"
select 1; -- so we don't skip checking
alter table t drop column if exists c cascade;
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn create_table_err() {
        let sql = r#"
select 1; -- so we don't skip checking
CREATE TABLE "core_bar" ( "id" serial NOT NULL PRIMARY KEY, "bravo" text NOT NULL);
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn alter_table_drop_constraint_err() {
        let sql = r#"
select 1; -- so we don't skip checking
ALTER TABLE "core_foo" DROP CONSTRAINT "core_foo_idx";
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn create_index_concurrently_unnamed_ok() {
        let sql = r#"
select 1; -- so we don't skip checking
CREATE INDEX CONCURRENTLY ON "table_name" ("field_name");
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn enable_row_level_security_err() {
        let sql = r#"
CREATE TABLE IF NOT EXISTS test();
ALTER TABLE IF EXISTS test ENABLE ROW LEVEL SECURITY;
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn enable_row_level_security_without_exists_check_err() {
        let sql = r#"
CREATE TABLE IF NOT EXISTS test();
ALTER TABLE test ENABLE ROW LEVEL SECURITY;
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn disable_row_level_security_err() {
        let sql = r#"
CREATE TABLE IF NOT EXISTS test();
ALTER TABLE IF EXISTS test DISABLE ROW LEVEL SECURITY;
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn double_add_after_drop_err() {
        let sql = r#"
ALTER TABLE "app_email" DROP CONSTRAINT IF EXISTS "email_uniq";
ALTER TABLE "app_email" ADD CONSTRAINT "email_uniq" UNIQUE USING INDEX "email_idx";
-- this second add constraint should error because it's not robust
ALTER TABLE "app_email" ADD CONSTRAINT "email_uniq" UNIQUE USING INDEX "email_idx";
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn alter_column_set_not_null() {
        let sql = r#"
select 1; -- so we don't skip checking
alter table t alter column c set not null;
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn drop_index_err() {
        let sql = r#"
select 1; -- so we don't skip checking
DROP INDEX CONCURRENTLY "email_idx";
        "#;
        let errors = lint(sql, Rule::PreferRobustStmts);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }
}
