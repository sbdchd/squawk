use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Edit, Fix, Linter, Rule, Violation};

pub(crate) fn ban_uncommitted_transaction(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    let mut uncommitted_begin: Option<ast::Begin> = None;

    for stmt in file.stmts() {
        match stmt {
            ast::Stmt::Begin(begin) => {
                uncommitted_begin = Some(begin);
            }
            ast::Stmt::Commit(_) | ast::Stmt::Rollback(_) => {
                uncommitted_begin = None;
            }
            _ => (),
        }
    }

    if let Some(begin) = uncommitted_begin {
        let end_pos = file.syntax().text_range().end();
        let fix = Fix::new("Add COMMIT", vec![Edit::insert("\nCOMMIT;\n", end_pos)]);

        ctx.report(
            Violation::for_node(
                Rule::BanUncommittedTransaction,
                "Transaction never committed or rolled back.".to_string(),
                begin.syntax(),
            )
            .help("Add a `COMMIT` or `ROLLBACK` statement to complete the transaction.")
            .fix(Some(fix)),
        );
    }
}

#[cfg(test)]
mod test {
    use insta::{assert_debug_snapshot, assert_snapshot};

    use crate::{Linter, Rule, test_utils::fix_sql};
    use squawk_syntax::SourceFile;

    #[test]
    fn uncommitted_transaction_err() {
        let sql = r#"
BEGIN;
CREATE TABLE users (id bigint);
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanUncommittedTransaction]);
        let errors = linter.lint(&file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors, @r#"
        [
            Violation {
                code: BanUncommittedTransaction,
                message: "Transaction never committed or rolled back.",
                text_range: 1..6,
                help: Some(
                    "Add a `COMMIT` or `ROLLBACK` statement to complete the transaction.",
                ),
                fix: Some(
                    Fix {
                        title: "Add COMMIT",
                        edits: [
                            Edit {
                                text_range: 48..48,
                                text: Some(
                                    "\nCOMMIT;\n",
                                ),
                            },
                        ],
                    },
                ),
            },
        ]
        "#);
    }

    #[test]
    fn committed_transaction_ok() {
        let sql = r#"
BEGIN;
CREATE TABLE users (id bigint);
COMMIT;
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanUncommittedTransaction]);
        let errors = linter.lint(&file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn rolled_back_transaction_ok() {
        let sql = r#"
BEGIN;
CREATE TABLE users (id bigint);
ROLLBACK;
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanUncommittedTransaction]);
        let errors = linter.lint(&file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn no_transaction_ok() {
        let sql = r#"
CREATE TABLE users (id bigint);
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanUncommittedTransaction]);
        let errors = linter.lint(&file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn multiple_transactions_last_uncommitted_err() {
        let sql = r#"
BEGIN;
CREATE TABLE users (id bigint);
COMMIT;

BEGIN;
CREATE TABLE posts (id bigint);
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanUncommittedTransaction]);
        let errors = linter.lint(&file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors, @r#"
        [
            Violation {
                code: BanUncommittedTransaction,
                message: "Transaction never committed or rolled back.",
                text_range: 49..54,
                help: Some(
                    "Add a `COMMIT` or `ROLLBACK` statement to complete the transaction.",
                ),
                fix: Some(
                    Fix {
                        title: "Add COMMIT",
                        edits: [
                            Edit {
                                text_range: 96..96,
                                text: Some(
                                    "\nCOMMIT;\n",
                                ),
                            },
                        ],
                    },
                ),
            },
        ]
        "#);
    }

    #[test]
    fn start_transaction_uncommitted_err() {
        let sql = r#"
START TRANSACTION;
CREATE TABLE users (id bigint);
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanUncommittedTransaction]);
        let errors = linter.lint(&file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors, @r#"
        [
            Violation {
                code: BanUncommittedTransaction,
                message: "Transaction never committed or rolled back.",
                text_range: 1..18,
                help: Some(
                    "Add a `COMMIT` or `ROLLBACK` statement to complete the transaction.",
                ),
                fix: Some(
                    Fix {
                        title: "Add COMMIT",
                        edits: [
                            Edit {
                                text_range: 60..60,
                                text: Some(
                                    "\nCOMMIT;\n",
                                ),
                            },
                        ],
                    },
                ),
            },
        ]
        "#);
    }

    #[test]
    fn nested_begin_only_last_uncommitted_err() {
        let sql = r#"
BEGIN;
BEGIN;
COMMIT;
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanUncommittedTransaction]);
        let errors = linter.lint(&file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn begin_work_uncommitted_err() {
        let sql = r#"
BEGIN WORK;
CREATE TABLE users (id bigint);
        "#;
        let file = SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanUncommittedTransaction]);
        let errors = linter.lint(&file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors, @r#"
        [
            Violation {
                code: BanUncommittedTransaction,
                message: "Transaction never committed or rolled back.",
                text_range: 1..11,
                help: Some(
                    "Add a `COMMIT` or `ROLLBACK` statement to complete the transaction.",
                ),
                fix: Some(
                    Fix {
                        title: "Add COMMIT",
                        edits: [
                            Edit {
                                text_range: 53..53,
                                text: Some(
                                    "\nCOMMIT;\n",
                                ),
                            },
                        ],
                    },
                ),
            },
        ]
        "#);
    }

    #[test]
    fn fix_adds_commit() {
        let sql = r#"
BEGIN;
CREATE TABLE users (id bigint);
        "#;
        let fixed = fix_sql(sql, Rule::BanUncommittedTransaction);
        assert_snapshot!(fixed, @r"
        BEGIN;
        CREATE TABLE users (id bigint);
                
        COMMIT;
        ");
    }

    #[test]
    fn fix_adds_commit_to_start_transaction() {
        let sql = r#"START TRANSACTION;
CREATE TABLE posts (id bigint);"#;
        let fixed = fix_sql(sql, Rule::BanUncommittedTransaction);
        assert_snapshot!(fixed, @r"START TRANSACTION;
CREATE TABLE posts (id bigint);
COMMIT;
");
    }
}
