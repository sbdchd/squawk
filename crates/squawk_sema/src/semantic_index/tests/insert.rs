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
fn on_conflict_expression_target() {
    assert_snapshot!(check(r#"
create table t (email text);
create unique index on t (email);

insert into t values ('a') on conflict (lower(email)) do nothing;

insert into t values ('a') on conflict (email collate "C") do nothing;

insert into t values ('a') on conflict (email text_pattern_ops) do nothing;

insert into t values ('a') on conflict (email text_ops) do nothing;
"#), @r#"
    error: there is no unique or exclusion constraint matching the `on conflict` specification
      ╭▸ 
    5 │ insert into t values ('a') on conflict (lower(email)) do nothing;
      ╰╴                                       ━━━━━━━━━━━━━━
    error: there is no unique or exclusion constraint matching the `on conflict` specification
      ╭▸ 
    7 │ insert into t values ('a') on conflict (email collate "C") do nothing;
      ╰╴                                       ━━━━━━━━━━━━━━━━━━━
    error: there is no unique or exclusion constraint matching the `on conflict` specification
      ╭▸ 
    9 │ insert into t values ('a') on conflict (email text_pattern_ops) do nothing;
      ╰╴                                       ━━━━━━━━━━━━━━━━━━━━━━━━
    "#);
}

#[test]
fn on_conflict_builtin_table_target() {
    assert_snapshot!(check(r#"
insert into information_schema.sql_features (feature_id) values ('x')
on conflict (feature_id) do nothing;
"#), @r#"
    error: there is no unique or exclusion constraint matching the `on conflict` specification
      ╭▸ 
    3 │ on conflict (feature_id) do nothing;
      ╰╴            ━━━━━━━━━━━━
    "#);
}

#[test]
fn on_conflict_rules_through_view() {
    assert_snapshot!(check(r#"
create table t (a int primary key);
create rule r as on update
  to t
  do instead nothing;
create view v as
  select * from t;

insert into v values (1) on conflict do nothing;
"#), @r#"
    error: `insert` with `on conflict` cannot be used with table that has `insert` or `update` rules
      ╭▸ 
    9 │ insert into v values (1) on conflict do nothing;
      │                          ━━━━━━━━━━━
      │
      ╰ note: `t` has an `on update` rule `r`
    "#);
}

#[test]
fn on_conflict_disabled_insert_rule() {
    assert_snapshot!(check(r#"
create table t (a int primary key);
create rule r as on insert
  to t
  do also notify x;
alter table t disable rule r;

insert into t values (1) on conflict do nothing;
"#), @"no diagnostics");
}

#[test]
fn on_conflict_view_instead_of_trigger_with_update_rule() {
    assert_snapshot!(check(r#"
create table t (a int primary key);
create view v as
  select * from t;

create function f() returns trigger
  language plpgsql
  as $$begin return new; end$$;

create trigger tg
  instead of insert
  on v
  for each row
  execute function f();
create rule r as on update
  to v
  do also notify x;

insert into v values (1) on conflict do nothing;
"#), @r#"
    error: `insert` with `on conflict` cannot be used with table that has `insert` or `update` rules
       ╭▸ 
    19 │ insert into v values (1) on conflict do nothing;
       │                          ━━━━━━━━━━━
       │
       ╰ note: `v` has an `on update` rule `r`
    "#);
}

#[test]
fn on_conflict_system_catalog_missing_target() {
    assert_snapshot!(check(r#"
insert into pg_class (oid) values (1) on conflict do update set relname = 'x';
"#), @r#"
    error: `on conflict do update` requires a conflict target
      ╭▸ 
    2 │ insert into pg_class (oid) values (1) on conflict do update set relname = 'x';
      │                                       ━━━━━━━━━━━
      │
      ╰ hint: specify a conflict target, like `on conflict (column_name)`
    "#);
}

#[test]
fn on_conflict_toast_table() {
    assert_snapshot!(check(r#"
insert into pg_toast.pg_toast_1255 values (1, 1, 'x') on conflict do nothing;
"#), @r#"
    error: `on conflict` is not supported with system catalog tables
      ╭▸ 
    2 │ insert into pg_toast.pg_toast_1255 values (1, 1, 'x') on conflict do nothing;
      ╰╴                                                      ━━━━━━━━━━━
    "#);
}
