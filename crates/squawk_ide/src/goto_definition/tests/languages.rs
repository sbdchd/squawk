use insta::assert_snapshot;

use super::goto;

#[test]
fn function_language_builtin() {
    assert_snapshot!(goto("
create function f() returns int
  language sql$0
  as 'select 1';
"), @"
    info: goto definition
         ╭▸ file0.sql:3:12
         │
       3 │   language sql
         │            ─── 1. source
         │
         ⸬  builtins.sql:4939:17
         │
    4939 │ create language sql;
         ╰╴                ─── 2. destination
    ");
}

#[test]
fn function_language_plpgsql() {
    assert_snapshot!(goto("
create function f() returns int
  language plpgsql$0
  as 'begin return 1; end';
"), @"
    info: goto definition
          ╭▸ file0.sql:3:12
          │
        3 │   language plpgsql
          │            ─────── 1. source
          │
          ⸬  builtins.sql:31967:3
          │
    31967 │   plpgsql
          ╰╴  ─────── 2. destination
    ");
}

#[test]
fn function_language_user() {
    assert_snapshot!(goto("
create language l;
create function f() returns int
  language l$0
  as '';
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create language l;
      │                 ─ 2. destination
    3 │ create function f() returns int
    4 │   language l
      ╰╴           ─ 1. source
    ");
}

#[test]
fn function_language_literal() {
    assert_snapshot!(goto("
create language l;
create function f() returns int
  language 'l$0'
  as '';
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create language l;
      │                 ─ 2. destination
    3 │ create function f() returns int
    4 │   language 'l'
      ╰╴           ─── 1. source
    ");
}

#[test]
fn do_language() {
    assert_snapshot!(goto("
create language l;
do language l$0 'x';
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create language l;
      │                 ─ 2. destination
    3 │ do language l 'x';
      ╰╴            ─ 1. source
    ");
}

#[test]
fn create_language_handler() {
    assert_snapshot!(goto("
create function h() returns language_handler
  language c
  as 'h';
create language l handler h$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function h() returns language_handler
      │                 ─ 2. destination
      ‡
    5 │ create language l handler h;
      ╰╴                          ─ 1. source
    ");
}

#[test]
fn drop_language_ref() {
    assert_snapshot!(goto("
create language l;
drop language l$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create language l;
      │                 ─ 2. destination
    3 │ drop language l;
      ╰╴              ─ 1. source
    ");
}
