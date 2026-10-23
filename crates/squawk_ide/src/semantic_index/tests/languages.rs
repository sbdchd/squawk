use insta::assert_snapshot;

use super::check;

#[test]
fn create_language() {
    assert_snapshot!(check("
create function h() returns language_handler language c as 'h';
create function i(internal) returns void language c as 'i';
create function v(oid) returns void language c as 'v';
create language l handler h inline i validator v;
create or replace procedural language l handler h;
create language l2 handler missing_h inline v validator i;
create language l;
create language sql;
"), @"
    error: function `missing_h` does not exist
      ╭▸ 
    7 │ create language l2 handler missing_h inline v validator i;
      ╰╴                           ━━━━━━━━━
    error: function `v` does not exist
      ╭▸ 
    7 │ create language l2 handler missing_h inline v validator i;
      ╰╴                                            ━
    error: function `i` does not exist
      ╭▸ 
    7 │ create language l2 handler missing_h inline v validator i;
      ╰╴                                                        ━
    error: language `l` already exists
      ╭▸ 
    8 │ create language l;
      ╰╴                ━
    error: language `sql` already exists
      ╭▸ 
    9 │ create language sql;
      ╰╴                ━━━
    ");
}

#[test]
fn function_language() {
    assert_snapshot!(check("
create function a() returns int language sql as 'select 1';
create function b() returns int language plpgsql as 'begin return 1; end';
create function c() returns int language 'sql' as 'select 1';
create function d() returns int language plpython3u as 'return 1';
create function e() returns int language 'SQL' as 'select 1';
create procedure p() language missing as '';
do $$ begin end $$;
do language plpgsql $$ begin end $$;
do language missing $$ begin end $$;
"), @"
    error: language `plpython3u` does not exist
      ╭▸ 
    5 │ create function d() returns int language plpython3u as 'return 1';
      ╰╴                                         ━━━━━━━━━━
    error: language `SQL` does not exist
      ╭▸ 
    6 │ create function e() returns int language 'SQL' as 'select 1';
      ╰╴                                         ━━━━━
    error: language `missing` does not exist
      ╭▸ 
    7 │ create procedure p() language missing as '';
      ╰╴                              ━━━━━━━
    error: language `missing` does not exist
       ╭▸ 
    10 │ do language missing $$ begin end $$;
       ╰╴            ━━━━━━━
    ");
}

#[test]
fn drop_language() {
    assert_snapshot!(check("
create language l;
drop language l;
drop language l;
drop procedural language if exists l;
create function f() returns int language l as '';
"), @"
    error: language `l` does not exist
      ╭▸ 
    4 │ drop language l;
      ╰╴              ━
    error: language `l` does not exist
      ╭▸ 
    6 │ create function f() returns int language l as '';
      ╰╴                                         ━
    ");
}

#[test]
fn language_rollback() {
    assert_snapshot!(check("
create language l;
begin;
drop language l;
create language l2;
rollback;
create function f() returns int language l as '';
create function g() returns int language l2 as '';
"), @"
    error: language `l2` does not exist
      ╭▸ 
    8 │ create function g() returns int language l2 as '';
      ╰╴                                         ━━
    ");
}

#[test]
fn alter_language() {
    assert_snapshot!(check("
create role r;
create language l;
create language l2;
alter language l owner to r;
alter language l owner to missing_role;
alter language missing owner to r;
alter language l rename to l2;
alter procedural language l rename to l3;
create function f() returns int language l3 as '';
create function g() returns int language l as '';
"), @"
    error: role `missing_role` does not exist
      ╭▸ 
    6 │ alter language l owner to missing_role;
      ╰╴                          ━━━━━━━━━━━━
    error: language `missing` does not exist
      ╭▸ 
    7 │ alter language missing owner to r;
      ╰╴               ━━━━━━━
    error: language `l` does not exist
       ╭▸ 
    11 │ create function g() returns int language l as '';
       ╰╴                                         ━
    error: language `l2` already exists
      ╭▸ 
    8 │ alter language l rename to l2;
      ╰╴                           ━━
    ");
}
