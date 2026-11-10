use smallvec::SmallVec;
use squawk_syntax::ast;

use crate::arena::IdxRange;
use crate::interner::Name;

#[derive(Clone, Copy)]
pub(crate) enum PartitionKey {
    Columns(IdxRange<Name>),
    Expressions,
}

#[derive(Clone)]
pub(crate) enum PartitionBound {
    Default,
    List(SmallVec<[ListValue; 4]>),
    Range {
        from: SmallVec<[RangeValue; 2]>,
        to: SmallVec<[RangeValue; 2]>,
    },
    Hash {
        modulus: u64,
        remainder: u64,
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ListValue {
    Int(i128),
    Bool(bool),
    Text(Name),
    Null,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum RangeValue {
    MinValue,
    Int(i128),
    MaxValue,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum BoundKeyType {
    Int,
    Bool,
    Text,
}

pub(crate) fn range_values(
    exprs: ast::AstChildren<ast::Expr>,
    key_count: usize,
) -> Option<SmallVec<[RangeValue; 2]>> {
    let mut values = SmallVec::with_capacity(key_count);
    for expr in exprs {
        values.push(range_value(&expr)?);
    }
    if values.len() != key_count {
        return None;
    }
    Some(values)
}

pub(crate) fn range_value(expr: &ast::Expr) -> Option<RangeValue> {
    match expr {
        ast::Expr::NameRef(name_ref) => match name_ref.range_bound_keyword()? {
            ast::RangeBoundKeyword::MinValue => Some(RangeValue::MinValue),
            ast::RangeBoundKeyword::MaxValue => Some(RangeValue::MaxValue),
        },
        ast::Expr::PrefixExpr(prefix) => {
            let Some(ast::PrefixOp::Minus(_)) = prefix.op() else {
                return None;
            };
            match range_value(&prefix.expr()?)? {
                RangeValue::Int(value) => Some(RangeValue::Int(-value)),
                RangeValue::MinValue | RangeValue::MaxValue => None,
            }
        }
        ast::Expr::Literal(literal) => match literal.kind()? {
            ast::LitKind::IntNumber(_) => {
                Some(RangeValue::Int(i128::from(literal.integer_value_u64()?)))
            }
            ast::LitKind::String(token) => {
                let text = token.text();
                let value = text.get(1..text.len() - 1)?.trim().parse().ok()?;
                Some(RangeValue::Int(value))
            }
            _ => None,
        },
        _ => None,
    }
}

pub(crate) fn partition_bounds_overlap(new: &PartitionBound, existing: &PartitionBound) -> bool {
    match (new, existing) {
        (PartitionBound::Default, PartitionBound::Default) => true,
        (PartitionBound::List(new), PartitionBound::List(existing)) => {
            new.iter().any(|value| existing.contains(value))
        }
        (
            PartitionBound::Range { from, to },
            PartitionBound::Range {
                from: existing_from,
                to: existing_to,
            },
        ) => from < existing_to && existing_from < to,
        (
            PartitionBound::Hash { modulus, remainder },
            PartitionBound::Hash {
                modulus: existing_modulus,
                remainder: existing_remainder,
            },
        ) => {
            let (small, small_remainder, large_remainder) = if modulus <= existing_modulus {
                (modulus, remainder, existing_remainder)
            } else {
                (existing_modulus, existing_remainder, remainder)
            };
            large_remainder % small == *small_remainder
        }
        (
            PartitionBound::Default
            | PartitionBound::List(_)
            | PartitionBound::Range { .. }
            | PartitionBound::Hash { .. },
            _,
        ) => false,
    }
}
