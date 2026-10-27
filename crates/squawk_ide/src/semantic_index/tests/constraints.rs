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
fn drop_key_column_drops_constraint() {
    assert_snapshot!(check("
create table d (
  a int primary key,
  b int
);
alter table d drop column a;
alter table d add primary key (b);

create table r (
  a int primary key,
  b int,
  c int
);
alter table r rename column a to x;
alter table r drop column x;
alter table r add primary key (b);

create table l (
  like r including indexes,
  z int
);
alter table l drop column b;
alter table l add primary key (c);

create table u (
  a int,
  b int,
  primary key (a, b)
);
alter table u drop column a;
alter table u add primary key (b);

create table k (
  a int primary key,
  b int
);
begin;
alter table k rename column a to x;
rollback;
alter table k drop column a;
alter table k add primary key (b);

create table w (
  a int,
  b int
);
create unique index w_a on w (a);
alter table w add primary key using index w_a;
alter table w rename column a to x;
alter table w drop column x;
alter table w add primary key (b);

create table pp (
  a int primary key,
  b int
)
  partition by range (a);
create table pc
  partition of pp
  for values from (1) to (2);
create table lp (like pc including indexes);
alter table lp drop column a;
alter table lp add primary key (b);
"), @"no diagnostics");
}

#[test]
fn rename_builtin_table_column() {
    assert_snapshot!(check("
create index on pg_class (relname);
alter table pg_class rename column relname to x;
"), @"no diagnostics");
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

#[test]
fn default_column_reference() {
    assert_snapshot!(check("
create table t (
  a int,
  b int default a,
  c int default t.a,
  d int default (t).a,
  e int default t
);
alter table t add column f int default b + 1;
alter table t alter column a set default c;
create view v as
  select a from t;
alter view v alter column a set default a;
create function f(x int, y int default x) returns int
  language sql
  as 'select 1';
create domain dm as int default 1;
create table ok (
  a int default 1,
  b timestamptz default now(),
  c text default current_user
);
create table pt (a int) partition by range (a);
create table pt_1
  partition of pt
  for values from (a) to (10);
"), @"
    error: column `c` does not exist
       ╭▸ 
    10 │ alter table t alter column a set default c;
       ╰╴                                         ━
    error: column `a` does not exist
       ╭▸ 
    13 │ alter view v alter column a set default a;
       ╰╴                                        ━
    error: column `x` does not exist
       ╭▸ 
    14 │ create function f(x int, y int default x) returns int
       ╰╴                                       ━
    error: column `a` does not exist
       ╭▸ 
    26 │   for values from (a) to (10);
       ╰╴                   ━
    error: cannot use column reference in DEFAULT expression
      ╭▸ 
    4 │   b int default a,
      ╰╴                ━
    error: cannot use column reference in DEFAULT expression
      ╭▸ 
    5 │   c int default t.a,
      ╰╴                ━━━
    error: cannot use column reference in DEFAULT expression
      ╭▸ 
    6 │   d int default (t).a,
      ╰╴                 ━
    error: cannot use column reference in DEFAULT expression
      ╭▸ 
    7 │   e int default t
      ╰╴                ━
    error: cannot use column reference in DEFAULT expression
      ╭▸ 
    9 │ alter table t add column f int default b + 1;
      ╰╴                                       ━
    ");
}

#[test]
fn generated_column_reference() {
    assert_snapshot!(check("
create table g1 (
  a int,
  b int generated always as (a + 1) stored,
  c int generated always as (b + 1) stored
);
create table g2 (
  a int,
  c int generated always as (b + 1) stored,
  b int generated always as (a + 1) stored
);
create table g3 (a int generated always as (a + 1) stored);
create table g4 (
  a int,
  b int generated always as (a + 1) virtual,
  c int generated always as (g4.b + 1) stored
);
create table g5 (
  a int,
  b int generated always as (a + 1) stored,
  c int generated always as (a + 2) stored
);
alter table g5 add column d int generated always as (b * 2) stored;
alter table g5 alter column c set expression as(b + 2);
create table g6 (
  a int,
  b int generated always as identity,
  c int generated always as (a + b) stored,
  d int default 1,
  e int generated always as (d + 1) stored
);
"), @"
    error: cannot use generated column `b` in column generation expression
      ╭▸ 
    5 │   c int generated always as (b + 1) stored
      ╰╴                             ━
    error: cannot use generated column `b` in column generation expression
      ╭▸ 
    9 │   c int generated always as (b + 1) stored,
      ╰╴                             ━
    error: cannot use generated column `a` in column generation expression
       ╭▸ 
    12 │ create table g3 (a int generated always as (a + 1) stored);
       ╰╴                                            ━
    error: cannot use generated column `b` in column generation expression
       ╭▸ 
    16 │   c int generated always as (g4.b + 1) stored
       ╰╴                                ━
    error: cannot use generated column `b` in column generation expression
       ╭▸ 
    23 │ alter table g5 add column d int generated always as (b * 2) stored;
       ╰╴                                                     ━
    error: cannot use generated column `b` in column generation expression
       ╭▸ 
    24 │ alter table g5 alter column c set expression as(b + 2);
       ╰╴                                                ━
    ");
}

#[test]
fn foreign_key_without_matching_unique_constraint() {
    assert_snapshot!(check("
create table p (
  id int primary key,
  u int unique,
  x int,
  y int,
  z int,
  w int
);
create unique index on p (y, z);
create unique index on p (w) where w > 0;
create table c1 (pid int references p (x));
create table c2 (
  pid int,
  u int,
  foreign key (pid, u) references p (id, u)
);

create table t (a int);
create table c3 (a int references t);
create table c4 (
  a int,
  foreign key (a) references t
);

create table ok1 (pid int references p);
create table ok2 (pid int references p (id));
create table ok3 (pid int references p (u));
create table ok4 (
  a int,
  b int,
  foreign key (a, b) references p (z, y)
);

create table c5 (a int references p (w));

create table s (
  id int,
  parent int references s (id),
  primary key (id)
);

create table s2 (
  parent int references s2,
  id int primary key
);

create table q (id int);
alter table q
  add primary key (id),
  add constraint q_fk foreign key (id) references q;

create table r (
  id int primary key,
  u int
);
alter table r add constraint r_fk foreign key (u) references r (u);
alter table r add unique (u);
alter table r add constraint r_fk2 foreign key (u) references r (u);

create table inc (
  a int,
  b int,
  unique (a) include (b)
);
create table c6 (b int references inc (b));
create table ok5 (a int references inc (a));

create table d (id int primary key);
alter table d drop constraint d_pkey;
create table c7 (id int references d);

create table e (
  a int,
  b int,
  primary key (a, b)
);
create table c8 (a int references e (a));
create table ok6 (
  x int,
  y int,
  foreign key (x, y) references e (b, a)
);
"), @"
    error: there is no unique constraint matching given keys for referenced table `p`
       ╭▸ 
    12 │ create table c1 (pid int references p (x));
       ╰╴                                       ━
    error: there is no unique constraint matching given keys for referenced table `p`
       ╭▸ 
    16 │   foreign key (pid, u) references p (id, u)
       ╰╴                                    ━━━━━━━
    error: there is no primary key for referenced table `t`
       ╭▸ 
    20 │ create table c3 (a int references t);
       ╰╴                                  ━
    error: there is no primary key for referenced table `t`
       ╭▸ 
    23 │   foreign key (a) references t
       ╰╴                             ━
    error: there is no unique constraint matching given keys for referenced table `p`
       ╭▸ 
    35 │ create table c5 (a int references p (w));
       ╰╴                                     ━
    error: there is no unique constraint matching given keys for referenced table `r`
       ╭▸ 
    57 │ alter table r add constraint r_fk foreign key (u) references r (u);
       ╰╴                                                               ━━━
    error: there is no unique constraint matching given keys for referenced table `inc`
       ╭▸ 
    66 │ create table c6 (b int references inc (b));
       ╰╴                                       ━
    error: there is no primary key for referenced table `d`
       ╭▸ 
    71 │ create table c7 (id int references d);
       ╰╴                                   ━
    error: there is no unique constraint matching given keys for referenced table `e`
       ╭▸ 
    78 │ create table c8 (a int references e (a));
       ╰╴                                     ━
    ");
}

#[test]
fn foreign_key_deferrable_key() {
    assert_snapshot!(check("
create table p (
  id int primary key deferrable,
  u int unique deferrable initially deferred,
  v int unique not deferrable,
  w int,
  x int,
  unique (w) deferrable
);
create unique index p_w on p (w);
create table c1 (pid int references p);
create table c2 (u int references p (u));
create table c3 (w int references p (w));
create table ok1 (v int references p (v));
create table p2 (
  id int,
  primary key (id) initially deferred
);
create table c4 (
  id int,
  foreign key (id) references p2
);
create table pa (id int primary key) partition by range (id);
create table pa1 (id int not null);
alter table pa attach partition pa1 for values from (0) to (10);
create table ok2 (id int references pa1 (id));
create table pe (a int);
create unique index on pe ((a));
create table ok3 (a int references pe (a));
"), @"
    error: cannot use a deferrable primary key for referenced table `p`
       ╭▸ 
    11 │ create table c1 (pid int references p);
       ╰╴                                    ━
    error: cannot use a deferrable unique constraint for referenced table `p`
       ╭▸ 
    12 │ create table c2 (u int references p (u));
       ╰╴                                     ━
    error: cannot use a deferrable primary key for referenced table `p2`
       ╭▸ 
    21 │   foreign key (id) references p2
       ╰╴                              ━━
    ");
}
