use squawk_diagnostics::Edit;
use squawk_rowan::TextSize;
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
    quote::quote_dollar_string_literal,
};

use squawk_sema::file::InFile;

use super::{ActionKind, CodeAction};

use squawk_ide_db::{Db, parse};

pub(super) fn rewrite_as_dollar_quoted_string(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let file = position.file_id;
    let offset = position.value;
    let string = parse(db, file)
        .tree()
        .syntax()
        .token_at_offset(offset)
        .find(|token| token.kind() == SyntaxKind::STRING)?;

    let literal = ast::Literal::cast(string.parent()?)?;
    let replacement = quote_dollar_string_literal(&literal.string_value()?)?;
    actions.push(CodeAction {
        title: "Rewrite as dollar-quoted string".to_owned(),
        edits: vec![Edit::replace(string.text_range(), replacement)],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_as_dollar_quoted_string;

    #[test]
    fn rewrite_string() {
        assert_snapshot!(apply_code_action(
            rewrite_as_dollar_quoted_string,
            "select 'fo$0o';"),
            @"select $$foo$$;"
        );
    }

    #[test]
    fn rewrite_string_with_single_quote() {
        assert_snapshot!(apply_code_action(
            rewrite_as_dollar_quoted_string,
            "select 'it''s$0 nice';"),
            @"select $$it's nice$$;"
        );
    }

    #[test]
    fn rewrite_string_with_dollar_signs() {
        assert_snapshot!(apply_code_action(
            rewrite_as_dollar_quoted_string,
            "select 'foo $$ ba$0r';"),
            @"select $q$foo $$ bar$q$;"
        );
    }

    #[test]
    fn rewrite_string_when_trailing_dollar() {
        assert_snapshot!(apply_code_action(
            rewrite_as_dollar_quoted_string,
            "select 'foo $'$0;"),
            @"select $q$foo $$q$;"
        );
    }

    #[test]
    fn rewrite_string_not_applicable() {
        assert!(code_action_not_applicable(
            rewrite_as_dollar_quoted_string,
            "select 1 + $0 2;"
        ));
    }

    #[test]
    fn rewrite_prefix_string_not_applicable() {
        assert!(code_action_not_applicable(
            rewrite_as_dollar_quoted_string,
            "select b'010$0';"
        ));
    }
}
