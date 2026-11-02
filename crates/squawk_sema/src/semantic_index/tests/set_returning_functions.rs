use insta::assert_snapshot;

use super::check;

#[test]
fn set_returning_function_placement() {
    assert_snapshot!(check("
create table t (
  a int,
  b int
);
create function f() returns setof int
  language sql
  as 'select 1';
create function g() returns table (x int)
  language sql
  as 'select 1';

select a from t where generate_series(1, 2) > 0;
select 1 from t join t u on generate_series(1, 2) > 0;
select 1 from t group by a having generate_series(1, 2) > 0;
select a from t limit generate_series(1, 2);
select a from t offset generate_series(1, 2);
select count(*) filter (where generate_series(1, 2) > 0) from t;
select count(*) over (
  order by a rows between generate_series(1, 2) preceding and current row
)
from t;
values (generate_series(1, 2));
insert into t (a) values (generate_series(1, 2)), (3);
insert into t (a) values (1) returning generate_series(1, 2);
update t set a = generate_series(1, 2);
delete from t where f() > 0;
select a from t where g() > 0;
select a from t where unnest(array[1]) > 0;
create table c (a int check (a > generate_series(1, 2)));
create table d (a int default generate_series(1, 2));
create index on t ((generate_series(1, a)));

select generate_series(1, 2) from t;
select generate_series(1, generate_series(1, 2)) from t;
select generate_series(1, 2) + 1 from t;
select 1 from t group by generate_series(1, 2);
select a from t order by generate_series(1, 2);
select distinct on (generate_series(1, 2)) a from t;
select count(*) over (partition by generate_series(1, 2)) from t;
insert into t (a) values (generate_series(1, 2));
insert into t (a) select generate_series(1, 2);
select a from t where (select generate_series(1, 1)) > 0;
"), @"
    error: set-returning functions are not allowed in `where`
       ╭▸ 
    13 │ select a from t where generate_series(1, 2) > 0;
       ╰╴                      ━━━━━━━━━━━━━━━━━━━━━
    error: set-returning functions are not allowed in `join` conditions
       ╭▸ 
    14 │ select 1 from t join t u on generate_series(1, 2) > 0;
       ╰╴                            ━━━━━━━━━━━━━━━━━━━━━
    error: set-returning functions are not allowed in `having`
       ╭▸ 
    15 │ select 1 from t group by a having generate_series(1, 2) > 0;
       ╰╴                                  ━━━━━━━━━━━━━━━━━━━━━
    error: set-returning functions are not allowed in `limit`
       ╭▸ 
    16 │ select a from t limit generate_series(1, 2);
       ╰╴                      ━━━━━━━━━━━━━━━━━━━━━
    error: set-returning functions are not allowed in `offset`
       ╭▸ 
    17 │ select a from t offset generate_series(1, 2);
       ╰╴                       ━━━━━━━━━━━━━━━━━━━━━
    error: set-returning functions are not allowed in `filter`
       ╭▸ 
    18 │ select count(*) filter (where generate_series(1, 2) > 0) from t;
       ╰╴                              ━━━━━━━━━━━━━━━━━━━━━
    error: set-returning functions are not allowed in window definitions
       ╭▸ 
    20 │   order by a rows between generate_series(1, 2) preceding and current row
       ╰╴                          ━━━━━━━━━━━━━━━━━━━━━
    error: set-returning functions are not allowed in `values`
       ╭▸ 
    23 │ values (generate_series(1, 2));
       ╰╴        ━━━━━━━━━━━━━━━━━━━━━
    error: set-returning functions are not allowed in `values`
       ╭▸ 
    24 │ insert into t (a) values (generate_series(1, 2)), (3);
       ╰╴                          ━━━━━━━━━━━━━━━━━━━━━
    error: set-returning functions are not allowed in `returning`
       ╭▸ 
    25 │ insert into t (a) values (1) returning generate_series(1, 2);
       ╰╴                                       ━━━━━━━━━━━━━━━━━━━━━
    error: set-returning functions are not allowed in `update`
       ╭▸ 
    26 │ update t set a = generate_series(1, 2);
       ╰╴                 ━━━━━━━━━━━━━━━━━━━━━
    error: set-returning functions are not allowed in `where`
       ╭▸ 
    27 │ delete from t where f() > 0;
       ╰╴                    ━━━
    error: set-returning functions are not allowed in `where`
       ╭▸ 
    28 │ select a from t where g() > 0;
       ╰╴                      ━━━
    error: set-returning functions are not allowed in `where`
       ╭▸ 
    29 │ select a from t where unnest(array[1]) > 0;
       ╰╴                      ━━━━━━━━━━━━━━━━
    error: set-returning functions are not allowed in check constraints
       ╭▸ 
    30 │ create table c (a int check (a > generate_series(1, 2)));
       ╰╴                                 ━━━━━━━━━━━━━━━━━━━━━
    error: set-returning functions are not allowed in `default` expressions
       ╭▸ 
    31 │ create table d (a int default generate_series(1, 2));
       ╰╴                              ━━━━━━━━━━━━━━━━━━━━━
    error: set-returning functions are not allowed in index expressions
       ╭▸ 
    32 │ create index on t ((generate_series(1, a)));
       ╰╴                    ━━━━━━━━━━━━━━━━━━━━━
    ");
}

#[test]
fn set_returning_function_in_expression() {
    assert_snapshot!(check("
create table t (
  a int,
  b int
);

select sum(generate_series(1, 2)) from t;
select sum(a order by generate_series(1, 2)) from t;
select sum(generate_series(1, 2)) over () from t;
select case when a > 0 then generate_series(1, 2) end from t;
select case generate_series(1, 2) when 1 then a end from t;
select coalesce(generate_series(1, 2), 0) from t;
select coalesce(case when a > 0 then generate_series(1, 2) end, 0) from t;

select sum((select generate_series(1, 1))) from t;
select case when a > 0 then (select generate_series(1, 1)) end from t;
select count(*) over (partition by generate_series(1, 2)) from t;
select nullif(generate_series(1, 2), 0) from t;
select greatest(generate_series(1, 2), 0) from t;
"), @"
    error: aggregate calls cannot contain set-returning function calls
      ╭▸ 
    7 │ select sum(generate_series(1, 2)) from t;
      │            ━━━━━━━━━━━━━━━━━━━━━
      │
      ╰ hint: you might be able to move the set-returning function into a `lateral` `from` item
    error: aggregate calls cannot contain set-returning function calls
      ╭▸ 
    8 │ select sum(a order by generate_series(1, 2)) from t;
      │                       ━━━━━━━━━━━━━━━━━━━━━
      │
      ╰ hint: you might be able to move the set-returning function into a `lateral` `from` item
    error: window function calls cannot contain set-returning function calls
      ╭▸ 
    9 │ select sum(generate_series(1, 2)) over () from t;
      │            ━━━━━━━━━━━━━━━━━━━━━
      │
      ╰ hint: you might be able to move the set-returning function into a `lateral` `from` item
    error: set-returning functions are not allowed in `case`
       ╭▸ 
    10 │ select case when a > 0 then generate_series(1, 2) end from t;
       │                             ━━━━━━━━━━━━━━━━━━━━━
       │
       ╰ hint: you might be able to move the set-returning function into a `lateral` `from` item
    error: set-returning functions are not allowed in `case`
       ╭▸ 
    11 │ select case generate_series(1, 2) when 1 then a end from t;
       │             ━━━━━━━━━━━━━━━━━━━━━
       │
       ╰ hint: you might be able to move the set-returning function into a `lateral` `from` item
    error: set-returning functions are not allowed in `coalesce`
       ╭▸ 
    12 │ select coalesce(generate_series(1, 2), 0) from t;
       │                 ━━━━━━━━━━━━━━━━━━━━━
       │
       ╰ hint: you might be able to move the set-returning function into a `lateral` `from` item
    error: set-returning functions are not allowed in `case`
       ╭▸ 
    13 │ select coalesce(case when a > 0 then generate_series(1, 2) end, 0) from t;
       │                                      ━━━━━━━━━━━━━━━━━━━━━
       │
       ╰ hint: you might be able to move the set-returning function into a `lateral` `from` item
    ");
}

#[test]
fn set_returning_function_in_from() {
    assert_snapshot!(check("
create table t (
  a int,
  b int
);

select * from generate_series(generate_series(1, 2), 3);
select * from abs(generate_series(1, 2));
select *
from rows from (
  generate_series(1, 2),
  generate_series(generate_series(1, 2), 3)
);
select * from unnest(array[1], array[generate_series(1, 2)]);
select * from t tablesample bernoulli (generate_series(1, 2));

select * from generate_series(1, 2);
select * from t, lateral generate_series(1, t.a);
select * from rows from (generate_series(1, 2), generate_series(1, 3));
select * from generate_series(1, (select max(x) from generate_series(1, 2) x));
"), @"
    error: set-returning functions must appear at top level of `from`
      ╭▸ 
    7 │ select * from generate_series(generate_series(1, 2), 3);
      ╰╴                              ━━━━━━━━━━━━━━━━━━━━━
    error: set-returning functions must appear at top level of `from`
      ╭▸ 
    8 │ select * from abs(generate_series(1, 2));
      ╰╴                  ━━━━━━━━━━━━━━━━━━━━━
    error: set-returning functions must appear at top level of `from`
       ╭▸ 
    12 │   generate_series(generate_series(1, 2), 3)
       ╰╴                  ━━━━━━━━━━━━━━━━━━━━━
    error: set-returning functions must appear at top level of `from`
       ╭▸ 
    14 │ select * from unnest(array[1], array[generate_series(1, 2)]);
       ╰╴                                     ━━━━━━━━━━━━━━━━━━━━━
    error: argument of `tablesample` must not return a set
       ╭▸ 
    15 │ select * from t tablesample bernoulli (generate_series(1, 2));
       ╰╴                                       ━━━━━━━━━━━━━━━━━━━━━
    ");
}
