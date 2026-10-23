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
      ╰╴             ━
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
      ╰╴             ━
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
