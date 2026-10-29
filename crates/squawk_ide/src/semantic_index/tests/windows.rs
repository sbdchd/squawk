use insta::assert_snapshot;

use super::check;

#[test]
fn window_clause() {
    assert_snapshot!(check("
create table t(a int, b int);
select count(*) over w from t window w as (partition by a order by b);
select count(*) over nope from t window w as ();
select count(*) over (nope2 order by a) from t;
select count(*) over w2 from t window w2 as (w2), w3 as (nope3);
select count(*) over w from t window w as (partition by nope4);
select (select count(*) over w from t) from t window w as ();
select count(*) over w from t window w as () order by sum(a) over w;
select * from (select count(*) over w from t window w as ()) s;
select * from t where false union select count(*) over w, 1 from t window w as ();
"), @"
    error: window `nope` does not exist
      ╭▸ 
    4 │ select count(*) over nope from t window w as ();
      ╰╴                     ━━━━
    error: window `nope2` does not exist
      ╭▸ 
    5 │ select count(*) over (nope2 order by a) from t;
      ╰╴                      ━━━━━
    error: window `w2` does not exist
      ╭▸ 
    6 │ select count(*) over w2 from t window w2 as (w2), w3 as (nope3);
      ╰╴                                             ━━
    error: window `nope3` does not exist
      ╭▸ 
    6 │ select count(*) over w2 from t window w2 as (w2), w3 as (nope3);
      ╰╴                                                         ━━━━━
    error: column `nope4` does not exist
      ╭▸ 
    7 │ select count(*) over w from t window w as (partition by nope4);
      ╰╴                                                        ━━━━━
    error: window `w` does not exist
      ╭▸ 
    8 │ select (select count(*) over w from t) from t window w as ();
      ╰╴                             ━
    ");
}

#[test]
fn similar_window() {
    assert_snapshot!(check("
create table t (a int, b int);
select count(*) over by_dpt from t window by_dept as (partition by a);
select count(*) over w from t window w as (by_dpt), by_dept as ();
"), @"
    error: window `by_dpt` does not exist
      ╭▸ 
    3 │ select count(*) over by_dpt from t window by_dept as (partition by a);
      │                      ━━━━━━
      ╰╴
    help: a window with a similar name exists
      ╭╴
    3 │ select count(*) over by_dept from t window by_dept as (partition by a);
      ╰╴                         +
    error: window `by_dpt` does not exist
      ╭▸ 
    4 │ select count(*) over w from t window w as (by_dpt), by_dept as ();
      ╰╴                                           ━━━━━━
    ");
}

#[test]
fn duplicate_window() {
    assert_snapshot!(check("
select 1 window w as (), w as ();
select 1 window w as (), x as (w);
"), @"
    error: window `w` is already defined
      ╭▸ 
    2 │ select 1 window w as (), w as ();
      ╰╴                         ━
    ");
}

#[test]
fn window_function_not_allowed() {
    assert_snapshot!(check("
create table t (a int);
select 1 from t where row_number() over () > 1;
select 1 from t join t u on row_number() over () > 1;
select 1 from t group by row_number() over ();
select 1 from t having row_number() over () > 1;
select count(*) filter (where row_number() over () > 1) from t;
select row_number() over (order by row_number() over ()) from t;
select 1 from t limit row_number() over ();
values (row_number() over ());
update t set a = row_number() over ();
update t set a = 1 returning row_number() over ();
select * from t, generate_series(1, row_number() over ());
create table c (a int check (row_number() over () > 1));
select lag(lag(a) over ()) over () from t;
select row_number() over () + 1 from t;
select row_number() over () from t order by row_number() over ();
select 1 from t where exists (select row_number() over () from t u);
insert into t (a[row_number() over ()]) values (1);
prepare pp(bigint) as select $1;
execute pp(row_number() over ());
"), @"
    error: window functions are not allowed in `where`
      ╭▸ 
    3 │ select 1 from t where row_number() over () > 1;
      ╰╴                      ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in `join` conditions
      ╭▸ 
    4 │ select 1 from t join t u on row_number() over () > 1;
      ╰╴                            ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in `group by`
      ╭▸ 
    5 │ select 1 from t group by row_number() over ();
      ╰╴                         ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in `having`
      ╭▸ 
    6 │ select 1 from t having row_number() over () > 1;
      ╰╴                       ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in `filter`
      ╭▸ 
    7 │ select count(*) filter (where row_number() over () > 1) from t;
      ╰╴                              ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in window definitions
      ╭▸ 
    8 │ select row_number() over (order by row_number() over ()) from t;
      ╰╴                                   ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in `limit`
      ╭▸ 
    9 │ select 1 from t limit row_number() over ();
      ╰╴                      ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in `values`
       ╭▸ 
    10 │ values (row_number() over ());
       ╰╴        ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in `update`
       ╭▸ 
    11 │ update t set a = row_number() over ();
       ╰╴                 ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in `returning`
       ╭▸ 
    12 │ update t set a = 1 returning row_number() over ();
       ╰╴                             ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in functions in `from`
       ╭▸ 
    13 │ select * from t, generate_series(1, row_number() over ());
       ╰╴                                    ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in check constraints
       ╭▸ 
    14 │ create table c (a int check (row_number() over () > 1));
       ╰╴                             ━━━━━━━━━━━━━━━━━━━━
    error: window function calls cannot be nested
       ╭▸ 
    15 │ select lag(lag(a) over ()) over () from t;
       ╰╴           ━━━━━━━━━━━━━━
    error: window functions are not allowed in `insert`
       ╭▸ 
    19 │ insert into t (a[row_number() over ()]) values (1);
       ╰╴                 ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in `execute` parameters
       ╭▸ 
    21 │ execute pp(row_number() over ());
       ╰╴           ━━━━━━━━━━━━━━━━━━━━
    ");
}

#[test]
fn window_function_without_over() {
    assert_snapshot!(check("
create table t (a int);
select rank() from t;
select rank() over () from t;
select lag(a) from t;
create function w(int) returns int window language c as 'w';
select w(a) from t;
select w(a) over () from t;
select count(*) from t;
"), @"
    error: window function `rank` requires an `over` clause
      ╭▸ 
    3 │ select rank() from t;
      ╰╴       ━━━━━━
    error: window function `lag` requires an `over` clause
      ╭▸ 
    5 │ select lag(a) from t;
      ╰╴       ━━━━━━
    error: window function `w` requires an `over` clause
      ╭▸ 
    7 │ select w(a) from t;
      ╰╴       ━━━━
    ");
}

#[test]
fn window_override() {
    assert_snapshot!(check("
create table t (
  a int,
  b int
);
select count(*) over w2
from t
window w as (partition by a), w2 as (w partition by b);
select count(*) over w2 from t window w as (order by a), w2 as (w order by b);
select count(*) over w3
from t
window w as (partition by a), w2 as (w order by b), w3 as (w2 partition by a);
select count(*) over w2
from t
window w as (order by a rows unbounded preceding), w2 as (w);
select count(*) over (w)
from t
window w as (order by a rows unbounded preceding);
select count(*) over (w rows unbounded preceding)
from t
window w as (order by a rows unbounded preceding);
select count(*) over (w order by b)
from t
window w as (partition by a order by a);
select count(*) over (w partition by b) from t window w as ();
select count(*) over w from t window w as (order by a rows unbounded preceding);
select count(*) over (w order by b rows unbounded preceding)
from t
window w as (partition by a);
"), @"
    error: cannot override `partition by` clause of window `w`
      ╭▸ 
    8 │ window w as (partition by a), w2 as (w partition by b);
      ╰╴                                     ━
    error: cannot override `order by` clause of window `w`
      ╭▸ 
    9 │ select count(*) over w2 from t window w as (order by a), w2 as (w order by b);
      ╰╴                                                                ━
    error: cannot override `partition by` clause of window `w2`
       ╭▸ 
    12 │ window w as (partition by a), w2 as (w order by b), w3 as (w2 partition by a);
       ╰╴                                                           ━━
    error: cannot copy window `w` because it has a frame clause
       ╭▸ 
    15 │ window w as (order by a rows unbounded preceding), w2 as (w);
       ╰╴                                                          ━
    error: cannot copy window `w` because it has a frame clause
       ╭▸ 
    16 │ select count(*) over (w)
       │                       ━
       │
       ╰ hint: omit the parentheses in this `over` clause
    error: cannot copy window `w` because it has a frame clause
       ╭▸ 
    19 │ select count(*) over (w rows unbounded preceding)
       ╰╴                      ━
    error: cannot override `order by` clause of window `w`
       ╭▸ 
    22 │ select count(*) over (w order by b)
       ╰╴                      ━
    error: cannot override `partition by` clause of window `w`
       ╭▸ 
    25 │ select count(*) over (w partition by b) from t window w as ();
       ╰╴                      ━
    ");
}

#[test]
fn window_aggregate_order_by_and_distinct() {
    assert_snapshot!(check("
create table t (
  a int,
  b int
);
select array_agg(a order by b) over () from t;
select string_agg(a::text, ',' order by b) over () from t;
select count(distinct a) over () from t;
select array_agg(a order by b), count(distinct a) from t;
"), @"
    error: aggregate `order by` is not implemented for window functions
      ╭▸ 
    6 │ select array_agg(a order by b) over () from t;
      ╰╴                   ━━━━━━━━━━
    error: aggregate `order by` is not implemented for window functions
      ╭▸ 
    7 │ select string_agg(a::text, ',' order by b) over () from t;
      ╰╴                               ━━━━━━━━━━
    error: `distinct` is not implemented for window functions
      ╭▸ 
    8 │ select count(distinct a) over () from t;
      ╰╴             ━━━━━━━━
    ");
}
