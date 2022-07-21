use std::collections::HashSet;

use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{ColumnDef, PGString, QualifiedName, RawStmt};

use super::utils::columns_create_or_modified;

// // bad ints
// //
// smallint
// integer
// int4
// serial
// serial2
// serial4
// smallserial

// // okay
// int8
// serial8
// bigserial
// bigint

#[must_use]
pub fn prefer_big_int(tree: &[RawStmt], _pg_version: Option<Version>) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        for column in columns_create_or_modified(&raw_stmt.stmt) {
            check_column_def(&mut errs, raw_stmt, column);
        }
    }
    errs
}

lazy_static! {
    static ref SMALL_INT_TYPES: HashSet<&'static str> = HashSet::from([
        "smallint",
        "integer",
        "int2",
        "int4",
        "serial",
        "serial2",
        "serial4",
        "smallserial",
    ]);
}

fn check_column_def(errs: &mut Vec<RuleViolation>, raw_stmt: &RawStmt, column_def: &ColumnDef) {
    if let Some(column_name) = column_def.type_name.names.last() {
        if SMALL_INT_TYPES.contains(column_name.string.str.as_str()) {
            errs.push(RuleViolation::new(
                RuleViolationKind::PreferBigInt,
                raw_stmt.into(),
                None,
            ));
        }
    }
}

#[cfg(test)]
mod test_rules {
    use crate::{check_sql, rules::test_utils::violations_to_kinds, violations::RuleViolationKind};

    lazy_static! {
        static ref EXCLUDED_RULES: Vec<RuleViolationKind> = vec![
            RuleViolationKind::PreferRobustStmts,
            RuleViolationKind::ChangingColumnType
        ];
    }

    #[test]
    fn test_create_table_ok() {
        let ok_sql = r#"
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
  "#;
        assert_eq!(check_sql(ok_sql, &EXCLUDED_RULES, None), Ok(vec![]));
    }
    #[test]
    fn test_create_table_bad() {
        let bad_sql = r#"
create table users (
    id smallint
);
create table users (
    id int2
);
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
    id serial2
);
create table users (
    id serial4
);
create table users (
    id smallserial
);
  "#;
        let res = check_sql(bad_sql, &EXCLUDED_RULES, None).unwrap();
        let violations = violations_to_kinds(res);
        assert_eq!(
            violations.len(),
            8,
            "we should have 8 statements with violations"
        );
        assert_eq!(
            violations.len(),
            violations
                .into_iter()
                .filter(|v| { *v == RuleViolationKind::PreferBigInt })
                .count(),
            "all violations should be big int violations"
        );
        // assert_eq!(check_sql(bad_sql, &EXCLUDED_RULES, None), Ok(vec![]));
    }
}
