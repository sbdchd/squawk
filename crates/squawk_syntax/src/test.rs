// based on https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/parser/src/tests.rs
use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
use camino::Utf8Path;
use dir_test::{Fixture, dir_test};
use insta::{assert_snapshot, with_settings};

use crate::{SourceFile, syntax_error::SyntaxError};

fn render_errors(sql: &str, errors: &[SyntaxError]) -> String {
    let mut rendered = String::new();
    let renderer = Renderer::plain().decor_style(DecorStyle::Unicode);

    for syntax_error in errors {
        let range = syntax_error.range();
        let start: usize = range.start().into();
        let end: usize = range.end().into();
        let label = "syntax-error";

        let snippet = Snippet::source(sql)
            .fold(true)
            .annotation(AnnotationKind::Primary.span(start..end));

        let rendered_error = renderer
            .render(&[Level::ERROR
                .primary_title(syntax_error.message())
                .id(label)
                .element(snippet)])
            .to_string();

        rendered.push_str(&rendered_error);
        rendered.push('\n');
    }

    rendered
}

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/test_data",
    glob: "**/*.sql",
)]
fn syntaxtest(fixture: Fixture<&str>) {
    let content = fixture.content();
    let absolute_fixture_path = Utf8Path::new(fixture.path());
    let input_file = absolute_fixture_path;
    let test_name = absolute_fixture_path
        .file_name()
        .and_then(|x| x.strip_suffix(".sql"))
        .unwrap();

    let parent_dir = input_file.parent().and_then(|x| x.file_name()).unwrap();
    let parse = SourceFile::parse(content);
    let errors = parse.errors();
    let mut buffer = format!("{:#?}", parse.syntax_node());
    if !errors.is_empty() {
        buffer.push('\n');
        buffer.push_str(&render_errors(content, &errors));
    }

    with_settings!({
      omit_expression => true,
      input_file => input_file,
    }, {
      assert_snapshot!(format!("{}_{}", test_name, parent_dir), buffer);
    });

    if test_name.ends_with("_ok") {
        assert_eq!(
            errors.len(),
            0,
            "tests defined in the `syntax/test_data` ending with `_ok` can't have errors."
        );
    } else {
        assert_ne!(
            errors.len(),
            0,
            "tests defined in the `syntax/test_data` must have errors."
        );
    }
}
