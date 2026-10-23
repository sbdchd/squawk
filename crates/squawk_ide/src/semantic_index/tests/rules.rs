use insta::assert_snapshot;

use super::check;

#[test]
fn create_rule() {
    assert_snapshot!(check("
create table t (a int, b int);
create table log (a int);
create rule r as on update to t
  where new.a > old.b and new.zz = 1
  do also (
    insert into log values (new.a, old.zz);
    update log set a = new.a where log.a = old.a;
    delete from log where a = old.b;
    select new.a;
    notify c;
  );
create rule r2 as on insert to t do instead nothing;
create rule r3 as on delete to t do instead delete from log where a = old.a;
"), @"
    error: column `zz` does not exist
      ╭▸ 
    5 │   where new.a > old.b and new.zz = 1
      ╰╴                              ━━
    error: column `zz` does not exist
      ╭▸ 
    7 │     insert into log values (new.a, old.zz);
      ╰╴                                       ━━
    ");
}

#[test]
fn create_rule_unresolved() {
    assert_snapshot!(check("
create rule r as on insert to missing_table do instead nothing;
create table t (a int);
create rule r as on insert to t do instead insert into missing_log values (new.a);
"), @"
    error: relation `missing_table` does not exist
      ╭▸ 
    2 │ create rule r as on insert to missing_table do instead nothing;
      ╰╴                              ━━━━━━━━━━━━━
    error: relation `missing_log` does not exist
      ╭▸ 
    4 │ create rule r as on insert to t do instead insert into missing_log values (new.a);
      ╰╴                                                       ━━━━━━━━━━━
    ");
}

#[test]
fn create_rule_where_unqualified_column() {
    assert_snapshot!(check("
create table t (a int);
create rule r as on update to t where a > 1 do instead nothing;
"), @"
    error: column reference `a` is ambiguous
      ╭▸ 
    3 │ create rule r as on update to t where a > 1 do instead nothing;
      ╰╴                                      ━
    ");
}

#[test]
fn create_rule_action_unqualified_column() {
    assert_snapshot!(check("
create table t (a int);
create table log (b int);
create rule r as on insert to t do also insert into log values (a);
"), @"
    error: column `a` does not exist
      ╭▸ 
    4 │ create rule r as on insert to t do also insert into log values (a);
      ╰╴                                                                ━
    ");
}

#[test]
fn create_rule_on_view() {
    assert_snapshot!(check("
create table t (a int);
create view v as select a from t;
create rule r as on insert to v do instead insert into t values (new.a);
"), @"no diagnostics");
}

#[test]
fn duplicate_rule() {
    assert_snapshot!(check(r#"
create table t (a int);
create table u (a int);
create rule r as on insert to t do instead nothing;
create rule r as on insert to t do instead nothing;
create rule "R" as on insert to t do instead nothing;
create rule r as on insert to u do instead nothing;
create or replace rule r as on insert to t do instead nothing;
"#), @"
    error: rule `r` for relation `t` already exists
      ╭▸ 
    5 │ create rule r as on insert to t do instead nothing;
      ╰╴            ━
    ");
}

#[test]
fn duplicate_rule_after_drop_table() {
    assert_snapshot!(check("
create table t (a int);
create rule r as on insert to t do instead nothing;
drop table t;
create table t (a int);
create rule r as on insert to t do instead nothing;
"), @"no diagnostics");
}

#[test]
fn duplicate_rule_after_rollback() {
    assert_snapshot!(check("
create table t (a int);
begin;
create rule r as on insert to t do instead nothing;
rollback;
create rule r as on insert to t do instead nothing;
"), @"no diagnostics");
}

#[test]
fn drop_rule() {
    assert_snapshot!(check("
create table t (a int);
create rule r as on insert to t do instead nothing;
drop rule r on t;
drop rule r on t;
drop rule if exists r on t;
drop rule if exists r on missing_table;
drop rule r on missing_table;
create rule r as on insert to t do instead nothing;
"), @"
    error: rule `r` does not exist
      ╭▸ 
    5 │ drop rule r on t;
      ╰╴          ━
    error: relation `missing_table` does not exist
      ╭▸ 
    8 │ drop rule r on missing_table;
      ╰╴               ━━━━━━━━━━━━━
    ");
}

#[test]
fn drop_rule_rollback() {
    assert_snapshot!(check("
create table t (a int);
create rule r as on insert to t do instead nothing;
begin;
drop rule r on t;
rollback;
drop rule r on t;
"), @"no diagnostics");
}

#[test]
fn alter_rule_rename() {
    assert_snapshot!(check("
create table t (a int);
create rule r as on insert to t do instead nothing;
create rule r2 as on update to t do instead nothing;
alter rule r on t rename to r2;
alter rule r on t rename to r3;
alter rule r on t rename to r4;
alter rule missing on t rename to r5;
alter rule r on missing_table rename to r6;
drop rule r3 on t;
"), @"
    error: rule `r` does not exist
      ╭▸ 
    7 │ alter rule r on t rename to r4;
      ╰╴           ━
    error: rule `missing` does not exist
      ╭▸ 
    8 │ alter rule missing on t rename to r5;
      ╰╴           ━━━━━━━
    error: relation `missing_table` does not exist
      ╭▸ 
    9 │ alter rule r on missing_table rename to r6;
      ╰╴                ━━━━━━━━━━━━━
    error: rule `r2` for relation `t` already exists
      ╭▸ 
    5 │ alter rule r on t rename to r2;
      ╰╴                            ━━
    ");
}

#[test]
fn alter_table_enable_disable_rule() {
    assert_snapshot!(check("
create table t (a int);
create rule r as on insert to t do instead nothing;
alter table t disable rule r;
alter table t enable rule r;
alter table t enable always rule r;
alter table t enable replica rule r;
alter table t disable rule missing;
alter table if exists missing_table disable rule r;
"), @"
    error: rule `missing` does not exist
      ╭▸ 
    8 │ alter table t disable rule missing;
      ╰╴                           ━━━━━━━
    ");
}

#[test]
fn view_return_rule() {
    assert_snapshot!(check(r#"
create table t (a int);
create view v as select a from t;
create materialized view m as select a from t;
create rule "_RETURN" as on select to v do instead select a from t;
create or replace rule "_RETURN" as on select to v do instead select a from t;
create rule _return as on select to v do instead select a from t;
drop rule "_RETURN" on v;
drop rule "_RETURN" on m;
drop rule "_RETURN" on t;
alter rule "_RETURN" on v rename to r;
alter rule "_RETURN" on m rename to r;
"#), @r#"
    error: rule `_RETURN` does not exist
       ╭▸ 
    10 │ drop rule "_RETURN" on t;
       ╰╴          ━━━━━━━━━
    error: rule `_RETURN` for relation `v` already exists
      ╭▸ 
    5 │ create rule "_RETURN" as on select to v do instead select a from t;
      ╰╴            ━━━━━━━━━
    error: cannot drop rule `_RETURN` on view `v` because view `v` requires it
      ╭▸ 
    8 │ drop rule "_RETURN" on v;
      ╰╴          ━━━━━━━━━
    error: cannot drop rule `_RETURN` on materialized view `m` because materialized view `m` requires it
      ╭▸ 
    9 │ drop rule "_RETURN" on m;
      ╰╴          ━━━━━━━━━
    error: renaming an ON SELECT rule is not allowed
       ╭▸ 
    11 │ alter rule "_RETURN" on v rename to r;
       ╰╴                                    ━
    error: renaming an ON SELECT rule is not allowed
       ╭▸ 
    12 │ alter rule "_RETURN" on m rename to r;
       ╰╴                                    ━
    "#);
}

#[test]
fn view_return_rule_after_replace_view() {
    assert_snapshot!(check(r#"
create table t (a int);
create view v as select a from t;
create or replace view v as select a from t;
create rule "_RETURN" as on select to v do instead select a from t;
"#), @r#"
    error: rule `_RETURN` for relation `v` already exists
      ╭▸ 
    5 │ create rule "_RETURN" as on select to v do instead select a from t;
      ╰╴            ━━━━━━━━━
    "#);
}
