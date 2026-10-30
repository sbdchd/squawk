use insta::assert_snapshot;

use super::check;

#[test]
fn unresolved_prepared_statement() {
    assert_snapshot!(check("
create table t (a int);
execute p;
prepare p (int) as select a from t where a = $1;
prepare q as select $1;
execute p(1);
execute q('a');
deallocate p;
execute p(1);
deallocate prepare q;
deallocate q;
prepare r as insert into t values ($1);
prepare s as delete from t where a = $1;
deallocate all;
execute r(1);
execute s(1);
"), @"
    error: prepared statement `p` does not exist
      ╭▸ 
    3 │ execute p;
      ╰╴        ━
    error: prepared statement `p` does not exist
      ╭▸ 
    9 │ execute p(1);
      ╰╴        ━
    error: prepared statement `q` does not exist
       ╭▸ 
    11 │ deallocate q;
       ╰╴           ━
    error: prepared statement `r` does not exist
       ╭▸ 
    15 │ execute r(1);
       ╰╴        ━
    error: prepared statement `s` does not exist
       ╭▸ 
    16 │ execute s(1);
       ╰╴        ━
    ");
}

#[test]
fn duplicate_prepared_statement() {
    assert_snapshot!(check("
prepare p as select 1;
prepare p as select 2;
execute p;
deallocate p;
prepare p as select 3;
"), @"
    error: prepared statement `p` already exists
      ╭▸ 
    3 │ prepare p as select 2;
      ╰╴        ━
    ");
}

#[test]
fn prepared_statement_not_transactional() {
    assert_snapshot!(check("
begin;
prepare p as select 1;
rollback;
execute p;
"), @"no diagnostics");
}

#[test]
fn prepared_statement_unresolved_body() {
    assert_snapshot!(check("
prepare p (int, text) as select b from t where a = $1;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    2 │ prepare p (int, text) as select b from t where a = $1;
      ╰╴                                       ━
    ");
}

#[test]
fn create_table_as_execute() {
    assert_snapshot!(check("
prepare p as select 1 a, 'x' b;
create table t as execute p;
select a, b, c from t;
create table u as execute q;
"), @"
    error: column `c` does not exist
      ╭▸ 
    4 │ select a, b, c from t;
      │              ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    4 - select a, b, c from t;
    4 + select a, b, a from t;
      ╰╴
    error: prepared statement `q` does not exist
      ╭▸ 
    5 │ create table u as execute q;
      ╰╴                          ━
    ");
}
