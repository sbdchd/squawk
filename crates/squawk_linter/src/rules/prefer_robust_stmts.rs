use std::collections::HashMap;

use squawk_syntax::{
    ast::{self, AstNode, HasIfExists, HasIfNotExists, HasModuleItem},
    Parse, SourceFile,
};

use crate::{text::trim_quotes, Linter, Rule, Violation};

#[derive(PartialEq)]
enum Constraint {
    Dropped,
    Added,
}

pub(crate) fn prefer_robust_stmts(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    let mut inside_transaction = ctx.settings.assume_in_transaction;
    let mut constraint_names: HashMap<String, Constraint> = HashMap::new();

    let mut total_stmts = 0;
    for _ in file.items() {
        total_stmts += 1;
        if total_stmts > 1 {
            break;
        }
    }
    if total_stmts <= 1 {
        // single stmts are fine
        return;
    }

    for item in file.items() {
        match item {
            ast::Item::Begin(_) => {
                inside_transaction = true;
            }
            ast::Item::Commit(_) => {
                inside_transaction = false;
            }
            ast::Item::AlterTable(alter_table) => {
                for action in alter_table.actions() {
                    match &action {
                        ast::AlterTableAction::DropConstraint(drop_constraint) => {
                            if let Some(constraint_name) = drop_constraint.name_ref() {
                                constraint_names.insert(
                                    trim_quotes(constraint_name.text().as_str()).to_string(),
                                    Constraint::Dropped,
                                );
                            }
                            if drop_constraint.if_exists().is_some() {
                                continue;
                            }
                        }
                        ast::AlterTableAction::AddColumn(add_column) => {
                            if add_column.if_not_exists().is_some() {
                                continue;
                            }
                        }
                        ast::AlterTableAction::ValidateConstraint(validate_constraint) => {
                            if let Some(constraint_name) = validate_constraint.name_ref() {
                                if constraint_names
                                    .contains_key(trim_quotes(constraint_name.text().as_str()))
                                {
                                    continue;
                                }
                            }
                        }
                        ast::AlterTableAction::AddConstraint(add_constraint) => {
                            let constraint = add_constraint.constraint();
                            if let Some(constraint_name) = constraint.and_then(|x| x.name()) {
                                let name_text = constraint_name.text();
                                let name = trim_quotes(name_text.as_str());
                                if let Some(constraint) = constraint_names.get_mut(name) {
                                    if *constraint == Constraint::Dropped {
                                        *constraint = Constraint::Added;
                                        continue;
                                    }
                                }
                            }
                        }
                        ast::AlterTableAction::DropColumn(drop_column) => {
                            if drop_column.if_exists().is_some() {
                                continue;
                            }
                        }
                        _ => (),
                    }

                    if inside_transaction {
                        continue;
                    }

                    ctx.report(Violation::new(
                        Rule::PreferRobustStmts,
                    "Missing `IF NOT EXISTS`, the migration can't be rerun if it fails part way through.".into(),
                        action.syntax().text_range(),
                        None,
                    ));
                }
            }
            ast::Item::CreateIndex(create_index)
                if create_index.if_not_exists().is_none()
                    && (create_index.concurrently_token().is_some() || !inside_transaction) =>
            {
                ctx.report(Violation::new(
                    Rule::PreferRobustStmts,
                    "Missing `IF NOT EXISTS`, the migration can't be rerun if it fails part way through.".into(),
                    create_index.syntax().text_range(),
                    "Use an explicit name for a concurrently created index".to_string(),
                ));
            }
            ast::Item::CreateTable(create_table)
                if create_table.if_not_exists().is_none() && !inside_transaction =>
            {
                ctx.report(Violation::new(
                    Rule::PreferRobustStmts,
                    "Missing `IF NOT EXISTS`, the migration can't be rerun if it fails part way through.".into(),
                    create_table.syntax().text_range(),
                    None,
                ));
            }
            ast::Item::DropIndex(drop_index)
                if drop_index.if_exists().is_none() && !inside_transaction =>
            {
                ctx.report(Violation::new(
                    Rule::PreferRobustStmts,
                    "Missing `IF NOT EXISTS`, the migration can't be rerun if it fails part way through.".into(),
                    drop_index.syntax().text_range(),
                    None,
                ));
            }
            ast::Item::DropTable(drop_table)
                if drop_table.if_exists().is_none() && !inside_transaction =>
            {
                ctx.report(Violation::new(
                    Rule::PreferRobustStmts,
                    "Missing `IF NOT EXISTS`, the migration can't be rerun if it fails part way through.".into(),
                    drop_table.syntax().text_range(),
                    None,
                ));
            }
            ast::Item::DropType(drop_type)
                if drop_type.if_exists().is_none() && !inside_transaction =>
            {
                ctx.report(Violation::new(
                    Rule::PreferRobustStmts,
                    "Missing `IF NOT EXISTS`, the migration can't be rerun if it fails part way through.".into(),
                    drop_type.syntax().text_range(),
                    None,
                ));
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::{Linter, Rule};

    #[test]
    fn drop_before_end_ok() {
        let sql = r#"
ALTER TABLE "app_email" DROP CONSTRAINT IF EXISTS "email_uniq";
ALTER TABLE "app_email" ADD CONSTRAINT "email_uniq" UNIQUE USING INDEX "email_idx";
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn drop_index_if_exists_ok() {
        let sql = r#"
select 1; -- so we don't skip checking
DROP INDEX CONCURRENTLY IF EXISTS "email_idx";
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn drop_before_add_foreign_key_ok() {
        let sql = r#"
ALTER TABLE "app_email" DROP CONSTRAINT IF EXISTS "fk_user";
ALTER TABLE "app_email" ADD CONSTRAINT "fk_user" FOREIGN KEY ("user_id") REFERENCES "app_user" ("id") DEFERRABLE INITIALLY DEFERRED NOT VALID;
ALTER TABLE "app_email" VALIDATE CONSTRAINT "fk_user";
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn prefer_robust_stmt_ok() {
        let sql = r#"
BEGIN;
ALTER TABLE "core_foo" ADD COLUMN "answer_id" integer NULL;
COMMIT;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn prefer_robust_stmt_part_2_ok() {
        let sql = r#"
select 1; -- so we don't skip checking
ALTER TABLE "core_foo" ADD COLUMN IF NOT EXISTS "answer_id" integer NULL;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn prefer_robust_stmt_part_3_ok() {
        let sql = r#"
select 1; -- so we don't skip checking
CREATE INDEX CONCURRENTLY IF NOT EXISTS "core_foo_idx" ON "core_foo" ("answer_id");
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
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
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
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
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
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
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn select_ok() {
        // select is fine, we're only interested in modifications to the tables
        let sql = r#"
select 1; -- so we don't skip checking
SELECT 1;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn insert_ok() {
        // select is fine, we're only interested in modifications to the tables
        let sql = r#"
select 1; -- so we don't skip checking
INSERT INTO tbl VALUES (a);
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn alter_table_ok() {
        // select is fine, we're only interested in modifications to the tables
        let sql = r#"
select 1; -- so we don't skip checking
ALTER TABLE "core_foo" DROP CONSTRAINT IF EXISTS "core_foo_idx";
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn assume_in_transaction_add_column_ok() {
        let sql = r#"
select 1; -- so we don't skip checking
ALTER TABLE "core_foo" ADD COLUMN "answer_id" integer NULL;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn assume_in_transaction_drop_table_ok() {
        let sql = r#"
DROP INDEX "core_bar_foo_id_idx";
DROP TABLE "core_bar";
DROP TYPE foo;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
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
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn ignore_single_stmts_ok() {
        // we don't include a placeholder stmt because we're actually checking
        // for the single stmt behavior here
        let sql = r#"
CREATE INDEX CONCURRENTLY ON "table_name" ("field_name");
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn create_index_concurrently_muli_stmts_err() {
        let sql = r#"
CREATE INDEX CONCURRENTLY ON "table_name" ("field_name");
CREATE INDEX CONCURRENTLY ON "table_name" ("field_name");
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
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
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn alter_table_err() {
        let sql = r#"
select 1; -- so we don't skip checking
ALTER TABLE "core_foo" ADD COLUMN "answer_id" integer NULL;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn create_index_concurrently_err() {
        let sql = r#"
select 1; -- so we don't skip checking
CREATE INDEX CONCURRENTLY "core_foo_idx" ON "core_foo" ("answer_id");
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn alter_table_drop_column_err() {
        let sql = r#"
select 1; -- so we don't skip checking
alter table t drop column c cascade;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn alter_table_drop_column_ok() {
        let sql = r#"
select 1; -- so we don't skip checking
alter table t drop column if exists c cascade;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn create_table_err() {
        let sql = r#"
select 1; -- so we don't skip checking
CREATE TABLE "core_bar" ( "id" serial NOT NULL PRIMARY KEY, "bravo" text NOT NULL);
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn alter_table_drop_constraint_err() {
        let sql = r#"
select 1; -- so we don't skip checking
ALTER TABLE "core_foo" DROP CONSTRAINT "core_foo_idx";
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn create_index_concurrently_unnamed_err() {
        let sql = r#"
select 1; -- so we don't skip checking
CREATE INDEX CONCURRENTLY ON "table_name" ("field_name");
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn enable_row_level_security_err() {
        let sql = r#"
CREATE TABLE IF NOT EXISTS test();
ALTER TABLE IF EXISTS test ENABLE ROW LEVEL SECURITY;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn enable_row_level_security_without_exists_check_err() {
        let sql = r#"
CREATE TABLE IF NOT EXISTS test();
ALTER TABLE test ENABLE ROW LEVEL SECURITY;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn disable_row_level_security_err() {
        let sql = r#"
CREATE TABLE IF NOT EXISTS test();
ALTER TABLE IF EXISTS test DISABLE ROW LEVEL SECURITY;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
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
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn drop_index_err() {
        let sql = r#"
select 1; -- so we don't skip checking
DROP INDEX CONCURRENTLY "email_idx";
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferRobustStmts]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }
}
