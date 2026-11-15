use insta::assert_snapshot;

use super::check;

#[test]
fn invalid_float_cast() {
    assert_snapshot!(check("
select 'abc'::float8, '1_0'::real, ''::double precision, '0x1p'::float4, '1e'::float;
"), @"
    error: invalid input value for type `float8`: `abc`
      ╭▸ 
    2 │ select 'abc'::float8, '1_0'::real, ''::double precision, '0x1p'::float4, '1e'::float;
      ╰╴       ━━━━━
    error: invalid input value for type `float4`: `1_0`
      ╭▸ 
    2 │ select 'abc'::float8, '1_0'::real, ''::double precision, '0x1p'::float4, '1e'::float;
      ╰╴                      ━━━━━
    error: invalid input value for type `float8`: ``
      ╭▸ 
    2 │ select 'abc'::float8, '1_0'::real, ''::double precision, '0x1p'::float4, '1e'::float;
      ╰╴                                   ━━
    error: invalid input value for type `float4`: `0x1p`
      ╭▸ 
    2 │ select 'abc'::float8, '1_0'::real, ''::double precision, '0x1p'::float4, '1e'::float;
      ╰╴                                                         ━━━━━━
    error: invalid input value for type `float8`: `1e`
      ╭▸ 
    2 │ select 'abc'::float8, '1_0'::real, ''::double precision, '0x1p'::float4, '1e'::float;
      ╰╴                                                                         ━━━━
    ");
}

#[test]
fn float_out_of_range() {
    assert_snapshot!(check("
select '1e400'::float8, '1e39'::float4, '2e-324'::double precision, '0x1p-150'::real;
"), @"
    error: value `1e400` is out of range for type `float8`
      ╭▸ 
    2 │ select '1e400'::float8, '1e39'::float4, '2e-324'::double precision, '0x1p-150'::real;
      ╰╴       ━━━━━━━
    error: value `1e39` is out of range for type `float4`
      ╭▸ 
    2 │ select '1e400'::float8, '1e39'::float4, '2e-324'::double precision, '0x1p-150'::real;
      ╰╴                        ━━━━━━
    error: value `2e-324` is out of range for type `float8`
      ╭▸ 
    2 │ select '1e400'::float8, '1e39'::float4, '2e-324'::double precision, '0x1p-150'::real;
      ╰╴                                        ━━━━━━━━
    error: value `0x1p-150` is out of range for type `float4`
      ╭▸ 
    2 │ select '1e400'::float8, '1e39'::float4, '2e-324'::double precision, '0x1p-150'::real;
      ╰╴                                                                    ━━━━━━━━━━
    ");
}

#[test]
fn valid_floats() {
    assert_snapshot!(check("
select
  '0x1p3'::float8,
  '4e-320'::float8,
  '1e-45'::float4,
  'nan(abc)'::float8,
  '-Infinity'::float4,
  ' 1.5 '::float8;
"), @"no diagnostics");
}

#[test]
fn float_column_literals() {
    assert_snapshot!(check("
create domain d as float4;
create table t (a float8, b d);
insert into t values ('1e400', '1e39');
select * from t where a = 'x' or b in ('1', 'y');
"), @"
    error: value `1e400` is out of range for type `float8`
      ╭▸ 
    4 │ insert into t values ('1e400', '1e39');
      ╰╴                      ━━━━━━━
    error: value `1e39` is out of range for type `float4`
      ╭▸ 
    4 │ insert into t values ('1e400', '1e39');
      ╰╴                               ━━━━━━
    error: invalid input value for type `float8`: `x`
      ╭▸ 
    5 │ select * from t where a = 'x' or b in ('1', 'y');
      ╰╴                          ━━━
    error: invalid input value for type `float4`: `y`
      ╭▸ 
    5 │ select * from t where a = 'x' or b in ('1', 'y');
      ╰╴                                            ━━━
    ");
}
