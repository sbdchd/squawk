use squawk_syntax::{
    ast::{self, AstNode, HasModuleItem},
    Parse, SourceFile,
};

use crate::{text::trim_quotes, Linter, Rule, Violation};

use super::constraint_missing_not_valid::tables_created_in_transaction;

pub(crate) fn require_concurrent_index_creation(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    let tables_created = tables_created_in_transaction(ctx.settings.assume_in_transaction, &file);
    for item in file.items() {
        if let ast::Item::CreateIndex(create_index) = item {
            if let Some(table_name) = create_index
                .path()
                .and_then(|x| x.segment())
                .and_then(|x| x.name_ref())
            {
                if create_index.concurrently_token().is_none()
                    && !tables_created.contains(trim_quotes(table_name.text().as_str()))
                {
                    ctx.report(Violation::new(
                        Rule::RequireConcurrentIndexCreation,
                "During normal index creation, table updates are blocked, but reads are still allowed.".into(),
                        create_index.syntax().text_range(),
                        "Use `CONCURRENTLY` to avoid blocking writes.".to_string(),
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::{Linter, Rule};

    /// ```sql
    /// -- instead of
    /// CREATE INDEX "field_name_idx" ON "table_name" ("field_name");
    /// -- use CONCURRENTLY
    /// CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
    /// ```
    #[test]
    fn adding_index_non_concurrently_err() {
        let sql = r#"
-- instead of
CREATE INDEX "field_name_idx" ON "table_name" ("field_name");
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::RequireConcurrentIndexCreation]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn adding_index_concurrently_ok() {
        let sql = r#"
-- use CONCURRENTLY
CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::RequireConcurrentIndexCreation]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn new_table_ok() {
        let sql = r#"
BEGIN;
CREATE TABLE "core_foo" (
"id" serial NOT NULL PRIMARY KEY,
"tenant_id" integer NULL
);
CREATE INDEX "core_foo_tenant_id_4d397ef9" ON "core_foo" ("tenant_id");
COMMIT;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::RequireConcurrentIndexCreation]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn new_table_in_assume_transaction_ok() {
        let sql = r#"
CREATE TABLE "core_foo" (
"id" serial NOT NULL PRIMARY KEY,
"tenant_id" integer NULL
);
CREATE INDEX "core_foo_tenant_id_4d397ef9" ON "core_foo" ("tenant_id");
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::RequireConcurrentIndexCreation]);
        linter.settings.assume_in_transaction = true;
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }
}
