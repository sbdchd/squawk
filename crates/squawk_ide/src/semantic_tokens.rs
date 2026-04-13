use rowan::{NodeOrToken, TextRange};
use salsa::Database as Db;
use squawk_syntax::{
    SyntaxElement, SyntaxKind,
    ast::{self, AstNode},
};

use crate::db::{File, parse};

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
        ast::Type::ArrayType(array_type) => {
            if let Some(ty) = array_type.ty() {
                highlight_type(out, ty);
            }
        }
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
        }
        ast::Type::ExprType(_) => (),
        ast::Type::IntervalType(interval_type) => {
            if let Some(token) = interval_type.interval_token() {
                out.push_type(token.into());
            }
        }
        ast::Type::PathType(path_type) => {
            if let Some(name_ref) = path_type
                .path()
                .and_then(|path| path.segment())
                .and_then(|ps| ps.name_ref())
            {
                out.push_type(name_ref.syntax().clone().into());
            }
        }
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
    Type,
    Parameter,
    PositionalParam,
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
                if let Some(target) = ast::Target::cast(node.clone())
                    && let Some(as_name) = target.as_name()
                    && let Some(name) = as_name.name()
                {
                    out.push_token(name.syntax().clone().into(), SemanticTokenType::Name);
                };

                if let Some(alias) = ast::Alias::cast(node.clone())
                    && let Some(column_list) = alias.column_list()
                {
                    for column in column_list.columns() {
                        if let Some(ty) = column.ty() {
                            highlight_type(&mut out, ty);
                        }
                    }
                }

                if let Some(cast_expr) = ast::CastExpr::cast(node.clone())
                    && let Some(ty) = cast_expr.ty()
                {
                    highlight_type(&mut out, ty);
                }

                if let Some(create_function) = ast::CreateFunction::cast(node) {
                    if let Some(param_list) = create_function.param_list() {
                        for param in param_list.params() {
                            if let Some(mode) = param.mode() {
                                highlight_param_mode(&mut out, mode);
                            }
                            if let Some(name) = param.name() {
                                out.push_token(
                                    name.syntax().clone().into(),
                                    SemanticTokenType::Parameter,
                                );
                            }
                            if let Some(ty) = param.ty() {
                                highlight_type(&mut out, ty);
                            }
                        }
                    }

                    if let Some(ret_type) = create_function.ret_type() {
                        if let Some(ty) = ret_type.ty() {
                            highlight_type(&mut out, ty);
                        }
                        if let Some(table_arg_list) = ret_type.table_arg_list() {
                            for arg in table_arg_list.args() {
                                if let ast::TableArg::Column(column) = arg
                                    && let Some(ty) = column.ty()
                                {
                                    highlight_type(&mut out, ty);
                                }
                            }
                        }
                    }
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
        ), @r#""a" @ 19..20: Parameter"#);
    }

    #[test]
    fn select_keywords() {
        assert_snapshot!(semantic_tokens("
select 1 and, 2 select;
"), @r#"
        "and" @ 10..13: Name
        "select" @ 17..23: Name
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
    fn from_alias_column_types() {
        assert_snapshot!(semantic_tokens(
            "
select *
from f as t(a int, b jsonb, c text, x int, ca char(5)[], ia int[][], r jbpop);
",
        ), @r#"
        "int" @ 24..27: Type
        "jsonb" @ 31..36: Type
        "text" @ 40..44: Type
        "int" @ 48..51: Type
        "char" @ 56..60: Type
        "int" @ 70..73: Type
        "jbpop" @ 81..86: Type
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
}
