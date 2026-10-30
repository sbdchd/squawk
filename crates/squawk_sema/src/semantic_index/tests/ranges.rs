use insta::assert_snapshot;

use super::check;

#[test]
fn multirange_type_name_string() {
    assert_snapshot!(check(r#"
create type floatrange as range (subtype = float8, multirange_type_name = 'MyMulti');
select '{}'::"MyMulti";
select '{}'::mymulti;
select '{}'::floatmultirange;
"#), @r#"
    error: type `mymulti` does not exist
      ╭▸ 
    4 │ select '{}'::mymulti;
      │              ━━━━━━━
      ╰╴
    help: a type with a similar name exists
      ╭╴
    4 - select '{}'::mymulti;
    4 + select '{}'::"MyMulti";
      ╰╴
    error: type `floatmultirange` does not exist
      ╭▸ 
    5 │ select '{}'::floatmultirange;
      │              ━━━━━━━━━━━━━━━
      ╰╴
    help: a type with a similar name exists
      ╭╴
    5 - select '{}'::floatmultirange;
    5 + select '{}'::datemultirange;
      ╰╴
    "#);
}

#[test]
fn create_range_type_functions() {
    assert_snapshot!(check("
create function diff(int, int) returns float8 language sql as 'select 1';
create function diff(text, text) returns float8 language sql as 'select 1';
create type r as range (subtype_diff = diff, subtype = int, subtype_opclass = int4_ops);
create type r2 as range (subtype = int, subtype_diff = nope);
create type r3 as range (subtype = numeric, subtype_diff = diff);
create type r4 as range (subtype = int, subtype_opclass = nope_ops);
create type r5;
create function canon(r5) returns r5 language sql as 'select null';
create type r5 as range (subtype = int, canonical = canon);
create type r6 as range (subtype = int, canonical = canon);
"), @"
    error: function `nope` does not exist
      ╭▸ 
    5 │ create type r2 as range (subtype = int, subtype_diff = nope);
      ╰╴                                                       ━━━━
    error: function `diff` does not exist
      ╭▸ 
    6 │ create type r3 as range (subtype = numeric, subtype_diff = diff);
      ╰╴                                                           ━━━━
    error: operator class `nope_ops` does not exist
      ╭▸ 
    7 │ create type r4 as range (subtype = int, subtype_opclass = nope_ops);
      ╰╴                                                          ━━━━━━━━
    error: cannot specify a canonical function without a pre-created shell type
       ╭▸ 
    11 │ create type r6 as range (subtype = int, canonical = canon);
       │                                                     ━━━━━
       │
       ╰ hint: create the type as a shell type, then create its canonicalization function, then do a full `create type`
    ");
}

#[test]
fn drop_renamed_range_type_drops_multirange() {
    assert_snapshot!(check("
create type r as range (subtype = int);
alter type r rename to s;
select '{}'::r_multirange;
drop type s;
select '{}'::r_multirange;
create type m as range (subtype = int);
drop type m_multirange;
select null::m;
"), @"
    error: type `r_multirange` does not exist
      ╭▸ 
    6 │ select '{}'::r_multirange;
      │              ━━━━━━━━━━━━
      ╰╴
    help: a type with a similar name exists
      ╭╴
    6 - select '{}'::r_multirange;
    6 + select '{}'::tsmultirange;
      ╰╴
    error: type `m` does not exist
      ╭▸ 
    9 │ select null::m;
      ╰╴             ━
    ");
}
