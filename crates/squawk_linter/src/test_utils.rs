use crate::{Edit, Linter, LinterSettings, Rule, Violation};
use annotate_snippets::{AnnotationKind, Level, Patch, Renderer, Snippet, renderer::DecorStyle};

fn lint(sql: &str, rule: Rule) -> Vec<Violation> {
    lint_settings(sql, Default::default(), rule)
}

pub(crate) fn lint_ok(sql: &str, rule: Rule) {
    let errors = lint(sql, rule);
    assert_eq!(
        errors.len(),
        0,
        "Expected no errors. Use `lint_errors` if you want ensure are errors."
    );
}

#[must_use]
pub(crate) fn lint_errors(sql: &str, rule: Rule) -> String {
    let errors = lint(sql, rule);
    assert_ne!(
        errors.len(),
        0,
        "Expected errors. Use `lint_ok` if you want to ensure there aren't errors."
    );
    format_violations(sql, &errors)
}

#[must_use]
fn lint_settings(sql: &str, settings: LinterSettings, rule: Rule) -> Vec<Violation> {
    let file = squawk_syntax::SourceFile::parse(sql);
    assert_eq!(file.errors().len(), 0);
    let mut linter = Linter::from([rule]);
    linter.settings = settings;
    linter.lint(&file, sql)
}

pub(crate) fn lint_ok_with(sql: &str, settings: LinterSettings, rule: Rule) {
    let errors = lint_settings(sql, settings, rule);
    assert_eq!(
        errors.len(),
        0,
        "Expected no errors. Use `lint_errors_with` if you want to ensure there are errors."
    );
}

#[must_use]
pub(crate) fn lint_errors_with(sql: &str, settings: LinterSettings, rule: Rule) -> String {
    let errors = lint_settings(sql, settings, rule);
    assert_ne!(
        errors.len(),
        0,
        "Expected errors. Use `lint_ok_with` if you want to ensure there aren't errors."
    );
    format_violations(sql, &errors)
}

pub(crate) fn fix_sql(sql: &str, rule: Rule) -> String {
    let errors = lint(sql, rule);
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

fn format_violations(sql: &str, violations: &[Violation]) -> String {
    let mut buf = String::new();
    let renderer = Renderer::plain().decor_style(DecorStyle::Unicode);

    for violation in violations {
        let start: usize = violation.text_range.start().into();
        let end: usize = violation.text_range.end().into();

        let snippet = Snippet::source(sql)
            .fold(true)
            .annotation(AnnotationKind::Primary.span(start..end));

        let code = format!("{}", violation.code);
        let mut group = Level::WARNING
            .primary_title(&violation.message)
            .id(&code)
            .element(snippet);

        if let Some(help) = &violation.help {
            group = group.element(Level::HELP.message(help));
        }

        if let Some(fix) = &violation.fix {
            let mut patch_snippet = Snippet::source(sql).fold(true);

            for edit in &fix.edits {
                let edit_start: usize = edit.text_range.start().into();
                let edit_end: usize = edit.text_range.end().into();
                let replacement = edit.text.as_deref().unwrap_or("");
                patch_snippet = patch_snippet.patch(Patch::new(edit_start..edit_end, replacement));
            }

            group = group.element(patch_snippet);
        }

        let rendered = renderer.render(&[group]).to_string();
        buf.push_str(&rendered);
        buf.push('\n');
    }

    buf
}
