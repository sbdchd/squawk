use squawk_syntax::{
    ast::{self, AstNode, HasModuleItem},
    Parse, SourceFile,
};

use crate::{Linter, Rule, Violation};

pub(crate) fn require_concurrent_index_deletion(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for item in file.items() {
        if let ast::Item::DropIndex(drop_index) = item {
            if drop_index.concurrently_token().is_none() {
                ctx.report(Violation::new(
                    Rule::RequireConcurrentIndexDeletion,
            "A normal `DROP INDEX` acquires an `ACCESS EXCLUSIVE` lock on the table, blocking other accesses until the index drop can complete.".into(),
                    drop_index.syntax().text_range(),
                    "Drop the index `CONCURRENTLY`.".to_string(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::{Linter, Rule};

    #[test]
    fn drop_index_missing_concurrently_err() {
        let sql = r#"
  -- instead of
  DROP INDEX IF EXISTS "field_name_idx";
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::RequireConcurrentIndexDeletion]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].code, Rule::RequireConcurrentIndexDeletion);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn drop_index_concurrently_ok() {
        let sql = r#"
DROP INDEX CONCURRENTLY IF EXISTS "field_name_idx";
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::RequireConcurrentIndexDeletion]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn regression_false_positive_drop_type_ok() {
        let sql = r#"
DROP INDEX CONCURRENTLY IF EXISTS "field_name_idx";
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::RequireConcurrentIndexDeletion]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn regression_false_positive_drop_table_ok() {
        let sql = r#"
DROP TABLE IF EXISTS some_table;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::RequireConcurrentIndexDeletion]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn regression_false_positive_drop_trigger_ok() {
        let sql = r#"
DROP TRIGGER IF EXISTS trigger on foo_table;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::RequireConcurrentIndexDeletion]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }
}
