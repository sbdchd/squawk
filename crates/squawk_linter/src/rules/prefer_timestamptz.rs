use squawk_diagnostics::{Edit, Fix};
use squawk_syntax::ast::{self, AstNode};

use crate::visitors::check_not_allowed_types;
use crate::{Linter, Rule, Violation};

pub(crate) fn is_not_allowed_timestamp(ty: &ast::Type) -> bool {
    match ty.element_type() {
        Some(ast::Type::PathType(path_type)) => path_type
            .path_ref()
            .and_then(|x| x.segment())
            .is_some_and(|segment| segment.text() == "timestamp"),
        Some(ast::Type::TimestampType(timestamp_type)) => !matches!(
            timestamp_type.timezone(),
            Some(ast::Timezone::WithTimezone(_))
        ),
        Some(
            ast::Type::ArrayType(_)
            | ast::Type::VarcharType(_)
            | ast::Type::CharacterType(_)
            | ast::Type::BitType(_)
            | ast::Type::BitVaryingType(_)
            | ast::Type::DoubleType(_)
            | ast::Type::TimeType(_)
            | ast::Type::IntervalType(_),
        )
        | None => false,
    }
}

fn fix_timestamp(ty: &ast::Type) -> Option<Fix> {
    let ty = ty.element_type()?;
    match &ty {
        ast::Type::TimestampType(_) => {
            let range = ty.syntax().text_range();
            let edit = Edit::replace(range, "timestamptz");
            Some(Fix::new("Replace with `timestamptz`", vec![edit]))
        }
        ast::Type::PathType(path_type) => {
            let segment = path_type.path_ref()?.segment()?;
            let edit = Edit::replace(segment.syntax().text_range(), "timestamptz");
            Some(Fix::new("Replace with `timestamptz`", vec![edit]))
        }
        _ => None,
    }
}

fn check_ty_for_timestamp(ctx: &mut Linter, ty: Option<ast::Type>) {
    if let Some(ty) = ty
        && is_not_allowed_timestamp(&ty)
    {
        let fix = fix_timestamp(&ty);
        ctx.report(Violation::for_node(
                Rule::PreferTimestampTz,
            "`timestamp` drops the UTC offset when storing a datetime".into(),
                ty.syntax(),
            )
            .note("`2019-10-11 21:11:24+02` and `2019-10-11 21:11:24-06` are both stored as `2019-10-11 21:11:24`, even though they are eight hours apart")
            .help("use `timestamptz` instead of `timestamp`").fix(fix));
    }
}

pub(crate) fn prefer_timestamptz(ctx: &mut Linter, file: &ast::SourceFile) {
    check_not_allowed_types(ctx, file, check_ty_for_timestamp);
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::Rule;
    use crate::test_utils::{fix_sql, lint_errors, lint_ok};

    #[must_use]
    fn fix(sql: &str) -> String {
        fix_sql(sql, Rule::PreferTimestampTz)
    }

    #[test]
    fn fix_timestamp_to_timestamptz() {
        assert_snapshot!(fix("
create table app.users
(
    created_ts   timestamp
);
"), @r"
        create table app.users
        (
            created_ts   timestamptz
        );
        ");
    }

    #[test]
    fn fix_timestamp_without_time_zone() {
        assert_snapshot!(fix("
create table app.accounts
(
    created_ts timestamp without time zone
);
"), @r"
        create table app.accounts
        (
            created_ts timestamptz
        );
        ");
    }

    #[test]
    fn fix_alter_table_timestamp() {
        assert_snapshot!(fix("
alter table app.users
    alter column created_ts type timestamp;
"), @r"
        alter table app.users
            alter column created_ts type timestamptz;
        ");
    }

    #[test]
    fn fix_timestamp_array() {
        assert_snapshot!(fix("
create table app.events
(
    timestamps timestamp[]
);
"), @r"
        create table app.events
        (
            timestamps timestamptz[]
        );
        ");
    }

    #[test]
    fn fix_path_timestamp() {
        assert_snapshot!(fix(r#"
create table t (
  a "timestamp",
  b pg_catalog.timestamp(3),
  c "timestamp"(2)[]
);
"#), @"
        create table t (
          a timestamptz,
          b pg_catalog.timestamptz(3),
          c timestamptz(2)[]
        );
        ");
    }

    #[test]
    fn create_table_with_path_timestamp_err() {
        let sql = r#"
create table t (
  a "timestamp",
  b pg_catalog.timestamp(3),
  c "timestamp"(2)[]
);
"#;
        assert_snapshot!(lint_errors(sql, Rule::PreferTimestampTz));
    }

    #[test]
    fn path_varchar_and_timestamptz_ok() {
        let sql = r#"
create table t (
  a "varchar"(100),
  b pg_catalog.varchar(10),
  c pg_catalog.timestamptz(3),
  d timestamptz
);
"#;
        lint_ok(sql, Rule::PreferTimestampTz);
    }

    #[test]
    fn create_table_with_timestamp_err() {
        let sql = r#"
create table app.users
(
    created_ts   timestamp
);
create table app.accounts
(
    created_ts timestamp without time zone
);
        "#;
        assert_snapshot!(lint_errors(sql, Rule::PreferTimestampTz));
    }

    #[test]
    fn alter_table_with_timestamp_err() {
        let sql = r#"
alter table app.users
    alter column created_ts type timestamp;
alter table app.accounts
    alter column created_ts type timestamp without time zone;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::PreferTimestampTz));
    }

    #[test]
    fn alter_table_with_time_zone_ok() {
        let sql = r#"
create table app.users
(
    created_ts   timestamptz
);
create table app.accounts
(
    created_ts timestamp with time zone
);
        "#;
        lint_ok(sql, Rule::PreferTimestampTz);
    }

    #[test]
    fn create_table_with_time_zone_ok() {
        let sql = r#"
create table app.users
(
    created_ts   timestamptz
);
create table app.accounts
(
    created_ts timestamp with time zone
);
        "#;
        lint_ok(sql, Rule::PreferTimestampTz);
    }
}
