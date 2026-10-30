use insta::assert_snapshot;

use super::check;

#[test]
fn create_aggregate_attributes() {
    assert_snapshot!(check("
create function add(int, int) returns int language sql return $1 + $2;
create function fin(int) returns numeric language sql return $1;
create aggregate s(int) (sfunc = add, stype = int, finalfunc = fin, combinefunc = add, initcond = '0');
create aggregate bad(int) (sfunc = nope, stype = nope_type, finalfunc = add, sortop = <);
create aggregate old (basetype = int, sfunc = add, stype = int);
select s(1);
"), @"
    error: function `nope` does not exist
      ╭▸ 
    5 │ create aggregate bad(int) (sfunc = nope, stype = nope_type, finalfunc = add, sortop = <);
      ╰╴                                   ━━━━
    error: type `nope_type` does not exist
      ╭▸ 
    5 │ create aggregate bad(int) (sfunc = nope, stype = nope_type, finalfunc = add, sortop = <);
      ╰╴                                                 ━━━━━━━━━
    error: function `add` does not exist
      ╭▸ 
    5 │ create aggregate bad(int) (sfunc = nope, stype = nope_type, finalfunc = add, sortop = <);
      ╰╴                                                                        ━━━
    ");
}

#[test]
fn create_aggregate_finalfunc_extra() {
    assert_snapshot!(check("
create function sfn(internal, int) returns internal language c as 'x';
create function ffn(internal, int) returns int language c as 'x';
create aggregate a(int) (sfunc = sfn, stype = internal, finalfunc = ffn, finalfunc_extra);
create aggregate b(int) (sfunc = sfn, stype = internal, finalfunc = ffn);
"), @"
    error: function `ffn` does not exist
      ╭▸ 
    5 │ create aggregate b(int) (sfunc = sfn, stype = internal, finalfunc = ffn);
      ╰╴                                                                    ━━━
    ");
}

#[test]
fn create_ordered_set_aggregate() {
    assert_snapshot!(check("
create function sfn(internal, text) returns internal language c as 'x';
create function ffn(internal, float8) returns text language c as 'x';
create aggregate pct(float8 order by text) (sfunc = sfn, stype = internal, finalfunc = ffn);
"), @"no diagnostics");
}

#[test]
fn ordered_set_aggregate_identity() {
    assert_snapshot!(check("
create function sfn(internal, text) returns internal language c as 'x';
create function sfn1(internal, float8) returns internal language c as 'x';
create function sfn2(internal, float8, text) returns internal language c as 'x';
create function ffn(internal, float8) returns text language c as 'x';
create aggregate pct(float8 order by text) (sfunc = sfn, stype = internal, finalfunc = ffn);
create aggregate pct(float8) (sfunc = sfn1, stype = internal);
create aggregate pct(float8, text) (sfunc = sfn2, stype = internal);
"), @"
    error: function `pct` already exists with same argument types
      ╭▸ 
    8 │ create aggregate pct(float8, text) (sfunc = sfn2, stype = internal);
      ╰╴                 ━━━
    ");
}

#[test]
fn drop_ordered_set_aggregate() {
    assert_snapshot!(check("
create function sfn(internal, text) returns internal language c as 'x';
create function ffn(internal, float8) returns text language c as 'x';
create aggregate pct(float8 order by text) (sfunc = sfn, stype = internal, finalfunc = ffn);
drop aggregate pct(float8);
drop aggregate pct(float8 order by text);
drop aggregate pct(float8 order by text);
create aggregate pct(float8 order by text) (sfunc = sfn, stype = internal, finalfunc = ffn);
alter aggregate pct(float8, text) rename to pct2;
drop aggregate pct2(float8, text);
"), @"
    error: aggregate `pct(float8)` does not exist
      ╭▸ 
    5 │ drop aggregate pct(float8);
      ╰╴               ━━━
    error: aggregate `pct(float8, text)` does not exist
      ╭▸ 
    7 │ drop aggregate pct(float8 order by text);
      ╰╴               ━━━
    ");
}

#[test]
fn ordered_set_aggregate_call() {
    assert_snapshot!(check("
create function sfn(internal, text) returns internal language c as 'x';
create function ffn(internal, float8) returns text language c as 'x';
create aggregate pct(float8 order by text) (sfunc = sfn, stype = internal, finalfunc = ffn);
create table t (a text, b int);
select pct(0.5) within group (order by a) from t;
select pct(0.5) within group (order by nope) from t;
"), @"
    error: column `nope` does not exist
      ╭▸ 
    7 │ select pct(0.5) within group (order by nope) from t;
      ╰╴                                       ━━━━
    ");
}

#[test]
fn old_style_aggregate_params() {
    assert_snapshot!(check(r#"
create function add(int, int) returns int language sql return $1 + $2;
create aggregate old (basetype = int, sfunc = add, stype = int);
create aggregate cnt (basetype = "any", sfunc = int8inc, stype = int8);
select old(1), cnt(*);
select old(1, 2);
drop aggregate old(int);
drop aggregate cnt(*);
select old(1), cnt(*);
create aggregate old (basetype = int, sfunc = add, stype = int);
create aggregate old(int) (sfunc = add, stype = int);
"#), @"
    error: function `old(int4, int4)` does not exist
      ╭▸ 
    6 │ select old(1, 2);
      │        ━━━
      │
      ╰ hint: consider adding explicit type casts
    error: function `old(int4)` does not exist
      ╭▸ 
    9 │ select old(1), cnt(*);
      │        ━━━
      ╰╴
    help: a function with a similar name exists
      ╭╴
    9 - select old(1), cnt(*);
    9 + select oid(1), cnt(*);
      ╰╴
    error: function `cnt()` does not exist
      ╭▸ 
    9 │ select old(1), cnt(*);
      │                ━━━
      ╰╴
    help: a function with a similar name exists
      ╭╴
    9 - select old(1), cnt(*);
    9 + select old(1), cot(*);
      ╰╴
    error: function `old` already exists with same argument types
       ╭▸ 
    11 │ create aggregate old(int) (sfunc = add, stype = int);
       ╰╴                 ━━━
    ");
}

#[test]
fn drop_aggregate() {
    assert_snapshot!(check("
create function add(int, int) returns int language sql return $1 + $2;
create aggregate s(int) (sfunc = add, stype = int);
create aggregate c(*) (sfunc = int8inc, stype = int8);
drop function s(int);
drop aggregate s(int), c(*);
select s(1), c();
drop aggregate s(int);
drop aggregate if exists s(int);
drop aggregate add(int, int);
"), @"
    error: function `s(int4)` does not exist
      ╭▸ 
    5 │ drop function s(int);
      ╰╴              ━
    error: function `s(int4)` does not exist
      ╭▸ 
    7 │ select s(1), c();
      │        ━
      │
      ╰ hint: consider adding explicit type casts
    error: function `c()` does not exist
      ╭▸ 
    7 │ select s(1), c();
      ╰╴             ━
    error: aggregate `s(int4)` does not exist
      ╭▸ 
    8 │ drop aggregate s(int);
      ╰╴               ━
    error: aggregate `add(int4, int4)` does not exist
       ╭▸ 
    10 │ drop aggregate add(int, int);
       ╰╴               ━━━
    ");
}

#[test]
fn alter_aggregate() {
    assert_snapshot!(check("
create schema x;
create function add(int, int) returns int language sql return $1 + $2;
create aggregate s(int) (sfunc = add, stype = int);
alter aggregate s(int) rename to t;
select s(1);
select t(1);
alter aggregate t(int) set schema x;
select x.t(1);
alter aggregate x.t(int) owner to nope;
alter aggregate add(int, int) rename to y;
"), @"
    error: function `s(int4)` does not exist
      ╭▸ 
    6 │ select s(1);
      │        ━
      ╰╴
    help: a function with a similar name exists
      ╭╴
    6 - select s(1);
    6 + select t(1);
      ╰╴
    error: role `nope` does not exist
       ╭▸ 
    10 │ alter aggregate x.t(int) owner to nope;
       ╰╴                                  ━━━━
    error: aggregate `add(int4, int4)` does not exist
       ╭▸ 
    11 │ alter aggregate add(int, int) rename to y;
       ╰╴                ━━━
    ");
}

#[test]
fn aggregate_not_allowed() {
    assert_snapshot!(check("
create table t (a int);
select 1 from t where count(*) > 1;
select 1 from t join t u on count(*) > 1;
select 1 from t group by count(*);
select count(*) filter (where count(*) > 1) from t;
select sum(1) over (rows between count(*) preceding and current row) from t;
select 1 from t limit count(*);
select 1 from t offset count(*);
values (count(*));
insert into t values (count(*));
insert into t values (1) returning count(*);
update t set a = count(*);
delete from t where count(*) > 1;
select * from t, generate_series(1, count(*));
merge into t using t u on t.a = u.a when matched and count(*) > 1 then do nothing;
merge into t using t u on count(*) > 1 when matched then do nothing;
merge into t using t u on t.a = u.a when matched then update set a = count(*);
merge into t using t u on t.a = u.a when not matched then insert values (count(*));
select 1 from t where exists (select 1 from t u where count(u.a) > 1);
select * from t tablesample bernoulli (count(*));
select * from t tablesample bernoulli (1) repeatable (count(*));
prepare pp(bigint) as select $1;
execute pp(count(*));
"), @"
    error: aggregate functions are not allowed in `where`
      ╭▸ 
    3 │ select 1 from t where count(*) > 1;
      ╰╴                      ━━━━━━━━
    error: aggregate functions are not allowed in `join` conditions
      ╭▸ 
    4 │ select 1 from t join t u on count(*) > 1;
      ╰╴                            ━━━━━━━━
    error: aggregate functions are not allowed in `group by`
      ╭▸ 
    5 │ select 1 from t group by count(*);
      ╰╴                         ━━━━━━━━
    error: aggregate functions are not allowed in `filter`
      ╭▸ 
    6 │ select count(*) filter (where count(*) > 1) from t;
      ╰╴                              ━━━━━━━━
    error: aggregate functions are not allowed in window `rows`
      ╭▸ 
    7 │ select sum(1) over (rows between count(*) preceding and current row) from t;
      ╰╴                                 ━━━━━━━━
    error: aggregate functions are not allowed in `limit`
      ╭▸ 
    8 │ select 1 from t limit count(*);
      ╰╴                      ━━━━━━━━
    error: aggregate functions are not allowed in `offset`
      ╭▸ 
    9 │ select 1 from t offset count(*);
      ╰╴                       ━━━━━━━━
    error: aggregate functions are not allowed in `values`
       ╭▸ 
    10 │ values (count(*));
       ╰╴        ━━━━━━━━
    error: aggregate functions are not allowed in `values`
       ╭▸ 
    11 │ insert into t values (count(*));
       ╰╴                      ━━━━━━━━
    error: aggregate functions are not allowed in `returning`
       ╭▸ 
    12 │ insert into t values (1) returning count(*);
       ╰╴                                   ━━━━━━━━
    error: aggregate functions are not allowed in `update`
       ╭▸ 
    13 │ update t set a = count(*);
       ╰╴                 ━━━━━━━━
    error: aggregate functions are not allowed in `where`
       ╭▸ 
    14 │ delete from t where count(*) > 1;
       ╰╴                    ━━━━━━━━
    error: aggregate functions are not allowed in functions in `from`
       ╭▸ 
    15 │ select * from t, generate_series(1, count(*));
       ╰╴                                    ━━━━━━━━
    error: aggregate functions are not allowed in `merge when` conditions
       ╭▸ 
    16 │ merge into t using t u on t.a = u.a when matched and count(*) > 1 then do nothing;
       ╰╴                                                     ━━━━━━━━
    error: aggregate functions are not allowed in `join` conditions
       ╭▸ 
    17 │ merge into t using t u on count(*) > 1 when matched then do nothing;
       ╰╴                          ━━━━━━━━
    error: aggregate functions are not allowed in `update`
       ╭▸ 
    18 │ merge into t using t u on t.a = u.a when matched then update set a = count(*);
       ╰╴                                                                     ━━━━━━━━
    error: aggregate functions are not allowed in `values`
       ╭▸ 
    19 │ merge into t using t u on t.a = u.a when not matched then insert values (count(*));
       ╰╴                                                                         ━━━━━━━━
    error: aggregate functions are not allowed in `where`
       ╭▸ 
    20 │ select 1 from t where exists (select 1 from t u where count(u.a) > 1);
       ╰╴                                                      ━━━━━━━━━━
    error: aggregate functions are not allowed in functions in `from`
       ╭▸ 
    21 │ select * from t tablesample bernoulli (count(*));
       ╰╴                                       ━━━━━━━━
    error: aggregate functions are not allowed in functions in `from`
       ╭▸ 
    22 │ select * from t tablesample bernoulli (1) repeatable (count(*));
       ╰╴                                                      ━━━━━━━━
    error: aggregate functions are not allowed in `execute` parameters
       ╭▸ 
    24 │ execute pp(count(*));
       ╰╴           ━━━━━━━━
    ");
}

#[test]
fn aggregate_allowed() {
    assert_snapshot!(check("
create table t (a int);
select count(*) from t having count(*) > 1;
select count(*) from t order by count(*);
select distinct on (count(*)) 1 from t;
select row_number() over (partition by count(*)) from t;
select 1 from t where (select count(*) from t u) > 1;
select 1 from t where t.a in (select count(*) from t u);
insert into t select count(*) from t;
select array_agg(a) filter (where a > 1) from t;
select 1 from t group by a having exists (select 1 from t u where count(t.a) > 1);
select sum(count(*)) over () from t;
select sum((select sum(a) from t u)) from t;
"), @"no diagnostics");
}

#[test]
fn nested_aggregate() {
    assert_snapshot!(check("
create table t (a int);
select sum(sum(1)) from t;
select string_agg(a::text, ',' order by count(*)) from t;
select percentile_cont(0.5) within group (order by count(*)) from t;
select sum(row_number() over ()) from t;
select max(sum(count(*)) over ()) from t;
select sum(abs(sum(1))) from t;
select 1 from t having sum(sum(1)) > 1;
select sum(sum(sum(1))) from t;
select sum(sum(1) + sum(2)) from t;
"), @"
    error: aggregate function calls cannot be nested
      ╭▸ 
    3 │ select sum(sum(1)) from t;
      ╰╴           ━━━━━━
    error: aggregate function calls cannot be nested
      ╭▸ 
    4 │ select string_agg(a::text, ',' order by count(*)) from t;
      ╰╴                                        ━━━━━━━━
    error: aggregate function calls cannot be nested
      ╭▸ 
    5 │ select percentile_cont(0.5) within group (order by count(*)) from t;
      ╰╴                                                   ━━━━━━━━
    error: aggregate function calls cannot contain window function calls
      ╭▸ 
    6 │ select sum(row_number() over ()) from t;
      ╰╴           ━━━━━━━━━━━━━━━━━━━━
    error: aggregate function calls cannot contain window function calls
      ╭▸ 
    7 │ select max(sum(count(*)) over ()) from t;
      ╰╴           ━━━━━━━━━━━━━━━━━━━━━
    error: aggregate function calls cannot be nested
      ╭▸ 
    8 │ select sum(abs(sum(1))) from t;
      ╰╴               ━━━━━━
    error: aggregate function calls cannot be nested
      ╭▸ 
    9 │ select 1 from t having sum(sum(1)) > 1;
      ╰╴                           ━━━━━━
    error: aggregate function calls cannot be nested
       ╭▸ 
    10 │ select sum(sum(sum(1))) from t;
       ╰╴               ━━━━━━
    error: aggregate function calls cannot be nested
       ╭▸ 
    11 │ select sum(sum(1) + sum(2)) from t;
       ╰╴           ━━━━━━
    ");
}

#[test]
fn aggregate_not_allowed_in_targets() {
    assert_snapshot!(check("
create table t (a int primary key, b int[]);
insert into t (b[count(*)]) values (array[1]);
update t set b[count(*)] = array[1];
merge into t using t u on t.a = u.a when matched then update set b[count(*)] = 1;
merge into t using t u on t.a = u.a when not matched then insert (b[count(*)]) values (1);
insert into t values (1) on conflict (a) do update set b[count(*)] = 1;
insert into t values (1) on conflict ((a + count(*))) do nothing;
insert into t values (1) on conflict (a) where count(*) > 1 do nothing;
"), @"
    error: aggregate functions are not allowed in `insert`
      ╭▸ 
    3 │ insert into t (b[count(*)]) values (array[1]);
      ╰╴                 ━━━━━━━━
    error: aggregate functions are not allowed in `update`
      ╭▸ 
    4 │ update t set b[count(*)] = array[1];
      ╰╴               ━━━━━━━━
    error: aggregate functions are not allowed in `update`
      ╭▸ 
    5 │ merge into t using t u on t.a = u.a when matched then update set b[count(*)] = 1;
      ╰╴                                                                   ━━━━━━━━
    error: aggregate functions are not allowed in `insert`
      ╭▸ 
    6 │ merge into t using t u on t.a = u.a when not matched then insert (b[count(*)]) values (1);
      ╰╴                                                                    ━━━━━━━━
    error: aggregate functions are not allowed in `update`
      ╭▸ 
    7 │ insert into t values (1) on conflict (a) do update set b[count(*)] = 1;
      ╰╴                                                         ━━━━━━━━
    error: aggregate functions are not allowed in index expressions
      ╭▸ 
    8 │ insert into t values (1) on conflict ((a + count(*))) do nothing;
      ╰╴                                           ━━━━━━━━
    error: aggregate functions are not allowed in index predicates
      ╭▸ 
    9 │ insert into t values (1) on conflict (a) where count(*) > 1 do nothing;
      ╰╴                                               ━━━━━━━━
    ");
}

#[test]
fn aggregate_not_allowed_in_ddl() {
    assert_snapshot!(check("
create table t (a int);
create table c1 (a int check (count(*) > 1));
create table c2 (a int default count(*));
create table c3 (a int generated always as (count(*)) stored);
create index on t ((count(*)));
create index on t (a) where count(*) > 1;
create table p (a int) partition by range ((count(*)));
create table p2 partition of p for values from (count(*)) to (2);
create policy pol on t using (count(*) > 1);
create domain d as int check (count(*) > 1);
alter table t alter column a type bigint using count(*);
create statistics s on (count(*)) from t;
copy t from stdin where count(*) > 1;
create procedure pr(bigint) language sql as 'select 1';
call pr(count(*));
create function tf() returns trigger language plpgsql as 'begin return new; end';
create trigger tr before insert on t for each row when (count(*) > 1) execute function tf();
create function fd(x bigint default count(*)) returns int language sql as 'select 1';
alter domain d set default count(*);
create table e (a int, exclude ((count(*)) with =));
alter table t add constraint ex exclude (a with =) where (count(*) > 1);
create rule r as on insert to t where count(*) > 1 do nothing;
"), @"
    error: aggregate functions are not allowed in check constraints
      ╭▸ 
    3 │ create table c1 (a int check (count(*) > 1));
      ╰╴                              ━━━━━━━━
    error: aggregate functions are not allowed in `default` expressions
      ╭▸ 
    4 │ create table c2 (a int default count(*));
      ╰╴                               ━━━━━━━━
    error: aggregate functions are not allowed in column generation expressions
      ╭▸ 
    5 │ create table c3 (a int generated always as (count(*)) stored);
      ╰╴                                            ━━━━━━━━
    error: aggregate functions are not allowed in index expressions
      ╭▸ 
    6 │ create index on t ((count(*)));
      ╰╴                    ━━━━━━━━
    error: aggregate functions are not allowed in index predicates
      ╭▸ 
    7 │ create index on t (a) where count(*) > 1;
      ╰╴                            ━━━━━━━━
    error: aggregate functions are not allowed in partition key expressions
      ╭▸ 
    8 │ create table p (a int) partition by range ((count(*)));
      ╰╴                                            ━━━━━━━━
    error: aggregate functions are not allowed in partition bound
      ╭▸ 
    9 │ create table p2 partition of p for values from (count(*)) to (2);
      ╰╴                                                ━━━━━━━━
    error: aggregate functions are not allowed in policy expressions
       ╭▸ 
    10 │ create policy pol on t using (count(*) > 1);
       ╰╴                              ━━━━━━━━
    error: aggregate functions are not allowed in check constraints
       ╭▸ 
    11 │ create domain d as int check (count(*) > 1);
       ╰╴                              ━━━━━━━━
    error: aggregate functions are not allowed in transform expressions
       ╭▸ 
    12 │ alter table t alter column a type bigint using count(*);
       ╰╴                                               ━━━━━━━━
    error: aggregate functions are not allowed in statistics expressions
       ╭▸ 
    13 │ create statistics s on (count(*)) from t;
       ╰╴                        ━━━━━━━━
    error: aggregate functions are not allowed in `copy from where` conditions
       ╭▸ 
    14 │ copy t from stdin where count(*) > 1;
       ╰╴                        ━━━━━━━━
    error: aggregate functions are not allowed in `call` arguments
       ╭▸ 
    16 │ call pr(count(*));
       ╰╴        ━━━━━━━━
    error: aggregate functions are not allowed in trigger `when` conditions
       ╭▸ 
    18 │ create trigger tr before insert on t for each row when (count(*) > 1) execute function tf();
       ╰╴                                                        ━━━━━━━━
    error: aggregate functions are not allowed in `default` expressions
       ╭▸ 
    19 │ create function fd(x bigint default count(*)) returns int language sql as 'select 1';
       ╰╴                                    ━━━━━━━━
    error: aggregate functions are not allowed in `default` expressions
       ╭▸ 
    20 │ alter domain d set default count(*);
       ╰╴                           ━━━━━━━━
    error: aggregate functions are not allowed in index expressions
       ╭▸ 
    21 │ create table e (a int, exclude ((count(*)) with =));
       ╰╴                                 ━━━━━━━━
    error: aggregate functions are not allowed in index predicates
       ╭▸ 
    22 │ alter table t add constraint ex exclude (a with =) where (count(*) > 1);
       ╰╴                                                          ━━━━━━━━
    error: aggregate functions are not allowed in `where`
       ╭▸ 
    23 │ create rule r as on insert to t where count(*) > 1 do nothing;
       ╰╴                                      ━━━━━━━━
    ");
}

#[test]
fn parameterless_aggregate_without_star() {
    assert_snapshot!(check("
select count();
select count() over ();
select pg_catalog.count();
select count(*), count(*) over (), now(), rank() over (order by 1);
"), @"
    error: `count(*)` must be used to call a parameterless aggregate function
      ╭▸ 
    2 │ select count();
      │        ━━━━━
      ╰╴
    help: use `*` to call an aggregate with no arguments
      ╭╴
    2 │ select count(*);
      ╰╴             +
    error: `count(*)` must be used to call a parameterless aggregate function
      ╭▸ 
    3 │ select count() over ();
      │        ━━━━━
      ╰╴
    help: use `*` to call an aggregate with no arguments
      ╭╴
    3 │ select count(*) over ();
      ╰╴             +
    error: `count(*)` must be used to call a parameterless aggregate function
      ╭▸ 
    4 │ select pg_catalog.count();
      │        ━━━━━━━━━━━━━━━━
      ╰╴
    help: use `*` to call an aggregate with no arguments
      ╭╴
    4 │ select pg_catalog.count(*);
      ╰╴                        +
    ");
}
