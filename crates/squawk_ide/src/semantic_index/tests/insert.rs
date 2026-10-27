use insta::assert_snapshot;

use super::check;

#[test]
fn insert() {
    assert_snapshot!(check("
create table t(a int, b int);
create table u(c int);
insert into t values (1, 2);
insert into t(a, z) values (1, 2);
insert into t select c, x from u;
insert into t default values returning a, y;
insert into missing values (1);
"), @"
    error: column `z` does not exist
      ╭▸ 
    5 │ insert into t(a, z) values (1, 2);
      ╰╴                 ━
    error: column `x` does not exist
      ╭▸ 
    6 │ insert into t select c, x from u;
      ╰╴                        ━
    error: column `y` does not exist
      ╭▸ 
    7 │ insert into t default values returning a, y;
      ╰╴                                          ━
    error: relation `missing` does not exist
      ╭▸ 
    8 │ insert into missing values (1);
      ╰╴            ━━━━━━━
    ");
}

#[test]
fn insert_values_cannot_see_target() {
    assert_snapshot!(check("
create table t(a int);
insert into t values (a);
insert into t select t.a;
"), @"
    error: column `a` does not exist
      ╭▸ 
    3 │ insert into t values (a);
      ╰╴                      ━
    error: missing FROM-clause entry for table `t`
      ╭▸ 
    4 │ insert into t select t.a;
      ╰╴                     ━
    ");
}

#[test]
fn insert_target_is_not_a_cte() {
    assert_snapshot!(check("
with t as (select 1 a)
insert into t values (1);
"), @"
    error: relation `t` does not exist
      ╭▸ 
    3 │ insert into t values (1);
      ╰╴            ━
    ");
}

#[test]
fn insert_in_cte() {
    assert_snapshot!(check("
create table t(a int);
with i as (insert into t values (1) returning a)
select a, b from i;
"), @"
    error: column `b` does not exist
      ╭▸ 
    4 │ select a, b from i;
      ╰╴          ━
    ");
}

#[test]
fn insert_with_cte_source() {
    assert_snapshot!(check("
create table t(a int);
with v as (select 1 x)
insert into t select x, y from v;
"), @"
    error: column `y` does not exist
      ╭▸ 
    4 │ insert into t select x, y from v;
      ╰╴                        ━
    error: INSERT has more expressions than target columns
      ╭▸ 
    4 │ insert into t select x, y from v;
      ╰╴                        ━
    ");
}

#[test]
fn insert_on_conflict() {
    assert_snapshot!(check("
create table t(a int unique, b int);
insert into t values (1, 2) on conflict (a, z) do nothing;
insert into t values (1, 2) on conflict (excluded.a) do nothing;
insert into t values (1, 2) on conflict (a) where b > 0 do update set b = excluded.b + t.b, z = 1 where excluded.y = 1;
insert into t values (1, 2) on conflict (a) do update set b = b;
insert into t values (1, 2) on conflict (a) do update set b = (select b);
insert into t values (1, 2) on conflict (a) do update set (a, b) = (excluded.a, excluded.b) returning excluded.a;
"), @"
    error: column `z` does not exist
      ╭▸ 
    3 │ insert into t values (1, 2) on conflict (a, z) do nothing;
      ╰╴                                            ━
    error: missing FROM-clause entry for table `excluded`
      ╭▸ 
    4 │ insert into t values (1, 2) on conflict (excluded.a) do nothing;
      ╰╴                                         ━━━━━━━━
    error: column `z` does not exist
      ╭▸ 
    5 │ insert into t values (1, 2) on conflict (a) where b > 0 do update set b = excluded.b + t.b, z = 1 where excluded.y = 1;
      ╰╴                                                                                            ━
    error: column `y` does not exist
      ╭▸ 
    5 │ insert into t values (1, 2) on conflict (a) where b > 0 do update set b = excluded.b + t.b, z = 1 where excluded.y = 1;
      ╰╴                                                                                                                 ━
    error: column reference `b` is ambiguous
      ╭▸ 
    6 │ insert into t values (1, 2) on conflict (a) do update set b = b;
      ╰╴                                                              ━
    error: column reference `b` is ambiguous
      ╭▸ 
    7 │ insert into t values (1, 2) on conflict (a) do update set b = (select b);
      ╰╴                                                                      ━
    error: missing FROM-clause entry for table `excluded`
      ╭▸ 
    8 │ insert into t values (1, 2) on conflict (a) do update set (a, b) = (excluded.a, excluded.b) returning excluded.a;
      ╰╴                                                                                                      ━━━━━━━━
    ");
}

#[test]
fn insert_alias() {
    assert_snapshot!(check("
create table t(a int unique);
insert into t as x values (1) on conflict (a) do update set a = x.a + t.a returning x.a;
"), @"
    error: missing FROM-clause entry for table `t`
      ╭▸ 
    3 │ insert into t as x values (1) on conflict (a) do update set a = x.a + t.a returning x.a;
      ╰╴                                                                      ━
    ");
}

#[test]
fn on_conflict_do_select_for_update_of_excluded() {
    assert_snapshot!(check("
create table t (a int primary key);
insert into t values (1) on conflict (a) do select for update of excluded;
"), @"
    error: missing FROM-clause entry for table `excluded`
      ╭▸ 
    3 │ insert into t values (1) on conflict (a) do select for update of excluded;
      ╰╴                                                                 ━━━━━━━━
    ");
}

#[test]
fn duplicate_target_column() {
    assert_snapshot!(check("
create table t (a int[], b int);
insert into t (a, a) values (1, 2);
insert into t (a, a[2]) values ('{1}', 2);
insert into t (a[1], a[2]) values (1, 2);
"), @"
    error: column `a` specified more than once
      ╭▸ 
    3 │ insert into t (a, a) values (1, 2);
      ╰╴                  ━
    error: column `a` specified more than once
      ╭▸ 
    4 │ insert into t (a, a[2]) values ('{1}', 2);
      ╰╴                  ━━━━
    ");
}

#[test]
fn insert_column_count_mismatch() {
    assert_snapshot!(check("
create table t (a int, b int);
create table u (a int, b int, c int);
insert into t (a) values (1, 2);
insert into t (a, b) values (1);
insert into t values (1, 2, 3);
insert into t values (default, default, default);
insert into t select 1, 2, 3;
insert into t (a, b) select 1;
insert into t (a, b) values ((1, 2));
insert into t select * from u;
insert into t values (1);
insert into t (a, b) values (1, 2), (3, 4);
insert into t select * from t;
insert into t (a, b) select * from t;
insert into t default values;
insert into t select * from missing;
insert into missing values (1, 2, 3);
insert into missing (a) values (1, 2);
"), @"
    error: INSERT has more expressions than target columns
      ╭▸ 
    4 │ insert into t (a) values (1, 2);
      ╰╴                             ━
    error: INSERT has more target columns than expressions
      ╭▸ 
    5 │ insert into t (a, b) values (1);
      ╰╴                  ━
    error: INSERT has more expressions than target columns
      ╭▸ 
    6 │ insert into t values (1, 2, 3);
      ╰╴                            ━
    error: INSERT has more expressions than target columns
      ╭▸ 
    7 │ insert into t values (default, default, default);
      ╰╴                                        ━━━━━━━
    error: INSERT has more expressions than target columns
      ╭▸ 
    8 │ insert into t select 1, 2, 3;
      ╰╴                           ━
    error: INSERT has more target columns than expressions
      ╭▸ 
    9 │ insert into t (a, b) select 1;
      ╰╴                  ━
    error: INSERT has more target columns than expressions
       ╭▸ 
    10 │ insert into t (a, b) values ((1, 2));
       │                   ━
       │
       ╰ hint: The insertion source is a row expression containing the same number of columns expected by the INSERT. Did you accidentally use extra parentheses?
    error: INSERT has more expressions than target columns
       ╭▸ 
    11 │ insert into t select * from u;
       ╰╴              ━━━━━━━━━━━━━━━
    error: relation `missing` does not exist
       ╭▸ 
    17 │ insert into t select * from missing;
       ╰╴                            ━━━━━━━
    error: relation `missing` does not exist
       ╭▸ 
    18 │ insert into missing values (1, 2, 3);
       ╰╴            ━━━━━━━
    error: relation `missing` does not exist
       ╭▸ 
    19 │ insert into missing (a) values (1, 2);
       ╰╴            ━━━━━━━
    error: INSERT has more expressions than target columns
       ╭▸ 
    19 │ insert into missing (a) values (1, 2);
       ╰╴                                   ━
    ");
}

#[test]
fn on_conflict_without_matching_arbiter() {
    assert_snapshot!(check("
create table t (
  id int primary key,
  a int,
  b int,
  c int unique,
  d int,
  e int unique deferrable,
  unique (a, b)
);
create unique index t_d on t (d) where d > 0;
create unique index t_lower on t ((a), (lower(b::text)));
insert into t (id) values (1) on conflict (id) do nothing;
insert into t (id) values (1) on conflict (a) do nothing;
insert into t (id) values (1) on conflict (b, a) do nothing;
insert into t (id) values (1) on conflict (a, b, a) do nothing;
insert into t (id) values (1) on conflict (a, b, c) do nothing;
insert into t (id) values (1) on conflict (c) do update set a = 1;
insert into t (id) values (1) on conflict (d) do nothing;
insert into t (id) values (1) on conflict (d) where d > 0 do nothing;
insert into t (id) values (1) on conflict (e) do nothing;
insert into t (id) values (1) on conflict ((id)) do nothing;
insert into t (id) values (1) on conflict (a, lower(b::text)) do nothing;
drop index t_d;
insert into t (id) values (1) on conflict (d) where d > 0 do nothing;
create table pa (id int primary key) partition by range (id);
create table pa1
  partition of pa
  for values from (0) to (10);
create table pa2 (id int not null);
alter table pa attach partition pa2 for values from (10) to (20);
insert into pa1 values (1) on conflict (id) do nothing;
insert into pa2 values (11) on conflict (id) do nothing;
insert into pa values (1) on conflict (id) do nothing;
create function f() returns void
  language sql
  as $$
  insert into t (id) values (1) on conflict (a) do nothing;
$$;
"), @"
    error: there is no unique or exclusion constraint matching the ON CONFLICT specification
       ╭▸ 
    14 │ insert into t (id) values (1) on conflict (a) do nothing;
       ╰╴                                          ━━━
    error: there is no unique or exclusion constraint matching the ON CONFLICT specification
       ╭▸ 
    17 │ insert into t (id) values (1) on conflict (a, b, c) do nothing;
       ╰╴                                          ━━━━━━━━━
    error: there is no unique or exclusion constraint matching the ON CONFLICT specification
       ╭▸ 
    19 │ insert into t (id) values (1) on conflict (d) do nothing;
       ╰╴                                          ━━━
    error: ON CONFLICT does not support deferrable unique constraints/exclusion constraints as arbiters
       ╭▸ 
    21 │ insert into t (id) values (1) on conflict (e) do nothing;
       ╰╴                                          ━━━
    error: there is no unique or exclusion constraint matching the ON CONFLICT specification
       ╭▸ 
    25 │ insert into t (id) values (1) on conflict (d) where d > 0 do nothing;
       ╰╴                                          ━━━
    ");
}
