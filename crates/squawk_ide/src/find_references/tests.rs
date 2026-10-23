use insta::assert_snapshot;

use crate::test_utils::render_targets;

use crate::find_references::find_references_with_index as find_references;

#[track_caller]
fn refs(sql: &str) -> String {
    refs_files(&[sql])
}

#[track_caller]
fn refs_files(files: &[&str]) -> String {
    render_targets(files, "find references", "reference", find_references)
        .unwrap_or_else(|| "no references".to_string())
}

#[test]
fn enum_label_definition() {
    assert_snapshot!(refs("
create type mood as enum ('happy', 'sad$0');
create table t (m mood);
select 'sad'::mood from t where m = 'sad' or m = 'happy';
"), @"
    info: find references
      ╭▸ file0.sql:2:36
      │
    2 │ create type mood as enum ('happy', 'sad');
      │                                    ┬────
      │                                    │
      │                                    1. source
      │                                    2. reference
    3 │ create table t (m mood);
    4 │ select 'sad'::mood from t where m = 'sad' or m = 'happy';
      ╰╴       ───── 3. reference           ───── 4. reference
    ");
}

#[test]
fn cte_from_reference() {
    assert_snapshot!(refs("
with t as (select 1 a)
select t.a from t$0;
"), @"
    info: find references
      ╭▸ file0.sql:2:6
      │
    2 │ with t as (select 1 a)
      │      ─ 2. reference
    3 │ select t.a from t;
      │        ┬        ┬
      │        │        │
      │        │        1. source
      │        │        4. reference
      ╰╴       3. reference
    ");
}

#[test]
fn cte_from_definition() {
    assert_snapshot!(refs("
with t$0 as (select 1 a)
select t.a from t;
"), @"
    info: find references
      ╭▸ file0.sql:2:6
      │
    2 │ with t as (select 1 a)
      │      ┬
      │      │
      │      1. source
      │      2. reference
    3 │ select t.a from t;
      │        ┬        ─ 4. reference
      │        │
      ╰╴       3. reference
    ");
}

#[test]
fn cte_column() {
    assert_snapshot!(refs("
with t as (select 1 a)
select a$0, t.a from t;
"), @"
    info: find references
      ╭▸ file0.sql:2:21
      │
    2 │ with t as (select 1 a)
      │                     ─ 2. reference
    3 │ select a, t.a from t;
      │        ┬    ─ 4. reference
      │        │
      │        1. source
      ╰╴       3. reference
    ");
}

#[test]
fn column_through_star() {
    assert_snapshot!(refs("
with t as (select 1 a), u as (select * from t)
select a$0 from u;
"), @"
    info: find references
      ╭▸ file0.sql:2:21
      │
    2 │ with t as (select 1 a), u as (select * from t)
      │                     ─ 2. reference
    3 │ select a from u;
      │        ┬
      │        │
      │        1. source
      ╰╴       3. reference
    ");
}

#[test]
fn table_across_files() {
    assert_snapshot!(refs_files(&["
create table users$0 (id int, name text);
", "
select name from users;
select u.id from public.users u;
"]), @"
    info: find references
      ╭▸ file0.sql:2:14
      │
    2 │ create table users (id int, name text);
      │              ┬────
      │              │
      │              1. source
      │              2. reference
      │
      ⸬  file1.sql:2:18
      │
    2 │ select name from users;
      │                  ───── 3. reference
    3 │ select u.id from public.users u;
      ╰╴                        ───── 4. reference
    ");
}

#[test]
fn table_column() {
    assert_snapshot!(refs("
create table users (id int, name$0 text);
select name from users;
select u.name, u.id from users u;
"), @"
    info: find references
      ╭▸ file0.sql:2:29
      │
    2 │ create table users (id int, name text);
      │                             ┬───
      │                             │
      │                             1. source
      │                             2. reference
    3 │ select name from users;
      │        ──── 3. reference
    4 │ select u.name, u.id from users u;
      ╰╴         ──── 4. reference
    ");
}

#[test]
fn alias() {
    assert_snapshot!(refs("
create table users (id int);
select u.id from users u$0 where u.id > 1;
"), @"
    info: find references
      ╭▸ file0.sql:3:8
      │
    3 │ select u.id from users u where u.id > 1;
      │        ┬               ┬       ─ 4. reference
      │        │               │
      │        │               1. source
      ╰╴       2. reference    3. reference
    ");
}

#[test]
fn join_using() {
    assert_snapshot!(refs("
create table a (id int);
create table b (id$0 int);
select id from a join b using (id);
"), @"
    info: find references
      ╭▸ file0.sql:3:17
      │
    3 │ create table b (id int);
      │                 ┬─
      │                 │
      │                 1. source
      │                 2. reference
    4 │ select id from a join b using (id);
      ╰╴                               ── 3. reference
    ");
}

#[test]
fn function() {
    assert_snapshot!(refs("
create function f$0() returns int language sql as 'select 1';
select f();
select public.f();
"), @"
    info: find references
      ╭▸ file0.sql:2:17
      │
    2 │ create function f() returns int language sql as 'select 1';
      │                 ┬
      │                 │
      │                 1. source
      │                 2. reference
    3 │ select f();
      │        ─ 3. reference
    4 │ select public.f();
      ╰╴              ─ 4. reference
    ");
}

#[test]
fn param_in_sql_body() {
    assert_snapshot!(refs("
create function f(user_id$0 int) returns int
  language sql
  as $$ select user_id + $1 $$;
"), @"
    info: find references
      ╭▸ file0.sql:2:19
      │
    2 │ create function f(user_id int) returns int
      │                   ┬──────
      │                   │
      │                   1. source
      │                   2. reference
    3 │   language sql
    4 │   as $$ select user_id + $1 $$;
      │                ┬──────   ── 4. reference
      │                │
      ╰╴               3. reference
    ");
}

#[test]
fn table_in_sql_body() {
    assert_snapshot!(refs("
create table users (id int, name text);
create function f() returns text
  language sql
  as $$ select name from users$0 $$;
select id from users;
"), @"
    info: find references
      ╭▸ file0.sql:2:14
      │
    2 │ create table users (id int, name text);
      │              ───── 2. reference
      ‡
    5 │   as $$ select name from users $$;
      │                          ┬────
      │                          │
      │                          1. source
      │                          3. reference
    6 │ select id from users;
      ╰╴               ───── 4. reference
    ");
}

#[test]
fn user_type() {
    assert_snapshot!(refs("
create type mood$0 as enum ('happy', 'sad');
create table t (a mood, b public.mood);
select 'happy'::mood;
"), @"
    info: find references
      ╭▸ file0.sql:2:13
      │
    2 │ create type mood as enum ('happy', 'sad');
      │             ┬───
      │             │
      │             1. source
      │             2. reference
    3 │ create table t (a mood, b public.mood);
      │                   ┬───           ──── 4. reference
      │                   │
      │                   3. reference
    4 │ select 'happy'::mood;
      ╰╴                ──── 5. reference
    ");
}

#[test]
fn unresolved() {
    assert_snapshot!(refs("
select a$0 from missing;
"), @"no references");
}

#[test]
fn column_through_star_from_definition() {
    assert_snapshot!(refs("
with t as (select 1 a$0), u as (select * from t), v as (select u.* from u)
select t.a, u.a, v.a from t, u, v;
"), @"
    info: find references
      ╭▸ file0.sql:2:21
      │
    2 │ with t as (select 1 a), u as (select * from t), v as (select u.* from u)
      │                     ┬
      │                     │
      │                     1. source
      │                     2. reference
    3 │ select t.a, u.a, v.a from t, u, v;
      │          ┬    ┬    ─ 5. reference
      │          │    │
      │          │    4. reference
      ╰╴         3. reference
    ");
}

#[test]
fn table_column_through_star() {
    assert_snapshot!(refs("
create table users (id$0 int);
with u as (select * from users)
select id from u;
"), @"
    info: find references
      ╭▸ file0.sql:2:21
      │
    2 │ create table users (id int);
      │                     ┬─
      │                     │
      │                     1. source
      │                     2. reference
    3 │ with u as (select * from users)
    4 │ select id from u;
      ╰╴       ── 3. reference
    ");
}

#[test]
fn window_definition() {
    assert_snapshot!(refs("
create table t(a int, b int);
select count(*) over w, sum(b) over (w order by b) from t window w$0 as (partition by a);
"), @"
    info: find references
      ╭▸ file0.sql:3:22
      │
    3 │ select count(*) over w, sum(b) over (w order by b) from t window w as (partition by a);
      │                      ┬               ┬                           ┬
      │                      │               │                           │
      │                      │               │                           1. source
      ╰╴                     2. reference    3. reference                4. reference
    ");
}

#[test]
fn index_references() {
    assert_snapshot!(refs("
create table t(x int);
create index idx$0 on t(x);
reindex index idx;
alter table t cluster on idx;
drop index idx;
"), @"
    info: find references
      ╭▸ file0.sql:3:14
      │
    3 │ create index idx on t(x);
      │              ┬──
      │              │
      │              1. source
      │              2. reference
    4 │ reindex index idx;
      │               ─── 3. reference
    5 │ alter table t cluster on idx;
      │                          ─── 4. reference
    6 │ drop index idx;
      ╰╴           ─── 5. reference
    ");
}

#[test]
fn savepoint() {
    assert_snapshot!(refs("
begin;
savepoint sp$0;
rollback to sp;
release sp;
"), @"
    info: find references
      ╭▸ file0.sql:3:11
      │
    3 │ savepoint sp;
      │           ┬─
      │           │
      │           1. source
      │           2. reference
    4 │ rollback to sp;
      │             ── 3. reference
    5 │ release sp;
      ╰╴        ── 4. reference
    ");
}

#[test]
fn prepared_transaction() {
    assert_snapshot!(refs("
prepare transaction 'foo'$0;
commit prepared 'foo';
"), @"
    info: find references
      ╭▸ file0.sql:2:21
      │
    2 │ prepare transaction 'foo';
      │                     ┬────
      │                     │
      │                     1. source
      │                     2. reference
    3 │ commit prepared 'foo';
      ╰╴                ───── 3. reference
    ");
}

#[test]
fn constraint_references() {
    assert_snapshot!(refs("
create table t(a int constraint c$0 unique);
insert into t values (1) on conflict on constraint c do nothing;
alter table t cluster on c;
alter table t drop constraint c;
"), @"
    info: find references
      ╭▸ file0.sql:2:33
      │
    2 │ create table t(a int constraint c unique);
      │                                 ┬
      │                                 │
      │                                 1. source
      │                                 2. reference
    3 │ insert into t values (1) on conflict on constraint c do nothing;
      │                                                    ─ 3. reference
    4 │ alter table t cluster on c;
      │                          ─ 4. reference
    5 │ alter table t drop constraint c;
      ╰╴                              ─ 5. reference
    ");
}

#[test]
fn collation_references() {
    assert_snapshot!(refs("
create collation c$0 (provider = icu, locale = 'und');
create collation d from c;
create table t(a text collate c);
create index i on t (a collate c);
select a collate c from t;
alter collation c refresh version;
drop collation c;
"), @"
    info: find references
      ╭▸ file0.sql:2:18
      │
    2 │ create collation c (provider = icu, locale = 'und');
      │                  ┬
      │                  │
      │                  1. source
      │                  2. reference
    3 │ create collation d from c;
      │                         ─ 3. reference
    4 │ create table t(a text collate c);
      │                               ─ 4. reference
    5 │ create index i on t (a collate c);
      │                                ─ 5. reference
    6 │ select a collate c from t;
      │                  ─ 6. reference
    7 │ alter collation c refresh version;
      │                 ─ 7. reference
    8 │ drop collation c;
      ╰╴               ─ 8. reference
    ");
}

#[test]
fn cursor_references() {
    assert_snapshot!(refs("
create table t (a int);
begin;
declare c$0 cursor for select a from t for update;
fetch c;
move c;
update t set a = 1 where current of c;
delete from t where current of c;
close c;
"), @"
    info: find references
      ╭▸ file0.sql:4:9
      │
    4 │ declare c cursor for select a from t for update;
      │         ┬
      │         │
      │         1. source
      │         2. reference
    5 │ fetch c;
      │       ─ 3. reference
    6 │ move c;
      │      ─ 4. reference
    7 │ update t set a = 1 where current of c;
      │                                     ─ 5. reference
    8 │ delete from t where current of c;
      │                                ─ 6. reference
    9 │ close c;
      ╰╴      ─ 7. reference
    ");
}

#[test]
fn schema() {
    assert_snapshot!(refs("
create schema s$0;
create table s.t (a int);
create function s.f() returns int language sql return 1;
select s.f(), a::s.t from s.t;
"), @"
    info: find references
      ╭▸ file0.sql:2:15
      │
    2 │ create schema s;
      │               ┬
      │               │
      │               1. source
      │               2. reference
    3 │ create table s.t (a int);
      │              ─ 3. reference
    4 │ create function s.f() returns int language sql return 1;
      │                 ─ 4. reference
    5 │ select s.f(), a::s.t from s.t;
      │        ┬         ┬        ─ 7. reference
      │        │         │
      │        │         6. reference
      ╰╴       5. reference
    ");
}

#[test]
fn tablespace() {
    assert_snapshot!(refs("
create tablespace ts$0 location '/data';
create table t (a int, unique (a) using index tablespace ts) tablespace ts;
create index i on t (a) tablespace ts;
reindex (tablespace ts) index i;
alter tablespace ts reset (seq_page_cost);
drop tablespace ts;
"), @"
    info: find references
      ╭▸ file0.sql:2:19
      │
    2 │ create tablespace ts location '/data';
      │                   ┬─
      │                   │
      │                   1. source
      │                   2. reference
    3 │ create table t (a int, unique (a) using index tablespace ts) tablespace ts;
      │                                                          ┬─             ── 4. reference
      │                                                          │
      │                                                          3. reference
    4 │ create index i on t (a) tablespace ts;
      │                                    ── 5. reference
    5 │ reindex (tablespace ts) index i;
      │                     ── 6. reference
    6 │ alter tablespace ts reset (seq_page_cost);
      │                  ── 7. reference
    7 │ drop tablespace ts;
      ╰╴                ── 8. reference
    ");
}

#[test]
fn database() {
    assert_snapshot!(refs("
create database d$0;
create database e template d;
alter database d with connection limit 5;
reindex database d;
drop database d;
"), @"
    info: find references
      ╭▸ file0.sql:2:17
      │
    2 │ create database d;
      │                 ┬
      │                 │
      │                 1. source
      │                 2. reference
    3 │ create database e template d;
      │                            ─ 3. reference
    4 │ alter database d with connection limit 5;
      │                ─ 4. reference
    5 │ reindex database d;
      │                  ─ 5. reference
    6 │ drop database d;
      ╰╴              ─ 6. reference
    ");
}

#[test]
fn extension() {
    assert_snapshot!(refs("
create extension e$0;
create table t (a int);
create index i on t (a);
alter index i depends on extension e;
alter extension e update;
drop extension e;
"), @"
    info: find references
      ╭▸ file0.sql:2:18
      │
    2 │ create extension e;
      │                  ┬
      │                  │
      │                  1. source
      │                  2. reference
      ‡
    5 │ alter index i depends on extension e;
      │                                    ─ 3. reference
    6 │ alter extension e update;
      │                 ─ 4. reference
    7 │ drop extension e;
      ╰╴               ─ 5. reference
    ");
}

#[test]
fn sequence() {
    assert_snapshot!(refs("
create sequence s$0;
alter sequence s restart;
select last_value from s;
select nextval('s'), setval('public.s', 1);
drop sequence public.s;
"), @"
    info: find references
      ╭▸ file0.sql:2:17
      │
    2 │ create sequence s;
      │                 ┬
      │                 │
      │                 1. source
      │                 2. reference
    3 │ alter sequence s restart;
      │                ─ 3. reference
    4 │ select last_value from s;
      │        ┬─────────      ─ 5. reference
      │        │
      │        4. reference
    5 │ select nextval('s'), setval('public.s', 1);
      │                 ─ 6. reference      ─ 7. reference
    6 │ drop sequence public.s;
      ╰╴                     ─ 8. reference
    ");
}

#[test]
fn operator() {
    assert_snapshot!(refs("
create operator ===$0 (leftarg = int, rightarg = int, function = int4eq);
create operator !== (leftarg = int, rightarg = int, function = int4ne, negator = ===);
alter operator === (int, int) set (restrict = eqsel);
drop operator === (int, text);
drop operator public.=== (int, int);
"), @"
    info: find references
      ╭▸ file0.sql:2:17
      │
    2 │ create operator === (leftarg = int, rightarg = int, function = int4eq);
      │                 ┬──
      │                 │
      │                 1. source
      │                 2. reference
    3 │ create operator !== (leftarg = int, rightarg = int, function = int4ne, negator = ===);
      │                                                                                  ─── 3. reference
    4 │ alter operator === (int, int) set (restrict = eqsel);
      │                ─── 4. reference
    5 │ drop operator === (int, text);
    6 │ drop operator public.=== (int, int);
      ╰╴                     ─── 5. reference
    ");
}

#[test]
fn cast_function() {
    assert_snapshot!(refs("
create function f$0(x int) returns text language sql return 'a';
create cast (int as text) with function f(int);
create cast (int as varchar) with function f;
"), @"
    info: find references
      ╭▸ file0.sql:2:17
      │
    2 │ create function f(x int) returns text language sql return 'a';
      │                 ┬
      │                 │
      │                 1. source
      │                 2. reference
    3 │ create cast (int as text) with function f(int);
      │                                         ─ 3. reference
    4 │ create cast (int as varchar) with function f;
      ╰╴                                           ─ 4. reference
    ");
}

#[test]
fn prepared_statement_references() {
    assert_snapshot!(refs("
prepare p$0 as select 1;
execute p;
explain execute p;
create table t as execute p;
deallocate p;
"), @"
    info: find references
      ╭▸ file0.sql:2:9
      │
    2 │ prepare p as select 1;
      │         ┬
      │         │
      │         1. source
      │         2. reference
    3 │ execute p;
      │         ─ 3. reference
    4 │ explain execute p;
      │                 ─ 4. reference
    5 │ create table t as execute p;
      │                           ─ 5. reference
    6 │ deallocate p;
      ╰╴           ─ 6. reference
    ");
}

#[test]
fn procedure_references() {
    assert_snapshot!(refs("
create procedure p$0() language sql begin atomic select 1; end;
call p();
alter procedure p() security definer;
drop procedure p;
"), @"
    info: find references
      ╭▸ file0.sql:2:18
      │
    2 │ create procedure p() language sql begin atomic select 1; end;
      │                  ┬
      │                  │
      │                  1. source
      │                  2. reference
    3 │ call p();
      │      ─ 3. reference
    4 │ alter procedure p() security definer;
      │                 ─ 4. reference
    5 │ drop procedure p;
      ╰╴               ─ 5. reference
    ");
}

#[test]
fn column_inside_dollar_quoted_body() {
    assert_snapshot!(refs("
create table t (a int);
create function f() returns int
  language sql
  as $$ select a$0 from t $$;
select a from t;
"), @"
    info: find references
      ╭▸ file0.sql:2:17
      │
    2 │ create table t (a int);
      │                 ─ 2. reference
      ‡
    5 │   as $$ select a from t $$;
      │                ┬
      │                │
      │                1. source
      │                3. reference
    6 │ select a from t;
      ╰╴       ─ 4. reference
    ");
}

#[test]
fn simple_table_reference() {
    assert_snapshot!(refs("
create table t();
drop table t$0;
"), @"
    info: find references
      ╭▸ file0.sql:2:14
      │
    2 │ create table t();
      │              ─ 2. reference
    3 │ drop table t;
      │            ┬
      │            │
      │            1. source
      ╰╴           3. reference
    ");
}

#[test]
fn multiple_references() {
    assert_snapshot!(refs("
create table users();
drop table users$0;
table users;
"), @"
    info: find references
      ╭▸ file0.sql:2:14
      │
    2 │ create table users();
      │              ───── 2. reference
    3 │ drop table users;
      │            ┬────
      │            │
      │            1. source
      ╰╴           3. reference
    ");
}

#[test]
fn join_using_column() {
    assert_snapshot!(refs("
create table t(id int);
create table u(id int);
select * from t join u using (id$0);
"), @"
    info: find references
      ╭▸ file0.sql:2:16
      │
    2 │ create table t(id int);
      │                ── 2. reference
    3 │ create table u(id int);
      │                ── 3. reference
    4 │ select * from t join u using (id);
      │                               ┬─
      │                               │
      │                               1. source
      ╰╴                              4. reference
    ");
}

#[test]
fn find_from_definition() {
    assert_snapshot!(refs("
create table t$0();
drop table t;
"), @"
    info: find references
      ╭▸ file0.sql:2:14
      │
    2 │ create table t();
      │              ┬
      │              │
      │              1. source
      │              2. reference
    3 │ drop table t;
      ╰╴           ─ 3. reference
    ");
}

#[test]
fn with_schema_qualified() {
    assert_snapshot!(refs("
create table public.users();
drop table public.users$0;
table users;
"), @"
    info: find references
      ╭▸ file0.sql:2:21
      │
    2 │ create table public.users();
      │                     ───── 2. reference
    3 │ drop table public.users;
      │                   ┬────
      │                   │
      │                   1. source
      ╰╴                  3. reference
    ");
}

#[test]
fn temp_table_shadows_public() {
    assert_snapshot!(refs("
create table t();
create temp table t$0();
drop table t;
"), @"
    info: find references
      ╭▸ file0.sql:3:19
      │
    3 │ create temp table t();
      │                   ┬
      │                   │
      │                   1. source
      │                   2. reference
    4 │ drop table t;
      ╰╴           ─ 3. reference
    ");
}

#[test]
fn different_schema_no_match() {
    assert_snapshot!(refs("
create table foo.t();
create table bar.t$0();
"), @"
    info: find references
      ╭▸ file0.sql:3:18
      │
    3 │ create table bar.t();
      │                  ┬
      │                  │
      │                  1. source
      ╰╴                 2. reference
    ");
}

#[test]
fn with_search_path() {
    assert_snapshot!(refs("
set search_path to myschema;
create table myschema.users$0();
drop table users;
"), @"
    info: find references
      ╭▸ file0.sql:3:23
      │
    3 │ create table myschema.users();
      │                       ┬────
      │                       │
      │                       1. source
      │                       2. reference
    4 │ drop table users;
      ╰╴           ───── 3. reference
    ");
}

#[test]
fn temp_table_with_pg_temp_schema() {
    assert_snapshot!(refs("
create temp table t();
drop table pg_temp.t$0;
"), @"
    info: find references
      ╭▸ file0.sql:2:19
      │
    2 │ create temp table t();
      │                   ─ 2. reference
    3 │ drop table pg_temp.t;
      │                    ┬
      │                    │
      │                    1. source
      ╰╴                   3. reference
    ");
}

#[test]
fn case_insensitive() {
    assert_snapshot!(refs("
create table Users();
drop table USERS$0;
table users;
"), @"
    info: find references
      ╭▸ file0.sql:2:14
      │
    2 │ create table Users();
      │              ───── 2. reference
    3 │ drop table USERS;
      │            ┬────
      │            │
      │            1. source
      ╰╴           3. reference
    ");
}

#[test]
fn case_insensitive_with_schema() {
    assert_snapshot!(refs("
create table Public.Users();
drop table PUBLIC.USERS$0;
table public.users;
"), @"
    info: find references
      ╭▸ file0.sql:2:21
      │
    2 │ create table Public.Users();
      │                     ───── 2. reference
    3 │ drop table PUBLIC.USERS;
      │                   ┬────
      │                   │
      │                   1. source
      ╰╴                  3. reference
    ");
}

#[test]
fn no_partial_match() {
    assert_snapshot!(refs("
create table t$0();
create table temp_t();
"), @"
    info: find references
      ╭▸ file0.sql:2:14
      │
    2 │ create table t();
      │              ┬
      │              │
      │              1. source
      ╰╴             2. reference
    ");
}

#[test]
fn identifier_boundaries() {
    assert_snapshot!(refs("
create table foo$0();
drop table foo;
drop table foo1;
drop table barfoo;
drop table foo_bar;
"), @"
    info: find references
      ╭▸ file0.sql:2:14
      │
    2 │ create table foo();
      │              ┬──
      │              │
      │              1. source
      │              2. reference
    3 │ drop table foo;
      ╰╴           ─── 3. reference
    ");
}

#[test]
fn builtin_function_references() {
    assert_snapshot!(refs("
select now$0();
select now();
"), @"
    info: find references
          ╭▸ file0.sql:2:8
          │
        2 │ select now();
          │        ┬──
          │        │
          │        1. source
          │        2. reference
        3 │ select now();
          │        ─── 3. reference
          │
          ⸬  builtins.sql:19278:28
          │
    19278 │ create function pg_catalog.now() returns timestamp with time zone
          ╰╴                           ─── 4. reference
    ");
}

#[test]
fn bit() {
    assert_snapshot!(refs("
create type pg_catalog.bit$0;

create function pg_catalog.bit(bigint, integer) returns bit
  language internal;

create function pg_catalog.bit(bit, integer, boolean) returns bit
  language internal;

create function pg_catalog.bit(integer, integer) returns bit
  language internal;
"), @"
    info: find references
       ╭▸ file0.sql:2:24
       │
     2 │ create type pg_catalog.bit;
       │                        ┬──
       │                        │
       │                        1. source
       │                        2. reference
     3 │
     4 │ create function pg_catalog.bit(bigint, integer) returns bit
       │                                                         ─── 3. reference
       ‡
     7 │ create function pg_catalog.bit(bit, integer, boolean) returns bit
       │                                ─── 4. reference               ─── 5. reference
       ‡
    10 │ create function pg_catalog.bit(integer, integer) returns bit
       ╰╴                                                         ─── 6. reference
    ");
}

#[test]
fn positional_param_and_named_param() {
    assert_snapshot!(refs("
create function f(x$0 int) returns int language sql return x + $1;
"), @"
    info: find references
      ╭▸ file0.sql:2:19
      │
    2 │ create function f(x int) returns int language sql return x + $1;
      │                   ┬                                      ┬   ── 4. reference
      │                   │                                      │
      │                   1. source                              3. reference
      ╰╴                  2. reference
    ");
}

#[test]
fn char() {
    assert_snapshot!(refs("
create type pg_catalog.bpchar$0;

select '1'::char;
select '1'::bpchar;
"), @"
    info: find references
      ╭▸ file0.sql:2:24
      │
    2 │ create type pg_catalog.bpchar;
      │                        ┬─────
      │                        │
      │                        1. source
      │                        2. reference
    3 │
    4 │ select '1'::char;
      │             ──── 3. reference
    5 │ select '1'::bpchar;
      ╰╴            ────── 4. reference
    ");
}

#[test]
fn access_method_ref() {
    assert_snapshot!(refs("
create access method heap2$0 type table handler heap_tableam_handler;
drop access method heap2;
"), @"
    info: find references
      ╭▸ file0.sql:2:22
      │
    2 │ create access method heap2 type table handler heap_tableam_handler;
      │                      ┬────
      │                      │
      │                      1. source
      │                      2. reference
    3 │ drop access method heap2;
      ╰╴                   ───── 3. reference
    ");
}

#[test]
fn channel_ref() {
    assert_snapshot!(refs("
listen updates$0;
notify updates;
"), @"
    info: find references
      ╭▸ file0.sql:2:8
      │
    2 │ listen updates;
      │        ┬──────
      │        │
      │        1. source
      │        2. reference
    3 │ notify updates;
      ╰╴       ─────── 3. reference
    ");
}

#[test]
fn column_name_ref() {
    assert_snapshot!(refs("
create table t(id$0 int);
create table u(t_id int references t(id));
"), @"
    info: find references
      ╭▸ file0.sql:2:16
      │
    2 │ create table t(id int);
      │                ┬─
      │                │
      │                1. source
      │                2. reference
    3 │ create table u(t_id int references t(id));
      ╰╴                                     ── 3. reference
    ");
}

#[test]
fn cursor_ref() {
    assert_snapshot!(refs("
declare c$0 scroll cursor for select * from t;
fetch forward 5 from c;
"), @"
    info: find references
      ╭▸ file0.sql:2:9
      │
    2 │ declare c scroll cursor for select * from t;
      │         ┬
      │         │
      │         1. source
      │         2. reference
    3 │ fetch forward 5 from c;
      ╰╴                     ─ 3. reference
    ");
}

#[test]
fn database_ref() {
    assert_snapshot!(refs("
create database mydb$0;
drop database mydb;
"), @"
    info: find references
      ╭▸ file0.sql:2:17
      │
    2 │ create database mydb;
      │                 ┬───
      │                 │
      │                 1. source
      │                 2. reference
    3 │ drop database mydb;
      ╰╴              ──── 3. reference
    ");
}

#[test]
fn event_trigger_ref() {
    assert_snapshot!(refs("
create event trigger et$0 on ddl_command_start execute function f();
drop event trigger et;
"), @"
    info: find references
      ╭▸ file0.sql:2:22
      │
    2 │ create event trigger et on ddl_command_start execute function f();
      │                      ┬─
      │                      │
      │                      1. source
      │                      2. reference
    3 │ drop event trigger et;
      ╰╴                   ── 3. reference
    ");
}

#[test]
fn extension_ref() {
    assert_snapshot!(refs("
create extension myext$0;
drop extension myext;
"), @"
    info: find references
      ╭▸ file0.sql:2:18
      │
    2 │ create extension myext;
      │                  ┬────
      │                  │
      │                  1. source
      │                  2. reference
    3 │ drop extension myext;
      ╰╴               ───── 3. reference
    ");
}

#[test]
fn foreign_data_wrapper_ref() {
    assert_snapshot!(refs("
create foreign data wrapper fdw$0;
create server srv foreign data wrapper fdw;
"), @"
    info: find references
      ╭▸ file0.sql:2:29
      │
    2 │ create foreign data wrapper fdw;
      │                             ┬──
      │                             │
      │                             1. source
      │                             2. reference
    3 │ create server srv foreign data wrapper fdw;
      ╰╴                                       ─── 3. reference
    ");
}

#[test]
fn json_path_name_ref() {
    assert_snapshot!(refs("
select * from json_table(
  '{}'::jsonb, '$' as root$0
  columns (value text path '$')
  plan (root)
);
"), @"
    info: find references
      ╭▸ file0.sql:3:23
      │
    3 │   '{}'::jsonb, '$' as root
      │                       ┬───
      │                       │
      │                       1. source
      │                       2. reference
    4 │   columns (value text path '$')
    5 │   plan (root)
      ╰╴        ──── 3. reference
    ");
}

#[test]
fn language_ref() {
    assert_snapshot!(refs("
create language mylang$0;
create function f() returns int language mylang as $$x$$;
"), @"
    info: find references
      ╭▸ file0.sql:2:17
      │
    2 │ create language mylang;
      │                 ┬─────
      │                 │
      │                 1. source
      │                 2. reference
    3 │ create function f() returns int language mylang as $$x$$;
      ╰╴                                         ────── 3. reference
    ");
}

#[test]
fn policy_ref() {
    assert_snapshot!(refs("
create table t(c int);
create policy p$0 on t;
drop policy p on t;
"), @"
    info: find references
      ╭▸ file0.sql:3:15
      │
    3 │ create policy p on t;
      │               ┬
      │               │
      │               1. source
      │               2. reference
    4 │ drop policy p on t;
      ╰╴            ─ 3. reference
    ");
}

#[test]
fn prepared_statement_ref() {
    assert_snapshot!(refs("
prepare stmt$0 as select 1;
execute stmt;
"), @"
    info: find references
      ╭▸ file0.sql:2:9
      │
    2 │ prepare stmt as select 1;
      │         ┬───
      │         │
      │         1. source
      │         2. reference
    3 │ execute stmt;
      ╰╴        ──── 3. reference
    ");
}

#[test]
fn publication_ref() {
    assert_snapshot!(refs("
create table t(id int);
create publication pub$0 for table t;
alter publication pub add table t;
"), @"
    info: find references
      ╭▸ file0.sql:3:20
      │
    3 │ create publication pub for table t;
      │                    ┬──
      │                    │
      │                    1. source
      │                    2. reference
    4 │ alter publication pub add table t;
      ╰╴                  ─── 3. reference
    ");
}

#[test]
fn role_ref() {
    assert_snapshot!(refs("
create role reader$0;
drop role reader;
"), @"
    info: find references
      ╭▸ file0.sql:2:13
      │
    2 │ create role reader;
      │             ┬─────
      │             │
      │             1. source
      │             2. reference
    3 │ drop role reader;
      ╰╴          ────── 3. reference
    ");
}

#[test]
fn rule_ref() {
    assert_snapshot!(refs("
create table t(a int);
create rule r$0 as on select to t do instead nothing;
drop rule r on t;
"), @"
    info: find references
      ╭▸ file0.sql:3:13
      │
    3 │ create rule r as on select to t do instead nothing;
      │             ┬
      │             │
      │             1. source
      │             2. reference
    4 │ drop rule r on t;
      ╰╴          ─ 3. reference
    ");
}

#[test]
fn savepoint_ref() {
    assert_snapshot!(refs("
begin;
savepoint sp$0;
release savepoint sp;
"), @"
    info: find references
      ╭▸ file0.sql:3:11
      │
    3 │ savepoint sp;
      │           ┬─
      │           │
      │           1. source
      │           2. reference
    4 │ release savepoint sp;
      ╰╴                  ── 3. reference
    ");
}

#[test]
fn schema_ref() {
    assert_snapshot!(refs("
create schema app$0;
drop schema app;
"), @"
    info: find references
      ╭▸ file0.sql:2:15
      │
    2 │ create schema app;
      │               ┬──
      │               │
      │               1. source
      │               2. reference
    3 │ drop schema app;
      ╰╴            ─── 3. reference
    ");
}

#[test]
fn server_ref() {
    assert_snapshot!(refs("
create server myserver$0 foreign data wrapper fdw;
drop server myserver;
"), @"
    info: find references
      ╭▸ file0.sql:2:15
      │
    2 │ create server myserver foreign data wrapper fdw;
      │               ┬───────
      │               │
      │               1. source
      │               2. reference
    3 │ drop server myserver;
      ╰╴            ──────── 3. reference
    ");
}

#[test]
fn subscription_ref() {
    assert_snapshot!(refs("
create subscription sub$0 connection $$host=localhost$$ publication pub;
alter subscription sub refresh publication;
"), @"
    info: find references
      ╭▸ file0.sql:2:21
      │
    2 │ create subscription sub connection $$host=localhost$$ publication pub;
      │                     ┬──
      │                     │
      │                     1. source
      │                     2. reference
    3 │ alter subscription sub refresh publication;
      ╰╴                   ─── 3. reference
    ");
}

#[test]
fn tablespace_ref() {
    assert_snapshot!(refs("
create tablespace ts$0 location '/tmp/ts';
drop tablespace ts;
"), @"
    info: find references
      ╭▸ file0.sql:2:19
      │
    2 │ create tablespace ts location '/tmp/ts';
      │                   ┬─
      │                   │
      │                   1. source
      │                   2. reference
    3 │ drop tablespace ts;
      ╰╴                ── 3. reference
    ");
}

#[test]
fn trigger_ref() {
    assert_snapshot!(refs("
create table t ();
create function f() returns trigger
  language plpgsql
  as $$begin return new; end$$;
create trigger tr$0
  before insert
  on t
  for each row
  execute function f();
drop trigger tr on t;
"), @"
    info: find references
       ╭▸ file0.sql:6:16
       │
     6 │ create trigger tr
       │                ┬─
       │                │
       │                1. source
       │                2. reference
       ‡
    11 │ drop trigger tr on t;
       ╰╴             ── 3. reference
    ");
}

#[test]
fn vertex_table_ref() {
    assert_snapshot!(refs("
create table v1(id int);
create table e1(source_id int references v1);
create property graph g
  vertex tables (v1 as source_vertex)
  edge tables (e1 source source_vertex$0 destination source_vertex);
"), @"
    info: find references
      ╭▸ file0.sql:5:24
      │
    5 │   vertex tables (v1 as source_vertex)
      │                        ───────────── 2. reference
    6 │   edge tables (e1 source source_vertex destination source_vertex);
      │                          ┬────────────             ───────────── 4. reference
      │                          │
      │                          1. source
      ╰╴                         3. reference
    ");
}

#[test]
fn window_ref() {
    assert_snapshot!(refs("
select row_number() over w
window w$0 as (partition by 1);
"), @"
    info: find references
      ╭▸ file0.sql:2:26
      │
    2 │ select row_number() over w
      │                          ─ 2. reference
    3 │ window w as (partition by 1);
      │        ┬
      │        │
      │        1. source
      ╰╴       3. reference
    ");
}

#[test]
fn case_insensitive_part_2() {
    // we should see refs for `drop table` and `table`
    assert_snapshot!(refs(r#"
create table actors();
create table "Actors"();
table actors;
drop table ACTORS$0;
"#), @r#"
    info: find references
      ╭▸ file0.sql:2:14
      │
    2 │ create table actors();
      │              ────── 2. reference
    3 │ create table "Actors"();
    4 │ table actors;
      │       ────── 3. reference
    5 │ drop table ACTORS;
      │            ┬─────
      │            │
      │            1. source
      ╰╴           4. reference
    "#);
}
