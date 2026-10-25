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
    error: window functions are not allowed in WHERE
      ╭▸ 
    3 │ select 1 from t where row_number() over () > 1;
      ╰╴                      ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in JOIN conditions
      ╭▸ 
    4 │ select 1 from t join t u on row_number() over () > 1;
      ╰╴                            ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in GROUP BY
      ╭▸ 
    5 │ select 1 from t group by row_number() over ();
      ╰╴                         ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in HAVING
      ╭▸ 
    6 │ select 1 from t having row_number() over () > 1;
      ╰╴                       ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in FILTER
      ╭▸ 
    7 │ select count(*) filter (where row_number() over () > 1) from t;
      ╰╴                              ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in window definitions
      ╭▸ 
    8 │ select row_number() over (order by row_number() over ()) from t;
      ╰╴                                   ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in LIMIT
      ╭▸ 
    9 │ select 1 from t limit row_number() over ();
      ╰╴                      ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in VALUES
       ╭▸ 
    10 │ values (row_number() over ());
       ╰╴        ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in UPDATE
       ╭▸ 
    11 │ update t set a = row_number() over ();
       ╰╴                 ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in RETURNING
       ╭▸ 
    12 │ update t set a = 1 returning row_number() over ();
       ╰╴                             ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in functions in FROM
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
    error: window functions are not allowed in INSERT
       ╭▸ 
    19 │ insert into t (a[row_number() over ()]) values (1);
       ╰╴                 ━━━━━━━━━━━━━━━━━━━━
    error: window functions are not allowed in EXECUTE parameters
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
    error: window function `rank` requires an OVER clause
      ╭▸ 
    3 │ select rank() from t;
      ╰╴       ━━━━━━
    error: window function `lag` requires an OVER clause
      ╭▸ 
    5 │ select lag(a) from t;
      ╰╴       ━━━━━━
    error: window function `w` requires an OVER clause
      ╭▸ 
    7 │ select w(a) from t;
      ╰╴       ━━━━
    ");
}
