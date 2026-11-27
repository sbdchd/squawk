use std::fmt;

use squawk_syntax::ast;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinTy {
    Any,
    Bit,
    Bool,
    Bytea,
    Cstring,
    Date,
    EventTrigger,
    Int4,
    Int8,
    Internal,
    Json,
    Jsonb,
    Name,
    Numeric,
    Oid,
    Record,
    Text,
    Time,
    Timestamp,
    Timestamptz,
    Timetz,
    Trigger,
    Unknown,
    Void,
    Xml,
}

impl BuiltinTy {
    pub fn from(lit: &ast::Literal) -> Option<BuiltinTy> {
        let ty = match lit.kind()? {
            ast::LitKind::IntNumber(_) => match lit.integer_value_u64() {
                Some(value) if i32::try_from(value).is_ok() => BuiltinTy::Int4,
                Some(value) if i64::try_from(value).is_ok() => BuiltinTy::Int8,
                Some(_) | None => BuiltinTy::Numeric,
            },
            ast::LitKind::NumericNumber(_) => BuiltinTy::Numeric,
            ast::LitKind::String(_)
            | ast::LitKind::DollarQuotedString(_)
            | ast::LitKind::EscString(_)
            | ast::LitKind::NationalString(_)
            | ast::LitKind::UnicodeEscString(_) => BuiltinTy::Text,
            ast::LitKind::BitString(_) | ast::LitKind::ByteString(_) => BuiltinTy::Bit,
            ast::LitKind::True(_) | ast::LitKind::False(_) => BuiltinTy::Bool,
            ast::LitKind::Default(_)
            | ast::LitKind::Null(_)
            | ast::LitKind::Off(_)
            | ast::LitKind::On(_)
            | ast::LitKind::PositionalParam(_) => return None,
        };
        Some(ty)
    }

    pub fn name(self) -> &'static str {
        match self {
            BuiltinTy::Any => "any",
            BuiltinTy::Bit => "bit",
            BuiltinTy::Bool => "bool",
            BuiltinTy::Bytea => "bytea",
            BuiltinTy::Cstring => "cstring",
            BuiltinTy::Date => "date",
            BuiltinTy::EventTrigger => "event_trigger",
            BuiltinTy::Int4 => "int4",
            BuiltinTy::Int8 => "int8",
            BuiltinTy::Internal => "internal",
            BuiltinTy::Json => "json",
            BuiltinTy::Jsonb => "jsonb",
            BuiltinTy::Name => "name",
            BuiltinTy::Numeric => "numeric",
            BuiltinTy::Oid => "oid",
            BuiltinTy::Record => "record",
            BuiltinTy::Text => "text",
            BuiltinTy::Time => "time",
            BuiltinTy::Timestamp => "timestamp",
            BuiltinTy::Timestamptz => "timestamptz",
            BuiltinTy::Timetz => "timetz",
            BuiltinTy::Trigger => "trigger",
            BuiltinTy::Unknown => "unknown",
            BuiltinTy::Void => "void",
            BuiltinTy::Xml => "xml",
        }
    }
}

impl fmt::Display for BuiltinTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}
