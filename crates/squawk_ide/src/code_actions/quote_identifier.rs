use rowan::TextSize;
use salsa::Database as Db;
use squawk_syntax::ast::{self, AstNode};

use crate::{db::File, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn quote_identifier(
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

    let text = name_node.text().to_string();

    if text.starts_with('"') {
        return None;
    }

    let quoted = format!(r#""{}""#, text.to_lowercase());

    actions.push(CodeAction {
        title: "Quote identifier".to_owned(),
        edits: vec![squawk_linter::Edit::replace(name_node.text_range(), quoted)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::quote_identifier;

    #[test]
    fn quote_identifier_on_name_ref() {
        assert_snapshot!(apply_code_action(
            quote_identifier,
            "select x$0 from t;"),
            @r#"select "x" from t;"#
        );
    }

    #[test]
    fn quote_identifier_on_name() {
        assert_snapshot!(apply_code_action(
            quote_identifier,
            "create table T(X$0 int);"),
            @r#"create table T("x" int);"#
        );
    }

    #[test]
    fn quote_identifier_lowercases() {
        assert_snapshot!(apply_code_action(
            quote_identifier,
            "create table T(COL$0 int);"),
            @r#"create table T("col" int);"#
        );
    }

    #[test]
    fn quote_identifier_not_applicable_when_already_quoted() {
        assert!(code_action_not_applicable(
            quote_identifier,
            r#"select "x"$0 from t;"#
        ));
    }

    #[test]
    fn quote_identifier_not_applicable_on_select_keyword() {
        assert!(code_action_not_applicable(
            quote_identifier,
            "sel$0ect x from t;"
        ));
    }

    #[test]
    fn quote_identifier_on_keyword_column_name() {
        assert_snapshot!(apply_code_action(
            quote_identifier,
            "select te$0xt from t;"),
            @r#"select "text" from t;"#
        );
    }

    #[test]
    fn quote_identifier_example_select() {
        assert_snapshot!(apply_code_action(
            quote_identifier,
            "select x$0 from t;"),
            @r#"select "x" from t;"#
        );
    }

    #[test]
    fn quote_identifier_example_create_table() {
        assert_snapshot!(apply_code_action(
            quote_identifier,
            "create table T(X$0 int);"),
            @r#"create table T("x" int);"#
        );
    }
}
