use rowan::TextSize;
use salsa::Database as Db;
use squawk_syntax::ast::{self, AstNode};

use crate::{db::File, offsets::token_from_offset, quote::unquote_ident};

use super::{ActionKind, CodeAction};

pub(super) fn unquote_identifier(
    db: &dyn Db,
    file: File,
    actions: &mut Vec<CodeAction>,
    offset: TextSize,
) -> Option<()> {
    let token = token_from_offset(db, file, offset)?;
    let parent = token.parent()?;

    let name_node = if let Some(name) = ast::Name::cast(parent.clone()) {
        name.syntax().clone()
    } else if let Some(name_ref) = ast::NameRef::cast(parent) {
        name_ref.syntax().clone()
    } else {
        return None;
    };

    let unquoted = unquote_ident(&name_node)?;

    actions.push(CodeAction {
        title: "Unquote identifier".to_owned(),
        edits: vec![squawk_linter::Edit::replace(
            name_node.text_range(),
            unquoted,
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::unquote_identifier;

    #[test]
    fn unquote_identifier_simple() {
        assert_snapshot!(apply_code_action(
            unquote_identifier,
            r#"select "x"$0 from t;"#),
            @"select x from t;"
        );
    }

    #[test]
    fn unquote_identifier_with_underscore() {
        assert_snapshot!(apply_code_action(
            unquote_identifier,
            r#"select "user_id"$0 from t;"#),
            @"select user_id from t;"
        );
    }

    #[test]
    fn unquote_identifier_with_digits() {
        assert_snapshot!(apply_code_action(
            unquote_identifier,
            r#"select "x123"$0 from t;"#),
            @"select x123 from t;"
        );
    }

    #[test]
    fn unquote_identifier_with_dollar() {
        assert_snapshot!(apply_code_action(
            unquote_identifier,
            r#"select "my_table$1"$0 from t;"#),
            @"select my_table$1 from t;"
        );
    }

    #[test]
    fn unquote_identifier_starts_with_underscore() {
        assert_snapshot!(apply_code_action(
            unquote_identifier,
            r#"select "_col"$0 from t;"#),
            @"select _col from t;"
        );
    }

    #[test]
    fn unquote_identifier_starts_with_unicode() {
        assert_snapshot!(apply_code_action(
            unquote_identifier,
            r#"select "é"$0 from t;"#),
            @"select é from t;"
        );
    }

    #[test]
    fn unquote_identifier_not_applicable() {
        // upper case
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "X"$0 from t;"#
        ));
        // upper case
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "Foo"$0 from t;"#
        ));
        // dash
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "my-col"$0 from t;"#
        ));
        // leading digits
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "123"$0 from t;"#
        ));
        // space
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "foo bar"$0 from t;"#
        ));
        // quotes
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "foo""bar"$0 from t;"#
        ));
        // already unquoted
        assert!(code_action_not_applicable(
            unquote_identifier,
            "select x$0 from t;"
        ));
        // brackets
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "my[col]"$0 from t;"#
        ));
        // curly brackets
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "my{}"$0 from t;"#
        ));
        // reserved word
        assert!(code_action_not_applicable(
            unquote_identifier,
            r#"select "select"$0 from t;"#
        ));
    }

    #[test]
    fn unquote_identifier_on_name() {
        assert_snapshot!(apply_code_action(
            unquote_identifier,
            r#"create table T("x"$0 int);"#),
            @"create table T(x int);"
        );
    }
}
