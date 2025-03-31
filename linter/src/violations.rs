use std::str::FromStr;

use crate::RULES;
use serde::{Deserialize, Serialize};
pub use squawk_parser::ast::Span;

#[derive(Debug, PartialEq, Clone, Serialize, Hash, Eq, Deserialize)]
pub enum RuleViolationKind {
    #[serde(rename = "require-concurrent-index-creation")]
    RequireConcurrentIndexCreation,
    #[serde(rename = "require-concurrent-index-deletion")]
    RequireConcurrentIndexDeletion,
    #[serde(rename = "constraint-missing-not-valid")]
    ConstraintMissingNotValid,
    #[serde(rename = "adding-field-with-default")]
    AddingFieldWithDefault,
    #[serde(rename = "adding-foreign-key-constraint")]
    AddingForeignKeyConstraint,
    #[serde(rename = "changing-column-type")]
    ChangingColumnType,
    #[serde(rename = "adding-not-nullable-field")]
    AddingNotNullableField,
    #[serde(rename = "adding-serial-primary-key-field")]
    AddingSerialPrimaryKeyField,
    #[serde(rename = "renaming-column")]
    RenamingColumn,
    #[serde(rename = "renaming-table")]
    RenamingTable,
    #[serde(rename = "disallowed-unique-constraint")]
    DisallowedUniqueConstraint,
    #[serde(rename = "ban-drop-database")]
    BanDropDatabase,
    // when we can't parse a Postgres statement, we report this error.
    #[serde(rename = "invalid-statement")]
    InvalidStatement,
    #[serde(rename = "prefer-big-int")]
    PreferBigInt,
    #[serde(rename = "prefer-bigint-over-int")]
    PreferBigintOverInt,
    #[serde(rename = "prefer-bigint-over-smallint")]
    PreferBigintOverSmallint,
    #[serde(rename = "prefer-identity")]
    PreferIdentity,
    #[serde(rename = "prefer-robust-stmts")]
    PreferRobustStmts,
    #[serde(rename = "prefer-text-field")]
    PreferTextField,
    #[serde(rename = "prefer-timestamptz")]
    PreferTimestampTz,
    #[serde(rename = "ban-char-field")]
    BanCharField,
    #[serde(rename = "ban-drop-column")]
    BanDropColumn,
    #[serde(rename = "ban-drop-table")]
    BanDropTable,
    #[serde(rename = "ban-drop-not-null")]
    BanDropNotNull,
    #[serde(rename = "transaction-nesting")]
    TransactionNesting,
    #[serde(rename = "adding-required-field")]
    AddingRequiredField,
    #[serde(rename = "ban-concurrent-index-creation-in-transaction")]
    BanConcurrentIndexCreationInTransaction,
    #[serde(rename = "ban-create-domain-with-constraint")]
    BanCreateDomainWithConstraint,
    #[serde(rename = "ban-alter-domain-with-add-constraint")]
    BanAlterDomainWithAddConstraint,
    // generator::new-rule-above
}

impl std::fmt::Display for RuleViolationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_plain::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
        serde_plain::from_str(s).map_err(|_| UnknownRuleName { val: s.to_string() })
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
