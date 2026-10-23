use insta::assert_snapshot;

use super::goto;

#[test]
fn case_nested_end_goes_to_inner_case() {
    assert_snapshot!(goto("
select case when true then case when false then 1 end$0 end;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:28
      │
    2 │ select case when true then case when false then 1 end end;
      ╰╴                           ──── 2. destination    ─── 1. source
    ");
}

#[test]
fn case_nested_outer_end() {
    assert_snapshot!(goto("
select case when true then case when false then 1 end end$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:8
      │
    2 │ select case when true then case when false then 1 end end;
      ╰╴       ──── 2. destination                            ─── 1. source
    ");
}

#[test]
fn case_multiple_whens() {
    assert_snapshot!(goto("
select case when 1 = 1 then 1 when$0 2 = 2 then 2 end;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:8
      │
    2 │ select case when 1 = 1 then 1 when 2 = 2 then 2 end;
      ╰╴       ──── 2. destination    ──── 1. source
    ");
}

#[test]
fn case_in_where() {
    assert_snapshot!(goto("
create table t (a int);
select * from t where case when a > 1 then true else$0 false end;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:23
      │
    3 │ select * from t where case when a > 1 then true else false end;
      ╰╴                      ──── 2. destination       ──── 1. source
    ");
}

#[test]
fn case_in_update_set() {
    assert_snapshot!(goto("
create table t (a int);
update t set a = case when a > 1 then 1 end$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:18
      │
    3 │ update t set a = case when a > 1 then 1 end;
      ╰╴                 ──── 2. destination    ─── 1. source
    ");
}

#[test]
fn case_simple_form() {
    assert_snapshot!(goto("
create table t (a int);
select case a when$0 1 then 'one' end from t;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:8
      │
    3 │ select case a when 1 then 'one' end from t;
      │        ┬───   ──── 1. source
      │        │
      ╰╴       2. destination
    ");
}
