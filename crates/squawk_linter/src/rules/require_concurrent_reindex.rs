use squawk_diagnostics::{Edit, Fix};
use squawk_rowan::TextRange;
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

fn concurrently_fix(reindex: &ast::Reindex) -> Option<Fix> {
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

pub(crate) fn concurrently_range(reindex: &ast::Reindex) -> Option<TextRange> {
    if let Some(token) = reindex
        .reindex_target()
        .and_then(|target| target.concurrently_token())
    {
        return Some(token.text_range());
    }
    reindex
        .reindex_option_list()?
        .reindex_options()
        .find_map(|option| match option {
            ast::ReindexOption::ReindexOptionConcurrently(concurrently) => {
                Some(concurrently.syntax().text_range())
            }
            ast::ReindexOption::ReindexOptionTablespace(_)
            | ast::ReindexOption::ReindexOptionVerbose(_) => None,
        })
}

pub(crate) fn require_concurrent_reindex(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::Reindex(reindex) = stmt {
            // REINDEX SYSTEM does not support CONCURRENTLY
            if matches!(
                reindex.reindex_target(),
                Some(ast::ReindexTarget::ReindexTargetSystem(_))
            ) {
                continue;
            }
            if concurrently_range(&reindex).is_none() {
                let fix = concurrently_fix(&reindex);
                ctx.report(
                    Violation::for_node(
                        Rule::RequireConcurrentReindex,
                        "reindexing a table or index without `concurrently` blocks reads and writes"
                            .into(),
                        reindex.syntax(),
                    )
                    .help("use `concurrently` to avoid blocking reads and writes")
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
}
