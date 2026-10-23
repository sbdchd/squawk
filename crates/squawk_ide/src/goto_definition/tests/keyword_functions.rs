use insta::assert_snapshot;

use super::goto;

#[test]
fn current_role_keyword() {
    assert_snapshot!(goto("
select current_role$0;
"), @"
    info: goto definition
         ╭▸ file0.sql:2:8
         │
       2 │ select current_role;
         │        ──────────── 1. source
         │
         ⸬  builtins.sql:9005:28
         │
    9005 │ create function pg_catalog.current_user() returns name
         ╰╴                           ──────────── 2. destination
    ");
}

#[test]
fn current_timestamp_keyword_ignores_column() {
    assert_snapshot!(goto("
with t as (
  select 1 current_timestamp
)
select current_timestamp$0 from t;
"), @"
    info: goto definition
          ╭▸ file0.sql:5:8
          │
        5 │ select current_timestamp from t;
          │        ───────────────── 1. source
          │
          ⸬  builtins.sql:19278:28
          │
    19278 │ create function pg_catalog.now() returns timestamp with time zone
          ╰╴                           ─── 2. destination
    ");
}

#[test]
fn current_user_keyword_ignores_search_path_function() {
    assert_snapshot!(goto("
create function public.current_user() returns name
  language sql
  as 'select 1';
select current_user$0;
"), @"
    info: goto definition
         ╭▸ file0.sql:5:8
         │
       5 │ select current_user;
         │        ──────────── 1. source
         │
         ⸬  builtins.sql:9005:28
         │
    9005 │ create function pg_catalog.current_user() returns name
         ╰╴                           ──────────── 2. destination
    ");
}

#[test]
fn at_time_zone_timestamptz() {
    assert_snapshot!(goto("
select now() at time$0 zone 'UTC';
"), @"
    info: goto definition
          ╭▸ file0.sql:2:14
          │
        2 │ select now() at time zone 'UTC';
          │              ──────────── 1. source
          │
          ⸬  builtins.sql:29016:28
          │
    29016 │ create function pg_catalog.timezone(
          ╰╴                           ──────── 2. destination
    ");
}

#[test]
fn at_local() {
    assert_snapshot!(goto("
select timestamp '2026-01-01' at local$0;
"), @"
    info: goto definition
          ╭▸ file0.sql:2:31
          │
        2 │ select timestamp '2026-01-01' at local;
          │                               ──────── 1. source
          │
          ⸬  builtins.sql:29054:28
          │
    29054 │ create function pg_catalog.timezone(
          ╰╴                           ──────── 2. destination
    ");
}

#[test]
fn collation_for() {
    assert_snapshot!(goto("
select collation for$0 ('x');
"), @r#"
    info: goto definition
          ╭▸ file0.sql:2:8
          │
        2 │ select collation for ('x');
          │        ───────────── 1. source
          │
          ⸬  builtins.sql:20707:28
          │
    20707 │ create function pg_catalog.pg_collation_for("any") returns text
          ╰╴                           ──────────────── 2. destination
    "#);
}

#[test]
fn extract_from_interval() {
    assert_snapshot!(goto("
select extract$0(epoch from interval '1 day');
"), @"
    info: goto definition
          ╭▸ file0.sql:2:8
          │
        2 │ select extract(epoch from interval '1 day');
          │        ─────── 1. source
          │
          ⸬  builtins.sql:10287:28
          │
    10287 │ create function pg_catalog.extract(text, interval) returns numeric
          ╰╴                           ─────── 2. destination
    ");
}

#[test]
fn is_not_normalized_with_form() {
    assert_snapshot!(goto("
select 'abc' is not nfkc normalized$0;
"), @"
    info: goto definition
          ╭▸ file0.sql:2:14
          │
        2 │ select 'abc' is not nfkc normalized;
          │              ────────────────────── 1. source
          │
          ⸬  builtins.sql:15717:28
          │
    15717 │ create function pg_catalog.is_normalized(
          ╰╴                           ───────────── 2. destination
    ");
}

#[test]
fn overlaps_timestamps() {
    assert_snapshot!(goto("
select (timestamp '2026-01-01', timestamp '2026-01-02') overlaps$0 row(
  timestamp '2026-01-02', interval '1 day'
);
"), @"
    info: goto definition
          ╭▸ file0.sql:2:57
          │
        2 │ select (timestamp '2026-01-01', timestamp '2026-01-02') overlaps row(
          │                                                         ──────── 1. source
          │
          ⸬  builtins.sql:20207:28
          │
    20207 │ create function pg_catalog.overlaps(
          ╰╴                           ──────── 2. destination
    ");
}

#[test]
fn substring_for_without_from() {
    assert_snapshot!(goto("
select substring$0('abc' for 2);
"), @"
    info: goto definition
          ╭▸ file0.sql:2:8
          │
        2 │ select substring('abc' for 2);
          │        ───────── 1. source
          │
          ⸬  builtins.sql:27222:28
          │
    27222 │ create function pg_catalog.substring(text, integer, integer) returns text
          ╰╴                           ───────── 2. destination
    ");
}

#[test]
fn substring_similar_escape() {
    assert_snapshot!(goto("
select substring$0('abc' similar 'a%' escape '#');
"), @"
    info: goto definition
          ╭▸ file0.sql:2:8
          │
        2 │ select substring('abc' similar 'a%' escape '#');
          │        ───────── 1. source
          │
          ⸬  builtins.sql:27236:28
          │
    27236 │ create function pg_catalog.substring(text, text, text) returns text
          ╰╴                           ───────── 2. destination
    ");
}

#[test]
fn overlay_with_for() {
    assert_snapshot!(goto("
select overlay$0('abcdef' placing 'xy' from 2 for 3);
"), @"
    info: goto definition
          ╭▸ file0.sql:2:8
          │
        2 │ select overlay('abcdef' placing 'xy' from 2 for 3);
          │        ─────── 1. source
          │
          ⸬  builtins.sql:20265:28
          │
    20265 │ create function pg_catalog.overlay(text, text, integer, integer) returns text
          ╰╴                           ─────── 2. destination
    ");
}

#[test]
fn trim_without_side_or_characters() {
    assert_snapshot!(goto("
select trim$0('  x  ');
"), @"
    info: goto definition
         ╭▸ file0.sql:2:8
         │
       2 │ select trim('  x  ');
         │        ──── 1. source
         │
         ⸬  builtins.sql:7850:28
         │
    7850 │ create function pg_catalog.btrim(text) returns text
         ╰╴                           ───── 2. destination
    ");
}

#[test]
fn position_bytea() {
    assert_snapshot!(goto("
select position$0('\\x01'::bytea in '\\x0102'::bytea);
"), @r"
    info: goto definition
          ╭▸ file0.sql:2:8
          │
        2 │ select position('\x01'::bytea in '\x0102'::bytea);
          │        ──────── 1. source
          │
          ⸬  builtins.sql:24796:28
          │
    24796 │ create function pg_catalog.position(bytea, bytea) returns integer
          ╰╴                           ──────── 2. destination
    ");
}

#[test]
fn xmlexists() {
    assert_snapshot!(goto("
create table t (doc xml);
select xmlexists$0('//town' passing by value doc) from t;
"), @"
    info: goto definition
          ╭▸ file0.sql:3:8
          │
        3 │ select xmlexists('//town' passing by value doc) from t;
          │        ───────── 1. source
          │
          ⸬  builtins.sql:31439:28
          │
    31439 │ create function pg_catalog.xmlexists(text, xml) returns boolean
          ╰╴                           ───────── 2. destination
    ");
}

#[test]
fn xmlexists_document_column() {
    assert_snapshot!(goto("
create table t (doc xml);
select xmlexists('//town' passing by value doc$0) from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (doc xml);
      │                 ─── 2. destination
    3 │ select xmlexists('//town' passing by value doc) from t;
      ╰╴                                           ─── 1. source
    ");
}
