use squawk_syntax::{
    ast::{self, HasModuleItem},
    Parse, SourceFile,
};

use crate::{Linter, Rule, Violation};

pub(crate) fn ban_concurrent_index_creation_in_transaction(
    ctx: &mut Linter,
    parse: &Parse<SourceFile>,
) {
    let mut in_transaction = ctx.settings.assume_in_transaction;
    let file = parse.tree();
    let mut errors = vec![];
    let mut stmt_count = 0;
    for item in file.items() {
        stmt_count += 1;
        match item {
            ast::Item::Begin(_) => {
                in_transaction = true;
            }
            ast::Item::Commit(_) => {
                in_transaction = false;
            }
            ast::Item::CreateIndex(create_index) => {
                if in_transaction {
                    if let Some(concurrently) = create_index.concurrently_token() {
                        errors.push(Violation::new(
                            Rule::BanConcurrentIndexCreationInTransaction,
                            "While regular index creation can happen inside a transaction, this is not allowed when the `CONCURRENTLY` option is used.".into(),
                            concurrently.text_range(),
                            "Build the index outside any transactions.".to_string(),
                        ));
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
    use insta::assert_debug_snapshot;

    use crate::{Linter, Rule};

    #[test]
    fn ban_concurrent_index_creation_in_transaction_err() {
        let sql = r#"
        -- instead of
        BEGIN;
        CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
        COMMIT;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanConcurrentIndexCreationInTransaction]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn ban_concurrent_index_creation_in_transaction_ok() {
        let sql = r#"
  -- run outside a transaction
  CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanConcurrentIndexCreationInTransaction]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn assuming_in_transaction_err() {
        let sql = r#"
  -- instead of
  CREATE UNIQUE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
  ALTER TABLE "table_name" ADD CONSTRAINT "field_name_id" UNIQUE USING INDEX "field_name_idx";
    "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanConcurrentIndexCreationInTransaction]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn assuming_in_transaction_ok() {
        let sql = r#"
  -- run index creation in a standalone migration
  CREATE UNIQUE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanConcurrentIndexCreationInTransaction]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
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
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanConcurrentIndexCreationInTransaction]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }
}
