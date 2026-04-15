use rowan::{NodeOrToken, TextRange};
use salsa::Database as Db;
use squawk_syntax::{
    SyntaxElement, SyntaxKind,
    ast::{self, AstNode},
};

use crate::db::{File, parse};
use crate::goto_definition::{LocationKind, goto_definition};

fn highlight_param_mode(out: &mut SemanticTokenBuilder, mode: ast::ParamMode) {
    match mode {
        ast::ParamMode::ParamIn(param_in) => {
            if let Some(token) = param_in.in_token() {
                out.push_keyword(token.into());
            }
        }
        ast::ParamMode::ParamInOut(param_in_out) => {
            if let Some(token) = param_in_out.in_token() {
                out.push_keyword(token.into());
            }
            if let Some(token) = param_in_out.inout_token() {
                out.push_keyword(token.into());
            }
            if let Some(token) = param_in_out.out_token() {
                out.push_keyword(token.into());
            }
        }
        ast::ParamMode::ParamOut(param_out) => {
            if let Some(token) = param_out.out_token() {
                out.push_keyword(token.into());
            }
        }
        ast::ParamMode::ParamVariadic(param_variadic) => {
            if let Some(token) = param_variadic.variadic_token() {
                out.push_keyword(token.into());
            }
        }
    }
}

fn highlight_type(out: &mut SemanticTokenBuilder, ty: ast::Type) {
    match ty {
        ast::Type::ArrayType(_) => (),
        ast::Type::BitType(bit_type) => {
            if let Some(token) = bit_type.bit_token() {
                out.push_type(token.into());
            }
        }
        ast::Type::CharType(char_type) => {
            if let Some(token) = char_type
                .varchar_token()
                .or_else(|| char_type.nchar_token())
                .or_else(|| char_type.character_token())
                .or_else(|| char_type.char_token())
            {
                out.push_type(token.into());
            };
        }
        ast::Type::DoubleType(double_type) => {
            if let Some(token) = double_type.double_token() {
                out.push_type(token.into());
            }
            if let Some(token) = double_type.precision_token() {
                out.push_type(token.into());
            }
        }
        ast::Type::ExprType(_) => (),
        ast::Type::IntervalType(interval_type) => {
            if let Some(token) = interval_type.interval_token() {
                out.push_type(token.into());
            }
        }
        ast::Type::PathType(_) => (),
        ast::Type::PercentType(_) => (),
        ast::Type::TimeType(time_type) => {
            if let Some(token) = time_type
                .timestamp_token()
                .or_else(|| time_type.time_token())
            {
                out.push_type(token.into());
            }
        }
    }
}

/// A semantic token with its position and classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticToken {
    pub range: TextRange,
    pub token_type: SemanticTokenType,
    pub modifiers: Option<SemanticTokenModifier>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum SemanticTokenModifier {
    Definition = 0,
    Readonly,
    Documentation,
}

/// Semantic token types supported by the language server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticTokenType {
    Keyword,
    String,
    Bool,
    Number,
    Function,
    Operator,
    Punctuation,
    Name,
    NameRef,
    Comment,
    Column,
    Type,
    Parameter,
    PositionalParam,
    Table,
    Schema,
}

impl TryFrom<LocationKind> for SemanticTokenType {
    type Error = LocationKind;

    fn try_from(kind: LocationKind) -> Result<Self, Self::Error> {
        match kind {
            LocationKind::Aggregate | LocationKind::Function | LocationKind::Procedure => {
                Ok(SemanticTokenType::Function)
            }
            LocationKind::Column => Ok(SemanticTokenType::Column),
            LocationKind::NamedArgParameter => Ok(SemanticTokenType::Parameter),
            LocationKind::Schema => Ok(SemanticTokenType::Schema),
            LocationKind::Sequence | LocationKind::Table | LocationKind::View => {
                Ok(SemanticTokenType::Table)
            }
            LocationKind::Type => Ok(SemanticTokenType::Type),
            LocationKind::CaseExpr
            | LocationKind::Channel
            | LocationKind::CommitBegin
            | LocationKind::CommitEnd
            | LocationKind::Cursor
            | LocationKind::Database
            | LocationKind::EventTrigger
            | LocationKind::Extension
            | LocationKind::Index
            | LocationKind::Policy
            | LocationKind::PreparedStatement
            | LocationKind::Role
            | LocationKind::Server
            | LocationKind::Tablespace
            | LocationKind::Trigger
            | LocationKind::Window => Err(kind),
        }
    }
}

fn token_type_for_node<T: AstNode>(db: &dyn Db, file: File, node: &T) -> Option<SemanticTokenType> {
    let offset = node.syntax().text_range().start();
    let location = goto_definition(db, file, offset).into_iter().next()?;

    SemanticTokenType::try_from(location.kind).ok()
}

#[derive(Default)]
struct SemanticTokenBuilder {
    tokens: Vec<SemanticToken>,
}

impl SemanticTokenBuilder {
    fn build(mut self) -> Vec<SemanticToken> {
        self.tokens
            .sort_by_key(|token| (token.range.start(), token.range.end()));
        self.tokens
    }

    fn push_keyword(&mut self, syntax_element: SyntaxElement) {
        self.push_token(syntax_element, SemanticTokenType::Keyword);
    }

    fn push_type(&mut self, syntax_element: SyntaxElement) {
        self.push_token(syntax_element, SemanticTokenType::Type);
    }

    fn push_token(&mut self, syntax_element: SyntaxElement, token_type: SemanticTokenType) {
        self.tokens.push(SemanticToken {
            range: syntax_element.text_range(),
            token_type,
            modifiers: None,
        });
    }
}

#[salsa::tracked]
pub fn semantic_tokens(
    db: &dyn Db,
    file: File,
    range_to_highlight: Option<TextRange>,
) -> Vec<SemanticToken> {
    let parse = parse(db, file);
    let tree = parse.tree();
    let root = tree.syntax();

    // Determine the root based on the given range.
    let (root, range_to_highlight) = {
        let source_file = root;
        match range_to_highlight {
            Some(range) => {
                let node = match source_file.covering_element(range) {
                    NodeOrToken::Node(it) => it,
                    NodeOrToken::Token(it) => it.parent().unwrap_or_else(|| source_file.clone()),
                };
                (node, range)
            }
            None => (source_file.clone(), source_file.text_range()),
        }
    };

    let mut out = SemanticTokenBuilder::default();

    // Taken from: https://github.com/rust-lang/rust-analyzer/blob/2efc80078029894eec0699f62ec8d5c1a56af763/crates/ide/src/syntax_highlighting.rs#L267C21-L267C21
    let preorder = root.preorder_with_tokens();
    for event in preorder {
        use rowan::WalkEvent::{Enter, Leave};

        let range = match &event {
            Enter(it) | Leave(it) => it.text_range(),
        };

        // Element outside of the viewport, no need to highlight
        if range_to_highlight.intersect(range).is_none() {
            continue;
        }

        match event {
            Enter(NodeOrToken::Node(node)) => {
                if let Some(name) = ast::Name::cast(node.clone())
                    && let Some(token_type) = token_type_for_node(db, file, &name)
                {
                    out.push_token(name.syntax().clone().into(), token_type);
                }

                if let Some(name_ref) = ast::NameRef::cast(node.clone())
                    && let Some(token_type) = token_type_for_node(db, file, &name_ref)
                {
                    out.push_token(name_ref.syntax().clone().into(), token_type);
                }

                if let Some(ty) = ast::Type::cast(node.clone()) {
                    highlight_type(&mut out, ty);
                }

                if let Some(mode) = ast::ParamMode::cast(node.clone()) {
                    highlight_param_mode(&mut out, mode);
                }

                // Cleanup various operators that the textmate grammar
                // highlights spuriously. These are for the select cases that
                // aren't easily handled in the textmate grammar.
                if let Some(like_clause) = ast::LikeClause::cast(node.clone())
                    && let Some(token) = like_clause.like_token()
                {
                    out.push_keyword(token.into());
                }
                if let Some(not_null_constraint) = ast::NotNullConstraint::cast(node.clone())
                    && let Some(token) = not_null_constraint.not_token()
                {
                    out.push_keyword(token.into());
                }
                if let Some(partition_for_values_in) = ast::PartitionForValuesIn::cast(node.clone())
                    && let Some(token) = partition_for_values_in.in_token()
                {
                    out.push_keyword(token.into());
                }
            }
            Enter(NodeOrToken::Token(token)) => {
                if token.kind() == SyntaxKind::WHITESPACE {
                    continue;
                }
                if token.kind() == SyntaxKind::POSITIONAL_PARAM {
                    out.push_token(token.into(), SemanticTokenType::PositionalParam);
                }
            }
            Leave(_) => {}
        }
    }

    out.build()
}

#[cfg(test)]
mod test {
    use crate::db::{Database, File};
    use insta::assert_snapshot;
    use std::fmt::Write;

    fn semantic_tokens(sql: &str) -> String {
        let db = Database::default();
        let file = File::new(&db, sql.to_string().into());
        let tokens = super::semantic_tokens(&db, file, None);

        let mut result = String::new();
        for token in tokens {
            let start: usize = token.range.start().into();
            let end: usize = token.range.end().into();
            let token_text = &sql[start..end];
            // TODO: once we get modfifiers, we'll need to update this
            let modifiers_text = "";
            writeln!(
                result,
                "{:?} @ {}..{}: {:?}{}",
                token_text, start, end, token.token_type, modifiers_text
            )
            .unwrap();
        }
        result
    }

    #[test]
    fn create_function_misc_params() {
        assert_snapshot!(semantic_tokens(
            "
create function add(
  in a int = 1,
  inout b text default 'x',
  in out c varchar(10)[],
  variadic d int[]
) returns int
as 'select $1 + $2'
language sql;
",
        ), @r#"
        "add" @ 17..20: Function
        "in" @ 24..26: Keyword
        "a" @ 27..28: Parameter
        "int" @ 29..32: Type
        "inout" @ 40..45: Keyword
        "b" @ 46..47: Parameter
        "text" @ 48..52: Type
        "in" @ 68..70: Keyword
        "out" @ 71..74: Keyword
        "c" @ 75..76: Parameter
        "varchar" @ 77..84: Type
        "variadic" @ 94..102: Keyword
        "d" @ 103..104: Parameter
        "int" @ 105..108: Type
        "int" @ 121..124: Type
        "#);
    }

    #[test]
    fn create_function_param_mode_type() {
        assert_snapshot!(semantic_tokens(
            "
create function f(int8 in int8)
returns void
as '' language sql;
",
        ), @r#"
        "f" @ 17..18: Function
        "int8" @ 19..23: Parameter
        "in" @ 24..26: Keyword
        "int8" @ 27..31: Type
        "void" @ 41..45: Type
        "#);
    }

    #[test]
    fn create_function_percent_type() {
        assert_snapshot!(semantic_tokens(
            "
create function f(a t.c%type) 
returns t.b%type 
as '' language plpgsql;
",
        ), @r#"
        "f" @ 17..18: Function
        "a" @ 19..20: Parameter
        "#);
    }

    #[test]
    fn select_keywords() {
        assert_snapshot!(semantic_tokens("
select 1 and, 2 select;
"), @r#"
        "and" @ 10..13: Column
        "select" @ 17..23: Column
        "#)
    }

    #[test]
    fn positional_param() {
        assert_snapshot!(semantic_tokens("
select $1, $2;
"), @r#"
        "$1" @ 8..10: PositionalParam
        "$2" @ 12..14: PositionalParam
        "#)
    }

    #[test]
    fn insert_column_list() {
        assert_snapshot!(semantic_tokens(
            "
create table products (product_no bigint, name text, price text);
insert into products (product_no, name, price) values
    (1, 'Cheese', 9.99),
    (2, 'Bread', 1.99),
    (3, 'Milk', 2.99);
",
        ), @r#"
        "products" @ 14..22: Table
        "product_no" @ 24..34: Column
        "bigint" @ 35..41: Type
        "name" @ 43..47: Column
        "text" @ 48..52: Type
        "price" @ 54..59: Column
        "text" @ 60..64: Type
        "products" @ 79..87: Table
        "product_no" @ 89..99: Column
        "name" @ 101..105: Column
        "price" @ 107..112: Column
        "#)
    }

    #[test]
    fn from_alias_column_types() {
        assert_snapshot!(semantic_tokens(
            "
select *
from f as t(a int, b jsonb, c text, x int, ca char(5)[], ia int[][], r text);
",
        ), @r#"
        "t" @ 20..21: Table
        "a" @ 22..23: Column
        "int" @ 24..27: Type
        "b" @ 29..30: Column
        "jsonb" @ 31..36: Type
        "c" @ 38..39: Column
        "text" @ 40..44: Type
        "x" @ 46..47: Column
        "int" @ 48..51: Type
        "ca" @ 53..55: Column
        "char" @ 56..60: Type
        "ia" @ 67..69: Column
        "int" @ 70..73: Type
        "r" @ 79..80: Column
        "text" @ 81..85: Type
        "#);
    }

    #[test]
    fn json_table_columns() {
        assert_snapshot!(semantic_tokens(
            "
select *
from my_films,
json_table(
  js,
  '$.favorites[*]' columns (
    id for ordinality,
    kind text path '$.kind'
  )
) as jt;
",
        ), @r#"
        "id" @ 76..78: Column
        "kind" @ 99..103: Column
        "text" @ 104..108: Type
        "jt" @ 132..134: Table
        "#);
    }

    #[test]
    fn xml_table_columns() {
        assert_snapshot!(semantic_tokens(
            "
select *
from xmltable(
  '/root/item'
  passing xmlparse(document '<root><item id=\"1\"/></root>')
  columns
    row_num for ordinality,
    item_id integer path '@id'
);
",
        ), @r#"
        "row_num" @ 113..120: Column
        "item_id" @ 141..148: Column
        "integer" @ 149..156: Type
        "#);
    }

    #[test]
    fn cast_types() {
        assert_snapshot!(semantic_tokens(
            "
select '1'::jsonb, '2'::json, cast(1 as integer), cast(1 as int4[][]), cast(1 as varchar(10));
",
        ), @r#"
        "jsonb" @ 13..18: Type
        "json" @ 25..29: Type
        "integer" @ 41..48: Type
        "int4" @ 61..65: Type
        "varchar" @ 82..89: Type
        "#);
    }

    #[test]
    fn cast_double() {
        assert_snapshot!(semantic_tokens(
            "
select '1'::double precision;
",
        ), @r#"
        "double" @ 13..19: Type
        "precision" @ 20..29: Type
        "#);
    }

    #[test]
    fn create_table_temporal_primary_key_column_types() {
        assert_snapshot!(semantic_tokens(
            "
-- temporal_primary_key
CREATE TABLE addresses (
    id int8 generated BY DEFAULT AS IDENTITY,
    valid_range tstzrange NOT NULL DEFAULT tstzrange(now(), 'infinity', '[)'),
    recipient text NOT NULL,
    PRIMARY KEY (id, valid_range WITHOUT OVERLAPS)
);
",
        ), @r#"
        "addresses" @ 38..47: Table
        "id" @ 54..56: Column
        "int8" @ 57..61: Type
        "valid_range" @ 100..111: Column
        "tstzrange" @ 112..121: Type
        "NOT" @ 122..125: Keyword
        "tstzrange" @ 139..148: Function
        "now" @ 149..152: Function
        "recipient" @ 179..188: Column
        "text" @ 189..193: Type
        "NOT" @ 194..197: Keyword
        "id" @ 221..223: Column
        "valid_range" @ 225..236: Column
        "#);
    }

    #[test]
    fn like_clause_keyword() {
        assert_snapshot!(semantic_tokens(
            "
create table products(a text);
create table test (
  like products
);
",
        ), @r#"
        "products" @ 14..22: Table
        "a" @ 23..24: Column
        "text" @ 25..29: Type
        "test" @ 45..49: Table
        "like" @ 54..58: Keyword
        "products" @ 59..67: Table
        "#)
    }

    #[test]
    fn partition_for_values_in_keywords() {
        assert_snapshot!(semantic_tokens(
            "
create table t(a int);
create table t_1 partition of t for values in (1);
",
        ), @r#"
        "t" @ 14..15: Table
        "a" @ 16..17: Column
        "int" @ 18..21: Type
        "t_1" @ 37..40: Table
        "t" @ 54..55: Table
        "in" @ 67..69: Keyword
        "#)
    }

    #[test]
    fn positional_param_and_cast_type() {
        assert_snapshot!(semantic_tokens(
            "
select $2::jsonb;
",
        ), @r#"
        "$2" @ 8..10: PositionalParam
        "jsonb" @ 12..17: Type
        "#);
    }

    #[test]
    fn select_target_column() {
        assert_snapshot!(semantic_tokens(
            "
create table t(a int, b text);
select a, b from t;
",
        ), @r#"
        "t" @ 14..15: Table
        "a" @ 16..17: Column
        "int" @ 18..21: Type
        "b" @ 23..24: Column
        "text" @ 25..29: Type
        "a" @ 39..40: Column
        "b" @ 42..43: Column
        "t" @ 49..50: Table
        "#);
    }

    #[test]
    fn select_target_qualified_column() {
        assert_snapshot!(semantic_tokens(
            "
create table t(a int);
select t.a from t;
",
        ), @r#"
        "t" @ 14..15: Table
        "a" @ 16..17: Column
        "int" @ 18..21: Type
        "t" @ 31..32: Table
        "a" @ 33..34: Column
        "t" @ 40..41: Table
        "#);
    }

    #[test]
    fn select_target_function_call() {
        assert_snapshot!(semantic_tokens(
            "
create function f() returns int as 'select 1' language sql;
select f();
",
        ), @r#"
        "f" @ 17..18: Function
        "int" @ 29..32: Type
        "f" @ 68..69: Function
        "#);
    }

    #[test]
    fn select_function_arg_and_qualified_column() {
        assert_snapshot!(semantic_tokens(
            "
create table t(a int);
create function b(t) returns int as 'select 1' language sql;
select b(t), t.b from t;
",
        ), @r#"
        "t" @ 14..15: Table
        "a" @ 16..17: Column
        "int" @ 18..21: Type
        "b" @ 40..41: Function
        "t" @ 42..43: Type
        "int" @ 53..56: Type
        "b" @ 92..93: Function
        "t" @ 94..95: Table
        "t" @ 98..99: Table
        "b" @ 100..101: Function
        "t" @ 107..108: Table
        "#);
    }

    #[test]
    fn policy_field_style_function_call() {
        assert_snapshot!(semantic_tokens(
            "
create table t(c int);
create function x(t) returns int as 'select 1' language sql;
create policy p on t
  with check (t.x > 0 and t.c > 0);
",
        ), @r#"
        "t" @ 14..15: Table
        "c" @ 16..17: Column
        "int" @ 18..21: Type
        "x" @ 40..41: Function
        "t" @ 42..43: Type
        "int" @ 53..56: Type
        "t" @ 104..105: Table
        "t" @ 120..121: Table
        "x" @ 122..123: Function
        "t" @ 132..133: Table
        "c" @ 134..135: Column
        "#);
    }

    #[test]
    fn with_cte_name() {
        assert_snapshot!(semantic_tokens(
            "
with t as (
  select 1
)
select * from t;
",
        ), @r#"
        "t" @ 6..7: Table
        "t" @ 40..41: Table
        "#);
    }

    #[test]
    fn select_target_schema_qualified() {
        assert_snapshot!(semantic_tokens(
            "
create schema s;
create table s.t(a int);
select s.t.a from s.t;
",
        ), @r#"
        "s" @ 15..16: Schema
        "s" @ 31..32: Schema
        "t" @ 33..34: Table
        "a" @ 35..36: Column
        "int" @ 37..40: Type
        "s" @ 50..51: Schema
        "t" @ 52..53: Table
        "a" @ 54..55: Column
        "s" @ 61..62: Schema
        "t" @ 63..64: Table
        "#);
    }
}
