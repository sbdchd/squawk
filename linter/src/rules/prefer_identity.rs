use std::collections::HashSet;

use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{ColumnDef, RawStmt};

use super::utils::columns_create_or_modified;

#[must_use]
pub fn prefer_identity(tree: &[RawStmt], _pg_version: Option<Version>) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        for column in columns_create_or_modified(&raw_stmt.stmt) {
            check_column_def(&mut errs, raw_stmt, column);
        }
    }
    errs
}

lazy_static! {
    static ref SERIAL_TYPES: HashSet<&'static str> = HashSet::from([
        "serial",
        "serial2",
        "serial4",
        "serial8",
        "smallserial",
        "bigserial",
    ]);
}

fn check_column_def(errs: &mut Vec<RuleViolation>, raw_stmt: &RawStmt, column_def: &ColumnDef) {
    if let Some(column_name) = column_def.type_name.names.last() {
        if SERIAL_TYPES.contains(column_name.string.str.as_str()) {
            errs.push(RuleViolation::new(
                RuleViolationKind::PreferIdentity,
                raw_stmt.into(),
                None,
            ));
        }
    }
}

#[cfg(test)]
mod test_rules {
    use insta::assert_debug_snapshot;

    use crate::{
        check_sql_with_rule,
        rules::test_utils::violations_to_kinds,
        violations::{RuleViolation, RuleViolationKind},
    };
    fn lint_sql(sql: &str) -> Vec<RuleViolation> {
        check_sql_with_rule(sql, &RuleViolationKind::PreferIdentity, None).unwrap()
    }

    #[test]
    fn test_prefer_identity_bad() {
        let bad_sql = r#"
create table users (
    id serial
);
create table users (
    id serial2
);
create table users (
    id serial4
);
create table users (
    id serial8
);
create table users (
    id smallserial
);
create table users (
    id bigserial
);
  "#;

        let res = lint_sql(bad_sql);
        let violations = violations_to_kinds(&res);
        assert_eq!(
            violations.len(),
            6,
            "we should have 6 statements with violations"
        );
        assert_eq!(
            violations.len(),
            violations
                .into_iter()
                .filter(|v| { *v == RuleViolationKind::PreferIdentity })
                .count(),
            "all violations should be prefer-identity violations"
        );

        assert_debug_snapshot!(res);
    }
    #[test]
    fn test_prefer_identity_ok() {
        let ok_sql = r#"
create table users (
    id  bigint generated by default as identity primary key
);
create table users (
    id  bigint generated always as identity primary key
);
  "#;

        assert_eq!(lint_sql(ok_sql), vec![]);
    }
}
