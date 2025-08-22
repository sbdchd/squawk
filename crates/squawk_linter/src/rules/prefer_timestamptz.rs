use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Edit, Fix, Linter, Rule, Violation};
use crate::{identifier::Identifier, visitors::check_not_allowed_types};

pub fn is_not_allowed_timestamp(ty: &ast::Type) -> bool {
    match ty {
        ast::Type::ArrayType(array_type) => {
            if let Some(ty) = array_type.ty() {
                is_not_allowed_timestamp(&ty)
            } else {
                false
            }
        }
        ast::Type::PercentType(_) => false,
        ast::Type::PathType(path_type) => {
            let Some(ty_name) = path_type
                .path()
                .and_then(|x| x.segment())
                .and_then(|x| x.name_ref())
                .map(|x| x.text().to_string())
            else {
                return false;
            };
            // if we don't have any args, then it's the same as `text`
            Identifier::new(ty_name.as_str()) == Identifier::new("varchar")
                && path_type.arg_list().is_some()
        }
        ast::Type::CharType(_) => false,
        ast::Type::BitType(_) => false,
        ast::Type::DoubleType(_) => false,
        ast::Type::TimeType(time_type) => {
            if time_type.timestamp_token().is_some()
                && !matches!(time_type.timezone(), Some(ast::Timezone::WithTimezone(_)))
            {
                return true;
            }
            false
        }
        ast::Type::IntervalType(_) => false,
    }
}

fn fix_timestamp(ty: &ast::Type) -> Option<Fix> {
    match ty {
        ast::Type::TimeType(_) => {
            let range = ty.syntax().text_range();
            let edit = Edit::replace(range, "timestamptz");
            Some(Fix::new("Replace with `timestamptz`", vec![edit]))
        }
        ast::Type::ArrayType(array_type) => {
            if let Some(inner_ty) = array_type.ty() {
                fix_timestamp(&inner_ty)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn check_ty_for_timestamp(ctx: &mut Linter, ty: Option<ast::Type>) {
    if let Some(ty) = ty {
        if is_not_allowed_timestamp(&ty) {
            let fix = fix_timestamp(&ty);
            ctx.report(Violation::for_node(
                Rule::PreferTimestampTz,
            "When Postgres stores a datetime in a `timestamp` field, Postgres drops the UTC offset. This means 2019-10-11 21:11:24+02 and 2019-10-11 21:11:24-06 will both be stored as 2019-10-11 21:11:24 in the database, even though they are eight hours apart in time.".into(),
                ty.syntax(),
            ).help("Use `timestamptz` instead of `timestamp` for your column type.").fix(fix));
        };
    }
}

pub(crate) fn prefer_timestamptz(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    check_not_allowed_types(ctx, &file, check_ty_for_timestamp);
}

#[cfg(test)]
mod test {
    use insta::{assert_debug_snapshot, assert_snapshot};

    use crate::Rule;
    use crate::test_utils::{fix_sql, lint};

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
        let errors = lint(sql, Rule::PreferTimestampTz);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn alter_table_with_timestamp_err() {
        let sql = r#"
alter table app.users
    alter column created_ts type timestamp;
alter table app.accounts
    alter column created_ts type timestamp without time zone;
        "#;
        let errors = lint(sql, Rule::PreferTimestampTz);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
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
        let errors = lint(sql, Rule::PreferTimestampTz);
        assert_eq!(errors.len(), 0);
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
        let errors = lint(sql, Rule::PreferTimestampTz);
        assert_eq!(errors.len(), 0);
    }
}
