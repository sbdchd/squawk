use insta::assert_snapshot;

use super::goto;

#[test]
fn execute_prepared_statement() {
    assert_snapshot!(goto("
prepare p as
  select 1;
execute p$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:9
      │
    2 │ prepare p as
      │         ─ 2. destination
    3 │   select 1;
    4 │ execute p;
      ╰╴        ─ 1. source
    ");
}

#[test]
fn execute_prepared_statement_args() {
    assert_snapshot!(goto("
create table t (a int);
prepare p(int) as
  select a from t where a = $1;
execute p((select a$0 from t));
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
      ‡
    5 │ execute p((select a from t));
      ╰╴                  ─ 1. source
    ");
}

#[test]
fn deallocate_prepared_statement() {
    assert_snapshot!(goto("
prepare p as
  select 1;
deallocate prepare p$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:9
      │
    2 │ prepare p as
      │         ─ 2. destination
    3 │   select 1;
    4 │ deallocate prepare p;
      ╰╴                   ─ 1. source
    ");
}

#[test]
fn explain_execute_prepared_statement() {
    assert_snapshot!(goto("
prepare p as
  select 1;
explain
  analyze
  execute p$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:9
      │
    2 │ prepare p as
      │         ─ 2. destination
      ‡
    6 │   execute p;
      ╰╴          ─ 1. source
    ");
}

#[test]
fn prepared_statement_positional_param() {
    assert_snapshot!(goto("
create table t (
  a int,
  b text
);
prepare p(int, text) as
  select a from t where a = $1 and b = $2$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:6:16
      │
    6 │ prepare p(int, text) as
      │                ──── 2. destination
    7 │   select a from t where a = $1 and b = $2;
      ╰╴                                       ── 1. source
    ");
}

#[test]
fn prepared_statement_query_column() {
    assert_snapshot!(goto("
create table t (a int);
prepare p as
  select a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ prepare p as
    4 │   select a from t;
      ╰╴         ─ 1. source
    ");
}

#[test]
fn create_table_as_execute_column() {
    assert_snapshot!(goto("
prepare p as
  select 1 a;
create table t as
  execute p;
select a$0 from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:12
      │
    3 │   select 1 a;
      │            ─ 2. destination
      ‡
    6 │ select a from t;
      ╰╴       ─ 1. source
    ");
}
