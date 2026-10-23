use insta::assert_snapshot;

use super::goto;

#[test]
fn update_where_current_of() {
    assert_snapshot!(goto("
create table t (a int);
begin;
declare c cursor
for select a from t;
update t set a = 1 where current of c$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:9
      │
    4 │ declare c cursor
      │         ─ 2. destination
    5 │ for select a from t;
    6 │ update t set a = 1 where current of c;
      ╰╴                                    ─ 1. source
    ");
}

#[test]
fn delete_where_current_of() {
    assert_snapshot!(goto("
create table t (a int);
begin;
declare c cursor
for select a from t;
delete from t where current of c$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:9
      │
    4 │ declare c cursor
      │         ─ 2. destination
    5 │ for select a from t;
    6 │ delete from t where current of c;
      ╰╴                               ─ 1. source
    ");
}

#[test]
fn fetch_cursor() {
    assert_snapshot!(goto("
begin;
declare c cursor
for select 1;
fetch next from c$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:9
      │
    3 │ declare c cursor
      │         ─ 2. destination
    4 │ for select 1;
    5 │ fetch next from c;
      ╰╴                ─ 1. source
    ");
}

#[test]
fn move_cursor() {
    assert_snapshot!(goto("
begin;
declare c cursor
for select 1;
move forward 2 in c$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:9
      │
    3 │ declare c cursor
      │         ─ 2. destination
    4 │ for select 1;
    5 │ move forward 2 in c;
      ╰╴                  ─ 1. source
    ");
}

#[test]
fn close_cursor() {
    assert_snapshot!(goto("
begin;
declare c cursor
for select 1;
close c$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:9
      │
    3 │ declare c cursor
      │         ─ 2. destination
    4 │ for select 1;
    5 │ close c;
      ╰╴      ─ 1. source
    ");
}

#[test]
fn declare_cursor_query_column() {
    assert_snapshot!(goto("
create table t (a int);
begin;
declare c cursor
for select a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
      ‡
    5 │ for select a from t;
      ╰╴           ─ 1. source
    ");
}
