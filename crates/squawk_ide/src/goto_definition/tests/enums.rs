use insta::assert_snapshot;

use super::goto;

#[test]
fn enum_label_cast() {
    assert_snapshot!(goto("
create type mood as enum (
  'happy',
  'sad'
);
select 'sad$0'::mood;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   'sad'
      │   ───── 2. destination
    5 │ );
    6 │ select 'sad'::mood;
      ╰╴       ───── 1. source
    ");
}

#[test]
fn enum_label_comparison() {
    assert_snapshot!(goto("
create type mood as enum (
  'happy',
  'sad'
);
create table t (m mood);
select * from t where m in ('happy', 'sad$0');
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   'sad'
      │   ───── 2. destination
      ‡
    7 │ select * from t where m in ('happy', 'sad');
      ╰╴                                     ───── 1. source
    ");
}

#[test]
fn enum_label_after_ref() {
    assert_snapshot!(goto("
create type mood as enum (
  'sad',
  'ok'
);
alter type mood add value 'happy' after 'ok'$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   'ok'
      │   ──── 2. destination
    5 │ );
    6 │ alter type mood add value 'happy' after 'ok';
      ╰╴                                        ──── 1. source
    ");
}

#[test]
fn enum_label_renamed_ref() {
    assert_snapshot!(goto("
create type mood as enum (
  'sad',
  'ok'
);
alter type mood rename value 'sad' to 'blue';
alter type mood add value 'x' before 'blue'$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:6:39
      │
    6 │ alter type mood rename value 'sad' to 'blue';
      │                                       ────── 2. destination
    7 │ alter type mood add value 'x' before 'blue';
      ╰╴                                     ────── 1. source
    ");
}

#[test]
fn enum_value_reference_before_rename() {
    assert_snapshot!(goto("
create type mood as enum (
  'sad',
  'ok'
);
select 'sad'$0::mood;
alter type mood rename value 'sad' to 'blue';
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   'sad',
      │   ───── 2. destination
      ‡
    6 │ select 'sad'::mood;
      ╰╴       ───── 1. source
    ");
}
