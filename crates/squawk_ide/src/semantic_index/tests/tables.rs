use insta::assert_snapshot;

use super::check;

#[test]
fn create_table_columns() {
    assert_snapshot!(check("
create table users (id int, name text);
select id, name, email from users;
"), @"
    error: column `email` does not exist
      ╭▸ 
    3 │ select id, name, email from users;
      ╰╴                 ━━━━━
    ");
}

#[test]
fn dropped_table() {
    assert_snapshot!(check("
create table t (a int);
create table u (a int);
create view v as select 1 a;
drop table t, nope;
drop table if exists nope2;
drop table v;
drop view v;
drop view u;
select * from t;
select * from v;
select 'x'::t;
"), @"
    error: relation `nope` does not exist
      ╭▸ 
    5 │ drop table t, nope;
      ╰╴              ━━━━
    error: relation `t` does not exist
       ╭▸ 
    10 │ select * from t;
       ╰╴              ━
    error: relation `v` does not exist
       ╭▸ 
    11 │ select * from v;
       ╰╴              ━
    error: type `t` does not exist
       ╭▸ 
    12 │ select 'x'::t;
       ╰╴            ━
    error: `v` is not a table
      ╭▸ 
    7 │ drop table v;
      │            ━
      │
      ╰ hint: Use DROP VIEW to remove a view.
    error: `u` is not a view
      ╭▸ 
    9 │ drop view u;
      │           ━
      │
      ╰ hint: Use DROP TABLE to remove a table.
    ");
}

#[test]
fn dropped_column() {
    assert_snapshot!(check("
create table t (a int, b int, c int);
select a from t;
alter table t drop column a, drop column nope, drop column if exists nope2;
alter table t drop column a;
alter table nope3 drop column x;
alter table if exists nope4 drop column x;
select a, b, c from t;
select t.a from t;
"), @"
    error: column `nope` does not exist
      ╭▸ 
    4 │ alter table t drop column a, drop column nope, drop column if exists nope2;
      ╰╴                                         ━━━━
    error: column `a` does not exist
      ╭▸ 
    5 │ alter table t drop column a;
      ╰╴                          ━
    error: relation `nope3` does not exist
      ╭▸ 
    6 │ alter table nope3 drop column x;
      ╰╴            ━━━━━
    error: column `a` does not exist
      ╭▸ 
    8 │ select a, b, c from t;
      ╰╴       ━
    error: column `a` does not exist
      ╭▸ 
    9 │ select t.a from t;
      ╰╴         ━
    ");
}

#[test]
fn drop_column_from_builtin_table() {
    assert_snapshot!(check("
alter table pg_class drop column relname;
select relname from pg_class;
"), @"no diagnostics");
}

#[test]
fn rollback_create_table() {
    assert_snapshot!(check("
begin;
create table t (a int);
select a from t;
rollback;
select a from t;
begin;
create table u (a int);
commit;
select a from u;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    6 │ select a from t;
      ╰╴              ━
    ");
}

#[test]
fn rollback_drop_table() {
    assert_snapshot!(check("
create table t (a int);
begin;
drop table t;
select a from t;
rollback;
select a from t;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    5 │ select a from t;
      ╰╴              ━
    ");
}

#[test]
fn rollback_drop_column() {
    assert_snapshot!(check("
create table t (a int, b int);
begin;
alter table t drop column a;
select a from t;
rollback;
select a, b from t;
"), @"
    error: column `a` does not exist
      ╭▸ 
    5 │ select a from t;
      ╰╴       ━
    ");
}

#[test]
fn long_column_name_truncated() {
    assert_snapshot!(check("
create table t(cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc_one int);
select cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc_two from t;
"), @"no diagnostics");
}

#[test]
fn duplicate_table_and_view() {
    assert_snapshot!(check("
create table t (a int);
create view t as select 1;
"), @"
    error: relation `t` already exists
      ╭▸ 
    3 │ create view t as select 1;
      ╰╴            ━
    ");
}

#[test]
fn duplicate_table() {
    assert_snapshot!(check("
create table t (a int);
create table t (b int);
create table public.t (c int);
create table t as select 1;
select 1 into t;
"), @"
    error: relation `t` already exists
      ╭▸ 
    3 │ create table t (b int);
      ╰╴             ━
    error: relation `t` already exists
      ╭▸ 
    4 │ create table public.t (c int);
      ╰╴                    ━
    error: relation `t` already exists
      ╭▸ 
    5 │ create table t as select 1;
      ╰╴             ━
    error: relation `t` already exists
      ╭▸ 
    6 │ select 1 into t;
      ╰╴              ━
    ");
}

#[test]
fn duplicate_table_in_other_schema() {
    assert_snapshot!(check("
create schema s;
create table t (a int);
create table s.t (a int);
create temp table t (a int);
"), @"no diagnostics");
}

#[test]
fn duplicate_table_if_not_exists() {
    assert_snapshot!(check("
create table t (a int);
create table if not exists t (a int);
create table if not exists t as select 1;
"), @"no diagnostics");
}

#[test]
fn duplicate_table_after_drop() {
    assert_snapshot!(check("
create table t (a int);
drop table t;
create table t (a int);
"), @"no diagnostics");
}

#[test]
fn duplicate_table_after_rollback() {
    assert_snapshot!(check("
begin;
create table t (a int);
rollback;
create table t (a int);
"), @"no diagnostics");
}

#[test]
fn truncate() {
    assert_snapshot!(check("
create table t (a int);
create view v as select 1 a;
truncate t;
truncate table only t, nope;
truncate v restart identity cascade;
"), @"
    error: relation `nope` does not exist
      ╭▸ 
    5 │ truncate table only t, nope;
      ╰╴                       ━━━━
    error: `v` is not a table
      ╭▸ 
    6 │ truncate v restart identity cascade;
      ╰╴         ━
    ");
}

#[test]
fn lock() {
    assert_snapshot!(check("
create table t (a int);
create view v as select 1 a;
create sequence s;
lock t;
lock table only t, v in access exclusive mode nowait;
lock nope;
lock s;
"), @"
    error: relation `nope` does not exist
      ╭▸ 
    7 │ lock nope;
      ╰╴     ━━━━
    error: `s` is not a table or view
      ╭▸ 
    8 │ lock s;
      ╰╴     ━
    ");
}

#[test]
fn analyze_and_vacuum() {
    assert_snapshot!(check("
create table t (a int, b int);
create view v as select 1 a;
analyze;
analyze t;
analyze verbose t (a, b), v;
analyze t (nope);
analyze nope;
vacuum;
vacuum full t;
vacuum (analyze, verbose) t (a), v;
vacuum t (nope), nope2;
"), @"
    error: column `nope` does not exist
      ╭▸ 
    7 │ analyze t (nope);
      ╰╴           ━━━━
    error: relation `nope` does not exist
      ╭▸ 
    8 │ analyze nope;
      ╰╴        ━━━━
    error: column `nope` does not exist
       ╭▸ 
    12 │ vacuum t (nope), nope2;
       ╰╴          ━━━━
    error: relation `nope2` does not exist
       ╭▸ 
    12 │ vacuum t (nope), nope2;
       ╰╴                 ━━━━━
    ");
}

#[test]
fn alter_table_rename() {
    assert_snapshot!(check("
create table t (a int);
create table u (a int);
alter table t rename to t2;
select a from t;
select a from t2;
create table x (r t2);
alter table t2 rename to u;
alter table nope rename to y;
alter table if exists nope rename to y;
begin;
alter table t2 rename to t3;
rollback;
select a from t3;
select a from t2;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    5 │ select a from t;
      ╰╴              ━
    error: relation `nope` does not exist
      ╭▸ 
    9 │ alter table nope rename to y;
      ╰╴            ━━━━
    error: relation `t3` does not exist
       ╭▸ 
    14 │ select a from t3;
       ╰╴              ━━
    error: relation `u` already exists
      ╭▸ 
    8 │ alter table t2 rename to u;
      ╰╴                         ━
    ");
}

#[test]
fn alter_table_set_schema() {
    assert_snapshot!(check("
create schema s;
create table t (a int);
create table s.u (a int);
create table u (a int);
alter table t set schema s;
select a from t;
select a from s.t;
alter table u set schema s;
alter table s.t set schema nope;
"), @"
    error: relation `t` does not exist
      ╭▸ 
    7 │ select a from t;
      ╰╴              ━
    error: schema `nope` does not exist
       ╭▸ 
    10 │ alter table s.t set schema nope;
       ╰╴                           ━━━━
    error: relation `u` already exists
      ╭▸ 
    9 │ alter table u set schema s;
      ╰╴                         ━
    ");
}

#[test]
fn alter_table_rename_column() {
    assert_snapshot!(check("
create table t (a int, b int);
alter table t rename column a to c;
select a from t;
select c from t;
alter table t rename c to b;
alter table t rename column nope to x;
create index i on t (c);
begin;
alter table t rename column c to d;
rollback;
select d from t;
select c from t;
"), @"
    error: column `a` does not exist
      ╭▸ 
    4 │ select a from t;
      ╰╴       ━
    error: column `nope` does not exist
      ╭▸ 
    7 │ alter table t rename column nope to x;
      ╰╴                            ━━━━
    error: column `d` does not exist
       ╭▸ 
    12 │ select d from t;
       ╰╴       ━
    error: column `b` of relation `t` already exists
      ╭▸ 
    6 │ alter table t rename c to b;
      ╰╴                          ━
    ");
}

#[test]
fn alter_table_add_column() {
    assert_snapshot!(check("
create table t (a int);
create table r (x int primary key);
alter table t add column b text;
select a, b from t;
alter table t add c int check (c > 0 and nope > 0), add column d int references r (x);
select c, d from t;
alter table t add column a int;
alter table t add column if not exists a int;
alter table t add column e nope_type;
alter table t add column f int collate nope_collation;
alter table t drop column b, add column b int;
select b from t;
alter table t add column g int constraint g_pk primary key;
drop index g_pk;
begin;
alter table t add column h int;
rollback;
select h from t;
"), @"
    error: column `nope` does not exist
      ╭▸ 
    6 │ alter table t add c int check (c > 0 and nope > 0), add column d int references r (x);
      ╰╴                                         ━━━━
    error: type `nope_type` does not exist
       ╭▸ 
    10 │ alter table t add column e nope_type;
       ╰╴                           ━━━━━━━━━
    error: collation `nope_collation` does not exist
       ╭▸ 
    11 │ alter table t add column f int collate nope_collation;
       ╰╴                                       ━━━━━━━━━━━━━━
    error: column `h` does not exist
       ╭▸ 
    19 │ select h from t;
       ╰╴       ━
    error: column `a` of relation `t` already exists
      ╭▸ 
    8 │ alter table t add column a int;
      ╰╴                         ━
    ");
}

#[test]
fn alter_table_alter_column() {
    assert_snapshot!(check("
create table t (a int, b text);
alter table t alter column a type bigint using a::bigint + length(b);
alter table t alter column a set data type nope_type;
alter table t alter column b type text collate nope_collation;
alter table t alter column a type int using nope;
alter table t alter column nope set default 1;
alter table t alter column a set default nope();
alter table t alter column a drop default, alter column b set storage plain;
alter table t alter column a set statistics 100;
alter table t add column c int, alter column c set default 1;
alter table t add column d int, add constraint d_check check (d > 0);
alter table t drop column c, alter column c set default 1;
"), @"
    error: type `nope_type` does not exist
      ╭▸ 
    4 │ alter table t alter column a set data type nope_type;
      ╰╴                                           ━━━━━━━━━
    error: collation `nope_collation` does not exist
      ╭▸ 
    5 │ alter table t alter column b type text collate nope_collation;
      ╰╴                                               ━━━━━━━━━━━━━━
    error: column `nope` does not exist
      ╭▸ 
    6 │ alter table t alter column a type int using nope;
      ╰╴                                            ━━━━
    error: column `nope` does not exist
      ╭▸ 
    7 │ alter table t alter column nope set default 1;
      ╰╴                           ━━━━
    error: function `nope` does not exist
      ╭▸ 
    8 │ alter table t alter column a set default nope();
      ╰╴                                         ━━━━
    error: column `c` does not exist
       ╭▸ 
    13 │ alter table t drop column c, alter column c set default 1;
       ╰╴                                          ━
    ");
}

#[test]
fn alter_table_alter_column_not_null() {
    assert_snapshot!(check("
create table t (a int);
alter table t alter column a set not null;
alter table t drop constraint t_a_not_null;
alter table t alter column a set not null;
alter table t alter column a drop not null;
alter table t drop constraint t_a_not_null;
"), @"
    error: constraint `t_a_not_null` does not exist
      ╭▸ 
    7 │ alter table t drop constraint t_a_not_null;
      ╰╴                              ━━━━━━━━━━━━
    ");
}

#[test]
fn alter_table_alter_column_type_changes_column() {
    assert_snapshot!(check("
create table t (a int);
create function f(text) returns int language sql as 'select 1';
select f(a) from t;
alter table t alter column a type text;
select f(a) from t;
begin;
alter table t alter column a type int;
rollback;
select f(a) from t;
"), @"
    error: function `f` does not exist
      ╭▸ 
    4 │ select f(a) from t;
      ╰╴       ━
    ");
}

#[test]
fn alter_table_alter_type_before_add_column() {
    assert_snapshot!(check("
create table t (a int);
create table u (a int);
alter table t add column x int, alter column x type text;
alter table u drop column a, alter column a type text;
alter table t alter column a type text, add column b int, alter column b set default 1;
"), @"
    error: column `x` does not exist
      ╭▸ 
    4 │ alter table t add column x int, alter column x type text;
      ╰╴                                             ━
    error: column `a` does not exist
      ╭▸ 
    5 │ alter table u drop column a, alter column a type text;
      ╰╴                                          ━
    ");
}
