use squawk_lexer::{Token, TokenKind, tokenize};

fn meaningful_tokens(text: &str) -> Vec<(TokenKind, &str)> {
    let mut tokens = Vec::new();
    let mut offset = 0;
    for Token { kind, len } in tokenize(text) {
        let len = len as usize;
        if kind != TokenKind::Eof && kind != TokenKind::Whitespace {
            tokens.push((kind, &text[offset..offset + len]));
        }
        offset += len;
    }
    tokens
}

fn tokens_equivalent(before: (TokenKind, &str), after: (TokenKind, &str)) -> bool {
    let (before_kind, before_text) = before;
    let (after_kind, after_text) = after;

    if before_kind == after_kind {
        return before_text.eq_ignore_ascii_case(after_text);
    }

    // The formatter removes unnecessary identifier quotes, so compare quoted
    // and unquoted identifier tokens by their contents.
    fn unquote<'a>(kind: &TokenKind, text: &'a str) -> Option<&'a str> {
        match kind {
            TokenKind::QuotedIdent { .. } => text
                .strip_prefix('"')
                .and_then(|text| text.strip_suffix('"')),
            TokenKind::Ident => Some(text),
            _ => None,
        }
    }

    match (
        unquote(&before_kind, before_text),
        unquote(&after_kind, after_text),
    ) {
        (Some(before), Some(after)) => before.eq_ignore_ascii_case(after),
        _ => false,
    }
}

pub fn assert_no_dropped_tokens(before: &str, after: &str) {
    let before_tokens = meaningful_tokens(before);
    let after_tokens = meaningful_tokens(after);

    let before_len = before_tokens.len();
    let after_len = after_tokens.len();

    for (index, (&before, &after)) in before_tokens.iter().zip(&after_tokens).enumerate() {
        assert!(
            tokens_equivalent(before, after),
            "token mismatch at position {index}:\n  before: {:?} {:?}\n  after:  {:?} {:?}",
            before.0,
            before.1,
            after.0,
            after.1
        );
    }

    assert!(
        before_len == after_len,
        "token count mismatch: before has {before_len} tokens, after has {after_len} tokens\n  {}",
        if before_len > after_len {
            let dropped = &before_tokens[after_len..];
            format!(
                "dropped {} token(s): {}",
                dropped.len(),
                dropped
                    .iter()
                    .map(|(kind, text)| format!("{kind:?} {text:?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            let extra = &after_tokens[before_len..];
            format!(
                "extra {} token(s): {}",
                extra.len(),
                extra
                    .iter()
                    .map(|(kind, text)| format!("{kind:?} {text:?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    );
}
