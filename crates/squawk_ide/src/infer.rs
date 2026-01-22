use std::fmt;

use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Type {
    Integer,
    Numeric,
    Text,
    Bit,
    Boolean,
    Unknown,
    Record,
    Array(Box<Type>),
    Other(String),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Integer => write!(f, "integer"),
            Type::Numeric => write!(f, "numeric"),
            Type::Text => write!(f, "text"),
            Type::Bit => write!(f, "bit"),
            Type::Boolean => write!(f, "boolean"),
            Type::Unknown => write!(f, "unknown"),
            Type::Record => write!(f, "record"),
            Type::Array(inner) => write!(f, "{inner}[]"),
            Type::Other(s) => write!(f, "{s}"),
        }
    }
}

pub(crate) fn infer_type_from_expr(expr: &ast::Expr) -> Option<Type> {
    match expr {
        ast::Expr::CastExpr(cast_expr) => infer_type_from_ty(&cast_expr.ty()?),
        ast::Expr::ArrayExpr(array_expr) => {
            let first_elem = array_expr.exprs().next()?;
            let elem_ty = infer_type_from_expr(&first_elem)?;
            Some(Type::Array(Box::new(elem_ty)))
        }
        ast::Expr::BinExpr(_bin_expr) => todo!(),
        ast::Expr::Literal(literal) => infer_type_from_literal(literal),
        ast::Expr::ParenExpr(paren) => paren.expr().and_then(|e| infer_type_from_expr(&e)),
        ast::Expr::TupleExpr(_) => Some(Type::Record),
        _ => None,
    }
}

pub(crate) fn infer_type_from_ty(ty: &ast::Type) -> Option<Type> {
    match ty {
        ast::Type::CharType(_) => Some(Type::Text),
        ast::Type::BitType(_) => Some(Type::Bit),
        ast::Type::PathType(path_type) => {
            let name = path_type.path()?.segment()?.name_ref()?;
            Some(Type::Other(name.syntax().text().to_string()))
        }
        _ => None,
    }
}

fn infer_type_from_literal(literal: &ast::Literal) -> Option<Type> {
    let token = literal.syntax().first_token()?;
    match token.kind() {
        SyntaxKind::INT_NUMBER => Some(Type::Integer),
        SyntaxKind::FLOAT_NUMBER => Some(Type::Numeric),
        SyntaxKind::STRING
        | SyntaxKind::DOLLAR_QUOTED_STRING
        | SyntaxKind::ESC_STRING
        | SyntaxKind::UNICODE_ESC_STRING => Some(Type::Text),
        SyntaxKind::BIT_STRING | SyntaxKind::BYTE_STRING => Some(Type::Bit),
        SyntaxKind::TRUE_KW | SyntaxKind::FALSE_KW => Some(Type::Boolean),
        SyntaxKind::NULL_KW => Some(Type::Unknown),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    fn infer(sql: &str) -> String {
        let parse = ast::SourceFile::parse(sql);
        for stmt in parse.tree().stmts() {
            match stmt {
                ast::Stmt::Select(select) => {
                    let select_clause = select.select_clause().expect("expected select clause");
                    let target_list = select_clause.target_list().expect("expected target list");

                    if let Some(target) = target_list.targets().next() {
                        let expr = target.expr().expect("expected expr");
                        let ty = infer_type_from_expr(&expr).expect("expected type");
                        return format!("{ty}");
                    }
                }
                _ => unreachable!("unexpected stmt type"),
            }
        }
        unreachable!("should always have at least one target")
    }

    #[test]
    fn integer_literal() {
        assert_snapshot!(infer("select 1"), @"integer");
    }

    #[test]
    fn float_literal() {
        assert_snapshot!(infer("select 1.5"), @"numeric");
    }

    #[test]
    fn string_literal() {
        assert_snapshot!(infer("select 'hello'"), @"text");
    }

    #[test]
    fn dollar_quoted_string() {
        assert_snapshot!(infer("select $$hello$$"), @"text");
    }

    #[test]
    fn escape_string() {
        assert_snapshot!(infer("select E'hello'"), @"text");
    }

    #[test]
    fn boolean_true() {
        assert_snapshot!(infer("select true"), @"boolean");
    }

    #[test]
    fn boolean_false() {
        assert_snapshot!(infer("select false"), @"boolean");
    }

    #[test]
    fn null_literal() {
        assert_snapshot!(infer("select null"), @"unknown");
    }

    #[test]
    fn cast_expr() {
        assert_snapshot!(infer("select 1::text"), @"text");
    }

    #[test]
    fn cast_expr_varchar() {
        assert_snapshot!(infer("select 1::varchar(255)"), @"text");
    }

    #[test]
    fn bit_string() {
        assert_snapshot!(infer("select b'100'"), @"bit");
    }

    #[test]
    fn bit_varying() {
        assert_snapshot!(infer("select b'100'::bit varying"), @"bit");
    }

    #[test]
    fn array() {
        assert_snapshot!(infer("select array['foo', 'bar']"), @"text[]");
    }

    #[test]
    fn record() {
        assert_snapshot!(infer("select (1, 2)"), @"record");
    }

    #[test]
    fn paren_expr() {
        assert_snapshot!(infer("select (1)"), @"integer");
    }

    #[test]
    fn nested_paren_expr() {
        assert_snapshot!(infer("select ((1.5))"), @"numeric");
    }
}
