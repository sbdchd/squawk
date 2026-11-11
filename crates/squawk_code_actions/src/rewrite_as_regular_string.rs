use squawk_diagnostics::Edit;
use squawk_rowan::TextSize;
use squawk_syntax::{
    SyntaxKind,
    ast::AstNode,
    quote::{quote_string_literal, strip_dollar_quotes},
};

use squawk_sema::file::InFile;

use super::{ActionKind, CodeAction};

use squawk_ide_db::{Db, parse};

pub(super) fn rewrite_as_regular_string(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let file = position.file_id;
    let offset = position.value;
    let dollar_string = parse(db, file)
        .tree()
        .syntax()
        .token_at_offset(offset)
        .find(|token| token.kind() == SyntaxKind::DOLLAR_QUOTED_STRING)?;

    let replacement = dollar_quoted_to_string(dollar_string.text())?;
    actions.push(CodeAction {
        title: "Rewrite as regular string".to_owned(),
        edits: vec![Edit::replace(dollar_string.text_range(), replacement)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

fn dollar_quoted_to_string(text: &str) -> Option<String> {
    Some(quote_string_literal(strip_dollar_quotes(text)?))
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_as_regular_string;

    #[test]
    fn rewrite_dollar_string() {
        assert_snapshot!(apply_code_action(
            rewrite_as_regular_string,
            "select $$fo$0o$$;"),
            @"select 'foo';"
        );
    }

    #[test]
    fn rewrite_dollar_string_with_tag() {
        assert_snapshot!(apply_code_action(
            rewrite_as_regular_string,
            "select $tag$fo$0o$tag$;"),
            @"select 'foo';"
        );
    }

    #[test]
    fn rewrite_dollar_string_with_quote() {
        assert_snapshot!(apply_code_action(
            rewrite_as_regular_string,
            "select $$it'$0s fine$$;"),
            @"select 'it''s fine';"
        );
    }

    #[test]
    fn rewrite_dollar_string_not_applicable() {
        assert!(code_action_not_applicable(
            rewrite_as_regular_string,
            "select 'foo$0';"
        ));
    }
}
