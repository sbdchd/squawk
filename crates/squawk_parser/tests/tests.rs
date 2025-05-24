// based on https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/parser/src/tests.rs
use camino::Utf8Path;
use dir_test::{dir_test, Fixture};
use insta::{assert_snapshot, with_settings};
use std::fs::remove_file;

mod utils;

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/tests/data/ok",
    glob: "*.sql",
)]
fn parser_ok(fixture: Fixture<&str>) {
    let content = fixture.content();
    let absolute_fixture_path = Utf8Path::new(fixture.path());
    let input_file = absolute_fixture_path;
    let test_name = absolute_fixture_path
        .file_name()
        .and_then(|x| x.strip_suffix(".sql"))
        .unwrap();

    let (parsed, has_errors) = utils::parse_text(content);

    with_settings!({
      omit_expression => true,
      input_file => input_file,
    }, {
      assert_snapshot!(format!("{}_ok", test_name), parsed);
    });

    // We check that all of our tests in `ok` also pass the Postgres parser,
    // if they don't, they should be moved to the `err` directory.
    assert!(
        !has_errors,
        "tests defined in the `ok` can't have parser errors."
    );
    // skipping pg17 specific stuff since our parser isn't using the latest parser
    if !test_name.ends_with("pg17") {
        let pg_result = pg_query::parse(content);
        if let Err(e) = &pg_result {
            assert!(
                &pg_result.is_ok(),
                "tests defined in the `ok` can't have Postgres parser errors. Found {}",
                e
            );
        }
    }
}

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/tests/data/err",
    glob: "*.sql",
)]
fn parser_err(fixture: Fixture<&str>) {
    let content = fixture.content();
    let absolute_fixture_path = Utf8Path::new(fixture.path());
    let input_file = absolute_fixture_path;
    let test_name = absolute_fixture_path
        .file_name()
        .and_then(|x| x.strip_suffix(".sql"))
        .unwrap();

    let (parsed, has_errors) = utils::parse_text(content);

    with_settings!({
      omit_expression => true,
      input_file => input_file,
    }, {
      assert_snapshot!(format!("{}_err", test_name), parsed);
    });

    assert!(
        has_errors,
        "tests defined in the `err` directory must have parser errors."
    );
}

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/tests/data/regression_suite",
    glob: "*.sql",
)]
#[dir_test_attr(
    #[ignore]
)]
fn regression_suite(fixture: Fixture<&str>) {
    let content = fixture.content();
    let absolute_fixture_path = Utf8Path::new(fixture.path());
    let input_file = absolute_fixture_path;
    let test_name = absolute_fixture_path
        .file_name()
        .and_then(|x| x.strip_suffix(".sql"))
        .unwrap();

    let (parsed, has_errors) = utils::parse_text(content);

    if has_errors {
        with_settings!({
          omit_expression => true,
          input_file => input_file,
          snapshot_path => "snapshots/regression_suite",
        }, {
          assert_snapshot!(test_name, parsed);
        });
    } else {
        let snapshot_path = Utf8Path::new("tests/snapshots/regression_suite")
            .join(format!("tests__{}.snap", test_name));
        let new_snapshot_path = Utf8Path::new("tests/snapshots/regression_suite")
            .join(format!("tests__{}.snap.new", test_name));

        let _ = remove_file(snapshot_path);
        let _ = remove_file(new_snapshot_path);
    }
}
