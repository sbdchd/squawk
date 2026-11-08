use insta::assert_snapshot;

use super::check;

#[test]
fn invalid_int_cast() {
    assert_snapshot!(check("
select 'abc'::int, '1.5'::int4, ''::bigint, '_1'::smallint;
"), @"
    error: invalid input value for type `int4`: `abc`
      ╭▸ 
    2 │ select 'abc'::int, '1.5'::int4, ''::bigint, '_1'::smallint;
      ╰╴       ━━━━━
    error: invalid input value for type `int4`: `1.5`
      ╭▸ 
    2 │ select 'abc'::int, '1.5'::int4, ''::bigint, '_1'::smallint;
      ╰╴                   ━━━━━
    error: invalid input value for type `int8`: ``
      ╭▸ 
    2 │ select 'abc'::int, '1.5'::int4, ''::bigint, '_1'::smallint;
      ╰╴                                ━━
    error: invalid input value for type `int2`: `_1`
      ╭▸ 
    2 │ select 'abc'::int, '1.5'::int4, ''::bigint, '_1'::smallint;
      ╰╴                                            ━━━━
    ");
}

#[test]
fn int_out_of_range() {
    assert_snapshot!(check("
select '32768'::int2, '-2147483649'::int4, '9223372036854775808'::int8;
"), @"
    error: value `32768` is out of range for type `int2`
      ╭▸ 
    2 │ select '32768'::int2, '-2147483649'::int4, '9223372036854775808'::int8;
      │        ━━━━━━━
      │
      ╰ hint: `int2` values range from `-32768` to `32767`
    error: value `-2147483649` is out of range for type `int4`
      ╭▸ 
    2 │ select '32768'::int2, '-2147483649'::int4, '9223372036854775808'::int8;
      │                       ━━━━━━━━━━━━━
      │
      ╰ hint: `int4` values range from `-2147483648` to `2147483647`
    error: value `9223372036854775808` is out of range for type `int8`
      ╭▸ 
    2 │ select '32768'::int2, '-2147483649'::int4, '9223372036854775808'::int8;
      │                                            ━━━━━━━━━━━━━━━━━━━━━
      │
      ╰ hint: `int8` values range from `-9223372036854775808` to `9223372036854775807`
    ");
}

#[test]
fn int_comparison() {
    assert_snapshot!(check("
create table t (a int2);
select * from t where a = 'x' or a in ('0x10', '40000');
"), @"
    error: invalid input value for type `int2`: `x`
      ╭▸ 
    3 │ select * from t where a = 'x' or a in ('0x10', '40000');
      ╰╴                          ━━━
    error: value `40000` is out of range for type `int2`
      ╭▸ 
    3 │ select * from t where a = 'x' or a in ('0x10', '40000');
      │                                                ━━━━━━━
      │
      ╰ hint: `int2` values range from `-32768` to `32767`
    ");
}
