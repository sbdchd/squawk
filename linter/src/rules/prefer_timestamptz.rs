use crate::{
    versions::Version,
    violations::{RuleViolation, RuleViolationKind},
};

use squawk_parser::ast::{ColumnDef, RawStmt};

use crate::rules::utils::columns_create_or_modified;

#[must_use]
pub fn prefer_timestamptz(
    tree: &[RawStmt],
    _pg_version: Option<Version>,
    _assume_in_transaction: bool,
) -> Vec<RuleViolation> {
    let mut errs = vec![];
    for raw_stmt in tree {
        for column in columns_create_or_modified(&raw_stmt.stmt) {
            check_column_def(&mut errs, column);
        }
    }
    errs
}

fn check_column_def(errs: &mut Vec<RuleViolation>, column_def: &ColumnDef) {
    if let Some(type_name) = column_def.type_name.names.last() {
        if type_name.string.sval == "timestamp" {
            errs.push(RuleViolation::new(
                RuleViolationKind::PreferTimestampTz,
                column_def.into(),
                None,
            ));
        }
    }
}

#[cfg(test)]
mod test_rules {
    use crate::check_sql_with_rule;
    use crate::rules::test_utils::violations_to_kinds;
    use crate::violations::{RuleViolation, RuleViolationKind};
    use insta::assert_debug_snapshot;

    fn lint_sql(sql: &str) -> Vec<RuleViolation> {
        check_sql_with_rule(sql, &RuleViolationKind::PreferTimestampTz, None, false).unwrap()
    }

    #[test]
    fn create_with_timestamp() {
        let bad_sql = r"
create table app.users
(
    created_ts   timestamp
);
create table app.accounts
(
    created_ts timestamp without time zone
);
  ";
        let res = lint_sql(bad_sql);
        assert_eq!(
            violations_to_kinds(&res),
            vec![
                RuleViolationKind::PreferTimestampTz,
                RuleViolationKind::PreferTimestampTz
            ]
        );

        let ok_sql = r"
create table app.users
(
    created_ts   timestamptz
);
create table app.accounts
(
    created_ts timestamp with time zone
);
  ";
        assert_eq!(lint_sql(ok_sql), vec![]);
    }

    #[test]
    fn alter_table() {
        let bad_sql = r"
    alter table app.users
        alter column created_ts type timestamp;
    alter table app.accounts
        alter column created_ts type timestamp without time zone;
  ";
        let res = lint_sql(bad_sql);

        assert_debug_snapshot!(res);
        assert_eq!(
            violations_to_kinds(&res),
            vec![
                RuleViolationKind::PreferTimestampTz,
                RuleViolationKind::PreferTimestampTz
            ]
        );

        let ok_sql = r"
alter table app.users
    alter column created_ts type timestamptz;
alter table app.accounts
    alter column created_ts type timestamp with time zone;
alter table app.accounts
    alter column created_ts type timestamptz using created_ts at time zone 'UTC';
  ";
        assert_eq!(lint_sql(ok_sql), vec![]);
    }
}
