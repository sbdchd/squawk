#[cfg(test)]
use crate::violations::{RuleViolation, RuleViolationKind};

#[cfg(test)]
pub fn violations_to_kinds(violations: Vec<RuleViolation>) -> Vec<RuleViolationKind> {
    violations.into_iter().map(|v| v.kind).collect::<Vec<_>>()
}
