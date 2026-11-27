use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BuiltinTy {
    Any,
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
    pub(crate) fn name(self) -> &'static str {
        match self {
            BuiltinTy::Any => "any",
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
