use insta::assert_snapshot;

use super::check;

#[test]
fn select_into() {
    assert_snapshot!(check("
create table s (a int, b int);
select a, b as c into t from s;
select a, c, b from t;
select * into u from nope;
select x from u;
"), @"
    error: column `b` does not exist
      ╭▸ 
    4 │ select a, c, b from t;
      │              ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    4 - select a, c, b from t;
    4 + select a, c, a from t;
      ╰╴
    error: relation `nope` does not exist
      ╭▸ 
    5 │ select * into u from nope;
      ╰╴                     ━━━━
    ");
}

#[test]
fn create_table_as() {
    assert_snapshot!(check("
create table s (a int, b int);
create table t (x) as select a, b from s;
select x, b, a from t;
create table u as select * from nope;
select z from u;
create table v as execute foo;
select z from v;
create temp table w as table s;
select a, nope2 from pg_temp.w;
"), @"
    error: column `a` does not exist
      ╭▸ 
    4 │ select x, b, a from t;
      │              ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    4 - select x, b, a from t;
    4 + select x, b, x from t;
      ╰╴
    error: relation `nope` does not exist
      ╭▸ 
    5 │ create table u as select * from nope;
      ╰╴                                ━━━━
    error: prepared statement `foo` does not exist
      ╭▸ 
    7 │ create table v as execute foo;
      ╰╴                          ━━━
    error: column `nope2` does not exist
       ╭▸ 
    10 │ select a, nope2 from pg_temp.w;
       ╰╴          ━━━━━
    ");
}

#[test]
fn create_table_as_column_count_mismatch() {
    assert_snapshot!(check("
create table t (a, b) as select 1;
create table u (a) as select 1, 2;
"), @"
    error: too many column names were specified
      ╭▸ 
    2 │ create table t (a, b) as select 1;
      ╰╴                   ━
    ");
}

#[test]
fn create_table_as_keeps_typmod() {
    assert_snapshot!(check("
create table t (a numeric(3, 0));
create table u as
  select a, '1'::numeric(3, 0) as b from t;
insert into u values ('1000', '1000');
"), @"
    error: value `1000` is out of range for type `numeric(3, 0)`
      ╭▸ 
    5 │ insert into u values ('1000', '1000');
      │                       ━━━━━━
      │
      ╰ note: a field with precision 3 and scale 0 must round to an absolute value less than `10^3`
    error: value `1000` is out of range for type `numeric(3, 0)`
      ╭▸ 
    5 │ insert into u values ('1000', '1000');
      │                               ━━━━━━
      │
      ╰ note: a field with precision 3 and scale 0 must round to an absolute value less than `10^3`
    ");
}
