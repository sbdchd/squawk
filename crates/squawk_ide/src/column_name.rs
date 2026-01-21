use squawk_syntax::{
    SyntaxKind, SyntaxNode,
    ast::{self, AstNode},
};

use crate::quote::normalize_identifier;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ColumnName {
    Column(String),
    /// There's a fallback mechanism that we need to propagate through the
    /// expressions/types.
    //
    /// We can see this with:
    /// ```sql
    /// select case when true then 'a' else now()::text end;
    /// -- column named `now`, propagating the function name
    /// -- vs
    /// select case when true then 'a' else 'b' end;
    /// -- column named `case`
    /// ```
    UnknownColumn(Option<String>),
    Star,
}

impl ColumnName {
    // Get the alias, otherwise infer the column name.
    pub(crate) fn from_target(target: ast::Target) -> Option<(ColumnName, SyntaxNode)> {
        if let Some(as_name) = target.as_name()
            && let Some(name_node) = as_name.name()
        {
            let text = name_node.text();
            let normalized = normalize_identifier(&text);
            return Some((ColumnName::Column(normalized), name_node.syntax().clone()));
        }
        Self::inferred_from_target(target)
    }

    // Ignore any aliases, just infer the what the column name.
    pub(crate) fn inferred_from_target(target: ast::Target) -> Option<(ColumnName, SyntaxNode)> {
        if let Some(expr) = target.expr()
            && let Some(name) = name_from_expr(expr, false)
        {
            return Some(name);
        } else if target.star_token().is_some() {
            return Some((ColumnName::Star, target.syntax().clone()));
        }
        None
    }

    fn new(name: String, unknown_column: bool) -> ColumnName {
        if unknown_column {
            ColumnName::UnknownColumn(Some(name))
        } else {
            ColumnName::Column(name)
        }
    }

    pub(crate) fn to_string(&self) -> Option<String> {
        match self {
            ColumnName::Column(string) => Some(string.to_string()),
            ColumnName::Star => None,
            ColumnName::UnknownColumn(c) => {
                Some(c.clone().unwrap_or_else(|| "?column?".to_string()))
            }
        }
    }
}

fn name_from_type(ty: ast::Type, unknown_column: bool) -> Option<(ColumnName, SyntaxNode)> {
    match ty {
        ast::Type::PathType(path_type) => {
            if let Some(name_ref) = path_type
                .path()
                .and_then(|x| x.segment())
                .and_then(|x| x.name_ref())
            {
                return name_from_name_ref(name_ref, true).map(|(column, node)| {
                    let column = match column {
                        ColumnName::Column(c) => ColumnName::new(c, unknown_column),
                        _ => column,
                    };
                    (column, node)
                });
            }
        }
        ast::Type::BitType(bit_type) => {
            let name = if bit_type.varying_token().is_some() {
                "varbit"
            } else {
                "bit"
            };
            return Some((
                ColumnName::new(name.to_string(), unknown_column),
                bit_type.syntax().clone(),
            ));
        }
        ast::Type::CharType(char_type) => {
            let name = if char_type.varchar_token().is_some() || char_type.varying_token().is_some()
            {
                "varchar"
            } else {
                "bpchar"
            };
            return Some((
                ColumnName::new(name.to_string(), unknown_column),
                char_type.syntax().clone(),
            ));
        }
        ast::Type::DoubleType(double_type) => {
            return Some((
                ColumnName::new("float8".to_string(), unknown_column),
                double_type.syntax().clone(),
            ));
        }
        ast::Type::IntervalType(interval_type) => {
            return Some((
                ColumnName::new("interval".to_string(), unknown_column),
                interval_type.syntax().clone(),
            ));
        }
        ast::Type::TimeType(time_type) => {
            let mut name = if time_type.timestamp_token().is_some() {
                "timestamp".to_owned()
            } else {
                "time".to_owned()
            };
            if let Some(ast::Timezone::WithTimezone(_)) = time_type.timezone() {
                // time -> timetz
                // timestamp -> timestamptz
                name.push_str("tz");
            };
            return Some((
                ColumnName::new(name.to_string(), unknown_column),
                time_type.syntax().clone(),
            ));
        }
        ast::Type::ArrayType(array_type) => {
            if let Some(inner_ty) = array_type.ty() {
                return name_from_type(inner_ty, unknown_column);
            }
        }
        // we shouldn't ever hit this since the following isn't valid syntax:
        // select cast('foo' as t.a%TYPE);
        ast::Type::PercentType(_) => return None,
        ast::Type::ExprType(expr_type) => {
            if let Some(expr) = expr_type.expr() {
                return name_from_expr(expr, true).map(|(column, node)| {
                    let column = match column {
                        ColumnName::Column(c) => ColumnName::new(c, unknown_column),
                        _ => column,
                    };
                    (column, node)
                });
            }
        }
    }
    None
}

fn name_from_name_ref(name_ref: ast::NameRef, in_type: bool) -> Option<(ColumnName, SyntaxNode)> {
    if in_type {
        for node in name_ref.syntax().children_with_tokens() {
            match node.kind() {
                SyntaxKind::BIGINT_KW => {
                    return Some((
                        ColumnName::Column("int8".to_owned()),
                        name_ref.syntax().clone(),
                    ));
                }
                SyntaxKind::INT_KW | SyntaxKind::INTEGER_KW => {
                    return Some((
                        ColumnName::Column("int4".to_owned()),
                        name_ref.syntax().clone(),
                    ));
                }
                SyntaxKind::SMALLINT_KW => {
                    return Some((
                        ColumnName::Column("int2".to_owned()),
                        name_ref.syntax().clone(),
                    ));
                }
                SyntaxKind::REAL_KW => {
                    return Some((
                        ColumnName::Column("float4".to_owned()),
                        name_ref.syntax().clone(),
                    ));
                }
                _ => (),
            }
        }
    }
    let text = name_ref.text();
    let normalized = normalize_identifier(&text);
    return Some((ColumnName::Column(normalized), name_ref.syntax().clone()));
}

/*
TODO:

unnest(anyarray, anyarray [, ... ]) → setof anyelement, anyelement [, ... ]

select * from unnest(ARRAY[1,2], ARRAY['foo','bar','baz']) →
 unnset | unnset
--------+-----
      1 | foo
      2 | bar
        | baz
*/

// NOTE: we have to have this in_type param because we parse some casts as exprs
// instead of types.
fn name_from_expr(expr: ast::Expr, in_type: bool) -> Option<(ColumnName, SyntaxNode)> {
    let node = expr.syntax().clone();
    match expr {
        ast::Expr::ArrayExpr(_) => {
            return Some((ColumnName::Column("array".to_string()), node));
        }
        ast::Expr::BetweenExpr(_) | ast::Expr::BinExpr(_) => {
            return Some((ColumnName::UnknownColumn(None), node));
        }
        ast::Expr::CallExpr(call_expr) => {
            if let Some(exists_fn) = call_expr.exists_fn() {
                return Some((
                    ColumnName::Column("exists".to_string()),
                    exists_fn.syntax().clone(),
                ));
            }
            if let Some(extract_fn) = call_expr.extract_fn() {
                return Some((
                    ColumnName::Column("extract".to_string()),
                    extract_fn.syntax().clone(),
                ));
            }
            if let Some(json_exists_fn) = call_expr.json_exists_fn() {
                return Some((
                    ColumnName::Column("json_exists".to_string()),
                    json_exists_fn.syntax().clone(),
                ));
            }
            if let Some(json_array_fn) = call_expr.json_array_fn() {
                return Some((
                    ColumnName::Column("json_array".to_string()),
                    json_array_fn.syntax().clone(),
                ));
            }
            if let Some(json_object_fn) = call_expr.json_object_fn() {
                return Some((
                    ColumnName::Column("json_object".to_string()),
                    json_object_fn.syntax().clone(),
                ));
            }
            if let Some(json_object_agg_fn) = call_expr.json_object_agg_fn() {
                return Some((
                    ColumnName::Column("json_objectagg".to_string()),
                    json_object_agg_fn.syntax().clone(),
                ));
            }
            if let Some(json_array_agg_fn) = call_expr.json_array_agg_fn() {
                return Some((
                    ColumnName::Column("json_arrayagg".to_string()),
                    json_array_agg_fn.syntax().clone(),
                ));
            }
            if let Some(json_query_fn) = call_expr.json_query_fn() {
                return Some((
                    ColumnName::Column("json_query".to_string()),
                    json_query_fn.syntax().clone(),
                ));
            }
            if let Some(json_scalar_fn) = call_expr.json_scalar_fn() {
                return Some((
                    ColumnName::Column("json_scalar".to_string()),
                    json_scalar_fn.syntax().clone(),
                ));
            }
            if let Some(json_serialize_fn) = call_expr.json_serialize_fn() {
                return Some((
                    ColumnName::Column("json_serialize".to_string()),
                    json_serialize_fn.syntax().clone(),
                ));
            }
            if let Some(json_value_fn) = call_expr.json_value_fn() {
                return Some((
                    ColumnName::Column("json_value".to_string()),
                    json_value_fn.syntax().clone(),
                ));
            }
            if let Some(json_fn) = call_expr.json_fn() {
                return Some((
                    ColumnName::Column("json".to_string()),
                    json_fn.syntax().clone(),
                ));
            }
            if let Some(substring_fn) = call_expr.substring_fn() {
                return Some((
                    ColumnName::Column("substring".to_string()),
                    substring_fn.syntax().clone(),
                ));
            }
            if let Some(position_fn) = call_expr.position_fn() {
                return Some((
                    ColumnName::Column("position".to_string()),
                    position_fn.syntax().clone(),
                ));
            }
            if let Some(overlay_fn) = call_expr.overlay_fn() {
                return Some((
                    ColumnName::Column("overlay".to_string()),
                    overlay_fn.syntax().clone(),
                ));
            }
            if let Some(trim_fn) = call_expr.trim_fn() {
                return Some((
                    ColumnName::Column("trim".to_string()),
                    trim_fn.syntax().clone(),
                ));
            }
            if let Some(xml_root_fn) = call_expr.xml_root_fn() {
                return Some((
                    ColumnName::Column("xml_root".to_string()),
                    xml_root_fn.syntax().clone(),
                ));
            }
            if let Some(xml_serialize_fn) = call_expr.xml_serialize_fn() {
                return Some((
                    ColumnName::Column("xml_serialize".to_string()),
                    xml_serialize_fn.syntax().clone(),
                ));
            }
            if let Some(xml_element_fn) = call_expr.xml_element_fn() {
                return Some((
                    ColumnName::Column("xml_element".to_string()),
                    xml_element_fn.syntax().clone(),
                ));
            }
            if let Some(xml_forest_fn) = call_expr.xml_forest_fn() {
                return Some((
                    ColumnName::Column("xml_forest".to_string()),
                    xml_forest_fn.syntax().clone(),
                ));
            }
            if let Some(xml_exists_fn) = call_expr.xml_exists_fn() {
                return Some((
                    ColumnName::Column("xml_exists".to_string()),
                    xml_exists_fn.syntax().clone(),
                ));
            }
            if let Some(xml_parse_fn) = call_expr.xml_parse_fn() {
                return Some((
                    ColumnName::Column("xml_parse".to_string()),
                    xml_parse_fn.syntax().clone(),
                ));
            }
            if let Some(xml_pi_fn) = call_expr.xml_pi_fn() {
                return Some((
                    ColumnName::Column("xml_pi".to_string()),
                    xml_pi_fn.syntax().clone(),
                ));
            }
            if let Some(func_name) = call_expr.expr() {
                match func_name {
                    ast::Expr::ArrayExpr(_)
                    | ast::Expr::BetweenExpr(_)
                    | ast::Expr::ParenExpr(_)
                    | ast::Expr::BinExpr(_)
                    | ast::Expr::CallExpr(_)
                    | ast::Expr::CaseExpr(_)
                    | ast::Expr::CastExpr(_)
                    | ast::Expr::Literal(_)
                    | ast::Expr::PostfixExpr(_)
                    | ast::Expr::PrefixExpr(_)
                    | ast::Expr::TupleExpr(_)
                    | ast::Expr::IndexExpr(_)
                    | ast::Expr::SliceExpr(_) => unreachable!("not possible in the grammar"),
                    ast::Expr::FieldExpr(field_expr) => {
                        if let Some(name_ref) = field_expr.field() {
                            return name_from_name_ref(name_ref, in_type);
                        }
                    }
                    ast::Expr::NameRef(name_ref) => {
                        return name_from_name_ref(name_ref, in_type);
                    }
                }
            }
        }
        ast::Expr::CaseExpr(case) => {
            if let Some(else_clause) = case.else_clause()
                && let Some(expr) = else_clause.expr()
                && let Some((column, node)) = name_from_expr(expr, in_type)
            {
                if !matches!(column, ColumnName::UnknownColumn(_)) {
                    return Some((column, node));
                }
            }
            return Some((ColumnName::Column("case".to_string()), node));
        }
        ast::Expr::CastExpr(cast_expr) => {
            let mut unknown_column = false;
            if let Some(expr) = cast_expr.expr()
                && let Some((column, node)) = name_from_expr(expr, in_type)
            {
                match column {
                    ColumnName::Column(_) => return Some((column, node)),
                    ColumnName::UnknownColumn(_) => unknown_column = true,
                    ColumnName::Star => (),
                }
            }
            if let Some(ty) = cast_expr.ty() {
                return name_from_type(ty, unknown_column);
            }
        }
        ast::Expr::FieldExpr(field_expr) => {
            if let Some(name_ref) = field_expr.field() {
                return name_from_name_ref(name_ref, in_type);
            }
        }
        ast::Expr::IndexExpr(index_expr) => {
            if let Some(base) = index_expr.base() {
                return name_from_expr(base, in_type);
            }
        }
        ast::Expr::SliceExpr(slice_expr) => {
            if let Some(base) = slice_expr.base() {
                return name_from_expr(base, in_type);
            }
        }
        ast::Expr::Literal(_) | ast::Expr::PrefixExpr(_) | ast::Expr::PostfixExpr(_) => {
            return Some((ColumnName::UnknownColumn(None), node));
        }
        ast::Expr::NameRef(name_ref) => {
            return name_from_name_ref(name_ref, in_type);
        }
        ast::Expr::ParenExpr(paren_expr) => {
            if let Some(expr) = paren_expr.expr() {
                return name_from_expr(expr, in_type);
            } else if let Some(select) = paren_expr.select()
                && let Some(mut targets) = select
                    .select_clause()
                    .and_then(|x| x.target_list())
                    .map(|x| x.targets())
                && let Some(target) = targets.next()
            {
                return ColumnName::from_target(target);
            }
        }
        ast::Expr::TupleExpr(_) => {
            return Some((ColumnName::Column("row".to_string()), node));
        }
    }
    None
}

#[test]
fn examples() {
    use insta::assert_snapshot;

    // array
    assert_snapshot!(name("array(select 1)"), @"array");
    assert_snapshot!(name("array[1, 2, 3]"), @"array");

    // unknown columns
    assert_snapshot!(name("1 between 0 and 10"), @"?column?");
    assert_snapshot!(name("1 + 2"), @"?column?");
    assert_snapshot!(name("42"), @"?column?");
    assert_snapshot!(name("'string'"), @"?column?");
    // prefix
    assert_snapshot!(name("-42"), @"?column?");
    assert_snapshot!(name("|/ 42"), @"?column?");
    // postfix
    assert_snapshot!(name("x is null"), @"?column?");
    assert_snapshot!(name("x is not null"), @"?column?");
    // paren expr
    assert_snapshot!(name("(1 * 2)"), @"?column?");
    assert_snapshot!(name("(select 1 as a)"), @"a");

    // func
    assert_snapshot!(name("count(*)"), @"count");
    assert_snapshot!(name("schema.func_name(1)"), @"func_name");

    // special funcs
    assert_snapshot!(name("extract(year from now())"), @"extract");
    assert_snapshot!(name("exists(select 1)"), @"exists");
    assert_snapshot!(name(r#"json_exists('{"a":1}', '$.a')"#), @"json_exists");
    assert_snapshot!(name("json_array(1, 2)"), @"json_array");
    assert_snapshot!(name("json_object('a': 1)"), @"json_object");
    assert_snapshot!(name("json_objectagg('a': 1)"), @"json_objectagg");
    assert_snapshot!(name("json_arrayagg(1)"), @"json_arrayagg");
    assert_snapshot!(name(r#"json_query('{"a":1}', '$.a')"#), @"json_query");
    assert_snapshot!(name("json_scalar(1)"), @"json_scalar");
    assert_snapshot!(name(r#"json_serialize('{"a":1}')"#), @"json_serialize");
    assert_snapshot!(name(r#"json_value('{"a":1}', '$.a')"#), @"json_value");
    assert_snapshot!(name(r#"json('{"a":1}')"#), @"json");
    assert_snapshot!(name("substring('hello' from 2 for 3)"), @"substring");
    assert_snapshot!(name("position('a' in 'abc')"), @"position");
    assert_snapshot!(name("overlay('hello' placing 'X' from 2)"), @"overlay");
    assert_snapshot!(name("trim('  hi  ')"), @"trim");
    assert_snapshot!(name("xmlroot('<a/>', version '1.0')"), @"xml_root");
    assert_snapshot!(name("xmlserialize(document '<a/>' as text)"), @"xml_serialize");
    assert_snapshot!(name("xmlelement(name foo, 'bar')"), @"xml_element");
    assert_snapshot!(name("xmlforest('bar' as foo)"), @"xml_forest");
    assert_snapshot!(name("xmlexists('//a' passing '<a/>')"), @"xml_exists");
    assert_snapshot!(name("xmlparse(document '<a/>')"), @"xml_parse");
    assert_snapshot!(name("xmlpi(name foo, 'bar')"), @"xml_pi");

    // index
    assert_snapshot!(name("foo[bar]"), @"foo");
    assert_snapshot!(name("foo[1]"), @"foo");

    // column
    assert_snapshot!(name("database.schema.table.column"), @"column");
    assert_snapshot!(name("t.a"), @"a");
    assert_snapshot!(name("col_name"), @"col_name");
    assert_snapshot!(name("(c)"), @"c");

    // case
    assert_snapshot!(name("case when true then 'foo' end"), @"case");
    assert_snapshot!(name("case when true then 'foo' else now()::text end"), @"now");
    assert_snapshot!(name("case when true then 'foo' else 'bar' end"), @"case");
    assert_snapshot!(name("case when true then 'foo' else '1'::bigint::text end"), @"case");

    // casts
    assert_snapshot!(name("now()::text"), @"now");
    assert_snapshot!(name("cast(col_name as text)"), @"col_name");
    assert_snapshot!(name("col_name::text"), @"col_name");
    assert_snapshot!(name("col_name::int::text"), @"col_name");
    assert_snapshot!(name("'1'::bigint"), @"int8");
    assert_snapshot!(name("'1'::int"), @"int4");
    assert_snapshot!(name("'1'::smallint"), @"int2");
    assert_snapshot!(name("'{{1, 2}, {3, 4}}'::bigint[][]"), @"int8");
    assert_snapshot!(name("'{{1, 2}, {3, 4}}'::int[][]"), @"int4");
    assert_snapshot!(name("'{{1, 2}, {3, 4}}'::smallint[]"), @"int2");
    assert_snapshot!(name("pg_catalog.varchar(100) '{1}'"), @"varchar");
    assert_snapshot!(name("'{1}'::integer[];"), @"int4");
    assert_snapshot!(name("'{1}'::pg_catalog.varchar(1)[]::integer[];"), @"int4");
    assert_snapshot!(name("'1'::bigint::smallint"), @"int2");

    // alias
    // with quoting
    assert_snapshot!(name(r#"'foo' as "FOO""#), @"FOO");
    assert_snapshot!(name(r#"'foo' as "foo""#), @"foo");
    // without quoting
    assert_snapshot!(name(r#"'foo' as FOO"#), @"foo");
    assert_snapshot!(name(r#"'foo' as foo"#), @"foo");

    // tuple
    assert_snapshot!(name("(1, 2, 3)"), @"row");
    assert_snapshot!(name("(1, 2, 3)::address"), @"row");

    // composite type
    assert_snapshot!(name("(x).city"), @"city");

    // array types
    assert_snapshot!(name("'{{1, 2}, {3, 4}}'::int[]"), @"int4");
    assert_snapshot!(name("cast('{foo}' as text[])"), @"text");

    // bit types
    assert_snapshot!(name("cast('1010' as bit varying(10))"), @"varbit");

    // char types
    assert_snapshot!(name("cast('hello' as character varying(10))"), @"varchar");
    assert_snapshot!(name("cast('hello' as char varying(5))"), @"varchar");
    assert_snapshot!(name("cast('hello' as char(5))"), @"bpchar");
    assert_snapshot!(name("cast('hello' as character)"), @"bpchar");
    assert_snapshot!(name("cast('hello' as bpchar)"), @"bpchar");

    assert_snapshot!(name(r#"cast('hello' as "char")"#), @"char");

    // double types
    assert_snapshot!(name("cast(1.5 as double precision)"), @"float8");
    // real
    assert_snapshot!(name("cast(1.5 as real)"), @"float4");

    // interval types
    assert_snapshot!(name("cast('1 hour' as interval hour to minute)"), @"interval");

    // percent types
    assert_snapshot!(name("cast(foo as schema.%TYPE)"), @"foo");

    // time types
    assert_snapshot!(name("cast('12:00:00' as time(6) without time zone)"), @"time");
    assert_snapshot!(name("cast('12:00:00' as time(6) with time zone)"), @"timetz");
    assert_snapshot!(name("cast('2024-01-01 12:00:00' as timestamp(6) with time zone)"), @"timestamptz");
    assert_snapshot!(name("cast('2024-01-01 12:00:00' as timestamp(6) without time zone)"), @"timestamp");

    #[track_caller]
    fn name(sql: &str) -> String {
        let sql = "select ".to_string() + sql;
        let parse = squawk_syntax::SourceFile::parse(&sql);
        assert_eq!(parse.errors(), vec![]);
        let file = parse.tree();

        let stmt = file.stmts().next().unwrap();
        let ast::Stmt::Select(select) = stmt else {
            unreachable!()
        };

        let target = select
            .select_clause()
            .and_then(|sc| sc.target_list())
            .and_then(|tl| tl.targets().next())
            .unwrap();

        ColumnName::from_target(target)
            .and_then(|x| x.0.to_string())
            .unwrap()
    }
}
