// based on https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/parser/src/tests.rs
use camino::Utf8Path;
use dir_test::{dir_test, Fixture};
use insta::{assert_snapshot, with_settings};

mod utils;

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/tests/data",
    glob: "{ok,err}/**/*.sql",
)]
fn parser(fixture: Fixture<&str>) {
    let content = fixture.content();
    let absolute_fixture_path = Utf8Path::new(fixture.path());
    let input_file = absolute_fixture_path;
    let test_name = absolute_fixture_path
        .file_name()
        .and_then(|x| x.strip_suffix(".sql"))
        .unwrap();

    let parent_dir = input_file.parent().and_then(|x| x.file_name()).unwrap();

    let (parsed, has_errors) = utils::parse_text(content);

    with_settings!({
      omit_expression => true,
      input_file => input_file,
    }, {
      assert_snapshot!(format!("{}_{}", test_name, parent_dir), parsed);
    });

    if parent_dir == "ok" {
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
    } else {
        assert_eq!(parent_dir, "err");
        assert!(
            has_errors,
            "tests defined in the `err` directory must have parser errors."
        );
    }
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
    }
}
