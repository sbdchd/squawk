#[cfg(test)]
use crate::violations::{RuleViolation, RuleViolationKind};

#[cfg(test)]
pub fn violations_to_kinds(violations: &[RuleViolation]) -> Vec<RuleViolationKind> {
    violations
        .iter()
        .map(|v| v.kind.clone())
        .collect::<Vec<_>>()
}
