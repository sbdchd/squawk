use crate::{Linter, Rule, Violation};

pub(crate) fn lint(sql: &str, rule: Rule) -> Vec<Violation> {
    let file = squawk_syntax::SourceFile::parse(sql);
    assert_eq!(file.errors().len(), 0);
    let mut linter = Linter::from([rule]);
    linter.lint(file, sql)
}

pub(crate) fn lint_with_assume_in_transaction(sql: &str, rule: Rule) -> Vec<Violation> {
    let file = squawk_syntax::SourceFile::parse(sql);
    assert_eq!(file.errors().len(), 0);
    let mut linter = Linter::from([rule]);
    linter.settings.assume_in_transaction = true;
    linter.lint(file, sql)
}
