use insta::assert_snapshot;

use super::check;

#[test]
fn valid_bool_literals() {
    assert_snapshot!(check("
select
  't'::bool,
  'TRUE'::bool,
  ' yes '::bool,
  'of'::bool,
  'ON'::bool,
  '1'::bool,
  '0'::boolean,
  'n'::bool,
  bool 'f',
  e'tr'::bool;
"), @"no diagnostics");
}

#[test]
fn invalid_bool_cast() {
    assert_snapshot!(check("
select 'maybe'::bool, 'o'::bool, '01'::bool, ''::boolean, bool 'nope';
"), @"
    error: invalid input value for type `bool`: `maybe`
      ╭▸ 
    2 │ select 'maybe'::bool, 'o'::bool, '01'::bool, ''::boolean, bool 'nope';
      │        ━━━━━━━
      │
      ╰ hint: expected one of `true`, `false`, `yes`, `no`, `on`, `off`, `1`, or `0`
    error: invalid input value for type `bool`: `o`
      ╭▸ 
    2 │ select 'maybe'::bool, 'o'::bool, '01'::bool, ''::boolean, bool 'nope';
      │                       ━━━
      │
      ╰ hint: expected one of `true`, `false`, `yes`, `no`, `on`, `off`, `1`, or `0`
    error: invalid input value for type `bool`: `01`
      ╭▸ 
    2 │ select 'maybe'::bool, 'o'::bool, '01'::bool, ''::boolean, bool 'nope';
      │                                  ━━━━
      │
      ╰ hint: expected one of `true`, `false`, `yes`, `no`, `on`, `off`, `1`, or `0`
    error: invalid input value for type `bool`: ``
      ╭▸ 
    2 │ select 'maybe'::bool, 'o'::bool, '01'::bool, ''::boolean, bool 'nope';
      │                                              ━━
      │
      ╰ hint: expected one of `true`, `false`, `yes`, `no`, `on`, `off`, `1`, or `0`
    error: invalid input value for type `bool`: `nope`
      ╭▸ 
    2 │ select 'maybe'::bool, 'o'::bool, '01'::bool, ''::boolean, bool 'nope';
      │                                                                ━━━━━━
      │
      ╰ hint: expected one of `true`, `false`, `yes`, `no`, `on`, `off`, `1`, or `0`
    ");
}

#[test]
fn similar_bool_value() {
    assert_snapshot!(check("
select 'ture'::bool, 'FLASE'::bool;
"), @"
    error: invalid input value for type `bool`: `ture`
      ╭▸ 
    2 │ select 'ture'::bool, 'FLASE'::bool;
      │        ━━━━━━
      ╰╴
    help: a similar boolean value exists
      ╭╴
    2 - select 'ture'::bool, 'FLASE'::bool;
    2 + select 'true'::bool, 'FLASE'::bool;
      ╰╴
    error: invalid input value for type `bool`: `FLASE`
      ╭▸ 
    2 │ select 'ture'::bool, 'FLASE'::bool;
      │                      ━━━━━━━
      ╰╴
    help: a similar boolean value exists
      ╭╴
    2 - select 'ture'::bool, 'FLASE'::bool;
    2 + select 'ture'::bool, 'false'::bool;
      ╰╴
    ");
}

#[test]
fn bool_comparison() {
    assert_snapshot!(check("
create table t (a bool);
select * from t where a = 'maybe';
select * from t where a in ('yes', 'nah');
select * from t where 'flase' <> a;
"), @"
    error: invalid input value for type `bool`: `maybe`
      ╭▸ 
    3 │ select * from t where a = 'maybe';
      │                           ━━━━━━━
      │
      ╰ hint: expected one of `true`, `false`, `yes`, `no`, `on`, `off`, `1`, or `0`
    error: invalid input value for type `bool`: `nah`
      ╭▸ 
    4 │ select * from t where a in ('yes', 'nah');
      │                                    ━━━━━
      │
      ╰ hint: expected one of `true`, `false`, `yes`, `no`, `on`, `off`, `1`, or `0`
    error: invalid input value for type `bool`: `flase`
      ╭▸ 
    5 │ select * from t where 'flase' <> a;
      │                       ━━━━━━━
      ╰╴
    help: a similar boolean value exists
      ╭╴
    5 - select * from t where 'flase' <> a;
    5 + select * from t where 'false' <> a;
      ╰╴
    ");
}

#[test]
fn bool_domain() {
    assert_snapshot!(check("
create domain d as bool;
select 'x'::d;
"), @"
    error: invalid input value for type `bool`: `x`
      ╭▸ 
    3 │ select 'x'::d;
      │        ━━━
      │
      ╰ hint: expected one of `true`, `false`, `yes`, `no`, `on`, `off`, `1`, or `0`
    ");
}
