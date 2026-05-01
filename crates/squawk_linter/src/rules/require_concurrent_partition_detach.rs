use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Edit, Fix, Linter, Rule, Violation};

fn concurrently_fix(detach_partition: &ast::DetachPartition) -> Option<Fix> {
    let path = detach_partition.path()?;
    let at = path.syntax().text_range().end();
    let edit = Edit::insert(" concurrently", at);
    Some(Fix::new("Add `concurrently`", vec![edit]))
}

pub(crate) fn require_concurrent_partition_detach(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::AlterTable(alter_table) = stmt {
            for action in alter_table.actions() {
                if let ast::AlterTableAction::DetachPartition(detach_partition) = action {
                    if detach_partition.concurrently_token().is_none()
                        && detach_partition.finalize_token().is_none()
                    {
                        let fix = concurrently_fix(&detach_partition);
                        ctx.report(
                            Violation::for_node(
                                Rule::RequireConcurrentPartitionDetach,
                                "Detaching a partition requires an `ACCESS EXCLUSIVE` lock, which prevents reads and writes to the table.".into(),
                                detach_partition.syntax(),
                            )
                            .help("Detach the partition `CONCURRENTLY`.")
                            .fix(fix),
                        );
                    }
                }
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
        fix_sql(sql, Rule::RequireConcurrentPartitionDetach)
    }

    #[test]
    fn detach_partition_missing_concurrently_err() {
        let sql = "ALTER TABLE t DETACH PARTITION p;";
        assert_snapshot!(lint_errors(sql, Rule::RequireConcurrentPartitionDetach), @"
        warning[require-concurrent-partition-detach]: Detaching a partition requires an `ACCESS EXCLUSIVE` lock, which prevents reads and writes to the table.
          ╭▸ 
        1 │ ALTER TABLE t DETACH PARTITION p;
          │               ━━━━━━━━━━━━━━━━━━
          │
          ├ help: Detach the partition `CONCURRENTLY`.
          ╭╴
        1 │ ALTER TABLE t DETACH PARTITION p concurrently;
          ╰╴                                 ++++++++++++
        ");
    }

    #[test]
    fn detach_partition_concurrently_ok() {
        let sql = "ALTER TABLE t DETACH PARTITION p CONCURRENTLY;";
        lint_ok(sql, Rule::RequireConcurrentPartitionDetach);
    }

    #[test]
    fn detach_partition_finalize_ok() {
        let sql = "ALTER TABLE t DETACH PARTITION p FINALIZE;";
        lint_ok(sql, Rule::RequireConcurrentPartitionDetach);
    }

    #[test]
    fn fix_add_concurrently() {
        let sql = "ALTER TABLE t DETACH PARTITION p;";
        let result = fix(sql);
        assert_snapshot!(result, @"ALTER TABLE t DETACH PARTITION p concurrently;");
    }
}
