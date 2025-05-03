// based on https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/parser/src/tests.rs
use camino::Utf8Path;
use dir_test::{dir_test, Fixture};
use insta::{assert_snapshot, with_settings};

use crate::SourceFile;

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
    let mut buffer = format!("{:#?}", parse.syntax_node());
    let errors = parse.errors();
    for syntax_error in &errors {
        let range = syntax_error.range();
        let text = syntax_error.message();
        // split into there own lines so that we can just grep
        // for error without hitting this part
        buffer += "\n";
        buffer += "ERROR";
        if range.start() == range.end() {
            buffer += &format!("@{:?} {:?}", range.start(), text);
        } else {
            buffer += &format!("@{:?}:{:?} {:?}", range.start(), range.end(), text);
        }
    }

    with_settings!({
      omit_expression => true,
      input_file => input_file,
    }, {
      assert_snapshot!(format!("{}_{}", test_name, parent_dir), buffer);
    });

    assert_ne!(
        errors.len(),
        0,
        "tests defined in the `syntax/test_data` must have errors."
    );
}
