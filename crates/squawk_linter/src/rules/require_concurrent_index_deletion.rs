use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Edit, Fix, Linter, Rule, Violation};

fn concurrently_fix(drop_index: &ast::DropIndex) -> Option<Fix> {
    let index_token = drop_index.index_token()?;
    let at = index_token.text_range().end();
    let edit = Edit::insert(" concurrently", at);
    Some(Fix::new("Add `concurrently`", vec![edit]))
}

pub(crate) fn require_concurrent_index_deletion(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for stmt in file.stmts() {
        if let ast::Stmt::DropIndex(drop_index) = stmt {
            if drop_index.concurrently_token().is_none() {
                let fix = concurrently_fix(&drop_index);

                ctx.report(Violation::for_node(
                    Rule::RequireConcurrentIndexDeletion,
            "A normal `DROP INDEX` acquires an `ACCESS EXCLUSIVE` lock on the table, blocking other accesses until the index drop can complete.".into(),
                    drop_index.syntax(),
                ).help("Drop the index `CONCURRENTLY`.").fix(fix));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use insta::{assert_debug_snapshot, assert_snapshot};

    use crate::{
        Rule,
        test_utils::{fix_sql, lint},
    };

    fn fix(sql: &str) -> String {
        fix_sql(sql, Rule::RequireConcurrentIndexDeletion)
    }

    #[test]
    fn fix_add_concurrently_simple() {
        let sql = "drop index i;";
        let result = fix(sql);
        assert_snapshot!(result, @"drop index concurrently i;");
    }

    #[test]
    fn fix_add_concurrently_if_exists() {
        let sql = r#"DROP INDEX IF EXISTS "field_name_idx";"#;
        let result = fix(sql);
        assert_snapshot!(result, @r#"DROP INDEX concurrently IF EXISTS "field_name_idx";"#);
    }

    #[test]
    fn fix_add_concurrently_multiple_indexes() {
        let sql = r#"DROP INDEX "idx1", "idx2";"#;
        let result = fix(sql);
        assert_snapshot!(result, @r#"DROP INDEX concurrently "idx1", "idx2";"#);
    }

    #[test]
    fn drop_index_missing_concurrently_err() {
        let sql = r#"
  -- instead of
  DROP INDEX IF EXISTS "field_name_idx";
        "#;
        let errors = lint(sql, Rule::RequireConcurrentIndexDeletion);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, Rule::RequireConcurrentIndexDeletion);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn drop_index_concurrently_ok() {
        let sql = r#"
DROP INDEX CONCURRENTLY IF EXISTS "field_name_idx";
        "#;
        let errors = lint(sql, Rule::RequireConcurrentIndexDeletion);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn regression_false_positive_drop_type_ok() {
        let sql = r#"
DROP INDEX CONCURRENTLY IF EXISTS "field_name_idx";
        "#;
        let errors = lint(sql, Rule::RequireConcurrentIndexDeletion);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn regression_false_positive_drop_table_ok() {
        let sql = r#"
DROP TABLE IF EXISTS some_table;
        "#;
        let errors = lint(sql, Rule::RequireConcurrentIndexDeletion);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn regression_false_positive_drop_trigger_ok() {
        let sql = r#"
DROP TRIGGER IF EXISTS trigger on foo_table;
        "#;
        let errors = lint(sql, Rule::RequireConcurrentIndexDeletion);
        assert_eq!(errors.len(), 0);
    }
}
