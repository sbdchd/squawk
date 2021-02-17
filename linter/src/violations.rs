use crate::RULES;
use serde::Serialize;
pub use squawk_parser::ast::Span;

#[derive(Debug, PartialEq, Clone, Serialize, Hash, Eq)]
pub enum RuleViolationKind {
    RequireConcurrentIndexCreation,
    ConstraintMissingNotValid,
    AddingFieldWithDefault,
    AddingForeignKeyConstraint,
    ChangingColumnType,
    AddingNotNullableField,
    AddingSerialPrimaryKeyField,
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
        let rule = RULES
            .iter()
            .find(|rule| rule.name == *self)
            .expect("We should always find ourself");

        write!(f, "{}", rule.id)
    }
}

impl std::convert::TryFrom<&str> for RuleViolationKind {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        RULES
            .iter()
            .find(|rule| rule.id == s)
            .map(|rule| rule.name.clone())
            .ok_or(())
    }
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
    #[must_use]
    pub fn new(
        kind: RuleViolationKind,
        span: Span,
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
            span,
            messages,
        }
    }
}
