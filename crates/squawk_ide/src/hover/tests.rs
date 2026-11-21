mod columns;
mod comments;
mod ctes;
mod functions;
mod inheritance;
mod literals;
mod objects;
mod operators;
mod property_graphs;
mod select;
mod sessions;
mod stars;
mod tables;
mod types;

use insta::assert_snapshot;

use crate::hover::Hover;
use crate::test_utils::render_hover;

#[track_caller]
fn hover(sql: &str) -> String {
    hover_files(&[sql])
}

#[track_caller]
fn hover_files(files: &[&str]) -> String {
    render_hover(files).unwrap_or_else(|| "no hover".to_string())
}

#[test]
fn markdown() {
    let hover = Hover {
        snippet: "column public.t.a int4".to_string(),
        substitutions: None,
        comment: None,
    };
    assert_snapshot!(hover.markdown(), @"
    ```sql
    column public.t.a int4
    ```
    ");
}

#[test]
fn markdown_with_comment() {
    let hover = Hover {
        snippet: "column public.t.a int4".to_string(),
        substitutions: None,
        comment: Some("Column docs".to_string()),
    };
    assert_snapshot!(hover.markdown(), @"
    ```sql
    column public.t.a int4
    ```
    ---
    Column docs
    ");
}

#[test]
fn markdown_with_substitutions() {
    let hover = Hover {
        snippet: "function public.f(anyelement) returns anyelement".to_string(),
        substitutions: Some("`anyelement` = `int4`".to_string()),
        comment: Some("Function docs".to_string()),
    };
    assert_snapshot!(hover.markdown(), @"
    ```sql
    function public.f(anyelement) returns anyelement
    ```
    ---
    `anyelement` = `int4`
    ---
    Function docs
    ");
}

#[test]
fn unresolved_reference() {
    assert_snapshot!(hover("select missing$0;"), @"no hover");
}
