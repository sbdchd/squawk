use std::fmt;

use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Type {
    Array(Box<Type>),
    Bigint,
    Bit,
    Boolean,
    Integer,
    Numeric,
    Other(String),
    Record,
    Text,
    Unknown,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Array(inner) => write!(f, "{inner}[]"),
            Type::Bigint => write!(f, "bigint"),
            Type::Bit => write!(f, "bit"),
            Type::Boolean => write!(f, "boolean"),
            Type::Integer => write!(f, "integer"),
            Type::Numeric => write!(f, "numeric"),
            Type::Other(s) => write!(f, "{s}"),
            Type::Record => write!(f, "record"),
            Type::Text => write!(f, "text"),
            Type::Unknown => write!(f, "unknown"),
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
        // TODO: we need to infer both lhs and rhs, BUT we also need to support
        // looking up the operator since there's operator overloading
        ast::Expr::BinExpr(_bin_expr) => None,
        ast::Expr::Collate(collate) => infer_type_from_expr(&collate.expr()?),
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
            let name = path_type.path_ref()?.segment()?.name_ref()?;
            Some(Type::Other(name.syntax().text().to_string()))
        }
        _ => None,
    }
}

fn infer_int_type(text: &str) -> Type {
    let cleaned = text.replace('_', "");
    let lower = cleaned.to_ascii_lowercase();
    let (digits, radix) = if let Some(rest) = lower.strip_prefix("0x") {
        (rest, 16)
    } else if let Some(rest) = lower.strip_prefix("0o") {
        (rest, 8)
    } else if let Some(rest) = lower.strip_prefix("0b") {
        (rest, 2)
    } else {
        (lower.as_str(), 10)
    };
    match u64::from_str_radix(digits, radix) {
        Ok(n) if n <= i32::MAX as u64 => Type::Integer,
        Ok(n) if n <= i64::MAX as u64 => Type::Bigint,
        _ => Type::Numeric,
    }
}

pub(crate) fn infer_type_from_literal(literal: &ast::Literal) -> Option<Type> {
    let token = literal.syntax().first_token()?;
    match token.kind() {
        SyntaxKind::INT_NUMBER => Some(infer_int_type(token.text())),
        SyntaxKind::NUMERIC_NUMBER => Some(Type::Numeric),
        // TODO: this isn't necessarily text, e.g., select 1 + '1';
        // We need to look at the context of the string's usage to be sure.
        SyntaxKind::STRING
        | SyntaxKind::DOLLAR_QUOTED_STRING
        | SyntaxKind::ESC_STRING
        | SyntaxKind::NATIONAL_STRING
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

    #[must_use]
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
    fn integer_max() {
        assert_snapshot!(infer("select 2147483647"), @"integer");
    }

    #[test]
    fn bigint_just_over_int_max() {
        assert_snapshot!(infer("select 2147483648"), @"bigint");
    }

    #[test]
    fn bigint_literal() {
        assert_snapshot!(infer("select 100000000000000"), @"bigint");
    }

    #[test]
    fn numeric_over_bigint() {
        assert_snapshot!(infer("select 100000000000000000000000"), @"numeric");
    }

    #[test]
    fn hex_literal() {
        assert_snapshot!(infer("select 0xFF"), @"integer");
    }

    #[test]
    fn hex_literal_bigint() {
        assert_snapshot!(infer("select 0xFFFFFFFFFF"), @"bigint");
    }

    #[test]
    fn octal_literal() {
        assert_snapshot!(infer("select 0o17"), @"integer");
    }

    #[test]
    fn binary_literal() {
        assert_snapshot!(infer("select 0b1001"), @"integer");
    }

    #[test]
    fn integer_with_underscores() {
        assert_snapshot!(infer("select 1_000_000"), @"integer");
    }

    #[test]
    fn float_literal() {
        assert_snapshot!(infer("select 1.5"), @"numeric");
    }

    #[test]
    fn float_with_zero_decimal() {
        assert_snapshot!(infer("select 10000.0"), @"numeric");
    }

    #[test]
    fn exponent_literal() {
        assert_snapshot!(infer("select 1e5"), @"numeric");
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
    fn unicode_escape_string() {
        assert_snapshot!(infer("select U&' \' UESCAPE '!'"), @"text");
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
    fn byte_string() {
        assert_snapshot!(infer("select x'FF'"), @"bit");
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
