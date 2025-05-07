use std::collections::HashSet;

use squawk_syntax::{
    ast::{self, AstNode, HasModuleItem},
    Parse, SourceFile,
};

use crate::{text::trim_quotes, Linter, Rule, Violation};

pub fn tables_created_in_transaction(
    assume_in_transaction: bool,
    file: &ast::SourceFile,
) -> HashSet<String> {
    let mut created_table_names = HashSet::new();
    let mut inside_transaction = assume_in_transaction;
    for item in file.items() {
        match item {
            ast::Item::Begin(_) => {
                inside_transaction = true;
            }
            ast::Item::Commit(_) => {
                inside_transaction = false;
            }
            ast::Item::CreateTable(create_table) if inside_transaction => {
                let Some(table_name) = create_table
                    .path()
                    .and_then(|x| x.segment())
                    .and_then(|x| x.name())
                else {
                    continue;
                };
                created_table_names.insert(trim_quotes(&table_name.text()).to_string());
            }
            _ => (),
        }
    }
    created_table_names
}

fn not_valid_validate_in_transaction(
    ctx: &mut Linter,
    assume_in_transaction: bool,
    file: &ast::SourceFile,
) {
    let mut inside_transaction = assume_in_transaction;
    let mut not_valid_names: HashSet<String> = HashSet::new();
    for item in file.items() {
        match item {
            ast::Item::AlterTable(alter_table) => {
                for action in alter_table.actions() {
                    match action {
                        ast::AlterTableAction::ValidateConstraint(validate_constraint) => {
                            if let Some(constraint_name) =
                                validate_constraint.name_ref().map(|x| x.text().to_string())
                            {
                                if inside_transaction
                                    && not_valid_names.contains(trim_quotes(&constraint_name))
                                {
                                    ctx.report(
                                        Violation::new(
                                        Rule::ConstraintMissingNotValid,
                                        "Using `NOT VALID` and `VALIDATE CONSTRAINT` in the same transaction will block all reads while the constraint is validated.".into(),
                                        validate_constraint.syntax().text_range(),
                                        "Add constraint as `NOT VALID` in one transaction and `VALIDATE CONSTRAINT` in a separate transaction.".to_string(),
                                    ))
                                }
                            }
                        }
                        ast::AlterTableAction::AddConstraint(add_constraint) => {
                            if add_constraint.not_valid().is_some() {
                                if let Some(constraint) = add_constraint.constraint() {
                                    if let Some(constraint_name) = constraint.name() {
                                        not_valid_names.insert(
                                            trim_quotes(&constraint_name.text()).to_string(),
                                        );
                                    }
                                }
                            }
                        }
                        _ => (),
                    }
                }
            }
            ast::Item::Begin(_) => {
                if !inside_transaction {
                    not_valid_names.clear();
                }
                inside_transaction = true;
            }
            ast::Item::Commit(_) => {
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

    let tables_created = tables_created_in_transaction(assume_in_transaction, &file);

    for item in file.items() {
        if let ast::Item::AlterTable(alter_table) = item {
            let Some(table_name) = alter_table
                .path()
                .and_then(|x| x.segment())
                .and_then(|x| x.name_ref())
                .map(|x| x.text().to_string())
            else {
                continue;
            };
            for action in alter_table.actions() {
                if let ast::AlterTableAction::AddConstraint(add_constraint) = action {
                    if !tables_created.contains(trim_quotes(&table_name))
                        && add_constraint.not_valid().is_none()
                    {
                        if let Some(ast::Constraint::UniqueConstraint(uc)) =
                            add_constraint.constraint()
                        {
                            if uc.using_index().is_some() {
                                continue;
                            }
                        }

                        ctx.report(Violation::new(
                            Rule::ConstraintMissingNotValid,
                            "By default new constraints require a table scan and block writes to the table while that scan occurs.".into(),
                            add_constraint.syntax().text_range(),
                            "Use `NOT VALID` with a later `VALIDATE CONSTRAINT` call.".to_string(),
                        ));
                    }
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
    fn not_valid_validate_transaction_err() {
        let sql = r#"
BEGIN;
ALTER TABLE "app_email" ADD CONSTRAINT "fk_user" FOREIGN KEY (user_id) REFERENCES "app_user" (id) NOT VALID;
ALTER TABLE "app_email" VALIDATE CONSTRAINT "fk_user";
COMMIT;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::ConstraintMissingNotValid]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn not_valid_validate_assume_transaction_err() {
        let sql = r#"
ALTER TABLE "app_email" ADD CONSTRAINT "fk_user" FOREIGN KEY (user_id) REFERENCES "app_user" (id) NOT VALID;
ALTER TABLE "app_email" VALIDATE CONSTRAINT "fk_user";
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::ConstraintMissingNotValid]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn not_valid_validate_with_assume_in_transaction_with_explicit_commit_err() {
        let sql = r#"
ALTER TABLE "app_email" ADD CONSTRAINT "fk_user" FOREIGN KEY (user_id) REFERENCES "app_user" (id) NOT VALID;
ALTER TABLE "app_email" VALIDATE CONSTRAINT "fk_user";
COMMIT;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::ConstraintMissingNotValid]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn adding_fk_err() {
        let sql = r#"
-- instead of
ALTER TABLE distributors ADD CONSTRAINT distfk FOREIGN KEY (address) REFERENCES addresses (address);
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::ConstraintMissingNotValid]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn adding_fk_not_valid_ok() {
        let sql = r#"
-- use `NOT VALID`
ALTER TABLE distributors ADD CONSTRAINT distfk FOREIGN KEY (address) REFERENCES addresses (address) NOT VALID;
ALTER TABLE distributors VALIDATE CONSTRAINT distfk;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::ConstraintMissingNotValid]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn adding_check_constraint_err() {
        let sql = r#"
-- instead of
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0);
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::ConstraintMissingNotValid]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn adding_check_constraint_ok() {
        let sql = r#"
-- use `NOT VALID`
ALTER TABLE "accounts" ADD CONSTRAINT "positive_balance" CHECK ("balance" >= 0) NOT VALID;
ALTER TABLE accounts VALIDATE CONSTRAINT positive_balance;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::ConstraintMissingNotValid]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
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
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::ConstraintMissingNotValid]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
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
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::ConstraintMissingNotValid]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
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
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::ConstraintMissingNotValid]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn using_unique_index_ok() {
        let sql = r#"
ALTER TABLE "app_email" ADD CONSTRAINT "email_uniq" UNIQUE USING INDEX "email_idx";
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::ConstraintMissingNotValid]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }
}
