use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Edit, Fix, Linter, Rule, Violation, identifier::Identifier};

use super::constraint_missing_not_valid::tables_created_in_transaction;

fn concurrently_fix(create_index: &ast::CreateIndex) -> Option<Fix> {
    let index_token = create_index.index_token()?;
    let at = index_token.text_range().end();
    let edit = Edit::insert(" concurrently", at);
    Some(Fix::new("Add `concurrently`", vec![edit]))
}

pub(crate) fn require_concurrent_index_creation(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    let tables_created = tables_created_in_transaction(ctx.settings.assume_in_transaction, &file);
    for stmt in file.stmts() {
        if let ast::Stmt::CreateIndex(create_index) = stmt {
            if let Some(table_name) = create_index
                .relation_name()
                .and_then(|x| x.path())
                .and_then(|x| x.segment())
                .and_then(|x| x.name_ref())
            {
                if create_index.concurrently_token().is_none()
                    && !tables_created.contains(&Identifier::new(&table_name.text()))
                {
                    let fix = concurrently_fix(&create_index);

                    ctx.report(Violation::for_node(
                        Rule::RequireConcurrentIndexCreation,
                "During normal index creation, table updates are blocked, but reads are still allowed.".into(),
                        create_index.syntax(),
                    )
                    .help("Use `concurrently` to avoid blocking writes.")
                    .fix(fix));
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use insta::{assert_debug_snapshot, assert_snapshot};

    use crate::{
        Rule,
        test_utils::{fix_sql, lint, lint_with_assume_in_transaction},
    };

    fn fix(sql: &str) -> String {
        fix_sql(sql, Rule::RequireConcurrentIndexCreation)
    }

    #[test]
    fn fix_add_concurrently_named_index() {
        assert_snapshot!(fix("CREATE INDEX i ON t (c);"), @"CREATE INDEX concurrently i ON t (c);");
    }

    #[test]
    fn fix_add_concurrently_unnamed_index() {
        assert_snapshot!(fix("
CREATE INDEX ON t (a);
"), @"CREATE INDEX concurrently ON t (a);");
    }

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
        let errors = lint(sql, Rule::RequireConcurrentIndexCreation);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn adding_index_concurrently_ok() {
        let sql = r#"
-- use CONCURRENTLY
CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
        "#;
        let errors = lint(sql, Rule::RequireConcurrentIndexCreation);
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
        let errors = lint(sql, Rule::RequireConcurrentIndexCreation);
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
        let errors = lint_with_assume_in_transaction(sql, Rule::RequireConcurrentIndexCreation);
        assert_eq!(errors.len(), 0);
    }
}
