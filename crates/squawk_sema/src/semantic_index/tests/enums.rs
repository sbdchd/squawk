use insta::assert_snapshot;

use super::check;

#[test]
fn enum_label_values() {
    assert_snapshot!(check("
create type mood as enum ('happy', 'sad');
create domain moody as mood;
create table t (m mood, d moody, s text);
select 'sad'::mood, cast('happy' as mood), mood 'sad', null::mood;
select 'nope'::mood;
select * from t where m = 'sad' or 'happy' <> m or d = 'sad';
select * from t where m = 'nope' or d in ('happy', 'nope');
select * from t where m in ('sad') and m is distinct from 'meh';
select * from t where s = 'nope';
alter type mood rename value 'sad' to 'blue';
select 'sad'::mood, 'blue'::mood;
"), @"
    error: invalid input value for enum `mood`: `nope`
      ╭▸ 
    6 │ select 'nope'::mood;
      ╰╴       ━━━━━━
    error: invalid input value for enum `mood`: `nope`
      ╭▸ 
    8 │ select * from t where m = 'nope' or d in ('happy', 'nope');
      ╰╴                          ━━━━━━
    error: invalid input value for enum `mood`: `nope`
      ╭▸ 
    8 │ select * from t where m = 'nope' or d in ('happy', 'nope');
      ╰╴                                                   ━━━━━━
    error: invalid input value for enum `mood`: `meh`
      ╭▸ 
    9 │ select * from t where m in ('sad') and m is distinct from 'meh';
      ╰╴                                                          ━━━━━
    error: invalid input value for enum `mood`: `sad`
       ╭▸ 
    12 │ select 'sad'::mood, 'blue'::mood;
       ╰╴       ━━━━━
    ");
}

#[test]
fn alter_type_enum_values() {
    assert_snapshot!(check("
create type e as enum ('a', 'b');
alter type e add value 'c' after 'b';
alter type e add value if not exists 'c';
alter type e rename value 'a' to 'z';
"), @"no diagnostics");
}

#[test]
fn enum_labels() {
    assert_snapshot!(check("
create type mood as enum ('sad', 'ok', 'sad');
alter type mood add value 'happy' after 'ok';
alter type mood add value 'meh' before 'nope';
alter type mood add value 'ok';
alter type mood add value if not exists 'ok';
alter type mood rename value 'sad' to 'blue';
alter type mood rename value 'sad' to 'grey';
alter type mood rename value 'blue' to 'ok';
alter type mood add value 'x' before 'blue';
create type c as (a int);
alter type c add value 'x';
alter type c rename value 'a' to 'b';
begin;
alter type mood add value 'temp';
alter type mood rename value 'ok' to 'fine';
rollback;
alter type mood add value 'a' after 'temp';
alter type mood add value 'b' after 'fine';
alter type mood add value 'c' after 'ok';
"), @"
    error: enum label `sad` already exists
      ╭▸ 
    2 │ create type mood as enum ('sad', 'ok', 'sad');
      ╰╴                                       ━━━━━
    error: enum label `nope` does not exist
      ╭▸ 
    4 │ alter type mood add value 'meh' before 'nope';
      ╰╴                                       ━━━━━━
    error: enum label `ok` already exists
      ╭▸ 
    5 │ alter type mood add value 'ok';
      ╰╴                          ━━━━
    error: enum label `sad` does not exist
      ╭▸ 
    8 │ alter type mood rename value 'sad' to 'grey';
      ╰╴                             ━━━━━
    error: enum label `ok` already exists
      ╭▸ 
    9 │ alter type mood rename value 'blue' to 'ok';
      ╰╴                                       ━━━━
    error: `c` is not an enum
       ╭▸ 
    12 │ alter type c add value 'x';
       ╰╴           ━
    error: `c` is not an enum
       ╭▸ 
    13 │ alter type c rename value 'a' to 'b';
       ╰╴           ━
    error: enum label `temp` does not exist
       ╭▸ 
    18 │ alter type mood add value 'a' after 'temp';
       ╰╴                                    ━━━━━━
    error: enum label `fine` does not exist
       ╭▸ 
    19 │ alter type mood add value 'b' after 'fine';
       ╰╴                                    ━━━━━━
    ");
}

#[test]
fn similar_enum_label() {
    assert_snapshot!(check("
create type status as enum ('active', 'inactive', 'it''s');
select 'actve'::status;
select 'ACTIVE'::status;
select 'its'::status;
select 'zzz'::status;
"), @"
    error: invalid input value for enum `status`: `actve`
      ╭▸ 
    3 │ select 'actve'::status;
      │        ━━━━━━━
      ╰╴
    help: an enum label with a similar name exists
      ╭╴
    3 │ select 'active'::status;
      ╰╴           +
    error: invalid input value for enum `status`: `ACTIVE`
      ╭▸ 
    4 │ select 'ACTIVE'::status;
      │        ━━━━━━━━
      ╰╴
    help: an enum label with a similar name exists
      ╭╴
    4 - select 'ACTIVE'::status;
    4 + select 'active'::status;
      ╰╴
    error: invalid input value for enum `status`: `its`
      ╭▸ 
    5 │ select 'its'::status;
      │        ━━━━━
      ╰╴
    help: an enum label with a similar name exists
      ╭╴
    5 │ select 'it''s'::status;
      ╰╴          ++
    error: invalid input value for enum `status`: `zzz`
      ╭▸ 
    6 │ select 'zzz'::status;
      ╰╴       ━━━━━
    ");
}

#[test]
fn similar_enum_label_in_alter_type() {
    assert_snapshot!(check("
create type status as enum ('active', 'inactive');
alter type status rename value 'actve' to 'live';
alter type status add value 'paused' before 'inactve';
alter type status add value 'done' after 'zzz';
"), @"
    error: enum label `actve` does not exist
      ╭▸ 
    3 │ alter type status rename value 'actve' to 'live';
      │                                ━━━━━━━
      ╰╴
    help: an enum label with a similar name exists
      ╭╴
    3 │ alter type status rename value 'active' to 'live';
      ╰╴                                   +
    error: enum label `inactve` does not exist
      ╭▸ 
    4 │ alter type status add value 'paused' before 'inactve';
      │                                             ━━━━━━━━━
      ╰╴
    help: an enum label with a similar name exists
      ╭╴
    4 │ alter type status add value 'paused' before 'inactive';
      ╰╴                                                  +
    error: enum label `zzz` does not exist
      ╭▸ 
    5 │ alter type status add value 'done' after 'zzz';
      ╰╴                                         ━━━━━
    ");
}
