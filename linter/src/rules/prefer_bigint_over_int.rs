use std::collections::HashSet;

use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{ColumnDef, RawStmt};

use super::utils::columns_create_or_modified;

#[must_use]
pub fn prefer_bigint_over_int(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        for column in columns_create_or_modified(&raw_stmt.stmt) {
            check_column_def(&mut errs, raw_stmt, column);
        }
    }
    errs
}

lazy_static! {
    static ref INT_TYPES: HashSet<&'static str> =
        HashSet::from(["integer", "int4", "serial", "serial4",]);
}

fn check_column_def(errs: &mut Vec<RuleViolation>, raw_stmt: &RawStmt, column_def: &ColumnDef) {
    if let Some(column_name) = column_def.type_name.names.last() {
        if INT_TYPES.contains(column_name.string.sval.as_str()) {
            errs.push(RuleViolation::new(
                RuleViolationKind::PreferBigintOverInt,
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
        check_sql_with_rule(sql, &RuleViolationKind::PreferBigintOverInt, None, false).unwrap()
    }

    #[test]
    fn test_create_table_ok() {
        let ok_sql = r"
create table users (
    id bigint
);
create table users (
    id int8
);
create table users (
    id bigserial
);
create table users (
    id serial8
);
create table users (
    id smallint
);
create table users (
    id int2
);
create table users (
    id smallserial
);
create table users (
    id serial2
);
  ";
        assert_eq!(lint_sql(ok_sql), vec![]);
    }
    #[test]
    fn test_create_table_bad() {
        let bad_sql = r"
create table users (
    id integer
);
create table users (
    id int4
);
create table users (
    id serial
);
create table users (
    id serial4
);
  ";
        let res = lint_sql(bad_sql);
        let violations = violations_to_kinds(&res);
        assert_eq!(
            violations.len(),
            4,
            "we should have 4 statements with violations"
        );
        assert_eq!(
            violations.len(),
            violations
                .into_iter()
                .filter(|v| { *v == RuleViolationKind::PreferBigintOverInt })
                .count(),
            "all violations should be big int violations"
        );
        assert_debug_snapshot!(res);
    }
}
