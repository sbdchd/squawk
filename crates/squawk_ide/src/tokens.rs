use squawk_syntax::SyntaxKind;

pub(crate) fn is_string_or_comment(kind: SyntaxKind) -> bool {
    matches!(
        kind,
        SyntaxKind::COMMENT
            | SyntaxKind::STRING
            | SyntaxKind::BYTE_STRING
            | SyntaxKind::UNICODE_ESC_STRING
            | SyntaxKind::BIT_STRING
            | SyntaxKind::DOLLAR_QUOTED_STRING
            | SyntaxKind::ESC_STRING
    )
}
