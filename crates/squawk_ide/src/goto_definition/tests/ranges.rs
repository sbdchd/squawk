use insta::assert_snapshot;

use super::goto;

#[test]
fn multirange_type() {
    assert_snapshot!(goto("
create type floatrange as range (subtype = float8);
select '{}'::floatmultirange$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type floatrange as range (subtype = float8);
      │             ────────── 2. destination
    3 │ select '{}'::floatmultirange;
      ╰╴             ─────────────── 1. source
    ");
}

#[test]
fn multirange_type_name_string() {
    assert_snapshot!(goto(r#"
create type floatrange as range (
  subtype = float8,
  multirange_type_name = 'MyMulti'
);
select '{}'::"MyMulti"$0;
"#), @r#"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type floatrange as range (
      │             ────────── 2. destination
      ‡
    6 │ select '{}'::"MyMulti";
      ╰╴             ───────── 1. source
    "#);
}

#[test]
fn multirange_type_name_string_with_dot() {
    assert_snapshot!(goto(r#"
create type floatrange as range (
  subtype = float8,
  multirange_type_name = 's.m'
);
select '{}'::"s.m"$0;
"#), @r#"
    info: goto definition
      ╭▸ file0.sql:2:13
      │
    2 │ create type floatrange as range (
      │             ────────── 2. destination
      ‡
    6 │ select '{}'::"s.m";
      ╰╴             ───── 1. source
    "#);
}

#[test]
fn range_subtype_diff_ref() {
    assert_snapshot!(goto("
create function diff(text, text) returns float8
  language sql
  as 'select 1';
create function diff(int, int) returns float8
  language sql
  as 'select 1';
create type r as range (subtype = text, subtype_diff = diff$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function diff(text, text) returns float8
      │                 ──── 2. destination
      ‡
    8 │ create type r as range (subtype = text, subtype_diff = diff);
      ╰╴                                                       ──── 1. source
    ");
}

#[test]
fn range_canonical_ref() {
    assert_snapshot!(goto("
create type r;
create function canon(r) returns r
  language sql
  as 'select null';
create type r as range (subtype = int, canonical = canon$0);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:17
      │
    3 │ create function canon(r) returns r
      │                 ───── 2. destination
      ‡
    6 │ create type r as range (subtype = int, canonical = canon);
      ╰╴                                                   ───── 1. source
    ");
}
