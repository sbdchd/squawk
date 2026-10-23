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
    error: type `nope_type` does not exist
      ╭▸ 
    5 │ create aggregate bad(int) (sfunc = nope, stype = nope_type, finalfunc = add, sortop = <);
      ╰╴                                                 ━━━━━━━━━
    error: function `nope` does not exist
      ╭▸ 
    5 │ create aggregate bad(int) (sfunc = nope, stype = nope_type, finalfunc = add, sortop = <);
      ╰╴                                   ━━━━
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
    error: aggregate `pct` does not exist
      ╭▸ 
    5 │ drop aggregate pct(float8);
      ╰╴               ━━━
    error: aggregate `pct` does not exist
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
    error: function `old` does not exist
      ╭▸ 
    6 │ select old(1, 2);
      ╰╴       ━━━
    error: function `old` does not exist
      ╭▸ 
    9 │ select old(1), cnt(*);
      ╰╴       ━━━
    error: function `cnt` does not exist
      ╭▸ 
    9 │ select old(1), cnt(*);
      ╰╴               ━━━
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
    error: function `s` does not exist
      ╭▸ 
    5 │ drop function s(int);
      ╰╴              ━
    error: function `s` does not exist
      ╭▸ 
    7 │ select s(1), c();
      ╰╴       ━
    error: function `c` does not exist
      ╭▸ 
    7 │ select s(1), c();
      ╰╴             ━
    error: aggregate `s` does not exist
      ╭▸ 
    8 │ drop aggregate s(int);
      ╰╴               ━
    error: aggregate `add` does not exist
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
    error: function `s` does not exist
      ╭▸ 
    6 │ select s(1);
      ╰╴       ━
    error: role `nope` does not exist
       ╭▸ 
    10 │ alter aggregate x.t(int) owner to nope;
       ╰╴                                  ━━━━
    error: aggregate `add` does not exist
       ╭▸ 
    11 │ alter aggregate add(int, int) rename to y;
       ╰╴                ━━━
    ");
}
