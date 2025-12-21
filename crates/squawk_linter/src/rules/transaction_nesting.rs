use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn transaction_nesting(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    let mut in_explicit_transaction = false;
    let assume_in_transaction_help = "Put migration statements in separate files to have them be in separate transactions or don't use the assume-in-transaction setting.";

    for stmt in file.stmts() {
        match stmt {
            ast::Stmt::Begin(_) => {
                if ctx.settings.assume_in_transaction {
                    ctx.report(Violation::for_node(
                        Rule::TransactionNesting,
                        "There is an existing transaction already in progress, managed by your migration tool.".to_string(),
                        stmt.syntax(),
                    ).help(assume_in_transaction_help));
                } else if in_explicit_transaction {
                    ctx.report(
                        Violation::for_node(
                            Rule::TransactionNesting,
                            "There is an existing transaction already in progress.".to_string(),
                            stmt.syntax(),
                        )
                        .help(assume_in_transaction_help),
                    );
                }
                in_explicit_transaction = true;
            }
            ast::Stmt::Commit(_) | ast::Stmt::Rollback(_) => {
                if ctx.settings.assume_in_transaction {
                    ctx.report(Violation::for_node(
                        Rule::TransactionNesting,
                        "Attempting to end the transaction that is managed by your migration tool"
                            .to_string(),
                        stmt.syntax(),
                    ).help(assume_in_transaction_help));
                } else if !in_explicit_transaction {
                    ctx.report(Violation::for_node(
                        Rule::TransactionNesting,
                        "There is no transaction to `COMMIT` or `ROLLBACK`.".to_string(),
                        stmt.syntax(),
                    ).help("`BEGIN` a transaction at an earlier point in the migration or remove this statement."));
                }
                in_explicit_transaction = false;
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::test_utils::{lint_errors, lint_ok};
    use crate::{LinterSettings, Rule};

    fn lint_errors_with(sql: &str, settings: LinterSettings) -> String {
        crate::test_utils::lint_errors_with(sql, settings, Rule::TransactionNesting)
    }

    fn lint_ok_with(sql: &str, settings: LinterSettings) {
        crate::test_utils::lint_ok_with(sql, settings, Rule::TransactionNesting);
    }

    #[test]
    fn begin_repeated_err() {
        let sql = r#"
BEGIN;
BEGIN;
SELECT 1;
COMMIT;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::TransactionNesting));
    }

    #[test]
    fn commit_repeated_err() {
        let sql = r#"
BEGIN;
SELECT 1;
COMMIT;
COMMIT;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::TransactionNesting));
    }

    #[test]
    fn commit_with_assume_in_transaction_err() {
        let sql = r#"
SELECT 1;
COMMIT;
        "#;
        let settings = LinterSettings {
            assume_in_transaction: true,
            ..Default::default()
        };
        assert_snapshot!(lint_errors_with(sql, settings));
    }

    #[test]
    fn rollback_with_assume_in_transaction_err() {
        let sql = r#"
SELECT 1;
-- Not sure why rollback would be used in a migration, but test for completeness
ROLLBACK;
        "#;
        let settings = LinterSettings {
            assume_in_transaction: true,
            ..Default::default()
        };
        assert_snapshot!(lint_errors_with(sql, settings));
    }

    #[test]
    fn begin_assume_transaction_err() {
        let sql = r#"
BEGIN;
BEGIN;
SELECT 1;
COMMIT;
        "#;
        let settings = LinterSettings {
            assume_in_transaction: true,
            ..Default::default()
        };
        assert_snapshot!(lint_errors_with(sql, settings));
    }

    #[test]
    fn no_nesting_ok() {
        let sql = r#"
BEGIN;
SELECT 1;
COMMIT;
        "#;
        lint_ok(sql, Rule::TransactionNesting);
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
        lint_ok(sql, Rule::TransactionNesting);
    }

    #[test]
    fn no_nesting_with_assume_transaction_ok() {
        let sql = r#"
SELECT 1;
        "#;
        let settings = LinterSettings {
            assume_in_transaction: true,
            ..Default::default()
        };
        lint_ok_with(sql, settings);
    }
}
