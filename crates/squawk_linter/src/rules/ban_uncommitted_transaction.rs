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
    use insta::assert_snapshot;

    use crate::{
        Rule,
        test_utils::{fix_sql, lint_errors, lint_ok},
    };

    #[test]
    fn uncommitted_transaction_err() {
        let sql = r#"
BEGIN;
CREATE TABLE users (id bigint);
        "#;
        assert_snapshot!(lint_errors(sql, Rule::BanUncommittedTransaction), @r"
        warning[ban-uncommitted-transaction]: Transaction never committed or rolled back.
          ╭▸ 
        2 │ BEGIN;
          │ ━━━━━
          │
          ├ help: Add a `COMMIT` or `ROLLBACK` statement to complete the transaction.
          ╭╴
        4 ± 
        5 + COMMIT;
          ╰╴
        ");
    }

    #[test]
    fn committed_transaction_ok() {
        let sql = r#"
BEGIN;
CREATE TABLE users (id bigint);
COMMIT;
        "#;
        lint_ok(sql, Rule::BanUncommittedTransaction);
    }

    #[test]
    fn rolled_back_transaction_ok() {
        let sql = r#"
BEGIN;
CREATE TABLE users (id bigint);
ROLLBACK;
        "#;
        lint_ok(sql, Rule::BanUncommittedTransaction);
    }

    #[test]
    fn no_transaction_ok() {
        let sql = r#"
CREATE TABLE users (id bigint);
        "#;
        lint_ok(sql, Rule::BanUncommittedTransaction);
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
        assert_snapshot!(lint_errors(sql, Rule::BanUncommittedTransaction), @r"
        warning[ban-uncommitted-transaction]: Transaction never committed or rolled back.
          ╭▸ 
        6 │ BEGIN;
          │ ━━━━━
          │
          ├ help: Add a `COMMIT` or `ROLLBACK` statement to complete the transaction.
          ╭╴
        8 ± 
        9 + COMMIT;
          ╰╴
        ");
    }

    #[test]
    fn start_transaction_uncommitted_err() {
        let sql = r#"
START TRANSACTION;
CREATE TABLE users (id bigint);
        "#;
        assert_snapshot!(lint_errors(sql, Rule::BanUncommittedTransaction), @r"
        warning[ban-uncommitted-transaction]: Transaction never committed or rolled back.
          ╭▸ 
        2 │ START TRANSACTION;
          │ ━━━━━━━━━━━━━━━━━
          │
          ├ help: Add a `COMMIT` or `ROLLBACK` statement to complete the transaction.
          ╭╴
        4 ± 
        5 + COMMIT;
          ╰╴
        ");
    }

    #[test]
    fn nested_begin_only_last_uncommitted_err() {
        let sql = r#"
BEGIN;
BEGIN;
COMMIT;
        "#;
        lint_ok(sql, Rule::BanUncommittedTransaction);
    }

    #[test]
    fn begin_work_uncommitted_err() {
        let sql = r#"
BEGIN WORK;
CREATE TABLE users (id bigint);
        "#;
        assert_snapshot!(lint_errors(sql, Rule::BanUncommittedTransaction), @r"
        warning[ban-uncommitted-transaction]: Transaction never committed or rolled back.
          ╭▸ 
        2 │ BEGIN WORK;
          │ ━━━━━━━━━━
          │
          ├ help: Add a `COMMIT` or `ROLLBACK` statement to complete the transaction.
          ╭╴
        4 ± 
        5 + COMMIT;
          ╰╴
        ");
    }

    #[test]
    fn fix_adds_commit() {
        assert_snapshot!(fix_sql(
            "
BEGIN;
CREATE TABLE users (id bigint);
        ",
            Rule::BanUncommittedTransaction,
        ), @r"
        BEGIN;
        CREATE TABLE users (id bigint);
                
        COMMIT;
        ");
    }

    #[test]
    fn fix_adds_commit_to_start_transaction() {
        assert_snapshot!(fix_sql(
            "START TRANSACTION;
CREATE TABLE posts (id bigint);",
            Rule::BanUncommittedTransaction,
        ), @r"START TRANSACTION;
CREATE TABLE posts (id bigint);
COMMIT;
");
    }
}
