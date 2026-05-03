use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Edit, Fix, Linter, Rule, Version, Violation};

fn vacuum_full_fix(vacuum: &ast::Vacuum) -> Option<Fix> {
    let tables = vacuum.table_and_columns_list()?;
    let tables = tables.syntax();
    let replacement = format!("repack (concurrently) {tables}");
    let edit = Edit::replace(vacuum.syntax().text_range(), replacement);
    Some(Fix::new("Replace with `repack (concurrently)`", vec![edit]))
}

fn cluster_fix(cluster: &ast::Cluster) -> Option<Fix> {
    let replacement = if let Some(on_path) = cluster.on_path() {
        let index = cluster.path()?;
        let table = on_path.path()?;
        format!(
            "repack (concurrently) {} using index {}",
            table.syntax(),
            index.syntax()
        )
    } else if let Some(table) = cluster.path() {
        if let Some(index_name) = cluster.using_method().and_then(|method| method.name_ref()) {
            format!(
                "repack (concurrently) {} using index {}",
                table.syntax(),
                index_name.syntax()
            )
        } else {
            format!("repack (concurrently) {}", table.syntax())
        }
    } else {
        "repack (concurrently)".to_string()
    };
    let edit = Edit::replace(cluster.syntax().text_range(), replacement);
    Some(Fix::new("Replace with `repack (concurrently)`", vec![edit]))
}

pub(crate) fn prefer_repack(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    if ctx.settings.pg_version < Version::new(19, None, None) {
        return;
    }
    for stmt in parse.tree().stmts() {
        match stmt {
            ast::Stmt::Cluster(cluster) => {
                let fix = cluster_fix(&cluster);
                ctx.report(
                    Violation::for_node(
                        Rule::PreferRepack,
                        "`cluster` requires an `ACCESS EXCLUSIVE` lock which blocks reads and writes to the table.".into(),
                        cluster.syntax(),
                    )
                    .help("Use `repack` to rewrite the table without blocking reads and writes.")
                    .fix(fix),
                );
            }
            ast::Stmt::Vacuum(vacuum) => {
                if vacuum.is_full() {
                    let fix = vacuum_full_fix(&vacuum);
                    ctx.report(
                        Violation::for_node(
                            Rule::PreferRepack,
                            "`vacuum full` requires an `ACCESS EXCLUSIVE` lock which blocks reads and writes to the table.".into(),
                            vacuum.syntax(),
                        )
                        .help("Use `repack` to rewrite the table without blocking reads and writes.")
                        .fix(fix),
                    );
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
        warning[prefer-repack]: `cluster` requires an `ACCESS EXCLUSIVE` lock which blocks reads and writes to the table.
          ╭▸ 
        1 │ CLUSTER foo;
          │ ━━━━━━━━━━━
          │
          ├ help: Use `repack` to rewrite the table without blocking reads and writes.
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
        warning[prefer-repack]: `cluster` requires an `ACCESS EXCLUSIVE` lock which blocks reads and writes to the table.
          ╭▸ 
        1 │ CLUSTER;
          │ ━━━━━━━
          │
          ├ help: Use `repack` to rewrite the table without blocking reads and writes.
          ╭╴
        1 - CLUSTER;
        1 + repack (concurrently);
          ╰╴
        ");
    }

    #[test]
    fn vacuum_full_err() {
        let sql = "VACUUM FULL foo;";
        assert_snapshot!(lint_errors_with(sql, pg19(), Rule::PreferRepack), @"
        warning[prefer-repack]: `vacuum full` requires an `ACCESS EXCLUSIVE` lock which blocks reads and writes to the table.
          ╭▸ 
        1 │ VACUUM FULL foo;
          │ ━━━━━━━━━━━━━━━
          │
          ├ help: Use `repack` to rewrite the table without blocking reads and writes.
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
        warning[prefer-repack]: `vacuum full` requires an `ACCESS EXCLUSIVE` lock which blocks reads and writes to the table.
          ╭▸ 
        1 │ VACUUM (FULL) foo;
          │ ━━━━━━━━━━━━━━━━━
          │
          ├ help: Use `repack` to rewrite the table without blocking reads and writes.
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
    fn fix_vacuum_full_multiple_tables() {
        assert_snapshot!(fix("VACUUM FULL foo, bar;"), @"repack (concurrently) foo, bar;");
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

    #[test]
    fn fix_cluster_no_path() {
        assert_snapshot!(fix("CLUSTER;"), @"repack (concurrently);");
    }
}
