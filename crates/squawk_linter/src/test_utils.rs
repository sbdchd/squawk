use crate::{Edit, Linter, Rule, Violation};

pub(crate) fn lint(sql: &str, rule: Rule) -> Vec<Violation> {
    let file = squawk_syntax::SourceFile::parse(sql);
    assert_eq!(file.errors().len(), 0);
    let mut linter = Linter::from([rule]);
    linter.lint(&file, sql)
}

pub(crate) fn lint_with_postgres_version(
    sql: &str,
    rule: Rule,
    postgres_version: &str,
) -> Vec<Violation> {
    let file = squawk_syntax::SourceFile::parse(sql);
    assert_eq!(file.errors().len(), 0);
    let mut linter = Linter::from([rule]);
    linter.settings.pg_version = postgres_version
        .parse()
        .expect("Invalid PostgreSQL version");
    linter.lint(&file, sql)
}

pub(crate) fn lint_with_assume_in_transaction(sql: &str, rule: Rule) -> Vec<Violation> {
    let file = squawk_syntax::SourceFile::parse(sql);
    assert_eq!(file.errors().len(), 0);
    let mut linter = Linter::from([rule]);
    linter.settings.assume_in_transaction = true;
    linter.lint(&file, sql)
}

pub(crate) fn fix_sql(sql: &str, rule: Rule) -> String {
    let file = squawk_syntax::SourceFile::parse(sql);
    assert_eq!(file.errors().len(), 0, "Shouldn't start with syntax errors");
    let mut linter = Linter::from([rule]);
    let errors = linter.lint(&file, sql);
    assert!(!errors.is_empty(), "Should start with linter errors");

    let fixes = errors.into_iter().flat_map(|x| x.fix).collect::<Vec<_>>();

    let mut result = sql.to_string();

    let mut all_edits: Vec<&Edit> = fixes.iter().flat_map(|fix| &fix.edits).collect();

    all_edits.sort_by(|a, b| b.text_range.start().cmp(&a.text_range.start()));

    for edit in all_edits {
        let start: usize = edit.text_range.start().into();
        let end: usize = edit.text_range.end().into();
        let text = edit.text.as_ref().map_or("", |v| v);
        result.replace_range(start..end, text);
    }

    let file = squawk_syntax::SourceFile::parse(&result);
    assert_eq!(
        file.errors(),
        vec![],
        "Shouldn't introduce any syntax errors"
    );
    let mut linter = Linter::from([rule]);
    let errors = linter.lint(&file, &result);
    assert_eq!(
        errors.len(),
        0,
        "Fixes should remove all the linter errors."
    );

    result
}
