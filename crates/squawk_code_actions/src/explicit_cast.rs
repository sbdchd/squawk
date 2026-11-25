use std::borrow::Cow;

use squawk_syntax::ast::{self, AstNode, CastKind};

pub(crate) fn explicit_cast_type(cast: &ast::CastExpr) -> Option<Cow<'static, str>> {
    let ty = cast.ty()?;
    let type_text = || Cow::Owned(ty.syntax().to_string());
    if cast.kind() != Some(CastKind::TypeLiteral) {
        return Some(type_text());
    }
    let text = match &ty {
        ast::Type::CharacterType(character) if character.arg_list().is_none() => {
            Cow::Borrowed("bpchar")
        }
        ast::Type::BitType(bit) if bit.arg_list().is_none() => Cow::Borrowed(r#""bit""#),
        ast::Type::ArrayType(_)
        | ast::Type::BitType(_)
        | ast::Type::BitVaryingType(_)
        | ast::Type::CharacterType(_)
        | ast::Type::DoubleType(_)
        | ast::Type::IntervalType(_)
        | ast::Type::PathType(_)
        | ast::Type::SetofType(_)
        | ast::Type::TimeType(_)
        | ast::Type::TimestampType(_)
        | ast::Type::VarcharType(_) => type_text(),
    };
    Some(text)
}
