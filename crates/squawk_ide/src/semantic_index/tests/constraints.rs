use insta::assert_snapshot;

use super::check;

#[test]
fn constraint_unknown_columns() {
    assert_snapshot!(check("
create table u(x int);
create table t(
  a int check (b > 0),
  primary key (c),
  unique (a) include (d),
  foreign key (e) references u (f)
);
"), @"
    error: column `b` does not exist
      ╭▸ 
    4 │   a int check (b > 0),
      ╰╴               ━
    error: column `c` does not exist
      ╭▸ 
    5 │   primary key (c),
      ╰╴               ━
    error: column `d` does not exist
      ╭▸ 
    6 │   unique (a) include (d),
      ╰╴                      ━
    error: column `e` does not exist
      ╭▸ 
    7 │   foreign key (e) references u (f)
      ╰╴               ━
    error: column `f` does not exist
      ╭▸ 
    7 │   foreign key (e) references u (f)
      ╰╴                                ━
    ");
}

#[test]
fn constraint_unknown_referenced_table() {
    assert_snapshot!(check("
create table t(a int references u(x), foreign key (a) references v);
"), @"
    error: relation `u` does not exist
      ╭▸ 
    2 │ create table t(a int references u(x), foreign key (a) references v);
      ╰╴                                ━
    error: relation `v` does not exist
      ╭▸ 
    2 │ create table t(a int references u(x), foreign key (a) references v);
      ╰╴                                                                 ━
    ");
}

#[test]
fn drop_constraint_unknown() {
    assert_snapshot!(check("
create table t(a int primary key);
alter table t drop constraint c;
alter table t drop constraint if exists d;
alter table t drop constraint t_pkey;
alter table t drop constraint t_pkey;
"), @"
    error: constraint `c` does not exist
      ╭▸ 
    3 │ alter table t drop constraint c;
      ╰╴                              ━
    error: constraint `t_pkey` does not exist
      ╭▸ 
    6 │ alter table t drop constraint t_pkey;
      ╰╴                              ━━━━━━
    ");
}

#[test]
fn drop_constraint_unknown_table() {
    assert_snapshot!(check("
alter table if exists t drop constraint c;
"), @"no diagnostics");
}

#[test]
fn drop_constraint_inherited_table() {
    assert_snapshot!(check("
create table p(a int check (a > 0));
create table c() inherits (p);
create table l(like p including all);
alter table c drop constraint p_a_check;
alter table l drop constraint p_a_check;
"), @"no diagnostics");
}

#[test]
fn on_conflict_on_constraint_unknown() {
    assert_snapshot!(check("
create table t(a int);
insert into t values (1) on conflict on constraint c do nothing;
"), @"
    error: constraint `c` does not exist
      ╭▸ 
    3 │ insert into t values (1) on conflict on constraint c do nothing;
      ╰╴                                                   ━
    ");
}

#[test]
fn set_constraints_unknown() {
    assert_snapshot!(check("
set constraints c deferred;
set constraints all immediate;
"), @"
    error: constraint `c` does not exist
      ╭▸ 
    2 │ set constraints c deferred;
      ╰╴                ━
    ");
}

#[test]
fn constraint_created_in_rolled_back_transaction() {
    assert_snapshot!(check("
create table t(a int);
begin;
alter table t add constraint c check (a > 0);
rollback;
alter table t drop constraint c;
"), @"
    error: constraint `c` does not exist
      ╭▸ 
    6 │ alter table t drop constraint c;
      ╰╴                              ━
    ");
}

#[test]
fn duplicate_constraint() {
    assert_snapshot!(check("
create table t (a int constraint c check (a > 0), b int);
alter table t add constraint c check (b > 0);
alter table t add constraint t unique (a);
create table u (a int constraint c check (a > 0));
"), @"
    error: constraint `c` for relation `t` already exists
      ╭▸ 
    3 │ alter table t add constraint c check (b > 0);
      ╰╴                             ━
    error: relation `t` already exists
      ╭▸ 
    4 │ alter table t add constraint t unique (a);
      ╰╴                             ━
    ");
}

#[test]
fn alter_table_rename_constraint() {
    assert_snapshot!(check("
create table t (a int constraint c check (a > 0), b int constraint d check (b > 0));
alter table t rename constraint c to c2;
alter table t drop constraint c;
alter table t rename constraint c2 to d;
alter table t rename constraint nope to x;
begin;
alter table t rename constraint c2 to c3;
rollback;
alter table t drop constraint c2;
alter table t drop constraint c3;
"), @"
    error: constraint `c` does not exist
      ╭▸ 
    4 │ alter table t drop constraint c;
      ╰╴                              ━
    error: constraint `nope` does not exist
      ╭▸ 
    6 │ alter table t rename constraint nope to x;
      ╰╴                                ━━━━
    error: constraint `c3` does not exist
       ╭▸ 
    11 │ alter table t drop constraint c3;
       ╰╴                              ━━
    error: constraint `d` for relation `t` already exists
      ╭▸ 
    5 │ alter table t rename constraint c2 to d;
      ╰╴                                      ━
    ");
}

#[test]
fn rename_index_backed_constraint() {
    assert_snapshot!(check("
create table t (a int constraint pk primary key, b int constraint u unique);
alter table t rename constraint pk to pk2;
drop index pk;
alter index pk2 set tablespace pg_default;
alter index u rename to u2;
alter table t drop constraint u;
alter table t drop constraint u2;
drop index u2;
alter table t rename constraint pk2 to t;
"), @"
    error: index `pk` does not exist
      ╭▸ 
    4 │ drop index pk;
      ╰╴           ━━
    error: constraint `u` does not exist
      ╭▸ 
    7 │ alter table t drop constraint u;
      ╰╴                              ━
    error: index `u2` does not exist
      ╭▸ 
    9 │ drop index u2;
      ╰╴           ━━
    error: relation `t` already exists
       ╭▸ 
    10 │ alter table t rename constraint pk2 to t;
       ╰╴                                       ━
    ");
}

#[test]
fn exclusion_constraint_operators() {
    assert_snapshot!(check("
create schema s;
create operator s.=== (leftarg = int, rightarg = int, function = int4eq);
create table t (
  a int,
  r tstzrange,
  exclude using gist (a with =, r with &&),
  exclude (a with operator(s.===)),
  exclude (a with operator(public.===)),
  exclude (a with !!!),
  exclude using gist (a with &&)
);
"), @"
    error: operator `===` does not exist
      ╭▸ 
    9 │   exclude (a with operator(public.===)),
      ╰╴                                  ━━━
    error: operator `!!!` does not exist
       ╭▸ 
    10 │   exclude (a with !!!),
       ╰╴                  ━━━
    error: operator `&&` does not exist
       ╭▸ 
    11 │   exclude using gist (a with &&)
       ╰╴                             ━━
    ");
}

#[test]
fn multiple_primary_keys() {
    assert_snapshot!(check("
create table t (
  a int primary key,
  b int primary key
);
create table u (
  a int,
  b int,
  primary key (a),
  primary key (b)
);

create table v (
  a int primary key,
  b int
);
alter table v add primary key (b);

create table w (
  a int primary key,
  b int
);
alter table w drop constraint w_pkey, add primary key (b);

create table x (
  a int primary key,
  b int
);
alter table x drop constraint x_pkey;
alter table x add primary key (b);

create table p (a int primary key);
create table c (b int primary key) inherits (p);

create table l1 (
  like v including all excluding indexes,
  primary key (b)
);
create table l2 (
  like v including all,
  primary key (b)
);
create table l3 (
  primary key (b),
  like v including indexes
);

create table l4 (like v including indexes);
alter table l4 add primary key (b);

create table l5 (like v including indexes);
alter table l5 drop constraint l5_pkey;
alter table l5 add primary key (b);

create table pp (
  a int,
  b int,
  primary key (a)
)
  partition by range (a);
create table pc
  partition of pp (primary key (b))
  for values from (1) to (2);

create table pd
  partition of pp
  for values from (2) to (3);
alter table pd add primary key (b);

create table pm
  partition of pp
  for values from (3) to (4)
  partition by range (a);
create table pg
  partition of pm (primary key (b))
  for values from (3) to (4);

create table l6 (
  like pd including indexes,
  primary key (b)
);
"), @"
    error: multiple primary keys for table `t` are not allowed
      ╭▸ 
    4 │     b int primary key
      │ ┏━━━━━━━━━┛
    5 │ ┃ );
      ╰╴┗━┛
    error: multiple primary keys for table `u` are not allowed
       ╭▸ 
    10 │   primary key (b)
       ╰╴  ━━━━━━━━━━━━━━━
    error: multiple primary keys for table `v` are not allowed
       ╭▸ 
    17 │ alter table v add primary key (b);
       ╰╴                  ━━━━━━━━━━━━━━━
    error: multiple primary keys for table `l2` are not allowed
       ╭▸ 
    41 │   primary key (b)
       ╰╴  ━━━━━━━━━━━━━━━
    error: multiple primary keys for table `l3` are not allowed
       ╭▸ 
    44 │   primary key (b),
       ╰╴  ━━━━━━━━━━━━━━━
    error: multiple primary keys for table `l4` are not allowed
       ╭▸ 
    49 │ alter table l4 add primary key (b);
       ╰╴                   ━━━━━━━━━━━━━━━
    error: multiple primary keys for table `pc` are not allowed
       ╭▸ 
    62 │   partition of pp (primary key (b))
       ╰╴                   ━━━━━━━━━━━━━━━
    error: multiple primary keys for table `pd` are not allowed
       ╭▸ 
    68 │ alter table pd add primary key (b);
       ╰╴                   ━━━━━━━━━━━━━━━
    error: multiple primary keys for table `pg` are not allowed
       ╭▸ 
    75 │   partition of pm (primary key (b))
       ╰╴                   ━━━━━━━━━━━━━━━
    error: multiple primary keys for table `l6` are not allowed
       ╭▸ 
    80 │   primary key (b)
       ╰╴  ━━━━━━━━━━━━━━━
    ");
}

#[test]
fn conflicting_nullability() {
    assert_snapshot!(check("
create table n1 (a int primary key null);
create table n2 (a int null primary key);
create table n3 (a int null not null);
create table n4 (a int not null null);
create table n5 (a int generated always as identity null);
create table n6 (a int null generated always as identity);
create table n7 (
  a int null null,
  b int not null not null,
  c int primary key not null,
  d int not null unique
);
alter table n7 add column e int not null null;
"), @"
    error: conflicting NULL/NOT NULL declarations for column `a` of table `n1`
      ╭▸ 
    2 │ create table n1 (a int primary key null);
      ╰╴                                   ━━━━
    error: conflicting NULL/NOT NULL declarations for column `a` of table `n2`
      ╭▸ 
    3 │ create table n2 (a int null primary key);
      ╰╴                            ━━━━━━━━━━━
    error: conflicting NULL/NOT NULL declarations for column `a` of table `n3`
      ╭▸ 
    4 │ create table n3 (a int null not null);
      ╰╴                            ━━━━━━━━
    error: conflicting NULL/NOT NULL declarations for column `a` of table `n4`
      ╭▸ 
    5 │ create table n4 (a int not null null);
      ╰╴                                ━━━━
    error: conflicting NULL/NOT NULL declarations for column `a` of table `n5`
      ╭▸ 
    6 │ create table n5 (a int generated always as identity null);
      ╰╴                                                    ━━━━
    error: conflicting NULL/NOT NULL declarations for column `a` of table `n6`
      ╭▸ 
    7 │ create table n6 (a int null generated always as identity);
      ╰╴                            ━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    error: conflicting NULL/NOT NULL declarations for column `e` of table `n7`
       ╭▸ 
    14 │ alter table n7 add column e int not null null;
       ╰╴                                         ━━━━
    ");
}
