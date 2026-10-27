use insta::assert_snapshot;

use super::check;

#[test]
fn create_policy() {
    assert_snapshot!(check("
create role a;
create table t (owner text, b int);
create policy p on t
  as restrictive
  for update
  to a, public, current_user, missing_role
  using (owner = current_user and t.b > 0 and zz = 1)
  with check (b > 0 and t.yy = 1);
create policy p2 on t;
"), @"
    error: role `missing_role` does not exist
      ╭▸ 
    7 │   to a, public, current_user, missing_role
      ╰╴                              ━━━━━━━━━━━━
    error: column `zz` does not exist
      ╭▸ 
    8 │   using (owner = current_user and t.b > 0 and zz = 1)
      ╰╴                                              ━━
    error: column `t.yy` does not exist
      ╭▸ 
    9 │   with check (b > 0 and t.yy = 1);
      ╰╴                          ━━
    ");
}

#[test]
fn create_policy_unresolved() {
    assert_snapshot!(check("
create policy p on missing_table using (a = 1);
create view v as select 1 as a;
create policy p on v using (a = 1);
"), @"
    error: relation `missing_table` does not exist
      ╭▸ 
    2 │ create policy p on missing_table using (a = 1);
      ╰╴                   ━━━━━━━━━━━━━
    error: `v` is not a table
      ╭▸ 
    4 │ create policy p on v using (a = 1);
      ╰╴                   ━
    ");
}

#[test]
fn create_policy_subquery() {
    assert_snapshot!(check("
create table t (a int);
create table u (a int, b int);
create policy p on t using (exists (select 1 from u where u.a = t.a and b = a));
"), @"no diagnostics");
}

#[test]
fn duplicate_policy() {
    assert_snapshot!(check(r#"
create table t (a int);
create table u (a int);
create policy p on t;
create policy p on t;
create policy "P" on t;
create policy p on u;
"#), @"
    error: policy `p` for table `t` already exists
      ╭▸ 
    5 │ create policy p on t;
      ╰╴              ━
    ");
}

#[test]
fn duplicate_policy_after_rollback() {
    assert_snapshot!(check("
create table t (a int);
begin;
create policy p on t;
rollback;
create policy p on t;
"), @"no diagnostics");
}

#[test]
fn drop_policy() {
    assert_snapshot!(check("
create table t (a int);
create policy p on t;
drop policy p on t;
drop policy p on t;
drop policy if exists p on t;
drop policy if exists p on missing_table;
drop policy p on missing_table;
create policy p on t;
"), @"
    error: policy `p` does not exist
      ╭▸ 
    5 │ drop policy p on t;
      ╰╴            ━
    error: relation `missing_table` does not exist
      ╭▸ 
    8 │ drop policy p on missing_table;
      ╰╴                 ━━━━━━━━━━━━━
    ");
}

#[test]
fn alter_policy() {
    assert_snapshot!(check("
create role a;
create table t (a int);
create policy p on t;
create policy p2 on t;
alter policy p on t to a, public, missing_role using (a > 0) with check (zz > 0);
alter policy p on t rename to p2;
alter policy p on t rename to p3;
alter policy p on t rename to p4;
alter policy missing on t to public;
alter policy p on missing_table to public;
drop policy p3 on t;
"), @"
    error: role `missing_role` does not exist
      ╭▸ 
    6 │ alter policy p on t to a, public, missing_role using (a > 0) with check (zz > 0);
      ╰╴                                  ━━━━━━━━━━━━
    error: column `zz` does not exist
      ╭▸ 
    6 │ alter policy p on t to a, public, missing_role using (a > 0) with check (zz > 0);
      ╰╴                                                                         ━━
    error: policy `p2` for table `t` already exists
      ╭▸ 
    7 │ alter policy p on t rename to p2;
      ╰╴                              ━━
    error: policy `p` does not exist
      ╭▸ 
    9 │ alter policy p on t rename to p4;
      ╰╴             ━
    error: policy `missing` does not exist
       ╭▸ 
    10 │ alter policy missing on t to public;
       ╰╴             ━━━━━━━
    error: relation `missing_table` does not exist
       ╭▸ 
    11 │ alter policy p on missing_table to public;
       ╰╴                  ━━━━━━━━━━━━━
    ");
}
