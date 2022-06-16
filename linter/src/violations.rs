use std::str::FromStr;

use crate::RULES;
use serde::{Deserialize, Serialize};
pub use squawk_parser::ast::Span;

use ::semver::{BuildMetadata, Prerelease, Version, VersionReq};

#[must_use]
pub fn default_pg_version() -> Version {
    Version {
        major: 9,
        minor: 4,
        patch: 0,
        pre: Prerelease::EMPTY,
        build: BuildMetadata::EMPTY,
    }
}

/// # Panics
///
/// Will panic if parse is passed bad value: <https://docs.rs/semver/0.10.0/semver/struct.VersionReq.html#errors>
#[must_use]
pub fn ok_non_null_pg_version_req() -> VersionReq {
    match VersionReq::parse("11.0.0") {
        Ok(version) => version,
        Err(e) => panic!("There was a problem parsing: {}", e),
    }
}

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
    #[serde(rename = "prefer-text-field")]
    PreferTextField,
    #[serde(rename = "prefer-robust-stmts")]
    PreferRobustStmts,
    #[serde(rename = "ban-char-field")]
    BanCharField,
    #[serde(rename = "ban-drop-column")]
    BanDropColumn,
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
