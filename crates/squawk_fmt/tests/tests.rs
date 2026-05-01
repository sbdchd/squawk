use camino::Utf8Path;
use dir_test::{Fixture, dir_test};
use insta::{assert_snapshot, with_settings};
use squawk_lexer::{Token, TokenKind, tokenize};

#[dir_test(
    dir: "$CARGO_MANIFEST_DIR/tests/before",
    glob: "*.sql",
)]
fn fmt(fixture: Fixture<&str>) {
    let content = fixture.content();
    let absolute_fixture_path = Utf8Path::new(fixture.path());
    let test_name = absolute_fixture_path
        .file_name()
        .and_then(|x| x.strip_suffix(".sql"))
        .unwrap();

    let formatted = squawk_fmt::fmt(content);

    assert_no_dropped_tokens(content, &formatted);

    with_settings!({
        omit_expression => true,
        input_file => absolute_fixture_path,
        snapshot_path => "after",
        prepend_module_to_snapshot => false,
    }, {
        assert_snapshot!(test_name, formatted);
    });
}

fn meaningful_tokens(text: &str) -> Vec<(TokenKind, &str)> {
    let mut tokens: Vec<(TokenKind, &str)> = vec![];
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

fn assert_no_dropped_tokens(before: &str, after: &str) {
    let before_tokens = meaningful_tokens(before);
    let after_tokens = meaningful_tokens(after);

    let before_len = before_tokens.len();
    let after_len = after_tokens.len();

    for (i, ((bkind, btext), (akind, atext))) in
        before_tokens.iter().zip(after_tokens.iter()).enumerate()
    {
        assert!(
            bkind == akind && btext.eq_ignore_ascii_case(atext),
            "token mismatch at position {i}:\n  before: {bkind:?} {btext:?}\n  after:  {akind:?} {atext:?}"
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
                    .map(|(k, t)| format!("{k:?} {t:?}"))
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
                    .map(|(k, t)| format!("{k:?} {t:?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    );
}
