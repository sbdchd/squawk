use insta::assert_snapshot;

use super::check;

#[test]
fn invalid_json_cast() {
    assert_snapshot!(check(r#"
select '{"a": 1,}'::json, ''::jsonb, '[1 2]'::json, '{"a": tru}'::jsonb;
"#), @r#"
    error: invalid input value for type `json`
      ╭▸ 
    2 │ select '{"a": 1,}'::json, ''::jsonb, '[1 2]'::json, '{"a": tru}'::jsonb;
      ╰╴       ━━━━━━━━━━━
    error: invalid input value for type `jsonb`
      ╭▸ 
    2 │ select '{"a": 1,}'::json, ''::jsonb, '[1 2]'::json, '{"a": tru}'::jsonb;
      ╰╴                          ━━
    error: invalid input value for type `json`
      ╭▸ 
    2 │ select '{"a": 1,}'::json, ''::jsonb, '[1 2]'::json, '{"a": tru}'::jsonb;
      ╰╴                                     ━━━━━━━
    error: invalid input value for type `jsonb`
      ╭▸ 
    2 │ select '{"a": 1,}'::json, ''::jsonb, '[1 2]'::json, '{"a": tru}'::jsonb;
      ╰╴                                                    ━━━━━━━━━━━━
    "#);
}

#[test]
fn jsonb_only_errors() {
    assert_snapshot!(check(r#"
select
  '["a", "\u0000"]'::jsonb,
  '{"a": "\ud800"}'::jsonb,
  '[1, 1e99999999999999999999]'::jsonb,
  '{"a": "\u0000", "b": "\ud800", "c": 1e99999999999999999999}'::json;
"#), @r#"
    error: unsupported Unicode escape `\u0000` for type `jsonb`
      ╭▸ 
    3 │   '["a", "\u0000"]'::jsonb,
      ╰╴          ━━━━━━
    error: invalid input value for type `jsonb`
      ╭▸ 
    4 │   '{"a": "\ud800"}'::jsonb,
      ╰╴          ━━━━━━
    error: value `1e99999999999999999999` is out of range for type `numeric`
      ╭▸ 
    5 │   '[1, 1e99999999999999999999]'::jsonb,
      ╰╴       ━━━━━━━━━━━━━━━━━━━━━━
    "#);
}

#[test]
fn jsonb_error_range_in_escaped_literal() {
    assert_snapshot!(check(r#"
select
  e'{"a": "\\u0000"}'::jsonb,
  '{"a": "it''s \u0000"}'::jsonb,
  $$["\u0000"]$$::jsonb;
"#), @r#"
    error: unsupported Unicode escape `\u0000` for type `jsonb`
      ╭▸ 
    3 │   e'{"a": "\\u0000"}'::jsonb,
      ╰╴           ━━━━━━━
    error: unsupported Unicode escape `\u0000` for type `jsonb`
      ╭▸ 
    4 │   '{"a": "it''s \u0000"}'::jsonb,
      ╰╴                ━━━━━━
    error: unsupported Unicode escape `\u0000` for type `jsonb`
      ╭▸ 
    5 │   $$["\u0000"]$$::jsonb;
      ╰╴      ━━━━━━
    "#);
}

#[test]
fn valid_jsons() {
    assert_snapshot!(check(r#"
select
  '{"a": [1, -0, 1.5e-3, 1E+2, true, false, null]}'::jsonb,
  '{"a": 1, "a": 2}'::jsonb,
  '"\ud83d\ude00"'::jsonb,
  '"\ud800"'::json;
"#), @"no diagnostics");
}

#[test]
fn json_column_literals() {
    assert_snapshot!(check(r#"
create domain d as jsonb;
create table t (
  a json,
  b d
);
insert into t values ('{', '["\u0000"]');
select *
from t
where b = '[1,]' or b in ('{}', 'x');
"#), @r#"
    error: invalid input value for type `json`
      ╭▸ 
    7 │ insert into t values ('{', '["\u0000"]');
      ╰╴                      ━━━
    error: unsupported Unicode escape `\u0000` for type `jsonb`
      ╭▸ 
    7 │ insert into t values ('{', '["\u0000"]');
      ╰╴                              ━━━━━━
    error: invalid input value for type `jsonb`
       ╭▸ 
    10 │ where b = '[1,]' or b in ('{}', 'x');
       ╰╴          ━━━━━━
    error: invalid input value for type `jsonb`
       ╭▸ 
    10 │ where b = '[1,]' or b in ('{}', 'x');
       ╰╴                                ━━━
    "#);
}
