use rowan::{TextRange, TextSize};
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_json_value_as_colon(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let key_value = token.parent_ancestors().find_map(ast::JsonKeyValue::cast)?;
    let value = key_value.value_token()?;
    let start = value
        .prev_token()
        .filter(|token| token.kind() == SyntaxKind::WHITESPACE)
        .map_or(value.text_range().start(), |token| {
            token.text_range().start()
        });

    actions.push(CodeAction {
        title: "Rewrite `value` as `:`".to_owned(),
        edits: vec![Edit::replace(
            TextRange::new(start, value.text_range().end()),
            ":".to_owned(),
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_json_value_as_colon;

    #[test]
    fn rewrites_value_as_colon() {
        assert_snapshot!(
            apply_code_action(
                rewrite_json_value_as_colon,
                "select json_object('key' VAL$0UE value);",
            ),
            @"select json_object('key': value);"
        );
        assert_snapshot!(
            apply_code_action(
                rewrite_json_value_as_colon,
                "select json_object('key'VAL$0UE value);",
            ),
            @"select json_object('key': value);"
        );
    }

    #[test]
    fn rewrites_json_objectagg_value_as_colon() {
        assert_snapshot!(
            apply_code_action(
                rewrite_json_value_as_colon,
                "select json_objectagg(lower($0key) VALUE value);",
            ),
            @"select json_objectagg(lower(key): value);"
        );
    }

    #[test]
    fn not_applicable_to_colon() {
        assert!(code_action_not_applicable(
            rewrite_json_value_as_colon,
            "select json_object('key' $0: value);"
        ));
    }
}
