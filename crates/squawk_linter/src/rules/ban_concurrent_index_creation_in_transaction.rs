use squawk_syntax::{Parse, SourceFile, ast};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_concurrent_index_creation_in_transaction(
    ctx: &mut Linter,
    parse: &Parse<SourceFile>,
) {
    let mut in_transaction = ctx.settings.assume_in_transaction;
    let file = parse.tree();
    let mut errors = vec![];
    let mut stmt_count = 0;
    for stmt in file.stmts() {
        stmt_count += 1;
        match stmt {
            ast::Stmt::Begin(_) => {
                in_transaction = true;
            }
            ast::Stmt::Commit(_) => {
                in_transaction = false;
            }
            ast::Stmt::CreateIndex(create_index) => {
                if in_transaction {
                    if let Some(concurrently) = create_index.concurrently_token() {
                        errors.push(Violation::for_range(
                            Rule::BanConcurrentIndexCreationInTransaction,
                            "While regular index creation can happen inside a transaction, this is not allowed when the `CONCURRENTLY` option is used.".into(),
                            concurrently.text_range(),
                        ).help("Build the index outside any transactions."));
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
}
