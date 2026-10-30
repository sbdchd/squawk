use insta::assert_snapshot;

use super::check;

#[test]
fn explain_without_analyze_creates_nothing() {
    assert_snapshot!(check("
explain create table t as select 1 a;
select * from t;
explain (analyze false) create table u as select 1 a;
select * from u;
explain create materialized view m as select 1 a;
select * from m;
explain select 1 a into v;
select * from v;
begin;
explain declare c cursor for select 1;
fetch c;
commit;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    3 │ select * from t;
      ╰╴              ━
    error: relation `u` does not exist
      ╭▸ 
    5 │ select * from u;
      ╰╴              ━
    error: relation `m` does not exist
      ╭▸ 
    7 │ select * from m;
      ╰╴              ━
    error: relation `v` does not exist
      ╭▸ 
    9 │ select * from v;
      ╰╴              ━
    error: cursor `c` does not exist
       ╭▸ 
    12 │ fetch c;
       ╰╴      ━
    ");
}

#[test]
fn explain_analyze_creates() {
    assert_snapshot!(check("
explain analyze create table t as select 1 a;
select a from t;
explain (analyze) create table u as select 1 a;
select a from u;
explain (analyze true, verbose) create materialized view m as select 1 a;
select a from m;
explain analyse select 1 a into v;
select a from v;
"), @"no diagnostics");
}

#[test]
fn explain_lowers_inner_stmt() {
    assert_snapshot!(check("
explain select b from t;
explain (costs off) insert into t values (1);
explain verbose update t set a = 1;
explain execute p;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    2 │ explain select b from t;
      ╰╴                      ━
    error: relation `t` does not exist
      ╭▸ 
    3 │ explain (costs off) insert into t values (1);
      ╰╴                                ━
    error: relation `t` does not exist
      ╭▸ 
    4 │ explain verbose update t set a = 1;
      ╰╴                       ━
    error: prepared statement `p` does not exist
      ╭▸ 
    5 │ explain execute p;
      ╰╴                ━
    ");
}
