use squawk_syntax::{Parse, SourceFile, ast};

use crate::{Linter, Rule, Violation, transactions::TransactionState};

pub(crate) fn ban_concurrent_index_creation_in_transaction(
    ctx: &mut Linter,
    parse: &Parse<SourceFile>,
) {
    let mut transaction = TransactionState::new(ctx.settings.assume_in_transaction);
    let file = parse.tree();
    let mut errors = vec![];
    let mut stmt_count = 0;
    for stmt in file.stmts() {
        stmt_count += 1;
        transaction.update(&stmt);
        match stmt {
            ast::Stmt::CreateIndex(create_index) => {
                if transaction.inside_transaction()
                    && let Some(concurrently) = create_index.concurrently_token()
                {
                    errors.push(Violation::for_range(
                            Rule::BanConcurrentIndexCreationInTransaction,
                            "while regular index creation can happen inside a transaction, this is not allowed when the `concurrently` option is used".into(),
                            concurrently.text_range(),
                        ).help("build the index outside any transactions"));
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
}
