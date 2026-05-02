use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Edit, Fix, Linter, Rule, Violation};

fn concurrently_fix(reindex: &ast::Reindex) -> Option<Fix> {
    let kw_token = reindex
        .table_token()
        .or_else(|| reindex.index_token())
        .or_else(|| reindex.schema_token())
        .or_else(|| reindex.database_token())?;
    let at = kw_token.text_range().end();
    let edit = Edit::insert(" concurrently", at);
    Some(Fix::new("Add `concurrently`", vec![edit]))
}

pub(crate) fn require_concurrent_reindex(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::Reindex(reindex) = stmt {
            // REINDEX SYSTEM does not support CONCURRENTLY
            if reindex.system_token().is_some() {
                continue;
            }
            if reindex.concurrently_token().is_none() {
                let fix = concurrently_fix(&reindex);
                ctx.report(
                    Violation::for_node(
                        Rule::RequireConcurrentReindex,
                        "Reindexing a table or index without `concurrently` blocks reads and writes."
                            .into(),
                        reindex.syntax(),
                    )
                    .help("Use `concurrently` to avoid blocking reads and writes.")
                    .fix(fix),
                );
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

    fn fix(sql: &str) -> String {
        fix_sql(sql, Rule::RequireConcurrentReindex)
    }

    #[test]
    fn reindex_table_no_concurrently_err() {
        let sql = "REINDEX TABLE foo;";
        assert_snapshot!(lint_errors(sql, Rule::RequireConcurrentReindex), @"
        warning[require-concurrent-reindex]: Reindexing a table or index without `concurrently` blocks reads and writes.
          ╭▸ 
        1 │ REINDEX TABLE foo;
          │ ━━━━━━━━━━━━━━━━━
          │
          ├ help: Use `concurrently` to avoid blocking reads and writes.
          ╭╴
        1 │ REINDEX TABLE concurrently foo;
          ╰╴              ++++++++++++
        ");
    }

    #[test]
    fn reindex_index_no_concurrently_err() {
        let sql = "REINDEX INDEX foo;";
        assert_snapshot!(lint_errors(sql, Rule::RequireConcurrentReindex), @"
        warning[require-concurrent-reindex]: Reindexing a table or index without `concurrently` blocks reads and writes.
          ╭▸ 
        1 │ REINDEX INDEX foo;
          │ ━━━━━━━━━━━━━━━━━
          │
          ├ help: Use `concurrently` to avoid blocking reads and writes.
          ╭╴
        1 │ REINDEX INDEX concurrently foo;
          ╰╴              ++++++++++++
        ");
    }

    #[test]
    fn reindex_schema_no_concurrently_err() {
        let sql = "REINDEX SCHEMA foo;";
        assert_snapshot!(lint_errors(sql, Rule::RequireConcurrentReindex), @"
        warning[require-concurrent-reindex]: Reindexing a table or index without `concurrently` blocks reads and writes.
          ╭▸ 
        1 │ REINDEX SCHEMA foo;
          │ ━━━━━━━━━━━━━━━━━━
          │
          ├ help: Use `concurrently` to avoid blocking reads and writes.
          ╭╴
        1 │ REINDEX SCHEMA concurrently foo;
          ╰╴               ++++++++++++
        ");
    }

    #[test]
    fn reindex_database_no_concurrently_err() {
        let sql = "REINDEX DATABASE foo;";
        assert_snapshot!(lint_errors(sql, Rule::RequireConcurrentReindex), @"
        warning[require-concurrent-reindex]: Reindexing a table or index without `concurrently` blocks reads and writes.
          ╭▸ 
        1 │ REINDEX DATABASE foo;
          │ ━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Use `concurrently` to avoid blocking reads and writes.
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
}
