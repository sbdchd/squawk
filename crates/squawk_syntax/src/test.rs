// based on https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/parser/src/tests.rs
use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
use camino::{Utf8Path, Utf8PathBuf};
use dir_test::{Fixture, dir_test};
use insta::{assert_snapshot, with_settings};

use crate::{
    Parse, SourceFile, SyntaxKind, SyntaxNode,
    ast::{self, AstNode},
    plpgsql::Plpgsql,
    syntax_error::SyntaxError,
};

pub(crate) fn render_errors(sql: &str, errors: &[SyntaxError]) -> String {
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
    dir: "$CARGO_MANIFEST_DIR/../squawk_parser/tests/data/ok",
    glob: "*.sql",
)]
fn parser_ok_validation(fixture: Fixture<&str>) {
    let content = fixture.content();
    let absolute_fixture_path = Utf8Path::new(fixture.path());
    let test_name = absolute_fixture_path
        .file_name()
        .and_then(|x| x.strip_suffix(".sql"))
        .unwrap();

    let parse = SourceFile::parse(content);
    let errors = parse.errors();

    assert!(
        errors.is_empty(),
        "parser ok test `{test_name}` has syntax validation errors:\n{}",
        render_errors(content, &errors)
    );
}

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/../../postgres/regression_suite",
    glob: "*.sql",
)]
fn regression_suite_validation(fixture: Fixture<&str>) {
    let content = fixture.content();
    let absolute_fixture_path = Utf8Path::new(fixture.path());
    let test_name = absolute_fixture_path
        .file_name()
        .and_then(|x| x.strip_suffix(".sql"))
        .unwrap();

    if absolute_fixture_path.to_string().contains("psql") {
        return;
    }

    let parse = SourceFile::parse(content);
    let mut errors = parse.errors();

    if test_name == "errors" {
        assert!(
            !errors.is_empty(),
            "the errors.sql regression test must have validation errors"
        );
        return;
    }

    // strings.sql intentionally has a comment between string continuation literals
    if test_name == "strings" {
        errors.retain(|e| e.message() != "Comments between string literals are not allowed.");
    }

    assert!(
        errors.is_empty(),
        "regression test `{test_name}` has syntax validation errors:\n{}",
        render_errors(content, &errors)
    );
}

fn plpgsql_bodies(parse: &Parse<SourceFile>) -> Vec<Plpgsql> {
    parse
        .tree()
        .syntax()
        .descendants()
        .filter_map(|node| {
            ast::CreateFunction::cast(node.clone())
                .and_then(|it| it.plpgsql())
                .or_else(|| ast::CreateProcedure::cast(node.clone()).and_then(|it| it.plpgsql()))
                .or_else(|| ast::Do::cast(node).and_then(|it| it.plpgsql()))
        })
        .collect()
}

fn plpgsql_fixture(sql: &str) -> (String, Vec<SyntaxError>) {
    let parse = SourceFile::parse(sql);
    assert!(
        parse.errors().is_empty(),
        "plpgsql fixtures must be valid sql:\n{}",
        render_errors(sql, &parse.errors())
    );

    let bodies = plpgsql_bodies(&parse);
    assert!(!bodies.is_empty(), "no plpgsql bodies found");

    let mut buffer = String::new();
    let mut errors = vec![];
    for body in bodies {
        if !buffer.is_empty() {
            buffer.push_str("---\n");
        }
        buffer.push_str(&format!("{:#?}", body.syntax()));
        errors.extend(body.errors());
    }

    if !errors.is_empty() {
        buffer.push('\n');
        buffer.push_str(&render_errors(sql, &errors));
    }

    (buffer, errors)
}

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/../squawk_parser/tests/data/plpgsql/ok",
    glob: "*.sql",
)]
fn plpgsql_ok(fixture: Fixture<&str>) {
    let content = fixture.content();
    let input_file = Utf8Path::new(fixture.path());
    let test_name = input_file
        .file_name()
        .and_then(|x| x.strip_suffix(".sql"))
        .unwrap();

    let (buffer, errors) = plpgsql_fixture(content);

    with_settings!({
      omit_expression => true,
      input_file => input_file,
    }, {
      assert_snapshot!(format!("plpgsql_{test_name}_ok"), buffer);
    });

    assert!(
        errors.is_empty(),
        "tests defined in `plpgsql/ok` can't have parser errors."
    );
}

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/../squawk_parser/tests/data/plpgsql/err",
    glob: "*.sql",
)]
fn plpgsql_err(fixture: Fixture<&str>) {
    let content = fixture.content();
    let input_file = Utf8Path::new(fixture.path());
    let test_name = input_file
        .file_name()
        .and_then(|x| x.strip_suffix(".sql"))
        .unwrap();

    let (buffer, errors) = plpgsql_fixture(content);

    with_settings!({
      omit_expression => true,
      input_file => input_file,
    }, {
      assert_snapshot!(format!("plpgsql_{test_name}_err"), buffer);
    });

    assert!(
        !errors.is_empty(),
        "tests defined in `plpgsql/err` must have parser errors."
    );
}

fn token_counts(node: &SyntaxNode) -> (usize, usize) {
    let mut total = 0;
    let mut unparsed = 0;
    for token in node
        .descendants_with_tokens()
        .filter_map(|element| element.into_token())
        .filter(|token| !token.kind().is_trivia())
    {
        total += 1;
        if token
            .parent_ancestors()
            .any(|ancestor| ancestor.kind() == SyntaxKind::ERROR)
        {
            unparsed += 1;
        }
    }
    (total, unparsed)
}

#[test]
fn plpgsql_suite_score() {
    let root = Utf8Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut files = std::fs::read_dir(root.join("postgres/plpgsql"))
        .unwrap()
        .map(|entry| Utf8PathBuf::try_from(entry.unwrap().path()).unwrap())
        .filter(|path| path.extension() == Some("sql"))
        .collect::<Vec<_>>();
    files.push(root.join("postgres/regression_suite/plpgsql.sql"));
    files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    let row = |label: &str, counts: [usize; 4]| {
        let [bodies, tokens, unparsed, err] = counts;
        format!("{label:<25}{bodies:>8}{tokens:>8}{unparsed:>10}{err:>6}\n")
    };

    let mut table = format!(
        "{:<25}{:>8}{:>8}{:>10}{:>6}\n",
        "file", "bodies", "tokens", "unparsed", "err"
    );
    let mut totals = [0; 4];

    for path in &files {
        let content = std::fs::read_to_string(path).unwrap();
        let parse = SourceFile::parse(&content);
        let file_name = path.file_name().unwrap();

        assert!(
            parse.errors().is_empty(),
            "`{file_name}` must parse as sql, otherwise the tree can hide bodies:\n{}",
            render_errors(&content, &parse.errors())
        );

        let bodies = plpgsql_bodies(&parse);

        let mut counts = [bodies.len(), 0, 0, 0];
        for body in bodies {
            let (tokens, unparsed) = token_counts(&body.syntax());
            counts[1] += tokens;
            counts[2] += unparsed;
            counts[3] += body.errors().len();
        }

        table.push_str(&row(file_name, counts));
        for (total, count) in totals.iter_mut().zip(counts) {
            *total += count;
        }
    }

    table.push_str(&row("total", totals));

    assert_snapshot!(table, @"
    file                       bodies  tokens  unparsed   err
    plpgsql.sql                   254    9746       784    65
    plpgsql_array.sql              26     913         0     0
    plpgsql_cache.sql               2      59        10     1
    plpgsql_call.sql               45    1591         7     1
    plpgsql_control.sql            27    1358         0     0
    plpgsql_copy.sql                4      28         0     0
    plpgsql_domain.sql             23     293         0     0
    plpgsql_misc.sql               16     260        29     3
    plpgsql_record.sql             65    1930         0     0
    plpgsql_simple.sql              9     210        23     3
    plpgsql_transaction.sql        37    1191         0     0
    plpgsql_trap.sql                7     340        21     1
    plpgsql_trigger.sql             1      55         0     0
    plpgsql_varprops.sql           33     699         3     1
    total                         549   18673       877    75
    ");
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

#[test]
fn parse_errors_are_sorted_by_position() {
    let sql = "from t select 1 + ;";
    let parse = SourceFile::parse(sql);
    let rendered = parse
        .errors()
        .iter()
        .map(|syntax_error| {
            let range = syntax_error.range();
            let start: u32 = range.start().into();
            let end: u32 = range.end().into();
            format!("{start}..{end}: {}", syntax_error.message())
        })
        .collect::<Vec<_>>()
        .join("\n");

    assert_snapshot!(rendered, @r"
    0..6: Leading from clauses are not supported in Postgres
    17..17: expected an expression, found SEMICOLON
    ");
}
