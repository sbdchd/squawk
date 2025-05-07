use squawk_syntax::{
    ast::{self, AstNode, HasModuleItem},
    Parse, SourceFile,
};

use crate::{Linter, Rule, Violation};

pub(crate) fn transaction_nesting(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    let mut in_explicit_transaction = false;
    let assume_in_transaction_help = "Put migration statements in separate files to have them be in separate transactions or don't use the assume-in-transaction setting.";

    for item in file.items() {
        match item {
            ast::Item::Begin(_) => {
                if ctx.settings.assume_in_transaction {
                    ctx.report(Violation::new(
                        Rule::TransactionNesting,
                        "There is an existing transaction already in progress, managed by your migration tool.".to_string(),
                        item.syntax().text_range(),
                        assume_in_transaction_help.to_string()
                    ));
                } else if in_explicit_transaction {
                    ctx.report(Violation::new(
                        Rule::TransactionNesting,
                        "There is an existing transaction already in progress.".to_string(),
                        item.syntax().text_range(),
                        assume_in_transaction_help.to_string(),
                    ));
                }
                in_explicit_transaction = true;
            }
            ast::Item::Commit(_) | ast::Item::Rollback(_) => {
                if ctx.settings.assume_in_transaction {
                    ctx.report(Violation::new(
                        Rule::TransactionNesting,
                        "Attempting to end the transaction that is managed by your migration tool"
                            .to_string(),
                        item.syntax().text_range(),
                        assume_in_transaction_help.to_string(),
                    ));
                } else if !in_explicit_transaction {
                    ctx.report(Violation::new(
                        Rule::TransactionNesting,
                        "There is no transaction to `COMMIT` or `ROLLBACK`.".to_string(),
                        item.syntax().text_range(),
                        "`BEGIN` a transaction at an earlier point in the migration or remove this statement.".to_string()
                    ));
                }
                in_explicit_transaction = false;
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::{Linter, Rule};
    use squawk_syntax::SourceFile;

    #[test]
    fn begin_repeated_err() {
        let sql = r#"
BEGIN;
BEGIN;
SELECT 1;
COMMIT;
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::TransactionNesting]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn commit_repeated_err() {
        let sql = r#"
BEGIN;
SELECT 1;
COMMIT;
COMMIT;
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::TransactionNesting]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn commit_with_assume_in_transaction_err() {
        let sql = r#"
SELECT 1;
COMMIT;
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::TransactionNesting]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn rollback_with_assume_in_transaction_err() {
        let sql = r#"
SELECT 1;
-- Not sure why rollback would be used in a migration, but test for completeness
ROLLBACK;
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::TransactionNesting]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn begin_assume_transaction_err() {
        let sql = r#"
BEGIN;
BEGIN;
SELECT 1;
COMMIT;
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::TransactionNesting]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn no_nesting_ok() {
        let sql = r#"
BEGIN;
SELECT 1;
COMMIT;
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::TransactionNesting]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn no_nesting_repeated_ok() {
        let sql = r#"
BEGIN;
SELECT 1;
COMMIT;
-- This probably shouldn't be done in a migration. However, Squawk may be linting several
-- migrations that are concatentated, so don't raise a warning here.
BEGIN;
SELECT 2;
COMMIT;
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::TransactionNesting]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn no_nesting_with_assume_transaction_ok() {
        let sql = r#"
SELECT 1;
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::TransactionNesting]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }
}
