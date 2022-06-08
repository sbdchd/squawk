use std::str::FromStr;

use crate::RULES;
use serde::{Deserialize, Serialize};
pub use squawk_parser::ast::Span;

#[derive(Debug, PartialEq, Clone, Serialize, Hash, Eq, Deserialize)]
pub enum RuleViolationKind {
    RequireConcurrentIndexCreation,
    RequireConcurrentIndexDeletion,
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
    BanDropColumn,
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

#[derive(Debug, Clone)]
pub struct UnknownRuleName {
    val: String,
}

impl std::fmt::Display for UnknownRuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid rule name {}", self.val)
    }
}

impl std::str::FromStr for RuleViolationKind {
    type Err = UnknownRuleName;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        RULES
            .iter()
            .find_map(|rule| {
                if rule.id == s {
                    Some(rule.name.clone())
                } else {
                    None
                }
            })
            .ok_or(UnknownRuleName { val: s.to_string() })
    }
}

impl std::convert::TryFrom<&str> for RuleViolationKind {
    type Error = UnknownRuleName;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        RuleViolationKind::from_str(s)
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
                .map_or_else(Vec::new, |x| x.messages.clone())
        });
        Self {
            kind,
            span,
            messages,
        }
    }
}
