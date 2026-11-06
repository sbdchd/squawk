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
    error: column `t.z` does not exist
      ╭▸ 
    5 │ insert into t(a, z) values (1, 2);
      │                  ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    5 - insert into t(a, z) values (1, 2);
    5 + insert into t(a, a) values (1, 2);
      ╰╴
    error: column `x` does not exist
      ╭▸ 
    6 │ insert into t select c, x from u;
      │                         ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    6 - insert into t select c, x from u;
    6 + insert into t select c, c from u;
      ╰╴
    error: column `y` does not exist
      ╭▸ 
    7 │ insert into t default values returning a, y;
      │                                           ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    7 - insert into t default values returning a, y;
    7 + insert into t default values returning a, a;
      ╰╴
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
    error: missing `from` clause entry for table `t`
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
      │           ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    4 - select a, b from i;
    4 + select a, a from i;
      ╰╴
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
      │                         ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    4 - insert into t select x, y from v;
    4 + insert into t select x, x from v;
      ╰╴
    error: `insert` has more expressions than target columns
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
      │                                             ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    3 - insert into t values (1, 2) on conflict (a, z) do nothing;
    3 + insert into t values (1, 2) on conflict (a, a) do nothing;
      ╰╴
    error: missing `from` clause entry for table `excluded`
      ╭▸ 
    4 │ insert into t values (1, 2) on conflict (excluded.a) do nothing;
      ╰╴                                         ━━━━━━━━
    error: column `t.z` does not exist
      ╭▸ 
    5 │ insert into t values (1, 2) on conflict (a) where b > 0 do update set b = excluded.b + t.b, z = 1 where excluded.y = 1;
      │                                                                                             ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    5 - insert into t values (1, 2) on conflict (a) where b > 0 do update set b = excluded.b + t.b, z = 1 where excluded.y = 1;
    5 + insert into t values (1, 2) on conflict (a) where b > 0 do update set b = excluded.b + t.b, a = 1 where excluded.y = 1;
      ╰╴
    error: column `excluded.y` does not exist
      ╭▸ 
    5 │ insert into t values (1, 2) on conflict (a) where b > 0 do update set b = excluded.b + t.b, z = 1 where excluded.y = 1;
      │                                                                                                                  ━
      ╰╴
    help: a column with a similar name exists
      ╭╴
    5 - insert into t values (1, 2) on conflict (a) where b > 0 do update set b = excluded.b + t.b, z = 1 where excluded.y = 1;
    5 + insert into t values (1, 2) on conflict (a) where b > 0 do update set b = excluded.b + t.b, z = 1 where excluded.a = 1;
      ╰╴
    error: column reference `b` is ambiguous
      ╭▸ 
    6 │ insert into t values (1, 2) on conflict (a) do update set b = b;
      ╰╴                                                              ━
    error: column reference `b` is ambiguous
      ╭▸ 
    7 │ insert into t values (1, 2) on conflict (a) do update set b = (select b);
      ╰╴                                                                      ━
    error: missing `from` clause entry for table `excluded`
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
    error: missing `from` clause entry for table `t`
      ╭▸ 
    3 │ insert into t as x values (1) on conflict (a) do update set a = x.a + t.a returning x.a;
      ╰╴                                                                      ━
    ");
}

#[test]
fn on_conflict_do_select_for_update_of_excluded() {
    assert_snapshot!(check("
create table t (a int primary key);
insert into t values (1) on conflict (a) do select for update of excluded returning *;
"), @"
    error: missing `from` clause entry for table `excluded`
      ╭▸ 
    3 │ insert into t values (1) on conflict (a) do select for update of excluded returning *;
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
    error: `insert` has more expressions than target columns
      ╭▸ 
    4 │ insert into t (a) values (1, 2);
      ╰╴                             ━
    error: `insert` has more target columns than expressions
      ╭▸ 
    5 │ insert into t (a, b) values (1);
      ╰╴                  ━
    error: `insert` has more expressions than target columns
      ╭▸ 
    6 │ insert into t values (1, 2, 3);
      ╰╴                            ━
    error: `insert` has more expressions than target columns
      ╭▸ 
    7 │ insert into t values (default, default, default);
      ╰╴                                        ━━━━━━━
    error: `insert` has more expressions than target columns
      ╭▸ 
    8 │ insert into t select 1, 2, 3;
      ╰╴                           ━
    error: `insert` has more target columns than expressions
      ╭▸ 
    9 │ insert into t (a, b) select 1;
      ╰╴                  ━
    error: `insert` has more target columns than expressions
       ╭▸ 
    10 │ insert into t (a, b) values ((1, 2));
       │                   ━
       │
       ╰ hint: the insertion source is a row expression containing the same number of columns expected by the `insert`; consider removing the extra parentheses
    error: `insert` has more expressions than target columns
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
    error: `insert` has more expressions than target columns
       ╭▸ 
    19 │ insert into missing (a) values (1, 2);
       ╰╴                                   ━
    ");
}

#[test]
fn on_conflict_partial_index() {
    assert_snapshot!(check("
create table t (
  email text,
  deleted_at timestamptz,
  tenant int
);
create unique index on t (email) where deleted_at is null;
create unique index on t (tenant);
create unique index on t (tenant) where deleted_at is null;

insert into t values ('a') on conflict (email) do nothing;

insert into t values ('a') on conflict (email) do update set tenant = 1;

insert into t
values ('a')
on conflict (email) where deleted_at is null do nothing;

insert into t values ('a') on conflict (tenant) do nothing;
"), @"
    error: there is no unique or exclusion constraint matching the `on conflict` specification
       ╭▸ 
    11 │ insert into t values ('a') on conflict (email) do nothing;
       ╰╴                                       ━━━━━━━
    error: there is no unique or exclusion constraint matching the `on conflict` specification
       ╭▸ 
    13 │ insert into t values ('a') on conflict (email) do update set tenant = 1;
       ╰╴                                       ━━━━━━━
    ");
}

#[test]
fn on_conflict_exclusion_constraint() {
    assert_snapshot!(check("
create table ex (
  a int,
  b int unique,
  exclude using btree (a with =)
);

alter table ex add constraint ex_b_excl exclude using btree (b with =);

insert into ex
values (1)
on conflict on constraint ex_a_excl do update set a = 2;

insert into ex
values (1)
on conflict on constraint ex_b_excl do update set a = 2;

insert into ex
values (1)
on conflict on constraint ex_a_excl do select
returning *;

insert into ex values (1) on conflict on constraint ex_a_excl do nothing;

insert into ex
values (1)
on conflict on constraint ex_b_key do update set a = 2;

create function f() returns void
  language sql
  as $$
  insert into ex values (1) on conflict on constraint ex_a_excl do update set a = 2;
$$;
"), @"
    error: `on conflict do update` not supported with exclusion constraints
       ╭▸ 
    12 │ on conflict on constraint ex_a_excl do update set a = 2;
       ╰╴                          ━━━━━━━━━
    error: `on conflict do update` not supported with exclusion constraints
       ╭▸ 
    16 │ on conflict on constraint ex_b_excl do update set a = 2;
       ╰╴                          ━━━━━━━━━
    error: `on conflict do select` not supported with exclusion constraints
       ╭▸ 
    20 │ on conflict on constraint ex_a_excl do select
       ╰╴                          ━━━━━━━━━
    ");
}

#[test]
fn on_conflict_on_deferrable_constraint() {
    assert_snapshot!(check("
create table t (
  a int,
  b int,
  c int,
  constraint t_a unique (a) deferrable,
  constraint t_b primary key (b) deferrable initially deferred,
  constraint t_c unique (c) not deferrable
);

create table ex (
  a int,
  constraint ex_a exclude using btree (a with =) deferrable
);

insert into t values (1) on conflict on constraint t_a do nothing;

insert into t values (1) on conflict on constraint t_b do update set a = 1;

insert into t values (1) on conflict on constraint t_c do nothing;

insert into ex values (1) on conflict on constraint ex_a do nothing;

insert into ex values (1) on conflict on constraint ex_a do update set a = 1;

create function f() returns void
  language sql
  as $$
  insert into t values (1) on conflict on constraint t_a do nothing;
$$;
"), @"
    error: `on conflict` does not support deferrable unique constraints/exclusion constraints as arbiters
       ╭▸ 
    16 │ insert into t values (1) on conflict on constraint t_a do nothing;
       ╰╴                                                   ━━━
    error: `on conflict` does not support deferrable unique constraints/exclusion constraints as arbiters
       ╭▸ 
    18 │ insert into t values (1) on conflict on constraint t_b do update set a = 1;
       ╰╴                                                   ━━━
    error: `on conflict` does not support deferrable unique constraints/exclusion constraints as arbiters
       ╭▸ 
    22 │ insert into ex values (1) on conflict on constraint ex_a do nothing;
       ╰╴                                                    ━━━━
    error: `on conflict do update` not supported with exclusion constraints
       ╭▸ 
    24 │ insert into ex values (1) on conflict on constraint ex_a do update set a = 1;
       ╰╴                                                    ━━━━
    ");
}

#[test]
fn on_conflict_on_constraint_without_index() {
    assert_snapshot!(check("
create table p (id int primary key);

create table t (
  a int primary key,
  b int,
  c int not null,
  constraint t_b check (b > 0),
  constraint t_fk foreign key (b) references p
);

insert into t values (1) on conflict on constraint t_b do nothing;

insert into t values (1) on conflict on constraint t_fk do update set b = 1;

insert into t values (1) on conflict on constraint t_c_not_null do nothing;

insert into t values (1) on conflict on constraint t_pkey do nothing;
"), @"
    error: constraint in `on conflict` clause has no associated index
       ╭▸ 
    12 │ insert into t values (1) on conflict on constraint t_b do nothing;
       │                                                    ━━━
       │
       ╰ hint: only primary key, unique, and exclusion constraints can be used as arbiters
    error: constraint in `on conflict` clause has no associated index
       ╭▸ 
    14 │ insert into t values (1) on conflict on constraint t_fk do update set b = 1;
       │                                                    ━━━━
       │
       ╰ hint: only primary key, unique, and exclusion constraints can be used as arbiters
    error: constraint in `on conflict` clause has no associated index
       ╭▸ 
    16 │ insert into t values (1) on conflict on constraint t_c_not_null do nothing;
       │                                                    ━━━━━━━━━━━━
       │
       ╰ hint: only primary key, unique, and exclusion constraints can be used as arbiters
    ");
}

#[test]
fn on_conflict_with_rules() {
    assert_snapshot!(check("
create table t (a int primary key);
create rule r as on update to t do instead nothing;
insert into t values (1) on conflict do nothing;

create table i (a int primary key);
create rule r as on insert to i do also notify x;
insert into i values (1) on conflict (a) do update set a = 2;

create table n (a int primary key);
create rule r1 as on insert to n do instead nothing;
create rule r2 as on insert to n do also nothing;
create rule r3 as on delete to n do also notify x;
insert into n values (1) on conflict do nothing;

create table d (a int primary key);
create rule r as on update to d do also notify x;
drop rule r on d;
insert into d values (1) on conflict do nothing;

create table b (a int primary key);

create view v as select * from b;
create rule r as on insert to v do also notify x;
create rule r2 as on update to v do also notify x;
insert into v values (1) on conflict do nothing;

create view w as select * from b;
create rule r as on insert to w do instead insert into t values (new.a);
insert into w values (1) on conflict do nothing;

create view u as select * from b;
create rule r as on insert to u do instead nothing;
create rule r2 as on update to u do also notify x;
insert into u values (1) on conflict do nothing;
"), @"
    error: `insert` with `on conflict` cannot be used with table that has `insert` or `update` rules
      ╭▸ 
    4 │ insert into t values (1) on conflict do nothing;
      │                          ━━━━━━━━━━━
      │
      ╰ note: `t` has an `on update` rule `r`
    error: `insert` with `on conflict` cannot be used with table that has `insert` or `update` rules
      ╭▸ 
    8 │ insert into i values (1) on conflict (a) do update set a = 2;
      │                          ━━━━━━━━━━━
      │
      ╰ note: `i` has an `on insert` rule `r`
    error: `insert` with `on conflict` cannot be used with table that has `insert` or `update` rules
       ╭▸ 
    30 │ insert into w values (1) on conflict do nothing;
       │                          ━━━━━━━━━━━
       │
       ╰ note: `w` has an `on insert` rule `r`
    error: `insert` with `on conflict` cannot be used with table that has `insert` or `update` rules
       ╭▸ 
    35 │ insert into u values (1) on conflict do nothing;
       │                          ━━━━━━━━━━━
       │
       ╰ note: `u` has an `on update` rule `r2`
    ");
}

#[test]
fn on_conflict_with_disabled_rules() {
    assert_snapshot!(check("
create table t1 (a int primary key);
create rule r as on insert
  to t1
  do also notify x;
alter table t1 disable rule r;
insert into t1 values (1) on conflict do nothing;

create table t2 (a int primary key);
create rule r as on update
  to t2
  do also notify x;
alter table t2 disable rule r;
insert into t2 values (1) on conflict do nothing;

create table t3 (a int primary key);
create rule r as on insert
  to t3
  do also notify x;
alter table t3 enable replica rule r;
insert into t3 values (1) on conflict do nothing;

create table t4 (a int primary key);
create rule r as on insert
  to t4
  do also notify x;
alter table t4 enable always rule r;
insert into t4 values (1) on conflict do nothing;

create table t5 (a int primary key);
create rule r as on insert
  to t5
  do also notify x;
alter table t5 disable rule r;
alter table t5 enable rule r;
insert into t5 values (1) on conflict do nothing;

create table t6 (a int primary key);
create rule r as on insert
  to t6
  do also notify x;
begin;
alter table t6 disable rule r;
rollback;
insert into t6 values (1) on conflict do nothing;

create table t7 (a int primary key);
create rule r as on insert
  to t7
  do also notify x;
alter table t7 disable rule r;
create or replace rule r as on insert
  to t7
  do also notify y;
insert into t7 values (1) on conflict do nothing;

create table t8 (a int primary key);
create rule r as on insert
  to t8
  do also notify x;
alter table t8 disable rule r;
alter rule r on t8 rename to r2;
insert into t8 values (1) on conflict do nothing;
"), @"
    error: `insert` with `on conflict` cannot be used with table that has `insert` or `update` rules
       ╭▸ 
    14 │ insert into t2 values (1) on conflict do nothing;
       │                           ━━━━━━━━━━━
       │
       ╰ note: `t2` has an `on update` rule `r`
    error: `insert` with `on conflict` cannot be used with table that has `insert` or `update` rules
       ╭▸ 
    28 │ insert into t4 values (1) on conflict do nothing;
       │                           ━━━━━━━━━━━
       │
       ╰ note: `t4` has an `on insert` rule `r`
    error: `insert` with `on conflict` cannot be used with table that has `insert` or `update` rules
       ╭▸ 
    36 │ insert into t5 values (1) on conflict do nothing;
       │                           ━━━━━━━━━━━
       │
       ╰ note: `t5` has an `on insert` rule `r`
    error: `insert` with `on conflict` cannot be used with table that has `insert` or `update` rules
       ╭▸ 
    45 │ insert into t6 values (1) on conflict do nothing;
       │                           ━━━━━━━━━━━
       │
       ╰ note: `t6` has an `on insert` rule `r`
    ");
}

#[test]
fn on_conflict_whole_row() {
    assert_snapshot!(check("
create table t (a int primary key);

insert into t values (1) on conflict (t) do nothing;

insert into t as x values (1) on conflict (x) do nothing;

insert into t values (1) on conflict ((t)) do nothing;

insert into t values (1) on conflict (a, t) do nothing;

insert into t values (1) on conflict (t) do update set a = 1;

insert into t values (1) on conflict (a) where t is not null do nothing;

create table c (t int primary key);
insert into c values (1) on conflict (t) do nothing;
"), @"
    error: whole row unique index inference specifications are not supported
      ╭▸ 
    4 │ insert into t values (1) on conflict (t) do nothing;
      │                                       ━
      │
      ╰ hint: list the columns of a unique index instead
    error: whole row unique index inference specifications are not supported
      ╭▸ 
    6 │ insert into t as x values (1) on conflict (x) do nothing;
      │                                            ━
      │
      ╰ hint: list the columns of a unique index instead
    error: whole row unique index inference specifications are not supported
      ╭▸ 
    8 │ insert into t values (1) on conflict ((t)) do nothing;
      │                                        ━
      │
      ╰ hint: list the columns of a unique index instead
    error: whole row unique index inference specifications are not supported
       ╭▸ 
    10 │ insert into t values (1) on conflict (a, t) do nothing;
       │                                          ━
       │
       ╰ hint: list the columns of a unique index instead
    error: whole row unique index inference specifications are not supported
       ╭▸ 
    12 │ insert into t values (1) on conflict (t) do update set a = 1;
       │                                       ━
       │
       ╰ hint: list the columns of a unique index instead
    ");
}

#[test]
fn on_conflict_system_catalog() {
    assert_snapshot!(check("
insert into pg_class (oid) values (1) on conflict do nothing;

insert into pg_catalog.pg_namespace (oid, nspname) values (1, 'x') on conflict (oid) do nothing;

insert into information_schema.sql_features (feature_id) values ('x') on conflict do nothing;
insert into information_schema.sql_features (feature_id) values ('x') on conflict (feature_id) do nothing;
insert into pg_catalog.pg_namespace (oid, nspname) values (1, 'x') on conflict (nspowner) do nothing;

create table pg_catalog.mine (a int);
insert into pg_catalog.mine values (1) on conflict do nothing;
"), @"
    error: `on conflict` is not supported with system catalog tables
      ╭▸ 
    2 │ insert into pg_class (oid) values (1) on conflict do nothing;
      ╰╴                                      ━━━━━━━━━━━
    error: `on conflict` is not supported with system catalog tables
      ╭▸ 
    4 │ insert into pg_catalog.pg_namespace (oid, nspname) values (1, 'x') on conflict (oid) do nothing;
      ╰╴                                                                   ━━━━━━━━━━━
    error: there is no unique or exclusion constraint matching the `on conflict` specification
      ╭▸ 
    7 │ insert into information_schema.sql_features (feature_id) values ('x') on conflict (feature_id) do nothing;
      ╰╴                                                                                  ━━━━━━━━━━━━
    error: `on conflict` is not supported with system catalog tables
      ╭▸ 
    8 │ insert into pg_catalog.pg_namespace (oid, nspname) values (1, 'x') on conflict (nspowner) do nothing;
      ╰╴                                                                   ━━━━━━━━━━━
    ");
}

#[test]
fn on_conflict_foreign_table() {
    assert_snapshot!(check("
create foreign data wrapper w;
create server s foreign data wrapper w;
create foreign table ft (a int not null, b int) server s;

insert into ft values (1) on conflict (a) do update set a = 1;

insert into ft values (1) on conflict ((a + 1)) do nothing;

insert into ft values (1) on conflict do nothing;
"), @"
    error: there is no unique or exclusion constraint matching the `on conflict` specification
      ╭▸ 
    6 │ insert into ft values (1) on conflict (a) do update set a = 1;
      │                                       ━━━
      │
      ╰ note: foreign tables can't have unique or exclusion constraints
    error: there is no unique or exclusion constraint matching the `on conflict` specification
      ╭▸ 
    8 │ insert into ft values (1) on conflict ((a + 1)) do nothing;
      │                                       ━━━━━━━━━
      │
      ╰ note: foreign tables can't have unique or exclusion constraints
    ");
}

#[test]
fn on_conflict_do_select_without_returning() {
    assert_snapshot!(check("
create table t (a int primary key);

insert into t values (1) on conflict (a) do select;

insert into t values (1) on conflict (a) do select for update where t.a > 0;

insert into t values (1) on conflict (a) do select returning *;
"), @"
    error: `on conflict do select` requires a `returning` clause
      ╭▸ 
    4 │ insert into t values (1) on conflict (a) do select;
      │                                          ━━━━━━━━━
      ╰╴
    help: add a `returning` clause to return the conflicting rows
      ╭╴
    4 │ insert into t values (1) on conflict (a) do select returning *;
      ╰╴                                                   +++++++++++
    error: `on conflict do select` requires a `returning` clause
      ╭▸ 
    6 │ insert into t values (1) on conflict (a) do select for update where t.a > 0;
      │                                          ━━━━━━━━━
      ╰╴
    help: add a `returning` clause to return the conflicting rows
      ╭╴
    6 │ insert into t values (1) on conflict (a) do select for update where t.a > 0 returning *;
      ╰╴                                                                            +++++++++++
    ");
}

#[test]
fn on_conflict_missing_target() {
    assert_snapshot!(check(r#"
create table t (
  id int primary key,
  b int
);

create table u (
  a int,
  "B c" int,
  unique (a, "B c")
);

create table p (
  a int unique,
  b int
);

alter table p add primary key (b);

create table n (
  a int unique deferrable,
  b int
);

create unique index on n (b) where b > 0;

insert into t values (1) on conflict do update set b = 1;

insert into u values (1) on conflict do update set a = 1;

insert into p values (1) on conflict do update set a = 1;

insert into n values (1) on conflict do update set a = 1;

insert into t values (1) on conflict do select for update returning *;

insert into t values (1) on conflict do nothing;

create function f() returns void
  language sql
  as $$
  insert into t values (1) on conflict do update set b = 1;
$$;
"#), @r#"
    error: `on conflict do update` requires a conflict target
       ╭▸ 
    27 │ insert into t values (1) on conflict do update set b = 1;
       │                          ━━━━━━━━━━━
       ╰╴
    help: specify the columns of a unique key
       ╭╴
    27 │ insert into t values (1) on conflict (id) do update set b = 1;
       ╰╴                                     ++++
    error: `on conflict do update` requires a conflict target
       ╭▸ 
    29 │ insert into u values (1) on conflict do update set a = 1;
       │                          ━━━━━━━━━━━
       ╰╴
    help: specify the columns of a unique key
       ╭╴
    29 │ insert into u values (1) on conflict (a, "B c") do update set a = 1;
       ╰╴                                     ++++++++++
    error: `on conflict do update` requires a conflict target
       ╭▸ 
    31 │ insert into p values (1) on conflict do update set a = 1;
       │                          ━━━━━━━━━━━
       ╰╴
    help: specify the columns of a unique key
       ╭╴
    31 │ insert into p values (1) on conflict (b) do update set a = 1;
       ╰╴                                     +++
    error: `on conflict do update` requires a conflict target
       ╭▸ 
    33 │ insert into n values (1) on conflict do update set a = 1;
       │                          ━━━━━━━━━━━
       │
       ╰ hint: specify a conflict target, like `on conflict (column_name)`
    error: `on conflict do select` requires a conflict target
       ╭▸ 
    35 │ insert into t values (1) on conflict do select for update returning *;
       │                          ━━━━━━━━━━━
       ╰╴
    help: specify the columns of a unique key
       ╭╴
    35 │ insert into t values (1) on conflict (id) do select for update returning *;
       ╰╴                                     ++++
    error: `on conflict do update` requires a conflict target
       ╭▸ 
    42 │   insert into t values (1) on conflict do update set b = 1;
       │                            ━━━━━━━━━━━
       │
       ╰ hint: specify a conflict target, like `on conflict (column_name)`
    "#);
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
    error: there is no unique or exclusion constraint matching the `on conflict` specification
       ╭▸ 
    14 │ insert into t (id) values (1) on conflict (a) do nothing;
       ╰╴                                          ━━━
    error: there is no unique or exclusion constraint matching the `on conflict` specification
       ╭▸ 
    17 │ insert into t (id) values (1) on conflict (a, b, c) do nothing;
       ╰╴                                          ━━━━━━━━━
    error: there is no unique or exclusion constraint matching the `on conflict` specification
       ╭▸ 
    19 │ insert into t (id) values (1) on conflict (d) do nothing;
       ╰╴                                          ━━━
    error: `on conflict` does not support deferrable unique constraints/exclusion constraints as arbiters
       ╭▸ 
    21 │ insert into t (id) values (1) on conflict (e) do nothing;
       ╰╴                                          ━━━
    error: there is no unique or exclusion constraint matching the `on conflict` specification
       ╭▸ 
    25 │ insert into t (id) values (1) on conflict (d) where d > 0 do nothing;
       ╰╴                                          ━━━
    ");
}

#[test]
fn write_identity_columns() {
    assert_snapshot!(check("
create table t (
  id int generated always as identity,
  name text,
  c int generated by default as identity
);
insert into t (id, name) values (1, 'a');
insert into t (id, name) values (default, 'a'), (1, 'b');
insert into t (id, name) values (default, 'a'), (default, 'b');
insert into t values (default, 'x');
insert into t (id) overriding system value values (2);
insert into t (id) overriding user value values (2);
insert into t (id) select 1;
insert into t (c) values (1);
update t set id = 1;
update t set id = default;
update t set (name, id) = ('x', 1);
update t set (name, id) = (select 'x', 1);
update t set c = 5;
insert into t (name) values ('a') on conflict do nothing;
alter table t alter column c  set generated always;
insert into t (c) values (1);
alter table t alter column id  set generated by default;
insert into t (id) values (1);
create view v as
  select * from t;
insert into v (name) values ('a');
"), @"
    error: cannot insert a non-default value into column `id`
      ╭▸ 
    7 │ insert into t (id, name) values (1, 'a');
      │                ━━
      │
      ├ note: `id` is an identity column defined as `generated always`
      ╰ hint: use `overriding system value` to override
    error: cannot insert a non-default value into column `id`
      ╭▸ 
    8 │ insert into t (id, name) values (default, 'a'), (1, 'b');
      │                ━━
      │
      ├ note: `id` is an identity column defined as `generated always`
      ╰ hint: use `overriding system value` to override
    error: cannot insert a non-default value into column `id`
       ╭▸ 
    13 │ insert into t (id) select 1;
       │                ━━
       │
       ├ note: `id` is an identity column defined as `generated always`
       ╰ hint: use `overriding system value` to override
    error: column `id` can only be updated to `default`
       ╭▸ 
    15 │ update t set id = 1;
       │              ━━
       │
       ╰ note: `id` is an identity column defined as `generated always`
    error: column `id` can only be updated to `default`
       ╭▸ 
    17 │ update t set (name, id) = ('x', 1);
       │                     ━━
       │
       ╰ note: `id` is an identity column defined as `generated always`
    error: column `id` can only be updated to `default`
       ╭▸ 
    18 │ update t set (name, id) = (select 'x', 1);
       │                     ━━
       │
       ╰ note: `id` is an identity column defined as `generated always`
    error: cannot insert a non-default value into column `c`
       ╭▸ 
    22 │ insert into t (c) values (1);
       │                ━
       │
       ├ note: `c` is an identity column defined as `generated always`
       ╰ hint: use `overriding system value` to override
    ");
}

#[test]
fn write_generated_columns() {
    assert_snapshot!(check("
create table t (
  name text,
  b int generated always as (1) stored
);
insert into t values ('a', 1);
insert into t (b) overriding system value values (2);
update t set b = default;
update t set b = 2;
"), @"
    error: cannot insert a non-default value into column `b`
      ╭▸ 
    6 │ insert into t values ('a', 1);
      │                            ━
      │
      ╰ note: `b` is a generated column
    error: cannot insert a non-default value into column `b`
      ╭▸ 
    7 │ insert into t (b) overriding system value values (2);
      │                ━
      │
      ╰ note: `b` is a generated column
    error: column `b` can only be updated to `default`
      ╭▸ 
    9 │ update t set b = 2;
      │              ━
      │
      ╰ note: `b` is a generated column
    ");
}

#[test]
fn on_conflict_with_rules_through_views() {
    assert_snapshot!(check("
create table t (a int primary key);
create rule r as on update to t do instead nothing;
create view v as select * from t;
insert into v values (1) on conflict do nothing;
create view v2 as select * from v where a > 0;
insert into v2 values (1) on conflict do nothing;

create table t2 (a int primary key);
create view v3 as select * from t2;
create rule r3 as on update to v3 do also notify x;
insert into v3 values (1) on conflict do nothing;
create function f() returns trigger
  language plpgsql
  as $$begin return new; end$$;
create trigger tg instead of insert on v3 for each row execute function f();
insert into v3 values (1) on conflict do nothing;

create view v4 as select distinct * from t;
insert into v4 values (1) on conflict do nothing;
create view v5 as select t.a from t join t2 using (a);
insert into v5 values (1) on conflict do nothing;
create view v6 as
  select count(*)::int as a from t;
insert into v6 values (1) on conflict do nothing;
create view v7 as
  select row_number() over ()::int as a from t;
insert into v7 values (1) on conflict do nothing;
create view v8 as
  select generate_series(1, a) as a from t;
insert into v8 values (1) on conflict do nothing;
create view v9 as
  select * from t tablesample system (50);
insert into v9 values (1) on conflict do nothing;
create view v10 as
  select * from t for update;
insert into v10 values (1) on conflict do nothing;
create table u (b int);
create view v11 as
  select (select max(t.a) from u) as a from t;
insert into v11 values (1) on conflict do nothing;
create view v12 as
  select 1 as a from t;
insert into v12 values (1) on conflict do nothing;
create view v13 as
  select t from t;
insert into v13 default values on conflict do nothing;
create view v14 as
  select a + 0 as x, a from t;
insert into v14 (a) values (1) on conflict do nothing;
"), @"
    error: `insert` with `on conflict` cannot be used with table that has `insert` or `update` rules
      ╭▸ 
    5 │ insert into v values (1) on conflict do nothing;
      │                          ━━━━━━━━━━━
      │
      ╰ note: `t` has an `on update` rule `r`
    error: `insert` with `on conflict` cannot be used with table that has `insert` or `update` rules
      ╭▸ 
    7 │ insert into v2 values (1) on conflict do nothing;
      │                           ━━━━━━━━━━━
      │
      ╰ note: `t` has an `on update` rule `r`
    error: `insert` with `on conflict` cannot be used with table that has `insert` or `update` rules
       ╭▸ 
    17 │ insert into v3 values (1) on conflict do nothing;
       │                           ━━━━━━━━━━━
       │
       ╰ note: `v3` has an `on update` rule `r3`
    error: `insert` with `on conflict` cannot be used with table that has `insert` or `update` rules
       ╭▸ 
    37 │ insert into v10 values (1) on conflict do nothing;
       │                            ━━━━━━━━━━━
       │
       ╰ note: `t` has an `on update` rule `r`
    error: `insert` with `on conflict` cannot be used with table that has `insert` or `update` rules
       ╭▸ 
    50 │ insert into v14 (a) values (1) on conflict do nothing;
       │                                ━━━━━━━━━━━
       │
       ╰ note: `t` has an `on update` rule `r`
    ");
}

#[test]
fn assignment_type_mismatch() {
    assert_snapshot!(check("
create table t (a int, b text, c int[], d bool);
insert into t (a) values (now());
insert into t (a) values ('1'::text);
insert into t (a) values (1.5);
insert into t (b) values (1);
insert into t (a, b) values (1, 'x'), (now() + interval '1 day', 'y');
insert into t select now();
insert into t (d) values (1);
insert into t (c) values (array['a'::text]);
insert into t (a) values ('1');
update t set a = '1'::text;
update t set (a, b) = (now(), 1);
create table u (a int default '1'::text);
create table v (a int default 1.5);
insert into t values (1, 'x', default, true);
alter table t alter column d set default 0;
"), @"
    error: column `a` is of type `int4` but expression is of type `timestamptz`
      ╭▸ 
    3 │ insert into t (a) values (now());
      │                           ━━━━━
      │
      ╰ hint: rewrite the expression to produce the column's type
    error: column `a` is of type `int4` but expression is of type `text`
      ╭▸ 
    4 │ insert into t (a) values ('1'::text);
      │                           ━━━━━━━━━
      ╰╴
    help: cast the expression to the column's type
      ╭╴
    4 │ insert into t (a) values ('1'::text::int4);
      ╰╴                                   ++++++
    error: column `a` is of type `int4` but expression is of type `timestamptz`
      ╭▸ 
    7 │ insert into t (a, b) values (1, 'x'), (now() + interval '1 day', 'y');
      │                                        ━━━━━━━━━━━━━━━━━━━━━━━━
      │
      ╰ hint: rewrite the expression to produce the column's type
    error: column `a` is of type `int4` but expression is of type `timestamptz`
      ╭▸ 
    8 │ insert into t select now();
      │                      ━━━━━
      │
      ╰ hint: rewrite the expression to produce the column's type
    error: column `d` is of type `bool` but expression is of type `int4`
      ╭▸ 
    9 │ insert into t (d) values (1);
      │                           ━
      ╰╴
    help: cast the expression to the column's type
      ╭╴
    9 │ insert into t (d) values (1::bool);
      ╰╴                           ++++++
    error: column `c` is of type `int4[]` but expression is of type `text[]`
       ╭▸ 
    10 │ insert into t (c) values (array['a'::text]);
       │                           ━━━━━━━━━━━━━━━━
       ╰╴
    help: cast the expression to the column's type
       ╭╴
    10 │ insert into t (c) values (array['a'::text]::int4[]);
       ╰╴                                          ++++++++
    error: column `a` is of type `int4` but expression is of type `text`
       ╭▸ 
    12 │ update t set a = '1'::text;
       │                  ━━━━━━━━━
       ╰╴
    help: cast the expression to the column's type
       ╭╴
    12 │ update t set a = '1'::text::int4;
       ╰╴                          ++++++
    error: column `a` is of type `int4` but expression is of type `timestamptz`
       ╭▸ 
    13 │ update t set (a, b) = (now(), 1);
       │                        ━━━━━
       │
       ╰ hint: rewrite the expression to produce the column's type
    error: column `a` is of type `int4` but default expression is of type `text`
       ╭▸ 
    14 │ create table u (a int default '1'::text);
       │                               ━━━━━━━━━
       ╰╴
    help: cast the expression to the column's type
       ╭╴
    14 │ create table u (a int default '1'::text::int4);
       ╰╴                                       ++++++
    error: column `d` is of type `bool` but default expression is of type `int4`
       ╭▸ 
    17 │ alter table t alter column d set default 0;
       │                                          ━
       ╰╴
    help: cast the expression to the column's type
       ╭╴
    17 │ alter table t alter column d set default 0::bool;
       ╰╴                                          ++++++
    ");
}

#[test]
fn insert_parenthesized_values_literals() {
    assert_snapshot!(check("
create table t (a int);
insert into t (a) (values ('2'));
insert into t (a) (values ('2'), ('3'));
insert into t (a) select '2';
insert into t (a) (select '2');
"), @"no diagnostics");
}
