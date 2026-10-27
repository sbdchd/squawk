use insta::assert_snapshot;

use super::check;

#[test]
fn operator_created_and_dropped() {
    assert_snapshot!(check("
create function f(int, int) returns bool as 'select true' language sql;
create function neg(int) returns int as 'select 1' language sql;
create operator === (leftarg = int, rightarg = int, function = f, commutator = ===, negator = !==, restrict = eqsel, join = eqjoinsel);
create operator - (rightarg = int, procedure = neg);
alter operator === (int, int) set (restrict = eqsel, join = eqjoinsel);
drop operator ===(int, int), -(none, int);
drop operator ===(int, int);
drop operator if exists ===(int, int);
"), @"
    error: operator does not exist: `int4 === int4`
      ╭▸ 
    8 │ drop operator ===(int, int);
      ╰╴              ━━━
    ");
}

#[test]
fn prefix_operator() {
    assert_snapshot!(check("
create function neg(int) returns int as 'select 1' language sql;
create function sub(int, int) returns int as 'select 1' language sql;
create operator ~~~ (rightarg = int, function = neg);
create operator ~~~ (rightarg = int, function = sub);
create operator ~~~ (leftarg = int, rightarg = int, function = neg);
drop operator ~~~ (int, int);
drop operator ~~~ (none, int);
drop operator ~~~ (none, int);
"), @"
    error: operator `~~~` already exists
      ╭▸ 
    5 │ create operator ~~~ (rightarg = int, function = sub);
      ╰╴                ━━━
    error: function `sub` does not exist
      ╭▸ 
    5 │ create operator ~~~ (rightarg = int, function = sub);
      ╰╴                                                ━━━
    error: function `neg` does not exist
      ╭▸ 
    6 │ create operator ~~~ (leftarg = int, rightarg = int, function = neg);
      ╰╴                                                               ━━━
    ");
}

#[test]
fn prefix_and_infix_operator() {
    assert_snapshot!(check("
create operator ~~~ (leftarg = int, rightarg = int, function = int4mi);
drop operator ~~~ (none, int);
create operator ~~~ (rightarg = int, function = int4um);
drop operator ~~~ (int, int);
drop operator ~~~ (int, int);
drop operator ~~~ (none, int);
"), @"
    error: operator does not exist: `~~~ int4`
      ╭▸ 
    3 │ drop operator ~~~ (none, int);
      ╰╴              ━━━
    error: operator does not exist: `int4 ~~~ int4`
      ╭▸ 
    6 │ drop operator ~~~ (int, int);
      ╰╴              ━━━
    ");
}

#[test]
fn operator_missing() {
    assert_snapshot!(check("
create operator === (leftarg = nope, rightarg = int, function = f, restrict = nope_sel);
create operator s.=== (leftarg = int, rightarg = int, function = int4eq);
alter operator !== (int, int) set (restrict = eqsel);
drop operator === (int, text);
"), @"
    error: type `nope` does not exist
      ╭▸ 
    2 │ create operator === (leftarg = nope, rightarg = int, function = f, restrict = nope_sel);
      ╰╴                               ━━━━
    error: function `f` does not exist
      ╭▸ 
    2 │ create operator === (leftarg = nope, rightarg = int, function = f, restrict = nope_sel);
      ╰╴                                                                ━
    error: function `nope_sel` does not exist
      ╭▸ 
    2 │ create operator === (leftarg = nope, rightarg = int, function = f, restrict = nope_sel);
      ╰╴                                                                              ━━━━━━━━
    error: schema `s` does not exist
      ╭▸ 
    3 │ create operator s.=== (leftarg = int, rightarg = int, function = int4eq);
      ╰╴                ━
    error: operator does not exist: `int4 !== int4`
      ╭▸ 
    4 │ alter operator !== (int, int) set (restrict = eqsel);
      ╰╴               ━━━
    error: operator does not exist: `int4 === text`
      ╭▸ 
    5 │ drop operator === (int, text);
      ╰╴              ━━━
    ");
}

#[test]
fn operator_schema() {
    assert_snapshot!(check("
create schema s;
create function s.f(int, int) returns bool as 'select true' language sql;
create operator s.=== (leftarg = int, rightarg = int, function = s.f, negator = operator(s.!==));
drop operator === (int, int);
drop operator s.=== (int, int);
"), @"
    error: operator does not exist: `int4 === int4`
      ╭▸ 
    5 │ drop operator === (int, int);
      ╰╴              ━━━
    ");
}

#[test]
fn operator_rollback() {
    assert_snapshot!(check("
create operator === (leftarg = int, rightarg = int, function = int4eq);
begin;
create operator !== (leftarg = int, rightarg = int, function = int4ne);
drop operator === (int, int);
rollback;
drop operator === (int, int);
drop operator !== (int, int);
"), @"
    error: operator does not exist: `int4 !== int4`
      ╭▸ 
    8 │ drop operator !== (int, int);
      ╰╴              ━━━
    ");
}

#[test]
fn duplicate_operator() {
    assert_snapshot!(check("
create operator === (leftarg = int, rightarg = int, function = int4eq);
create operator === (leftarg = int, rightarg = text, function = int4eq);
create operator === (leftarg = int, rightarg = int, function = int4eq);
"), @"
    error: function `int4eq` does not exist
      ╭▸ 
    3 │ create operator === (leftarg = int, rightarg = text, function = int4eq);
      ╰╴                                                                ━━━━━━
    error: operator `===` already exists
      ╭▸ 
    4 │ create operator === (leftarg = int, rightarg = int, function = int4eq);
      ╰╴                ━━━
    ");
}

#[test]
fn builtin_operator() {
    assert_snapshot!(check("
alter operator = (int, int) set (restrict = eqsel);
drop operator if exists pg_catalog.!= (int, int);
"), @"no diagnostics");
}

#[test]
fn alter_operator_set_schema() {
    assert_snapshot!(check("
create schema s;
create schema s2;
create operator === (leftarg = int, rightarg = int, function = int4eq);
create operator s2.=== (leftarg = int, rightarg = int, function = int4eq);
alter operator === (int, int) set schema s;
alter operator public.=== (int, int) owner to current_user;
alter operator s.=== (int, int) owner to current_user;
alter operator s.=== (int, int) set schema s2;
alter operator s.=== (int, int) set schema nope;
begin;
alter operator s.=== (int, int) set schema public;
rollback;
drop operator public.=== (int, int);
drop operator s.=== (int, int);
"), @"
    error: operator does not exist: `int4 public.=== int4`
      ╭▸ 
    7 │ alter operator public.=== (int, int) owner to current_user;
      ╰╴                      ━━━
    error: operator `===` already exists in schema `s2`
      ╭▸ 
    9 │ alter operator s.=== (int, int) set schema s2;
      ╰╴                                           ━━
    error: schema `nope` does not exist
       ╭▸ 
    10 │ alter operator s.=== (int, int) set schema nope;
       ╰╴                                           ━━━━
    error: operator does not exist: `int4 public.=== int4`
       ╭▸ 
    14 │ drop operator public.=== (int, int);
       ╰╴                     ━━━
    ");
}

#[test]
fn operator_bin_expr_unresolved() {
    assert_snapshot!(check("
create operator === (leftarg = int, rightarg = int, function = int4eq);
select 1 === 2, 'a'::text === 'b'::text, 1 operator(===) 2;
"), @"
    error: operator does not exist: `text === text`
      ╭▸ 
    3 │ select 1 === 2, 'a'::text === 'b'::text, 1 operator(===) 2;
      │                           ━━━
      │
      ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn operator_prefix_expr_unknown_literal() {
    assert_snapshot!(check("
select @ '-4.5', -'1';
"), @"
    error: operator `-` is not unique
      ╭▸ 
    2 │ select @ '-4.5', -'1';
      ╰╴                 ━
    ");
}

#[test]
fn operator_call_unknown_schema() {
    assert_snapshot!(check("
select 1 operator(nope.+) 2, nope.f(1);
"), @"
    error: schema `nope` does not exist
      ╭▸ 
    2 │ select 1 operator(nope.+) 2, nope.f(1);
      ╰╴                  ━━━━
    error: operator does not exist: `int4 nope.+ int4`
      ╭▸ 
    2 │ select 1 operator(nope.+) 2, nope.f(1);
      │                        ━
      │
      ╰ hint: consider adding explicit type casts
    error: schema `nope` does not exist
      ╭▸ 
    2 │ select 1 operator(nope.+) 2, nope.f(1);
      ╰╴                             ━━━━
    error: function `nope.f(int4)` does not exist
      ╭▸ 
    2 │ select 1 operator(nope.+) 2, nope.f(1);
      │                                   ━
      │
      ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn custom_operator_chain_is_left_associative() {
    assert_eq!(
        check(
            "
create table t (j jsonb);
select j -> 'a' ->> 'b' from t;
"
        ),
        "no diagnostics"
    );
}

#[test]
fn keyword_operator_unresolved() {
    assert_snapshot!(check("
create type mood as enum ('happy', 'sad');
create table t (i int, s text, m mood);
select i like 'a%', i similar to 'a', i not ilike 'a', m is distinct from 1, i in (s), s not in (select i from t) from t;
"), @"
    error: operator does not exist: `int4 ~~ unknown`
      ╭▸ 
    4 │ select i like 'a%', i similar to 'a', i not ilike 'a', m is distinct from 1, i in (s), s not in (select i from t) from t;
      │          ━━━━
      │
      ╰ hint: consider adding explicit type casts
    error: operator does not exist: `int4 ~ text`
      ╭▸ 
    4 │ select i like 'a%', i similar to 'a', i not ilike 'a', m is distinct from 1, i in (s), s not in (select i from t) from t;
      │                       ━━━━━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    error: operator does not exist: `int4 !~~* unknown`
      ╭▸ 
    4 │ select i like 'a%', i similar to 'a', i not ilike 'a', m is distinct from 1, i in (s), s not in (select i from t) from t;
      │                                         ━━━━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    error: operator does not exist: `mood = int4`
      ╭▸ 
    4 │ select i like 'a%', i similar to 'a', i not ilike 'a', m is distinct from 1, i in (s), s not in (select i from t) from t;
      │                                                          ━━━━━━━━━━━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    error: operator does not exist: `int4 = text`
      ╭▸ 
    4 │ select i like 'a%', i similar to 'a', i not ilike 'a', m is distinct from 1, i in (s), s not in (select i from t) from t;
      │                                                                                ━━
      │
      ╰ hint: consider adding explicit type casts
    error: operator does not exist: `text <> int4`
      ╭▸ 
    4 │ select i like 'a%', i similar to 'a', i not ilike 'a', m is distinct from 1, i in (s), s not in (select i from t) from t;
      │                                                                                          ━━━━━━
      │
      ╰ hint: consider adding explicit type casts
    ");
}

#[test]
fn row_length_mismatch() {
    assert_snapshot!(check("
select row(1, 2) = row(1);
select (1, 2) < (1, 2, 3);
select (1, 2) is distinct from (1, 2, 3);
select (1, 2) in ((1, 2), (1, 2, 3));
select (1, 2) between (1, 2) and (1, 2, 3);
select row(1) = row(1), (1, 2) = (1, 2), (1, 2) in ((1, 2), (3, 4));
"), @"
    error: unequal number of entries in row expressions
      ╭▸ 
    2 │ select row(1, 2) = row(1);
      ╰╴                   ━━━━━━
    error: unequal number of entries in row expressions
      ╭▸ 
    3 │ select (1, 2) < (1, 2, 3);
      ╰╴                ━━━━━━━━━
    error: unequal number of entries in row expressions
      ╭▸ 
    4 │ select (1, 2) is distinct from (1, 2, 3);
      ╰╴                               ━━━━━━━━━
    error: unequal number of entries in row expressions
      ╭▸ 
    5 │ select (1, 2) in ((1, 2), (1, 2, 3));
      ╰╴                          ━━━━━━━━━
    error: unequal number of entries in row expressions
      ╭▸ 
    6 │ select (1, 2) between (1, 2) and (1, 2, 3);
      ╰╴                                 ━━━━━━━━━
    ");
}

#[test]
fn unresolved_operator_types() {
    assert_snapshot!(check("
create table t (a int, b text[]);
select 1 = true;
select a + b from t;
select - true;
select a === 1 from t;
"), @"
    error: operator does not exist: `int4 = bool`
      ╭▸ 
    3 │ select 1 = true;
      │          ━
      │
      ╰ hint: consider adding explicit type casts
    error: operator does not exist: `int4 + text[]`
      ╭▸ 
    4 │ select a + b from t;
      │          ━
      │
      ╰ hint: consider adding explicit type casts
    error: operator does not exist: `- bool`
      ╭▸ 
    5 │ select - true;
      │        ━
      │
      ╰ hint: consider adding explicit type casts
    error: operator does not exist: `int4 === int4`
      ╭▸ 
    6 │ select a === 1 from t;
      │          ━━━
      │
      ╰ hint: consider adding explicit type casts
    ");
}
