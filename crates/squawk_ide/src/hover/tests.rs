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
        comment: None,
    };
    assert_eq!(hover.markdown(), "\n```sql\ncolumn public.t.a int4\n```\n");
}

#[test]
fn markdown_with_comment() {
    let hover = Hover {
        snippet: "column public.t.a int4".to_string(),
        comment: Some("Column docs".to_string()),
    };
    assert_eq!(
        hover.markdown(),
        "\n```sql\ncolumn public.t.a int4\n```\n---\nColumn docs\n",
    );
}

#[test]
fn unresolved_reference() {
    assert_snapshot!(hover("select missing$0;"), @"no hover");
}
