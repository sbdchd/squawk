use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{ColumnDef, PGString, QualifiedName, RawStmt};

use crate::rules::utils::columns_create_or_modified;

#[must_use]
pub fn prefer_timestamptz(tree: &[RawStmt], _pg_version: Option<Version>) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        for column in columns_create_or_modified(&raw_stmt.stmt) {
            check_column_def(&mut errs, raw_stmt, column);
        }
    }
    errs
}

fn check_column_def(errs: &mut Vec<RuleViolation>, raw_stmt: &RawStmt, column_def: &ColumnDef) {
    if column_def.type_name.names
        == vec![
            QualifiedName {
                string: PGString {
                    str: "pg_catalog".to_string(),
                },
            },
            QualifiedName {
                string: PGString {
                    str: "timestamp".to_string(),
                },
            },
        ]
    {
        errs.push(RuleViolation::new(
            RuleViolationKind::PreferTimestampTz,
            raw_stmt.into(),
            None,
        ));
    }
}

#[cfg(test)]
mod test_rules {
    use crate::check_sql;
    use crate::violations::{RuleViolation, RuleViolationKind};
    use insta::assert_debug_snapshot;

    lazy_static! {
        static ref EXCLUDED_RULES: Vec<RuleViolationKind> = vec![
            RuleViolationKind::PreferRobustStmts,
            RuleViolationKind::ChangingColumnType
        ];
    }

    fn violations_to_kinds(violations: Vec<RuleViolation>) -> Vec<RuleViolationKind> {
        violations.into_iter().map(|v| v.kind).collect::<Vec<_>>()
    }

    #[test]
    fn test_create_with_timestamp() {
        let bad_sql = r#"
create table app.users
(
    created_ts   timestamp
);
create table app.accounts
(
    created_ts timestamp without time zone
);
  "#;
        let res = check_sql(bad_sql, &EXCLUDED_RULES, None).unwrap();
        assert_eq!(
            violations_to_kinds(res),
            vec![
                RuleViolationKind::PreferTimestampTz,
                RuleViolationKind::PreferTimestampTz
            ]
        );

        let ok_sql = r#"
create table app.users
(
    created_ts   timestamptz
);
create table app.accounts
(
    created_ts timestamp with time zone
);
  "#;
        assert_eq!(check_sql(ok_sql, &EXCLUDED_RULES, None), Ok(vec![]));
    }

    #[test]
    fn test_alter_table() {
        let bad_sql = r#"
    alter table app.users
        alter column created_ts type timestamp;
    alter table app.accounts
        alter column created_ts type timestamp without time zone;
  "#;
        let res = check_sql(bad_sql, &EXCLUDED_RULES, None).unwrap();

        assert_debug_snapshot!(res);
        assert_eq!(
            violations_to_kinds(res),
            vec![
                RuleViolationKind::PreferTimestampTz,
                RuleViolationKind::PreferTimestampTz
            ]
        );

        let ok_sql = r#"
alter table app.users
    alter column created_ts type timestamptz;
alter table app.accounts
    alter column created_ts type timestamp with time zone;
alter table app.accounts
    alter column created_ts type timestamptz using created_ts at time zone 'UTC';
  "#;
        assert_eq!(check_sql(ok_sql, &EXCLUDED_RULES, None), Ok(vec![]));
    }
}
