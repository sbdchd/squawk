use insta::assert_snapshot;

use super::check;

#[test]
fn transform_missing() {
    assert_snapshot!(check("
create language plpython3u;
create type t as (a int);
create function from_t(internal) returns internal language sql return null;
create function to_t(internal) returns t language sql return null;
create transform for t language plpython3u (
  from sql with function from_t(internal),
  to sql with function to_t(internal)
);
create transform for nope language plpython3u (
  from sql with function nope_from(internal),
  to sql with function to_t(int)
);
drop transform for t language plpython3u;
drop transform for nope language plpython3u;
drop transform if exists for nope language plpython3u;
"), @"
    error: type `nope` does not exist
       ╭▸ 
    10 │ create transform for nope language plpython3u (
       ╰╴                     ━━━━
    error: type `nope` does not exist
       ╭▸ 
    15 │ drop transform for nope language plpython3u;
       ╰╴                   ━━━━
    error: function `nope_from(internal)` does not exist
       ╭▸ 
    11 │   from sql with function nope_from(internal),
       ╰╴                         ━━━━━━━━━
    error: function `to_t(int4)` does not exist
       ╭▸ 
    12 │   to sql with function to_t(int)
       ╰╴                       ━━━━
    ");
}

#[test]
fn transform_created_and_dropped() {
    assert_snapshot!(check("
create language plpython3u;
create type t as (a int);
create function from_t(internal) returns internal language sql return null;
create function to_t(internal) returns t language sql return null;
create transform for t language plpython3u (from sql with function from_t(internal));
create transform for t language plpython3u (from sql with function from_t(internal));
create or replace transform for t language plpython3u (to sql with function to_t(internal));
comment on transform for t language plpython3u is 'c';
drop transform for t language plpython3u;
drop transform for t language plpython3u;
comment on transform for t language plpython3u is 'c';
begin;
create transform for t language plpython3u (from sql with function from_t(internal));
rollback;
drop transform for t language plpython3u;
create transform for t language plpython3u (from sql with function from_t(internal));
drop function from_t(internal) cascade;
drop transform for t language plpython3u;
"), @"
    error: transform for type `t` language `plpython3u` already exists
      ╭▸ 
    7 │ create transform for t language plpython3u (from sql with function from_t(internal));
      ╰╴                     ━
    error: transform for type `t` language `plpython3u` does not exist
       ╭▸ 
    11 │ drop transform for t language plpython3u;
       ╰╴                   ━
    error: transform for type `t` language `plpython3u` does not exist
       ╭▸ 
    12 │ comment on transform for t language plpython3u is 'c';
       ╰╴           ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    error: transform for type `t` language `plpython3u` does not exist
       ╭▸ 
    16 │ drop transform for t language plpython3u;
       ╰╴                   ━
    error: transform for type `t` language `plpython3u` does not exist
       ╭▸ 
    19 │ drop transform for t language plpython3u;
       ╰╴                   ━
    ");
}
