use squawk_diagnostics::{Edit, Fix};
use squawk_rowan::TextRange;
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::transactions::TransactionState;
use crate::{Linter, Rule, Version, Violation};

fn vacuum_full_fix(vacuum: &ast::Vacuum) -> Option<Fix> {
    let mut tables = vacuum.table_and_columns_list()?.table_and_columnss();
    let table = tables.next()?;
    if tables.next().is_some() || table.column_ref_list().is_some() {
        return None;
    }
    let relation = table.table_relation_name()?;
    if relation.only_token().is_some() || relation.star_token().is_some() {
        return None;
    }
    let path = relation.table_name_ref()?.path_ref()?;
    let replacement = format!("repack (concurrently) {}", path.syntax());
    let range = vacuum.syntax().text_range();
    let end = vacuum
        .semicolon_token()
        .map_or(range.end(), |semicolon| semicolon.text_range().start());
    let edit = Edit::replace(TextRange::new(range.start(), end), replacement);
    Some(Fix::new("Replace with `repack (concurrently)`", vec![edit]))
}

fn cluster_fix(cluster: &ast::Cluster) -> Option<Fix> {
    let replacement = if let Some(legacy) = cluster.cluster_legacy() {
        let index = legacy.index_ref()?;
        let table = legacy.on_path()?.table_name_ref()?.path_ref()?;
        format!(
            "repack (concurrently) {} using index {}",
            table.syntax(),
            index.syntax()
        )
    } else {
        let table = cluster.table_name_ref()?.path_ref()?;
        if let Some(index_name) = cluster
            .cluster_using_index()
            .and_then(|using| using.index_ref())
        {
            format!(
                "repack (concurrently) {} using index {}",
                table.syntax(),
                index_name.syntax()
            )
        } else {
            format!("repack (concurrently) {}", table.syntax())
        }
    };
    let range = cluster.syntax().text_range();
    let end = cluster
        .semicolon_token()
        .map_or(range.end(), |semicolon| semicolon.text_range().start());
    let edit = Edit::replace(TextRange::new(range.start(), end), replacement);
    Some(Fix::new("Replace with `repack (concurrently)`", vec![edit]))
}

pub(crate) fn prefer_repack(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    if ctx.settings.pg_version < Version::new(19, None, None) {
        return;
    }
    let help = "use `repack` to rewrite the table without blocking reads and writes";
    let transaction_help = "use `repack (concurrently)` outside of a transaction to rewrite the table without blocking reads and writes";
    let mut transaction = TransactionState::new(ctx.settings.assume_in_transaction);
    for stmt in parse.tree().stmts() {
        transaction.update(&stmt);
        let in_transaction = transaction.inside_transaction();
        match stmt {
            ast::Stmt::Cluster(cluster) => {
                let violation = Violation::for_node(
                    Rule::PreferRepack,
                    "`cluster` requires an `access exclusive` lock which blocks reads and writes to the table".into(),
                    cluster.syntax(),
                );
                let violation = if in_transaction {
                    violation.help(transaction_help)
                } else {
                    violation.help(help).fix(cluster_fix(&cluster))
                };
                ctx.report(violation);
            }
            ast::Stmt::Vacuum(vacuum) => {
                if vacuum.is_full() {
                    let violation = Violation::for_node(
                        Rule::PreferRepack,
                        "`vacuum full` requires an `access exclusive` lock which blocks reads and writes to the table".into(),
                        vacuum.syntax(),
                    );
                    let violation = if in_transaction {
                        violation.help(transaction_help)
                    } else {
                        violation.help(help).fix(vacuum_full_fix(&vacuum))
                    };
                    ctx.report(violation);
                }
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::{
        LinterSettings, Rule,
        test_utils::{fix_sql_with, lint_errors_with, lint_ok, lint_ok_with},
    };

    fn pg19() -> LinterSettings {
        LinterSettings {
            pg_version: "19".parse().expect("Invalid PostgreSQL version"),
            ..Default::default()
        }
    }

    #[test]
    fn cluster_err() {
        let sql = "CLUSTER foo;";
        assert_snapshot!(lint_errors_with(sql, pg19(), Rule::PreferRepack), @"
        warning[prefer-repack]: `cluster` requires an `access exclusive` lock which blocks reads and writes to the table
          ╭▸ 
        1 │ CLUSTER foo;
          │ ━━━━━━━━━━━━
          │
          ├ help: use `repack` to rewrite the table without blocking reads and writes
          ╭╴
        1 - CLUSTER foo;
        1 + repack (concurrently) foo;
          ╰╴
        ");
    }

    #[test]
    fn cluster_no_path_err() {
        let sql = "CLUSTER;";
        assert_snapshot!(lint_errors_with(sql, pg19(), Rule::PreferRepack), @"
        warning[prefer-repack]: `cluster` requires an `access exclusive` lock which blocks reads and writes to the table
          ╭▸ 
        1 │ CLUSTER;
          │ ━━━━━━━━
          │
          ╰ help: use `repack` to rewrite the table without blocking reads and writes
        ");
    }

    #[test]
    fn vacuum_full_err() {
        let sql = "VACUUM FULL foo;";
        assert_snapshot!(lint_errors_with(sql, pg19(), Rule::PreferRepack), @"
        warning[prefer-repack]: `vacuum full` requires an `access exclusive` lock which blocks reads and writes to the table
          ╭▸ 
        1 │ VACUUM FULL foo;
          │ ━━━━━━━━━━━━━━━━
          │
          ├ help: use `repack` to rewrite the table without blocking reads and writes
          ╭╴
        1 - VACUUM FULL foo;
        1 + repack (concurrently) foo;
          ╰╴
        ");
    }

    #[test]
    fn vacuum_full_option_list_err() {
        let sql = "VACUUM (FULL) foo;";
        assert_snapshot!(lint_errors_with(sql, pg19(), Rule::PreferRepack), @"
        warning[prefer-repack]: `vacuum full` requires an `access exclusive` lock which blocks reads and writes to the table
          ╭▸ 
        1 │ VACUUM (FULL) foo;
          │ ━━━━━━━━━━━━━━━━━━
          │
          ├ help: use `repack` to rewrite the table without blocking reads and writes
          ╭╴
        1 - VACUUM (FULL) foo;
        1 + repack (concurrently) foo;
          ╰╴
        ");
    }

    #[test]
    fn vacuum_full_false_option_list_ok() {
        let sql = "VACUUM (FULL FALSE) foo;";
        lint_ok_with(sql, pg19(), Rule::PreferRepack);
    }

    #[test]
    fn cluster_below_pg19_ok() {
        let sql = "CLUSTER foo;";
        lint_ok(sql, Rule::PreferRepack);
    }

    #[test]
    fn vacuum_full_below_pg19_ok() {
        let sql = "VACUUM FULL foo;";
        lint_ok(sql, Rule::PreferRepack);
    }

    #[test]
    fn vacuum_no_full_ok() {
        let sql = "VACUUM foo;";
        lint_ok_with(sql, pg19(), Rule::PreferRepack);
    }

    #[test]
    fn vacuum_analyze_ok() {
        let sql = "VACUUM ANALYZE foo;";
        lint_ok_with(sql, pg19(), Rule::PreferRepack);
    }

    #[test]
    fn vacuum_freeze_ok() {
        let sql = "VACUUM FREEZE foo;";
        lint_ok_with(sql, pg19(), Rule::PreferRepack);
    }

    #[must_use]
    fn fix(sql: &str) -> String {
        fix_sql_with(sql, pg19(), Rule::PreferRepack)
    }

    #[test]
    fn fix_vacuum_full() {
        assert_snapshot!(fix("VACUUM FULL foo;"), @"repack (concurrently) foo;");
    }

    #[test]
    fn fix_vacuum_full_option_list() {
        assert_snapshot!(fix("VACUUM (FULL) foo;"), @"repack (concurrently) foo;");
    }

    #[test]
    fn fix_keeps_following_stmt() {
        assert_snapshot!(fix("cluster foo; -- c\nselect 1;"), @"
        repack (concurrently) foo; -- c
        select 1;
        ");
    }

    #[test]
    fn without_valid_fix_err() {
        let sql = "
vacuum full a, b;
vacuum (full) c (x);
vacuum full only d;
cluster;
";
        assert_snapshot!(lint_errors_with(sql, pg19(), Rule::PreferRepack), @"
        warning[prefer-repack]: `vacuum full` requires an `access exclusive` lock which blocks reads and writes to the table
          ╭▸ 
        2 │ vacuum full a, b;
          │ ━━━━━━━━━━━━━━━━━
          │
          ╰ help: use `repack` to rewrite the table without blocking reads and writes
        warning[prefer-repack]: `vacuum full` requires an `access exclusive` lock which blocks reads and writes to the table
          ╭▸ 
        3 │ vacuum (full) c (x);
          │ ━━━━━━━━━━━━━━━━━━━━
          │
          ╰ help: use `repack` to rewrite the table without blocking reads and writes
        warning[prefer-repack]: `vacuum full` requires an `access exclusive` lock which blocks reads and writes to the table
          ╭▸ 
        4 │ vacuum full only d;
          │ ━━━━━━━━━━━━━━━━━━━
          │
          ╰ help: use `repack` to rewrite the table without blocking reads and writes
        warning[prefer-repack]: `cluster` requires an `access exclusive` lock which blocks reads and writes to the table
          ╭▸ 
        5 │ cluster;
          │ ━━━━━━━━
          │
          ╰ help: use `repack` to rewrite the table without blocking reads and writes
        ");
    }

    #[test]
    fn in_transaction_err() {
        let sql = "
begin;
cluster foo;
vacuum full foo;
commit;
";
        assert_snapshot!(lint_errors_with(sql, pg19(), Rule::PreferRepack), @"
        warning[prefer-repack]: `cluster` requires an `access exclusive` lock which blocks reads and writes to the table
          ╭▸ 
        3 │ cluster foo;
          │ ━━━━━━━━━━━━
          │
          ╰ help: use `repack (concurrently)` outside of a transaction to rewrite the table without blocking reads and writes
        warning[prefer-repack]: `vacuum full` requires an `access exclusive` lock which blocks reads and writes to the table
          ╭▸ 
        4 │ vacuum full foo;
          │ ━━━━━━━━━━━━━━━━
          │
          ╰ help: use `repack (concurrently)` outside of a transaction to rewrite the table without blocking reads and writes
        ");
    }

    #[test]
    fn fix_cluster_no_index() {
        assert_snapshot!(fix("CLUSTER foo;"), @"repack (concurrently) foo;");
    }

    #[test]
    fn fix_cluster_with_index() {
        assert_snapshot!(fix("CLUSTER foo USING foo_pkey;"), @"repack (concurrently) foo using index foo_pkey;");
    }

    #[test]
    fn fix_cluster_legacy_on_syntax() {
        assert_snapshot!(fix("CLUSTER verbose foo_pkey ON foo;"), @"repack (concurrently) foo using index foo_pkey;");
    }
}
