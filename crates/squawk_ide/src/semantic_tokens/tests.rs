use std::fmt::Write;

use insta::assert_snapshot;
use rowan::{TextRange, TextSize};
use squawk_syntax::SourceFile;

use crate::{
    file::InFile, semantic_index_builder::SemanticIndexBuilder,
    semantic_tokens::semantic_tokens_with_index as semantic_tokens,
};

#[must_use]
#[track_caller]
fn check_semantic_tokens(sql: &str) -> String {
    check_semantic_tokens_in_range(sql, None)
}

#[must_use]
#[track_caller]
fn check_semantic_tokens_in_range(sql: &str, range: Option<TextRange>) -> String {
    let parse = SourceFile::parse(sql);
    assert_eq!(parse.errors(), vec![]);
    let tree = parse.tree();
    let db = crate::db::Database::default();
    let file = InFile::new(crate::db::File::new(&db, sql.into()), &tree);
    let mut builder = SemanticIndexBuilder::new();
    builder.add_file(file);
    let index = builder.build();
    let mut result = String::new();
    for token in semantic_tokens(&index, file, range) {
        let start = usize::from(token.range.start());
        let end = usize::from(token.range.end());
        write!(
            result,
            "{:?} @ {start}..{end}: {:?}",
            &sql[start..end],
            token.token_type
        )
        .unwrap();
        if let Some(modifiers) = token.modifiers {
            write!(result, " ({modifiers:?})").unwrap();
        }
        result.push('\n');
    }
    result
}

#[test]
fn create_function_misc_params() {
    assert_snapshot!(check_semantic_tokens("
create function add(
  in a int = 1,
  inout b text default 'x',
  in out c varchar(10)[],
  variadic d int[]
) returns int
  as 'select $1 + $2'
  language sql;
"), @r#"
    "create" @ 1..7: Keyword
    "function" @ 8..16: Keyword
    "add" @ 17..20: Function (Definition)
    "(" @ 20..21: Punctuation
    "in" @ 24..26: Keyword
    "a" @ 27..28: Parameter (Definition)
    "int" @ 29..32: Type
    "=" @ 33..34: Operator
    "1" @ 35..36: Number
    "," @ 36..37: Punctuation
    "inout" @ 40..45: Keyword
    "b" @ 46..47: Parameter (Definition)
    "text" @ 48..52: Type
    "default" @ 53..60: Keyword
    "'x'" @ 61..64: String
    "," @ 64..65: Punctuation
    "in" @ 68..70: Keyword
    "out" @ 71..74: Keyword
    "c" @ 75..76: Parameter (Definition)
    "varchar" @ 77..84: Type
    "(" @ 84..85: Punctuation
    "10" @ 85..87: Number
    ")" @ 87..88: Punctuation
    "[" @ 88..89: Punctuation
    "]" @ 89..90: Punctuation
    "," @ 90..91: Punctuation
    "variadic" @ 94..102: Keyword
    "d" @ 103..104: Parameter (Definition)
    "int" @ 105..108: Type
    "[" @ 108..109: Punctuation
    "]" @ 109..110: Punctuation
    ")" @ 111..112: Punctuation
    "returns" @ 113..120: Keyword
    "int" @ 121..124: Type
    "as" @ 127..129: Keyword
    "'" @ 130..131: String
    "select" @ 131..137: Keyword
    "$1" @ 138..140: Parameter
    "+" @ 141..142: Operator
    "$2" @ 143..145: Parameter
    "'" @ 145..146: String
    "language" @ 149..157: Keyword
    "sql" @ 158..161: Language
    ";" @ 161..162: Punctuation
    "#);
}

#[test]
fn create_function_param_mode_type() {
    assert_snapshot!(check_semantic_tokens("
create function f(int8 in int8) returns void
  as ''
  language sql;
"), @r#"
    "create" @ 1..7: Keyword
    "function" @ 8..16: Keyword
    "f" @ 17..18: Function (Definition)
    "(" @ 18..19: Punctuation
    "int8" @ 19..23: Parameter (Definition)
    "in" @ 24..26: Keyword
    "int8" @ 27..31: Type
    ")" @ 31..32: Punctuation
    "returns" @ 33..40: Keyword
    "void" @ 41..45: Type
    "as" @ 48..50: Keyword
    "'" @ 51..52: String
    "'" @ 52..53: String
    "language" @ 56..64: Keyword
    "sql" @ 65..68: Language
    ";" @ 68..69: Punctuation
    "#);
}

#[test]
fn select_keywords() {
    assert_snapshot!(check_semantic_tokens("
select 1 and, 2 select;
"), @r#"
    "select" @ 1..7: Keyword
    "1" @ 8..9: Number
    "and" @ 10..13: Column (Definition)
    "," @ 13..14: Punctuation
    "2" @ 15..16: Number
    "select" @ 17..23: Column (Definition)
    ";" @ 23..24: Punctuation
    "#);
}

#[test]
fn positional_param() {
    assert_snapshot!(check_semantic_tokens("
select $1, $2;
"), @r#"
    "select" @ 1..7: Keyword
    "$1" @ 8..10: PositionalParam
    "," @ 10..11: Punctuation
    "$2" @ 12..14: PositionalParam
    ";" @ 14..15: Punctuation
    "#);
}

#[test]
fn insert_column_list() {
    assert_snapshot!(check_semantic_tokens("
create table products (
  product_no bigint,
  name text,
  price text
);
insert into products (product_no, name, price) values
  (1, 'Cheese', 9.99),
  (2, 'Bread', 1.99),
  (3, 'Milk', 2.99);
"), @r#"
    "create" @ 1..7: Keyword
    "table" @ 8..13: Keyword
    "products" @ 14..22: Table (Definition)
    "(" @ 23..24: Punctuation
    "product_no" @ 27..37: Column (Definition)
    "bigint" @ 38..44: Type
    "," @ 44..45: Punctuation
    "name" @ 48..52: Column (Definition)
    "text" @ 53..57: Type
    "," @ 57..58: Punctuation
    "price" @ 61..66: Column (Definition)
    "text" @ 67..71: Type
    ")" @ 72..73: Punctuation
    ";" @ 73..74: Punctuation
    "insert" @ 75..81: Keyword
    "into" @ 82..86: Keyword
    "products" @ 87..95: Table
    "(" @ 96..97: Punctuation
    "product_no" @ 97..107: Column
    "," @ 107..108: Punctuation
    "name" @ 109..113: Column
    "," @ 113..114: Punctuation
    "price" @ 115..120: Column
    ")" @ 120..121: Punctuation
    "values" @ 122..128: Keyword
    "(" @ 131..132: Punctuation
    "1" @ 132..133: Number
    "," @ 133..134: Punctuation
    "'Cheese'" @ 135..143: String
    "," @ 143..144: Punctuation
    "9.99" @ 145..149: Number
    ")" @ 149..150: Punctuation
    "," @ 150..151: Punctuation
    "(" @ 154..155: Punctuation
    "2" @ 155..156: Number
    "," @ 156..157: Punctuation
    "'Bread'" @ 158..165: String
    "," @ 165..166: Punctuation
    "1.99" @ 167..171: Number
    ")" @ 171..172: Punctuation
    "," @ 172..173: Punctuation
    "(" @ 176..177: Punctuation
    "3" @ 177..178: Number
    "," @ 178..179: Punctuation
    "'Milk'" @ 180..186: String
    "," @ 186..187: Punctuation
    "2.99" @ 188..192: Number
    ")" @ 192..193: Punctuation
    ";" @ 193..194: Punctuation
    "#);
}

#[test]
fn from_alias_column_types() {
    assert_snapshot!(check_semantic_tokens("
create function f() returns setof record
  as 'select 1'
  language sql;
select * from f() as t (a int, b jsonb, ca char(5)[], ia int[][]);
"), @r#"
    "create" @ 1..7: Keyword
    "function" @ 8..16: Keyword
    "f" @ 17..18: Function (Definition)
    "(" @ 18..19: Punctuation
    ")" @ 19..20: Punctuation
    "returns" @ 21..28: Keyword
    "setof" @ 29..34: Type
    "record" @ 35..41: Type
    "as" @ 44..46: Keyword
    "'" @ 47..48: String
    "select" @ 48..54: Keyword
    "1" @ 55..56: Number
    "'" @ 56..57: String
    "language" @ 60..68: Keyword
    "sql" @ 69..72: Language
    ";" @ 72..73: Punctuation
    "select" @ 74..80: Keyword
    "*" @ 81..82: Operator
    "from" @ 83..87: Keyword
    "f" @ 88..89: Function
    "(" @ 89..90: Punctuation
    ")" @ 90..91: Punctuation
    "as" @ 92..94: Keyword
    "t" @ 95..96: Table (Definition)
    "(" @ 97..98: Punctuation
    "a" @ 98..99: Column (Definition)
    "int" @ 100..103: Type
    "," @ 103..104: Punctuation
    "b" @ 105..106: Column (Definition)
    "jsonb" @ 107..112: Type
    "," @ 112..113: Punctuation
    "ca" @ 114..116: Column (Definition)
    "char" @ 117..121: Type
    "(" @ 121..122: Punctuation
    "5" @ 122..123: Number
    ")" @ 123..124: Punctuation
    "[" @ 124..125: Punctuation
    "]" @ 125..126: Punctuation
    "," @ 126..127: Punctuation
    "ia" @ 128..130: Column (Definition)
    "int" @ 131..134: Type
    "[" @ 134..135: Punctuation
    "]" @ 135..136: Punctuation
    "[" @ 136..137: Punctuation
    "]" @ 137..138: Punctuation
    ")" @ 138..139: Punctuation
    ";" @ 139..140: Punctuation
    "#);
}

#[test]
fn json_table_columns() {
    assert_snapshot!(check_semantic_tokens("
create table my_films (js jsonb);
select *
from my_films,
  json_table(
    js,
    '$.favorites[*]'
    columns (id for ordinality, kind text path '$.kind')
  ) as jt;
"), @r#"
    "create" @ 1..7: Keyword
    "table" @ 8..13: Keyword
    "my_films" @ 14..22: Table (Definition)
    "(" @ 23..24: Punctuation
    "js" @ 24..26: Column (Definition)
    "jsonb" @ 27..32: Type
    ")" @ 32..33: Punctuation
    ";" @ 33..34: Punctuation
    "select" @ 35..41: Keyword
    "*" @ 42..43: Operator
    "from" @ 44..48: Keyword
    "my_films" @ 49..57: Table
    "," @ 57..58: Punctuation
    "json_table" @ 61..71: Keyword
    "(" @ 71..72: Punctuation
    "js" @ 77..79: Column
    "," @ 79..80: Punctuation
    "'$.favorites[*]'" @ 85..101: String
    "columns" @ 106..113: Keyword
    "(" @ 114..115: Punctuation
    "id" @ 115..117: Column (Definition)
    "for" @ 118..121: Keyword
    "ordinality" @ 122..132: Keyword
    "," @ 132..133: Punctuation
    "kind" @ 134..138: Column (Definition)
    "text" @ 139..143: Type
    "path" @ 144..148: Keyword
    "'$.kind'" @ 149..157: String
    ")" @ 157..158: Punctuation
    ")" @ 161..162: Punctuation
    "as" @ 163..165: Keyword
    "jt" @ 166..168: Table (Definition)
    ";" @ 168..169: Punctuation
    "#);
}

#[test]
fn cast_types() {
    assert_snapshot!(check_semantic_tokens("
select
  '1'::jsonb,
  '2'::json,
  cast(
    1 as integer
  ),
  cast(
    1 as int4[][]
  ),
  cast(
    1 as varchar(10)
  );
"), @r#"
    "select" @ 1..7: Keyword
    "'1'" @ 10..13: String
    ":" @ 13..14: Operator
    ":" @ 14..15: Operator
    "jsonb" @ 15..20: Type
    "," @ 20..21: Punctuation
    "'2'" @ 24..27: String
    ":" @ 27..28: Operator
    ":" @ 28..29: Operator
    "json" @ 29..33: Type
    "," @ 33..34: Punctuation
    "cast" @ 37..41: Keyword
    "(" @ 41..42: Punctuation
    "1" @ 47..48: Number
    "as" @ 49..51: Keyword
    "integer" @ 52..59: Type
    ")" @ 62..63: Punctuation
    "," @ 63..64: Punctuation
    "cast" @ 67..71: Keyword
    "(" @ 71..72: Punctuation
    "1" @ 77..78: Number
    "as" @ 79..81: Keyword
    "int4" @ 82..86: Type
    "[" @ 86..87: Punctuation
    "]" @ 87..88: Punctuation
    "[" @ 88..89: Punctuation
    "]" @ 89..90: Punctuation
    ")" @ 93..94: Punctuation
    "," @ 94..95: Punctuation
    "cast" @ 98..102: Keyword
    "(" @ 102..103: Punctuation
    "1" @ 108..109: Number
    "as" @ 110..112: Keyword
    "varchar" @ 113..120: Type
    "(" @ 120..121: Punctuation
    "10" @ 121..123: Number
    ")" @ 123..124: Punctuation
    ")" @ 127..128: Punctuation
    ";" @ 128..129: Punctuation
    "#);
}

#[test]
fn cast_double() {
    assert_snapshot!(check_semantic_tokens("
select '1'::double precision;
"), @r#"
    "select" @ 1..7: Keyword
    "'1'" @ 8..11: String
    ":" @ 11..12: Operator
    ":" @ 12..13: Operator
    "double" @ 13..19: Type
    "precision" @ 20..29: Type
    ";" @ 29..30: Punctuation
    "#);
}

#[test]
fn cast_time_and_timestamp_time_zone() {
    assert_snapshot!(check_semantic_tokens("
select
  cast(
    1 as timestamp with time zone
  ),
  cast(
    1 as timestamp without time zone
  ),
  cast(
    1 as time with time zone
  ),
  cast(
    1 as time without time zone
  );
"), @r#"
    "select" @ 1..7: Keyword
    "cast" @ 10..14: Keyword
    "(" @ 14..15: Punctuation
    "1" @ 20..21: Number
    "as" @ 22..24: Keyword
    "timestamp" @ 25..34: Type
    "with" @ 35..39: Type
    "time" @ 40..44: Type
    "zone" @ 45..49: Type
    ")" @ 52..53: Punctuation
    "," @ 53..54: Punctuation
    "cast" @ 57..61: Keyword
    "(" @ 61..62: Punctuation
    "1" @ 67..68: Number
    "as" @ 69..71: Keyword
    "timestamp" @ 72..81: Type
    "without" @ 82..89: Type
    "time" @ 90..94: Type
    "zone" @ 95..99: Type
    ")" @ 102..103: Punctuation
    "," @ 103..104: Punctuation
    "cast" @ 107..111: Keyword
    "(" @ 111..112: Punctuation
    "1" @ 117..118: Number
    "as" @ 119..121: Keyword
    "time" @ 122..126: Type
    "with" @ 127..131: Type
    "time" @ 132..136: Type
    "zone" @ 137..141: Type
    ")" @ 144..145: Punctuation
    "," @ 145..146: Punctuation
    "cast" @ 149..153: Keyword
    "(" @ 153..154: Punctuation
    "1" @ 159..160: Number
    "as" @ 161..163: Keyword
    "time" @ 164..168: Type
    "without" @ 169..176: Type
    "time" @ 177..181: Type
    "zone" @ 182..186: Type
    ")" @ 189..190: Punctuation
    ";" @ 190..191: Punctuation
    "#);
}

#[test]
fn cast_national_character_varying_type() {
    assert_snapshot!(check_semantic_tokens("
select 'foo'::national character varying;
"), @r#"
    "select" @ 1..7: Keyword
    "'foo'" @ 8..13: String
    ":" @ 13..14: Operator
    ":" @ 14..15: Operator
    "national" @ 15..23: Type
    "character" @ 24..33: Type
    "varying" @ 34..41: Type
    ";" @ 41..42: Punctuation
    "#);
}

#[test]
fn create_function_returns_setof_type() {
    assert_snapshot!(check_semantic_tokens("
create function f() returns setof int
  as 'select 1'
  language sql;
"), @r#"
    "create" @ 1..7: Keyword
    "function" @ 8..16: Keyword
    "f" @ 17..18: Function (Definition)
    "(" @ 18..19: Punctuation
    ")" @ 19..20: Punctuation
    "returns" @ 21..28: Keyword
    "setof" @ 29..34: Type
    "int" @ 35..38: Type
    "as" @ 41..43: Keyword
    "'" @ 44..45: String
    "select" @ 45..51: Keyword
    "1" @ 52..53: Number
    "'" @ 53..54: String
    "language" @ 57..65: Keyword
    "sql" @ 66..69: Language
    ";" @ 69..70: Punctuation
    "#);
}

#[test]
fn create_table_column_constraints() {
    assert_snapshot!(check_semantic_tokens("
create table addresses (
  id int8 generated by default as identity,
  valid_range tstzrange not null default tstzrange(now(), 'infinity', '[)'),
  recipient text not null,
  primary key (id, valid_range without overlaps)
);
"), @r#"
    "create" @ 1..7: Keyword
    "table" @ 8..13: Keyword
    "addresses" @ 14..23: Table (Definition)
    "(" @ 24..25: Punctuation
    "id" @ 28..30: Column (Definition)
    "int8" @ 31..35: Type
    "generated" @ 36..45: Keyword
    "by" @ 46..48: Keyword
    "default" @ 49..56: Keyword
    "as" @ 57..59: Keyword
    "identity" @ 60..68: Keyword
    "," @ 68..69: Punctuation
    "valid_range" @ 72..83: Column (Definition)
    "tstzrange" @ 84..93: Type
    "not" @ 94..97: Keyword
    "null" @ 98..102: Keyword
    "default" @ 103..110: Keyword
    "tstzrange" @ 111..120: Function
    "(" @ 120..121: Punctuation
    "now" @ 121..124: Function
    "(" @ 124..125: Punctuation
    ")" @ 125..126: Punctuation
    "," @ 126..127: Punctuation
    "'infinity'" @ 128..138: String
    "," @ 138..139: Punctuation
    "'[)'" @ 140..144: String
    ")" @ 144..145: Punctuation
    "," @ 145..146: Punctuation
    "recipient" @ 149..158: Column (Definition)
    "text" @ 159..163: Type
    "not" @ 164..167: Keyword
    "null" @ 168..172: Keyword
    "," @ 172..173: Punctuation
    "primary" @ 176..183: Keyword
    "key" @ 184..187: Keyword
    "(" @ 188..189: Punctuation
    "id" @ 189..191: Column
    "," @ 191..192: Punctuation
    "valid_range" @ 193..204: Column
    "without" @ 205..212: Keyword
    "overlaps" @ 213..221: Keyword
    ")" @ 221..222: Punctuation
    ")" @ 223..224: Punctuation
    ";" @ 224..225: Punctuation
    "#);
}

#[test]
fn like_clause_keyword() {
    assert_snapshot!(check_semantic_tokens("
create table products (a text);
create table test (like products);
"), @r#"
    "create" @ 1..7: Keyword
    "table" @ 8..13: Keyword
    "products" @ 14..22: Table (Definition)
    "(" @ 23..24: Punctuation
    "a" @ 24..25: Column (Definition)
    "text" @ 26..30: Type
    ")" @ 30..31: Punctuation
    ";" @ 31..32: Punctuation
    "create" @ 33..39: Keyword
    "table" @ 40..45: Keyword
    "test" @ 46..50: Table (Definition)
    "(" @ 51..52: Punctuation
    "like" @ 52..56: Keyword
    "products" @ 57..65: Table
    ")" @ 65..66: Punctuation
    ";" @ 66..67: Punctuation
    "#);
}

#[test]
fn partition_for_values_in_keywords() {
    assert_snapshot!(check_semantic_tokens("
create table t (a int) partition by list (a);
create table t_1
  partition of t
  for values in (1);
"), @r#"
    "create" @ 1..7: Keyword
    "table" @ 8..13: Keyword
    "t" @ 14..15: Table (Definition)
    "(" @ 16..17: Punctuation
    "a" @ 17..18: Column (Definition)
    "int" @ 19..22: Type
    ")" @ 22..23: Punctuation
    "partition" @ 24..33: Keyword
    "by" @ 34..36: Keyword
    "list" @ 37..41: NameRef
    "(" @ 42..43: Punctuation
    "a" @ 43..44: Column
    ")" @ 44..45: Punctuation
    ";" @ 45..46: Punctuation
    "create" @ 47..53: Keyword
    "table" @ 54..59: Keyword
    "t_1" @ 60..63: Table (Definition)
    "partition" @ 66..75: Keyword
    "of" @ 76..78: Keyword
    "t" @ 79..80: Table
    "for" @ 83..86: Keyword
    "values" @ 87..93: Keyword
    "in" @ 94..96: Keyword
    "(" @ 97..98: Punctuation
    "1" @ 98..99: Number
    ")" @ 99..100: Punctuation
    ";" @ 100..101: Punctuation
    "#);
}

#[test]
fn select_target_qualified_column() {
    assert_snapshot!(check_semantic_tokens("
create table t (a int);
select t.a from t;
"), @r#"
    "create" @ 1..7: Keyword
    "table" @ 8..13: Keyword
    "t" @ 14..15: Table (Definition)
    "(" @ 16..17: Punctuation
    "a" @ 17..18: Column (Definition)
    "int" @ 19..22: Type
    ")" @ 22..23: Punctuation
    ";" @ 23..24: Punctuation
    "select" @ 25..31: Keyword
    "t" @ 32..33: Table
    "." @ 33..34: Punctuation
    "a" @ 34..35: Column
    "from" @ 36..40: Keyword
    "t" @ 41..42: Table
    ";" @ 42..43: Punctuation
    "#);
}

#[test]
fn select_table_alias() {
    assert_snapshot!(check_semantic_tokens("
create table t (a int);
select x.a from t as x;
"), @r#"
    "create" @ 1..7: Keyword
    "table" @ 8..13: Keyword
    "t" @ 14..15: Table (Definition)
    "(" @ 16..17: Punctuation
    "a" @ 17..18: Column (Definition)
    "int" @ 19..22: Type
    ")" @ 22..23: Punctuation
    ";" @ 23..24: Punctuation
    "select" @ 25..31: Keyword
    "x" @ 32..33: Table
    "." @ 33..34: Punctuation
    "a" @ 34..35: Column
    "from" @ 36..40: Keyword
    "t" @ 41..42: Table
    "as" @ 43..45: Keyword
    "x" @ 46..47: Table (Definition)
    ";" @ 47..48: Punctuation
    "#);
}

#[test]
fn select_function_arg_and_field_call() {
    assert_snapshot!(check_semantic_tokens("
create table t (a int);
create function b(t) returns int
  as 'select 1'
  language sql;
select b(t), t.b from t;
"), @r#"
    "create" @ 1..7: Keyword
    "table" @ 8..13: Keyword
    "t" @ 14..15: Table (Definition)
    "(" @ 16..17: Punctuation
    "a" @ 17..18: Column (Definition)
    "int" @ 19..22: Type
    ")" @ 22..23: Punctuation
    ";" @ 23..24: Punctuation
    "create" @ 25..31: Keyword
    "function" @ 32..40: Keyword
    "b" @ 41..42: Function (Definition)
    "(" @ 42..43: Punctuation
    "t" @ 43..44: Table
    ")" @ 44..45: Punctuation
    "returns" @ 46..53: Keyword
    "int" @ 54..57: Type
    "as" @ 60..62: Keyword
    "'" @ 63..64: String
    "select" @ 64..70: Keyword
    "1" @ 71..72: Number
    "'" @ 72..73: String
    "language" @ 76..84: Keyword
    "sql" @ 85..88: Language
    ";" @ 88..89: Punctuation
    "select" @ 90..96: Keyword
    "b" @ 97..98: Function
    "(" @ 98..99: Punctuation
    "t" @ 99..100: Table
    ")" @ 100..101: Punctuation
    "," @ 101..102: Punctuation
    "t" @ 103..104: Table
    "." @ 104..105: Punctuation
    "b" @ 105..106: Function
    "from" @ 107..111: Keyword
    "t" @ 112..113: Table
    ";" @ 113..114: Punctuation
    "#);
}

#[test]
fn with_cte_name() {
    assert_snapshot!(check_semantic_tokens("
with t as (
  select 1 as a
)
select a from t;
"), @r#"
    "with" @ 1..5: Keyword
    "t" @ 6..7: Table (Definition)
    "as" @ 8..10: Keyword
    "(" @ 11..12: Punctuation
    "select" @ 15..21: Keyword
    "1" @ 22..23: Number
    "as" @ 24..26: Keyword
    "a" @ 27..28: Column (Definition)
    ")" @ 29..30: Punctuation
    "select" @ 31..37: Keyword
    "a" @ 38..39: Column
    "from" @ 40..44: Keyword
    "t" @ 45..46: Table
    ";" @ 46..47: Punctuation
    "#);
}

#[test]
fn create_property_graph() {
    assert_snapshot!(check_semantic_tokens("
create table bar (a int);
create property graph foo vertex tables (bar key (a) no properties);
"), @r#"
    "create" @ 1..7: Keyword
    "table" @ 8..13: Keyword
    "bar" @ 14..17: Table (Definition)
    "(" @ 18..19: Punctuation
    "a" @ 19..20: Column (Definition)
    "int" @ 21..24: Type
    ")" @ 24..25: Punctuation
    ";" @ 25..26: Punctuation
    "create" @ 27..33: Keyword
    "property" @ 34..42: Keyword
    "graph" @ 43..48: Keyword
    "foo" @ 49..52: PropertyGraph (Definition)
    "vertex" @ 53..59: Keyword
    "tables" @ 60..66: Keyword
    "(" @ 67..68: Punctuation
    "bar" @ 68..71: Table
    "key" @ 72..75: Keyword
    "(" @ 76..77: Punctuation
    "a" @ 77..78: Column
    ")" @ 78..79: Punctuation
    "no" @ 80..82: Keyword
    "properties" @ 83..93: Keyword
    ")" @ 93..94: Punctuation
    ";" @ 94..95: Punctuation
    "#);
}

#[test]
fn select_target_schema_qualified() {
    assert_snapshot!(check_semantic_tokens("
create schema s;
create table s.t (a int);
select s.t.a from s.t;
"), @r#"
    "create" @ 1..7: Keyword
    "schema" @ 8..14: Keyword
    "s" @ 15..16: Schema (Definition)
    ";" @ 16..17: Punctuation
    "create" @ 18..24: Keyword
    "table" @ 25..30: Keyword
    "s" @ 31..32: Schema
    "." @ 32..33: Punctuation
    "t" @ 33..34: Table (Definition)
    "(" @ 35..36: Punctuation
    "a" @ 36..37: Column (Definition)
    "int" @ 38..41: Type
    ")" @ 41..42: Punctuation
    ";" @ 42..43: Punctuation
    "select" @ 44..50: Keyword
    "s" @ 51..52: Schema
    "." @ 52..53: Punctuation
    "t" @ 53..54: Table
    "." @ 54..55: Punctuation
    "a" @ 55..56: Column
    "from" @ 57..61: Keyword
    "s" @ 62..63: Schema
    "." @ 63..64: Punctuation
    "t" @ 64..65: Table
    ";" @ 65..66: Punctuation
    "#);
}

#[test]
fn create_function_embedded_sql() {
    assert_snapshot!(check_semantic_tokens("
create table t (a int);
create function f(x int) returns int
  as $$ select a from t where a = x and a = $1 $$
  language sql;
"), @r#"
    "create" @ 1..7: Keyword
    "table" @ 8..13: Keyword
    "t" @ 14..15: Table (Definition)
    "(" @ 16..17: Punctuation
    "a" @ 17..18: Column (Definition)
    "int" @ 19..22: Type
    ")" @ 22..23: Punctuation
    ";" @ 23..24: Punctuation
    "create" @ 25..31: Keyword
    "function" @ 32..40: Keyword
    "f" @ 41..42: Function (Definition)
    "(" @ 42..43: Punctuation
    "x" @ 43..44: Parameter (Definition)
    "int" @ 45..48: Type
    ")" @ 48..49: Punctuation
    "returns" @ 50..57: Keyword
    "int" @ 58..61: Type
    "as" @ 64..66: Keyword
    "$$" @ 67..69: String
    "select" @ 70..76: Keyword
    "a" @ 77..78: Column
    "from" @ 79..83: Keyword
    "t" @ 84..85: Table
    "where" @ 86..91: Keyword
    "a" @ 92..93: Column
    "=" @ 94..95: Operator
    "x" @ 96..97: Parameter
    "and" @ 98..101: Keyword
    "a" @ 102..103: Column
    "=" @ 104..105: Operator
    "$1" @ 106..108: Parameter
    "$$" @ 109..111: String
    "language" @ 114..122: Keyword
    "sql" @ 123..126: Language
    ";" @ 126..127: Punctuation
    "#);
}

#[test]
fn create_procedure_embedded_sql_escaped() {
    assert_snapshot!(check_semantic_tokens("
create table t (a text);
create procedure p()
  language sql
  as 'insert into t(a) select ''x'' from t';
"), @r#"
    "create" @ 1..7: Keyword
    "table" @ 8..13: Keyword
    "t" @ 14..15: Table (Definition)
    "(" @ 16..17: Punctuation
    "a" @ 17..18: Column (Definition)
    "text" @ 19..23: Type
    ")" @ 23..24: Punctuation
    ";" @ 24..25: Punctuation
    "create" @ 26..32: Keyword
    "procedure" @ 33..42: Keyword
    "p" @ 43..44: Function (Definition)
    "(" @ 44..45: Punctuation
    ")" @ 45..46: Punctuation
    "language" @ 49..57: Keyword
    "sql" @ 58..61: Language
    "as" @ 64..66: Keyword
    "'" @ 67..68: String
    "insert" @ 68..74: Keyword
    "into" @ 75..79: Keyword
    "t" @ 80..81: Table
    "(" @ 81..82: Punctuation
    "a" @ 82..83: Column
    ")" @ 83..84: Punctuation
    "select" @ 85..91: Keyword
    "''x''" @ 92..97: String
    "from" @ 98..102: Keyword
    "t" @ 103..104: Table
    "'" @ 104..105: String
    ";" @ 105..106: Punctuation
    "#);
}

#[test]
fn create_function_embedded_non_sql() {
    assert_snapshot!(check_semantic_tokens("
create table t (a int);
create function f() returns int
  as $$ select a from t $$
  language plpgsql;
"), @r#"
    "create" @ 1..7: Keyword
    "table" @ 8..13: Keyword
    "t" @ 14..15: Table (Definition)
    "(" @ 16..17: Punctuation
    "a" @ 17..18: Column (Definition)
    "int" @ 19..22: Type
    ")" @ 22..23: Punctuation
    ";" @ 23..24: Punctuation
    "create" @ 25..31: Keyword
    "function" @ 32..40: Keyword
    "f" @ 41..42: Function (Definition)
    "(" @ 42..43: Punctuation
    ")" @ 43..44: Punctuation
    "returns" @ 45..52: Keyword
    "int" @ 53..56: Type
    "as" @ 59..61: Keyword
    "$$ select a from t $$" @ 62..83: String
    "language" @ 86..94: Keyword
    "plpgsql" @ 95..102: Language
    ";" @ 102..103: Punctuation
    "#);
}

#[test]
fn embedded_sql_lexical_tokens() {
    assert_snapshot!(check_semantic_tokens("
create function f() returns int
  language sql
  as $body$
-- embedded
select unknown_name, 42, 1.5, true, false, null, 'text', E'escape', B'01', X'ff';
$body$;
"), @r#"
    "create" @ 1..7: Keyword
    "function" @ 8..16: Keyword
    "f" @ 17..18: Function (Definition)
    "(" @ 18..19: Punctuation
    ")" @ 19..20: Punctuation
    "returns" @ 21..28: Keyword
    "int" @ 29..32: Type
    "language" @ 35..43: Keyword
    "sql" @ 44..47: Language
    "as" @ 50..52: Keyword
    "$body$" @ 53..59: String
    "-- embedded" @ 60..71: Comment
    "select" @ 72..78: Keyword
    "unknown_name" @ 79..91: NameRef
    "," @ 91..92: Punctuation
    "42" @ 93..95: Number
    "," @ 95..96: Punctuation
    "1.5" @ 97..100: Number
    "," @ 100..101: Punctuation
    "true" @ 102..106: Bool
    "," @ 106..107: Punctuation
    "false" @ 108..113: Bool
    "," @ 113..114: Punctuation
    "null" @ 115..119: Keyword
    "," @ 119..120: Punctuation
    "'text'" @ 121..127: String
    "," @ 127..128: Punctuation
    "E'escape'" @ 129..138: String
    "," @ 138..139: Punctuation
    "B'01'" @ 140..145: String
    "," @ 145..146: Punctuation
    "X'ff'" @ 147..152: String
    ";" @ 152..153: Punctuation
    "$body$" @ 154..160: String
    ";" @ 160..161: Punctuation
    "#);
}

#[test]
fn range() {
    let sql = "
create table t (a int);
select a from t;
";
    let start = TextSize::try_from(sql.find("select").unwrap()).unwrap();
    let range = TextRange::new(start, TextSize::of(sql));
    assert_snapshot!(check_semantic_tokens_in_range(sql, Some(range)), @r#"
    "select" @ 25..31: Keyword
    "a" @ 32..33: Column
    "from" @ 34..38: Keyword
    "t" @ 39..40: Table
    ";" @ 40..41: Punctuation
    "#);
}

#[test]
fn object_kinds() {
    assert_snapshot!(check_semantic_tokens(r#"
create role r;
create type mood as enum (
  'happy',
  'sad'
);
create table t (
  a int constraint pos check (a > 0),
  m mood
);
create index i on t (a);
alter table t drop constraint pos;
drop index i;
select sum(a) over w from t window w as (order by a);
select a from t where m = 'happy';
select a collate "C" from t;
begin;
savepoint s;
rollback to savepoint s;
commit;
declare c cursor
for select 1;
fetch c;
prepare p as
  select 1;
execute p;
listen ch;
notify ch;
grant select on t to r;
create function f() returns int
  as 'select 1'
  language sql;
create extension hstore;
"#), @r#"
    "create" @ 1..7: Keyword
    "role" @ 8..12: Keyword
    "r" @ 13..14: Role (Definition)
    ";" @ 14..15: Punctuation
    "create" @ 16..22: Keyword
    "type" @ 23..27: Keyword
    "mood" @ 28..32: Type (Definition)
    "as" @ 33..35: Keyword
    "enum" @ 36..40: Keyword
    "(" @ 41..42: Punctuation
    "'happy'" @ 45..52: EnumMember (Definition)
    "," @ 52..53: Punctuation
    "'sad'" @ 56..61: EnumMember (Definition)
    ")" @ 62..63: Punctuation
    ";" @ 63..64: Punctuation
    "create" @ 65..71: Keyword
    "table" @ 72..77: Keyword
    "t" @ 78..79: Table (Definition)
    "(" @ 80..81: Punctuation
    "a" @ 84..85: Column (Definition)
    "int" @ 86..89: Type
    "constraint" @ 90..100: Keyword
    "pos" @ 101..104: Constraint (Definition)
    "check" @ 105..110: Keyword
    "(" @ 111..112: Punctuation
    "a" @ 112..113: Column
    ">" @ 114..115: Operator
    "0" @ 116..117: Number
    ")" @ 117..118: Punctuation
    "," @ 118..119: Punctuation
    "m" @ 122..123: Column (Definition)
    "mood" @ 124..128: Type
    ")" @ 129..130: Punctuation
    ";" @ 130..131: Punctuation
    "create" @ 132..138: Keyword
    "index" @ 139..144: Keyword
    "i" @ 145..146: Index (Definition)
    "on" @ 147..149: Keyword
    "t" @ 150..151: Table
    "(" @ 152..153: Punctuation
    "a" @ 153..154: Column
    ")" @ 154..155: Punctuation
    ";" @ 155..156: Punctuation
    "alter" @ 157..162: Keyword
    "table" @ 163..168: Keyword
    "t" @ 169..170: Table
    "drop" @ 171..175: Keyword
    "constraint" @ 176..186: Keyword
    "pos" @ 187..190: Constraint
    ";" @ 190..191: Punctuation
    "drop" @ 192..196: Keyword
    "index" @ 197..202: Keyword
    "i" @ 203..204: Index
    ";" @ 204..205: Punctuation
    "select" @ 206..212: Keyword
    "sum" @ 213..216: Function
    "(" @ 216..217: Punctuation
    "a" @ 217..218: Column
    ")" @ 218..219: Punctuation
    "over" @ 220..224: Keyword
    "w" @ 225..226: Window
    "from" @ 227..231: Keyword
    "t" @ 232..233: Table
    "window" @ 234..240: Keyword
    "w" @ 241..242: Window (Definition)
    "as" @ 243..245: Keyword
    "(" @ 246..247: Punctuation
    "order" @ 247..252: Keyword
    "by" @ 253..255: Keyword
    "a" @ 256..257: Column
    ")" @ 257..258: Punctuation
    ";" @ 258..259: Punctuation
    "select" @ 260..266: Keyword
    "a" @ 267..268: Column
    "from" @ 269..273: Keyword
    "t" @ 274..275: Table
    "where" @ 276..281: Keyword
    "m" @ 282..283: Column
    "=" @ 284..285: Operator
    "'happy'" @ 286..293: EnumMember
    ";" @ 293..294: Punctuation
    "select" @ 295..301: Keyword
    "a" @ 302..303: Column
    "collate" @ 304..311: Keyword
    "\"C\"" @ 312..315: Collation
    "from" @ 316..320: Keyword
    "t" @ 321..322: Table
    ";" @ 322..323: Punctuation
    "begin" @ 324..329: Keyword
    ";" @ 329..330: Punctuation
    "savepoint" @ 331..340: Keyword
    "s" @ 341..342: Savepoint (Definition)
    ";" @ 342..343: Punctuation
    "rollback" @ 344..352: Keyword
    "to" @ 353..355: Keyword
    "savepoint" @ 356..365: Keyword
    "s" @ 366..367: Savepoint
    ";" @ 367..368: Punctuation
    "commit" @ 369..375: Keyword
    ";" @ 375..376: Punctuation
    "declare" @ 377..384: Keyword
    "c" @ 385..386: Cursor (Definition)
    "cursor" @ 387..393: Keyword
    "for" @ 394..397: Keyword
    "select" @ 398..404: Keyword
    "1" @ 405..406: Number
    ";" @ 406..407: Punctuation
    "fetch" @ 408..413: Keyword
    "c" @ 414..415: Cursor
    ";" @ 415..416: Punctuation
    "prepare" @ 417..424: Keyword
    "p" @ 425..426: PreparedStatement (Definition)
    "as" @ 427..429: Keyword
    "select" @ 432..438: Keyword
    "1" @ 439..440: Number
    ";" @ 440..441: Punctuation
    "execute" @ 442..449: Keyword
    "p" @ 450..451: PreparedStatement
    ";" @ 451..452: Punctuation
    "listen" @ 453..459: Keyword
    "ch" @ 460..462: Channel (Definition)
    ";" @ 462..463: Punctuation
    "notify" @ 464..470: Keyword
    "ch" @ 471..473: Channel
    ";" @ 473..474: Punctuation
    "grant" @ 475..480: Keyword
    "select" @ 481..487: Keyword
    "on" @ 488..490: Keyword
    "t" @ 491..492: Table
    "to" @ 493..495: Keyword
    "r" @ 496..497: Role
    ";" @ 497..498: Punctuation
    "create" @ 499..505: Keyword
    "function" @ 506..514: Keyword
    "f" @ 515..516: Function (Definition)
    "(" @ 516..517: Punctuation
    ")" @ 517..518: Punctuation
    "returns" @ 519..526: Keyword
    "int" @ 527..530: Type
    "as" @ 533..535: Keyword
    "'" @ 536..537: String
    "select" @ 537..543: Keyword
    "1" @ 544..545: Number
    "'" @ 545..546: String
    "language" @ 549..557: Keyword
    "sql" @ 558..561: Language
    ";" @ 561..562: Punctuation
    "create" @ 563..569: Keyword
    "extension" @ 570..579: Keyword
    "hstore" @ 580..586: Extension (Definition)
    ";" @ 586..587: Punctuation
    "#);
}

#[test]
fn grant_to_public() {
    assert_snapshot!(check_semantic_tokens(r#"
create table t (a int);
grant select on t to public;
"#), @r#"
    "create" @ 1..7: Keyword
    "table" @ 8..13: Keyword
    "t" @ 14..15: Table (Definition)
    "(" @ 16..17: Punctuation
    "a" @ 17..18: Column (Definition)
    "int" @ 19..22: Type
    ")" @ 22..23: Punctuation
    ";" @ 23..24: Punctuation
    "grant" @ 25..30: Keyword
    "select" @ 31..37: Keyword
    "on" @ 38..40: Keyword
    "t" @ 41..42: Table
    "to" @ 43..45: Keyword
    "public" @ 46..52: Keyword
    ";" @ 52..53: Punctuation
    "#);
}
