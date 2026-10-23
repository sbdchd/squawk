use insta::assert_snapshot;

use super::goto;

#[test]
fn text_search_dictionary_template() {
    assert_snapshot!(goto("
create text search template t (
  lexize = dsimple_lexize
);
create text search dictionary d (
  template = t$0
);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:29
      │
    2 │ create text search template t (
      │                             ─ 2. destination
      ‡
    6 │   template = t
      ╰╴             ─ 1. source
    ");
}

#[test]
fn text_search_configuration_copy_builtin() {
    assert_snapshot!(goto("
create text search configuration c (
  copy = english$0
);
"), @"
    info: goto definition
          ╭▸ file0.sql:3:10
          │
        3 │   copy = english
          │          ─────── 1. source
          │
          ⸬  builtins.sql:43435:45
          │
    43435 │ create text search configuration pg_catalog.english (
          ╰╴                                            ─────── 2. destination
    ");
}

#[test]
fn text_search_mapping_dictionary() {
    assert_snapshot!(goto("
create text search dictionary d (
  template = simple
);
create text search configuration c (
  copy = english
);
alter text search configuration c add mapping for asciiword with d$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:31
      │
    2 │ create text search dictionary d (
      │                               ─ 2. destination
      ‡
    8 │ alter text search configuration c add mapping for asciiword with d;
      ╰╴                                                                 ─ 1. source
    ");
}

#[test]
fn text_search_parser_function() {
    assert_snapshot!(goto("
create function ps(internal, int) returns internal
  language c
  as 'f';
create text search parser p (
  start = ps$0,
  gettoken = ps,
  end = ps,
  lextypes = ps
);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function ps(internal, int) returns internal
      │                 ── 2. destination
      ‡
    6 │   start = ps,
      ╰╴          ── 1. source
    ");
}

#[test]
fn drop_text_search_configuration_ref() {
    assert_snapshot!(goto(r#"
create text search configuration c (
  parser = "default"
);
drop text search configuration c$0;
"#), @"
    info: goto definition
      ╭▸ file0.sql:2:34
      │
    2 │ create text search configuration c (
      │                                  ─ 2. destination
      ‡
    5 │ drop text search configuration c;
      ╰╴                               ─ 1. source
    ");
}
