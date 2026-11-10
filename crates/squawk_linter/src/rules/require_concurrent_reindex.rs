use squawk_diagnostics::{Edit, Fix};
use squawk_syntax::ast::{self, AstNode};

use crate::{Linter, Rule, Violation, transactions::TransactionState};

fn concurrently_fix(reindex: &ast::Reindex) -> Option<Fix> {
    if let Some(options) = reindex.reindex_option_list()
        && let Some(literal) = options.reindex_options().find_map(|option| match option {
            ast::ReindexOption::ReindexOptionConcurrently(option) => option.literal(),
            ast::ReindexOption::ReindexOptionTablespace(_)
            | ast::ReindexOption::ReindexOptionVerbose(_) => None,
        })
    {
        return Some(Fix::new(
            "Enable `concurrently`",
            vec![Edit::replace(literal.syntax().text_range(), "true")],
        ));
    }
    let kw_token = match reindex.reindex_target()? {
        ast::ReindexTarget::ReindexTargetDatabase(it) => it.database_token(),
        ast::ReindexTarget::ReindexTargetIndex(it) => it.index_token(),
        ast::ReindexTarget::ReindexTargetSchema(it) => it.schema_token(),
        ast::ReindexTarget::ReindexTargetSystem(it) => it.system_token(),
        ast::ReindexTarget::ReindexTargetTable(it) => it.table_token(),
    }?;
    let at = kw_token.text_range().end();
    let edit = Edit::insert(" concurrently", at);
    Some(Fix::new("Add `concurrently`", vec![edit]))
}

pub(crate) fn require_concurrent_reindex(ctx: &mut Linter, file: &ast::SourceFile) {
    let mut transaction = TransactionState::new(ctx.settings.assume_in_transaction);
    for stmt in file.stmts() {
        transaction.update(&stmt);
        if let ast::Stmt::Reindex(reindex) = stmt {
            // REINDEX SYSTEM does not support CONCURRENTLY
            if matches!(
                reindex.reindex_target(),
                Some(ast::ReindexTarget::ReindexTargetSystem(_))
            ) {
                continue;
            }
            if !reindex.is_concurrently() {
                let violation = Violation::for_node(
                    Rule::RequireConcurrentReindex,
                    "reindexing a table or index without `concurrently` blocks reads and writes"
                        .into(),
                    reindex.syntax(),
                );
                let violation = if transaction.inside_transaction() {
                    violation.help(
                        "use `concurrently` outside of a transaction to avoid blocking reads and writes",
                    )
                } else {
                    violation
                        .help("use `concurrently` to avoid blocking reads and writes")
                        .fix(concurrently_fix(&reindex))
                };
                ctx.report(violation);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::{
        Rule,
        test_utils::{fix_sql, lint_errors, lint_ok},
    };

    #[must_use]
    fn fix(sql: &str) -> String {
        fix_sql(sql, Rule::RequireConcurrentReindex)
    }

    #[test]
    fn reindex_table_no_concurrently_err() {
        let sql = "REINDEX TABLE foo;";
        assert_snapshot!(lint_errors(sql, Rule::RequireConcurrentReindex), @"
        warning[require-concurrent-reindex]: reindexing a table or index without `concurrently` blocks reads and writes
          ╭▸ 
        1 │ REINDEX TABLE foo;
          │ ━━━━━━━━━━━━━━━━━━
          │
          ├ help: use `concurrently` to avoid blocking reads and writes
          ╭╴
        1 │ REINDEX TABLE concurrently foo;
          ╰╴              ++++++++++++
        ");
    }

    #[test]
    fn reindex_index_no_concurrently_err() {
        let sql = "REINDEX INDEX foo;";
        assert_snapshot!(lint_errors(sql, Rule::RequireConcurrentReindex), @"
        warning[require-concurrent-reindex]: reindexing a table or index without `concurrently` blocks reads and writes
          ╭▸ 
        1 │ REINDEX INDEX foo;
          │ ━━━━━━━━━━━━━━━━━━
          │
          ├ help: use `concurrently` to avoid blocking reads and writes
          ╭╴
        1 │ REINDEX INDEX concurrently foo;
          ╰╴              ++++++++++++
        ");
    }

    #[test]
    fn reindex_schema_no_concurrently_err() {
        let sql = "REINDEX SCHEMA foo;";
        assert_snapshot!(lint_errors(sql, Rule::RequireConcurrentReindex), @"
        warning[require-concurrent-reindex]: reindexing a table or index without `concurrently` blocks reads and writes
          ╭▸ 
        1 │ REINDEX SCHEMA foo;
          │ ━━━━━━━━━━━━━━━━━━━
          │
          ├ help: use `concurrently` to avoid blocking reads and writes
          ╭╴
        1 │ REINDEX SCHEMA concurrently foo;
          ╰╴               ++++++++++++
        ");
    }

    #[test]
    fn reindex_database_no_concurrently_err() {
        let sql = "REINDEX DATABASE foo;";
        assert_snapshot!(lint_errors(sql, Rule::RequireConcurrentReindex), @"
        warning[require-concurrent-reindex]: reindexing a table or index without `concurrently` blocks reads and writes
          ╭▸ 
        1 │ REINDEX DATABASE foo;
          │ ━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: use `concurrently` to avoid blocking reads and writes
          ╭╴
        1 │ REINDEX DATABASE concurrently foo;
          ╰╴                 ++++++++++++
        ");
    }

    #[test]
    fn reindex_table_concurrently_ok() {
        let sql = "REINDEX TABLE CONCURRENTLY foo;";
        lint_ok(sql, Rule::RequireConcurrentReindex);
    }

    #[test]
    fn reindex_index_concurrently_ok() {
        let sql = "REINDEX INDEX CONCURRENTLY foo;";
        lint_ok(sql, Rule::RequireConcurrentReindex);
    }

    #[test]
    fn reindex_options_concurrently_ok() {
        let sql = "REINDEX (CONCURRENTLY) TABLE foo;";
        lint_ok(sql, Rule::RequireConcurrentReindex);
    }

    #[test]
    fn reindex_system_ok() {
        let sql = "REINDEX SYSTEM mydb;";
        lint_ok(sql, Rule::RequireConcurrentReindex);
    }

    #[test]
    fn fix_reindex_table() {
        let sql = "REINDEX TABLE foo;";
        assert_snapshot!(fix(sql), @"REINDEX TABLE concurrently foo;");
    }

    #[test]
    fn fix_reindex_index() {
        let sql = "REINDEX INDEX foo;";
        assert_snapshot!(fix(sql), @"REINDEX INDEX concurrently foo;");
    }

    #[test]
    fn in_transaction_err() {
        let sql = r#"
begin;
reindex index i;
commit;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RequireConcurrentReindex));
    }
}
