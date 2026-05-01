use rowan::TextSize;
use salsa::Database as Db;
use squawk_syntax::ast::{self, AstNode};

use crate::{db::File, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_timestamp_type(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(db, file, offset)?;
    let time_type = token.parent_ancestors().find_map(ast::TimeType::cast)?;

    let replacement = match time_type.timezone()? {
        ast::Timezone::WithoutTimezone(_) => {
            if time_type.timestamp_token().is_some() {
                "timestamp"
            } else {
                "time"
            }
        }
        ast::Timezone::WithTimezone(_) => {
            if time_type.timestamp_token().is_some() {
                "timestamptz"
            } else {
                "timetz"
            }
        }
    };

    actions.push(CodeAction {
        title: format!("Rewrite as `{replacement}`"),
        edits: vec![squawk_linter::Edit::replace(
            time_type.syntax().text_range(),
            replacement,
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_timestamp_type;

    #[test]
    fn rewrite_timestamp_without_tz_column() {
        assert_snapshot!(apply_code_action(
            rewrite_timestamp_type,
            "create table t(a time$0stamp without time zone);"),
            @"create table t(a timestamp);"
        );
    }

    #[test]
    fn rewrite_timestamp_without_tz_cast() {
        assert_snapshot!(apply_code_action(
            rewrite_timestamp_type,
            "select timestamp$0 without time zone '2021-01-01';"),
            @"select timestamp '2021-01-01';"
        );
    }

    #[test]
    fn rewrite_time_without_tz() {
        assert_snapshot!(apply_code_action(
            rewrite_timestamp_type,
            "create table t(a ti$0me without time zone);"),
            @"create table t(a time);"
        );
    }

    #[test]
    fn rewrite_timestamp_without_tz_not_applicable_plain() {
        assert!(code_action_not_applicable(
            rewrite_timestamp_type,
            "create table t(a time$0stamp);"
        ));
    }

    #[test]
    fn rewrite_timestamp_with_tz_column() {
        assert_snapshot!(apply_code_action(
            rewrite_timestamp_type,
            "create table t(a time$0stamp with time zone);"),
            @"create table t(a timestamptz);"
        );
    }

    #[test]
    fn rewrite_timestamp_with_tz_cast() {
        assert_snapshot!(apply_code_action(
            rewrite_timestamp_type,
            "select timestamp$0 with time zone '2021-01-01';"),
            @"select timestamptz '2021-01-01';"
        );
    }

    #[test]
    fn rewrite_time_with_tz() {
        assert_snapshot!(apply_code_action(
            rewrite_timestamp_type,
            "create table t(a ti$0me with time zone);"),
            @"create table t(a timetz);"
        );
    }
}
