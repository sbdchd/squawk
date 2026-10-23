use insta::assert_snapshot;

use super::goto;

#[test]
fn constraint_definition_returns_self() {
    assert_snapshot!(goto("
create table t (a int constraint c$0 check (a > 0));
"), @"
    info: goto definition
      ╭▸ file0.sql:2:34
      │
    2 │ create table t (a int constraint c check (a > 0));
      │                                  ┬
      │                                  │
      │                                  1. source
      ╰╴                                 2. destination
    ");
}

#[test]
fn check_constraint_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  check (a$0 > 0)
);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   a int,
      │   ─ 2. destination
    4 │   check (a > 0)
      ╰╴         ─ 1. source
    ");
}

#[test]
fn column_check_constraint_column() {
    assert_snapshot!(goto("
create table t (a int check (a$0 > 0));
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int check (a > 0));
      │                 ┬            ─ 1. source
      │                 │
      ╰╴                2. destination
    ");
}

#[test]
fn default_constraint_function() {
    assert_snapshot!(goto("
create function f() returns int
  as 'select 1';
create table t (a int default f$0());
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create function f() returns int
      │                 ─ 2. destination
    3 │   as 'select 1';
    4 │ create table t (a int default f());
      ╰╴                              ─ 1. source
    ");
}

#[test]
fn primary_key_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int,
  primary key (a, b$0)
);
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int,
      │   ─ 2. destination
    5 │   primary key (a, b)
      ╰╴                  ─ 1. source
    ");
}

#[test]
fn unique_include_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int,
  unique (a) include (b$0)
);
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   b int,
      │   ─ 2. destination
    5 │   unique (a) include (b)
      ╰╴                      ─ 1. source
    ");
}

#[test]
fn foreign_key_local_column() {
    assert_snapshot!(goto("
create table u (x int primary key);
create table t (
  a int,
  foreign key (a$0) references u (x)
);
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   a int,
      │   ─ 2. destination
    5 │   foreign key (a) references u (x)
      ╰╴               ─ 1. source
    ");
}

#[test]
fn foreign_key_table() {
    assert_snapshot!(goto("
create table u (x int primary key);
create table t (
  a int,
  foreign key (a) references u$0 (x)
);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table u (x int primary key);
      │              ─ 2. destination
      ‡
    5 │   foreign key (a) references u (x)
      ╰╴                             ─ 1. source
    ");
}

#[test]
fn foreign_key_referenced_column() {
    assert_snapshot!(goto("
create table u (x int primary key);
create table t (
  a int,
  foreign key (a) references u (x$0)
);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table u (x int primary key);
      │                 ─ 2. destination
      ‡
    5 │   foreign key (a) references u (x)
      ╰╴                                ─ 1. source
    ");
}

#[test]
fn foreign_key_self_reference() {
    assert_snapshot!(goto("
create table t (
  id int primary key,
  parent int references t$0 (id)
);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:14
      │
    2 │ create table t (
      │              ─ 2. destination
    3 │   id int primary key,
    4 │   parent int references t (id)
      ╰╴                        ─ 1. source
    ");
}

#[test]
fn references_column() {
    assert_snapshot!(goto("
create table u (x int primary key);
create table t (a int references u (x$0));
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table u (x int primary key);
      │                 ─ 2. destination
    3 │ create table t (a int references u (x));
      ╰╴                                    ─ 1. source
    ");
}

#[test]
fn foreign_key_on_delete_set_null_column() {
    assert_snapshot!(goto("
create table u (x int primary key);
create table t (
  a int,
  b int,
  foreign key (a) references u (x) on delete set null (a$0)
);
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   a int,
      │   ─ 2. destination
    5 │   b int,
    6 │   foreign key (a) references u (x) on delete set null (a)
      ╰╴                                                       ─ 1. source
    ");
}

#[test]
fn exclude_constraint_column() {
    assert_snapshot!(goto("
create table t (
  a int,
  exclude (a$0 with =)
);
"), @"
    info: goto definition
      ╭▸ file0.sql:3:3
      │
    3 │   a int,
      │   ─ 2. destination
    4 │   exclude (a with =)
      ╰╴           ─ 1. source
    ");
}

#[test]
fn alter_table_add_constraint_column() {
    assert_snapshot!(goto("
create table t (a int);
alter table t add constraint c check (a$0 > 0);
"), @"
    info: goto definition
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. destination
    3 │ alter table t add constraint c check (a > 0);
      ╰╴                                      ─ 1. source
    ");
}

#[test]
fn alter_table_drop_constraint() {
    assert_snapshot!(goto("
create table t (a int constraint c check (a > 0));
alter table t drop constraint c$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:34
      │
    2 │ create table t (a int constraint c check (a > 0));
      │                                  ─ 2. destination
    3 │ alter table t drop constraint c;
      ╰╴                              ─ 1. source
    ");
}

#[test]
fn alter_table_drop_generated_primary_key() {
    assert_snapshot!(goto("
create table t (a int primary key);
alter table t drop constraint t_pkey$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:23
      │
    2 │ create table t (a int primary key);
      │                       ─────────── 2. destination
    3 │ alter table t drop constraint t_pkey;
      ╰╴                              ────── 1. source
    ");
}

#[test]
fn alter_table_drop_generated_unique() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int,
  unique (a, b)
);
alter table t drop constraint t_a_b_key$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:3
      │
    5 │   unique (a, b)
      │   ───────────── 2. destination
    6 │ );
    7 │ alter table t drop constraint t_a_b_key;
      ╰╴                              ───────── 1. source
    ");
}

#[test]
fn alter_table_drop_generated_column_check() {
    assert_snapshot!(goto("
create table t (a int check (a > 0));
alter table t drop constraint t_a_check$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:23
      │
    2 │ create table t (a int check (a > 0));
      │                       ───────────── 2. destination
    3 │ alter table t drop constraint t_a_check;
      ╰╴                              ───────── 1. source
    ");
}

#[test]
fn alter_table_drop_generated_table_check() {
    assert_snapshot!(goto("
create table t (
  a int,
  b int,
  check (a > b)
);
alter table t drop constraint t_check$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:5:3
      │
    5 │   check (a > b)
      │   ───────────── 2. destination
    6 │ );
    7 │ alter table t drop constraint t_check;
      ╰╴                              ─────── 1. source
    ");
}

#[test]
fn alter_table_drop_generated_check_deduplicated() {
    assert_snapshot!(goto("
create table t (
  a int check (a > 0),
  check (a < 10)
);
alter table t drop constraint t_a_check1$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   check (a < 10)
      │   ────────────── 2. destination
    5 │ );
    6 │ alter table t drop constraint t_a_check1;
      ╰╴                              ────────── 1. source
    ");
}

#[test]
fn alter_table_drop_generated_foreign_key() {
    assert_snapshot!(goto("
create table u (x int primary key);
create table t (a int references u);
alter table t drop constraint t_a_fkey$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:23
      │
    3 │ create table t (a int references u);
      │                       ──────────── 2. destination
    4 │ alter table t drop constraint t_a_fkey;
      ╰╴                              ──────── 1. source
    ");
}

#[test]
fn alter_table_drop_generated_not_null() {
    assert_snapshot!(goto("
create table t (a int not null);
alter table t drop constraint t_a_not_null$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:23
      │
    2 │ create table t (a int not null);
      │                       ──────── 2. destination
    3 │ alter table t drop constraint t_a_not_null;
      ╰╴                              ──────────── 1. source
    ");
}

#[test]
fn alter_table_drop_primary_key_not_null() {
    assert_snapshot!(goto("
create table t (
  a int,
  primary key (a)
);
alter table t drop constraint t_a_not_null$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   primary key (a)
      │   ─────────────── 2. destination
    5 │ );
    6 │ alter table t drop constraint t_a_not_null;
      ╰╴                              ──────────── 1. source
    ");
}

#[test]
fn alter_table_drop_generated_name_truncated() {
    assert_snapshot!(goto("
create table a_very_long_table_name_that_goes_on_and_on (
  a_very_long_column_name_too int unique
);
alter table a_very_long_table_name_that_goes_on_and_on
  drop constraint a_very_long_table_name_that_goe_a_very_long_column_name_too_key$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:35
      │
    3 │     a_very_long_column_name_too int unique
      │ ┌───────────────────────────────────┘
    4 │ │ );
      │ └─┘ 2. destination
    5 │   alter table a_very_long_table_name_that_goes_on_and_on
    6 │     drop constraint a_very_long_table_name_that_goe_a_very_long_column_name_too_key;
      ╰╴                    ─────────────────────────────────────────────────────────────── 1. source
    ");
}

#[test]
fn alter_table_validate_constraint() {
    assert_snapshot!(goto("
create table t (a int);
alter table t add constraint c check (a > 0) not valid;
alter table t validate constraint c$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:30
      │
    3 │ alter table t add constraint c check (a > 0) not valid;
      │                              ─ 2. destination
    4 │ alter table t validate constraint c;
      ╰╴                                  ─ 1. source
    ");
}

#[test]
fn alter_table_alter_constraint() {
    assert_snapshot!(goto("
create table u (x int primary key);
create table t (a int constraint fk references u (x));
alter table t alter constraint fk$0 deferrable;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:34
      │
    3 │ create table t (a int constraint fk references u (x));
      │                                  ── 2. destination
    4 │ alter table t alter constraint fk deferrable;
      ╰╴                               ── 1. source
    ");
}

#[test]
fn alter_table_add_constraint_using_index() {
    assert_snapshot!(goto("
create table t (a int);
create unique index idx on t (a);
alter table t add constraint c unique using index idx$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:21
      │
    3 │ create unique index idx on t (a);
      │                     ─── 2. destination
    4 │ alter table t add constraint c unique using index idx;
      ╰╴                                                  ─── 1. source
    ");
}

#[test]
fn dropped_constraint() {
    assert_snapshot!(goto("
create table t (a int constraint c check (a > 0));
alter table t drop constraint c;
alter table t drop constraint if exists c$0;
"), @"no definition");
}

#[test]
fn constraint_on_other_table() {
    assert_snapshot!(goto("
create table t (a int constraint c check (a > 0));
create table u (a int);
alter table u drop constraint if exists c$0;
"), @"no definition");
}

#[test]
fn constraint_drop_rolled_back() {
    assert_snapshot!(goto("
create table t (a int constraint c check (a > 0));
begin;
alter table t drop constraint c;
rollback;
alter table t drop constraint c$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:34
      │
    2 │ create table t (a int constraint c check (a > 0));
      │                                  ─ 2. destination
      ‡
    6 │ alter table t drop constraint c;
      ╰╴                              ─ 1. source
    ");
}

#[test]
fn on_conflict_on_constraint() {
    assert_snapshot!(goto("
create table t (a int constraint t_a unique);
insert into t values (1) on conflict on constraint t_a$0 do nothing;
"), @"
    info: goto definition
      ╭▸ file0.sql:2:34
      │
    2 │ create table t (a int constraint t_a unique);
      │                                  ─── 2. destination
    3 │ insert into t values (1) on conflict on constraint t_a do nothing;
      ╰╴                                                   ─── 1. source
    ");
}

#[test]
fn set_constraints() {
    assert_snapshot!(goto("
create table u (x int primary key);
create table t (a int constraint fk references u (x) deferrable);
set constraints fk$0 deferred;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:34
      │
    3 │ create table t (a int constraint fk references u (x) deferrable);
      │                                  ── 2. destination
    4 │ set constraints fk deferred;
      ╰╴                ── 1. source
    ");
}

#[test]
fn set_constraints_with_schema() {
    assert_snapshot!(goto("
create table u (x int primary key);
create table s.t (a int constraint fk references u (x) deferrable);
set constraints s.fk$0 deferred;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:36
      │
    3 │ create table s.t (a int constraint fk references u (x) deferrable);
      │                                    ── 2. destination
    4 │ set constraints s.fk deferred;
      ╰╴                  ── 1. source
    ");
}

#[test]
fn alter_table_drop_generated_primary_key_avoids_constraint_name() {
    assert_snapshot!(goto("
create table t (
  a int constraint t_pkey check (a > 0),
  primary key (a)
);
alter table t drop constraint t_pkey1$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:4:3
      │
    4 │   primary key (a)
      │   ─────────────── 2. destination
    5 │ );
    6 │ alter table t drop constraint t_pkey1;
      ╰╴                              ─────── 1. source
    ");
}

#[test]
fn comment_on_constraint_ref() {
    assert_snapshot!(goto("
create table t (a int constraint c check (a > 0));
comment on constraint c$0 on t is 'x';
"), @"
    info: goto definition
      ╭▸ file0.sql:2:34
      │
    2 │ create table t (a int constraint c check (a > 0));
      │                                  ─ 2. destination
    3 │ comment on constraint c on t is 'x';
      ╰╴                      ─ 1. source
    ");
}

#[test]
fn alter_table_renamed_constraint_ref() {
    assert_snapshot!(goto("
create table t (a int constraint c check (a > 0));
alter table t rename constraint c to c2;
alter table t drop constraint c2$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:38
      │
    3 │ alter table t rename constraint c to c2;
      │                                      ── 2. destination
    4 │ alter table t drop constraint c2;
      ╰╴                              ── 1. source
    ");
}

#[test]
fn renamed_index_constraint_ref() {
    assert_snapshot!(goto("
create table t (a int constraint u unique);
alter index u rename to u2;
alter table t drop constraint u2$0;
"), @"
    info: goto definition
      ╭▸ file0.sql:3:25
      │
    3 │ alter index u rename to u2;
      │                         ── 2. destination
    4 │ alter table t drop constraint u2;
      ╰╴                              ── 1. source
    ");
}
