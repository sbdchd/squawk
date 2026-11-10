use squawk_syntax::ast::{self, AstNode};

use crate::{Linter, Rule, Violation, transactions::TransactionState};

pub(crate) fn ban_concurrent_index_creation_in_transaction(
    ctx: &mut Linter,
    file: &ast::SourceFile,
) {
    let mut transaction = TransactionState::new(ctx.settings.assume_in_transaction);
    let mut errors = vec![];
    let mut stmt_count = 0;
    for stmt in file.stmts() {
        stmt_count += 1;
        transaction.update(&stmt);
        if !transaction.inside_transaction() {
            continue;
        }
        match stmt {
            ast::Stmt::CreateIndex(create_index) => {
                if let Some(concurrently) = create_index.concurrently_token() {
                    errors.push(Violation::for_range(
                            Rule::BanConcurrentIndexCreationInTransaction,
                            "while regular index creation can happen inside a transaction, this is not allowed when the `concurrently` option is used".into(),
                            concurrently.text_range(),
                        ).help("build the index outside any transactions"));
                }
            }
            ast::Stmt::DropIndex(drop_index) => {
                if let Some(concurrently) = drop_index.concurrently_token() {
                    errors.push(
                        Violation::for_range(
                            Rule::BanConcurrentIndexCreationInTransaction,
                            "`drop index concurrently` can't run inside a transaction".into(),
                            concurrently.text_range(),
                        )
                        .help("drop the index outside any transactions"),
                    );
                }
            }
            ast::Stmt::Reindex(reindex) => {
                if let Some(concurrently) = reindex.concurrently_range() {
                    errors.push(
                        Violation::for_range(
                            Rule::BanConcurrentIndexCreationInTransaction,
                            "`reindex concurrently` can't run inside a transaction".into(),
                            concurrently,
                        )
                        .help("reindex outside any transactions"),
                    );
                }
            }
            ast::Stmt::AlterTable(alter_table) => {
                for action in alter_table.actions() {
                    if let ast::AlterTableAction::DetachPartition(detach_partition) = action
                        && let Some(ast::DetachPartitionOption::DetachConcurrently(concurrently)) =
                            detach_partition.detach_partition_option()
                    {
                        errors.push(
                            Violation::for_node(
                                Rule::BanConcurrentIndexCreationInTransaction,
                                "`detach partition concurrently` can't run inside a transaction"
                                    .into(),
                                concurrently.syntax(),
                            )
                            .help("detach the partition outside any transactions"),
                        );
                    }
                }
            }
            _ => (),
        }
    }
    if stmt_count > 1 {
        for error in errors {
            ctx.report(error);
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::test_utils::{lint_errors, lint_ok};
    use crate::{LinterSettings, Rule};

    fn lint_ok_with(sql: &str, settings: LinterSettings) {
        crate::test_utils::lint_ok_with(
            sql,
            settings,
            Rule::BanConcurrentIndexCreationInTransaction,
        );
    }

    #[must_use]
    fn lint_errors_with(sql: &str, settings: LinterSettings) -> String {
        crate::test_utils::lint_errors_with(
            sql,
            settings,
            Rule::BanConcurrentIndexCreationInTransaction,
        )
    }

    #[test]
    fn ban_concurrent_index_creation_in_transaction_err() {
        let sql = r#"
        -- instead of
        BEGIN;
        CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
        COMMIT;
        "#;
        assert_snapshot!(lint_errors(
            sql,
            Rule::BanConcurrentIndexCreationInTransaction
        ));
    }

    #[test]
    fn ban_concurrent_index_creation_in_transaction_ok() {
        let sql = r#"
  -- run outside a transaction
  CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
        "#;
        lint_ok(sql, Rule::BanConcurrentIndexCreationInTransaction);
    }

    #[test]
    fn assuming_in_transaction_err() {
        let sql = r#"
  -- instead of
  CREATE UNIQUE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
  ALTER TABLE "table_name" ADD CONSTRAINT "field_name_id" UNIQUE USING INDEX "field_name_idx";
    "#;
        assert_snapshot!(lint_errors_with(
            sql,
            LinterSettings {
                assume_in_transaction: true,
                ..Default::default()
            },
        ));
    }

    #[test]
    fn assuming_in_transaction_ok() {
        let sql = r#"
  -- run index creation in a standalone migration
  CREATE UNIQUE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
        "#;
        lint_ok_with(
            sql,
            LinterSettings {
                assume_in_transaction: true,
                ..Default::default()
            },
        );
    }

    #[test]
    fn assuming_in_transaction_ok_also() {
        let sql = r#"
  -- the following will work too
  COMMIT;
  CREATE UNIQUE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
  BEGIN;
  ALTER TABLE "table_name" ADD CONSTRAINT "field_name_id" UNIQUE USING INDEX "field_name_idx";
    "#;
        lint_ok_with(
            sql,
            LinterSettings {
                assume_in_transaction: true,
                ..Default::default()
            },
        );
    }

    #[test]
    fn squawk_disable_assume_in_transaction_overrides() {
        let sql = r#"
-- squawk-disable-assume-in-transaction
CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
ALTER TABLE "table_name" ADD CONSTRAINT "field_name_id" UNIQUE USING INDEX "field_name_idx";
    "#;
        lint_ok_with(
            sql,
            LinterSettings {
                assume_in_transaction: true,
                ..Default::default()
            },
        );
    }

    #[test]
    fn commit_and_chain_err() {
        let sql = r#"
begin;
create table t (id bigint);
commit and chain;
create index concurrently i on t (id);
commit;
        "#;
        assert_snapshot!(lint_errors(
            sql,
            Rule::BanConcurrentIndexCreationInTransaction
        ));
    }

    #[test]
    fn rollback_to_savepoint_err() {
        let sql = r#"
begin;
savepoint s;
rollback to savepoint s;
create index concurrently i on t (a);
commit;
        "#;
        assert_snapshot!(lint_errors(
            sql,
            Rule::BanConcurrentIndexCreationInTransaction
        ));
    }

    #[test]
    fn after_rollback_ok() {
        let sql = r#"
begin;
select 1;
rollback;
create index concurrently i on t (a);
        "#;
        lint_ok(sql, Rule::BanConcurrentIndexCreationInTransaction);
    }

    #[test]
    fn other_concurrent_stmts_in_transaction_err() {
        let sql = r#"
begin;
drop index concurrently i;
reindex index concurrently i;
reindex (concurrently) table t;
alter table t detach partition p concurrently;
commit;
        "#;
        assert_snapshot!(lint_errors(
            sql,
            Rule::BanConcurrentIndexCreationInTransaction
        ));
    }

    #[test]
    fn other_concurrent_stmts_outside_transaction_ok() {
        let sql = r#"
drop index concurrently i;
reindex index concurrently i;
alter table t detach partition p concurrently;
        "#;
        lint_ok(sql, Rule::BanConcurrentIndexCreationInTransaction);
    }
}
