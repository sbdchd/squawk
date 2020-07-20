use crate::RULES;
use serde::Serialize;
use squawk_parser::ast::RawStmt;

#[derive(Debug, PartialEq, Clone, Serialize, Hash, Eq)]
pub enum RuleViolationKind {
    RequireConcurrentIndexCreation,
    ConstraintMissingNotValid,
    AddingFieldWithDefault,
    ChangingColumnType,
    AddingNotNullableField,
    RenamingColumn,
    RenamingTable,
    DisallowedUniqueConstraint,
    BanDropDatabase,
    PreferTextField,
    PreferRobustStmts,
    BanCharField,
}

impl std::fmt::Display for RuleViolationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_value = match self {
            Self::RequireConcurrentIndexCreation => "require-concurrent-index-creation",
            Self::ConstraintMissingNotValid => "constraint-missing-not-valid",
            Self::AddingFieldWithDefault => "adding-field-with-default",
            Self::ChangingColumnType => "changing-column-type",
            Self::AddingNotNullableField => "adding-not-nullable-field",
            Self::RenamingColumn => "renaming-column",
            Self::RenamingTable => "renaming-table",
            Self::DisallowedUniqueConstraint => "disallowed-unique-constraint",
            Self::BanDropDatabase => "ban-drop-database",
            Self::PreferTextField => "prefer-text-field",
            Self::PreferRobustStmts => "prefer-robust-stmts",
            Self::BanCharField => "ban-char-filed",
        };
        write!(f, "{}", str_value)
    }
}

impl std::convert::TryFrom<&str> for RuleViolationKind {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "require-concurrent-index-creation" => Ok(Self::RequireConcurrentIndexCreation),
            "constraint-missing-not-valid" => Ok(Self::ConstraintMissingNotValid),
            "adding-field-with-default" => Ok(Self::AddingFieldWithDefault),
            "changing-column-type" => Ok(Self::ChangingColumnType),
            "adding-not-nullable-field" => Ok(Self::AddingNotNullableField),
            "renaming-column" => Ok(Self::RenamingColumn),
            "renaming-table" => Ok(Self::RenamingTable),
            "disallowed-unique-constraint" => Ok(Self::DisallowedUniqueConstraint),
            "ban-drop-database" => Ok(Self::BanDropDatabase),
            "prefer-text-field" => Ok(Self::PreferTextField),
            "prefer-robust-stmts" => Ok(Self::PreferRobustStmts),
            "ban-char-field" => Ok(Self::BanCharField),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Span {
    pub start: i32,
    pub len: Option<i32>,
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum ViolationMessage {
    Note(String),
    Help(String),
}

#[derive(Debug, PartialEq)]
pub struct RuleViolation {
    pub kind: RuleViolationKind,
    pub span: Span,
    pub messages: Vec<ViolationMessage>,
}

impl RuleViolation {
    pub fn new(
        kind: RuleViolationKind,
        node: &RawStmt,
        messages: Option<Vec<ViolationMessage>>,
    ) -> Self {
        let messages = messages.unwrap_or_else(|| {
            RULES
                .iter()
                .find(|r| r.name == kind)
                .unwrap()
                .messages
                .clone()
        });
        Self {
            kind,
            span: Span {
                start: node.stmt_location,
                len: node.stmt_len,
            },
            messages,
        }
    }
}
