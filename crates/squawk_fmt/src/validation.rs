use std::ops::Range;

use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet};
use anyhow::{Result, bail, ensure};
use squawk_lexer::{BOM, LiteralKind, Token, TokenKind, tokenize};
use squawk_line_index::UniversalNewlines;

use crate::fmt::{FormatOptions, fmt_str};

pub fn validate_format(before: &str, formatted: &str, options: FormatOptions) -> Result<()> {
    validate_no_dropped_tokens(before, formatted)?;
    validate_parses(formatted)?;
    validate_no_extra_spaces(formatted)?;
    validate_indentation(formatted, options.indent)?;
    validate_comment_spacing(formatted)?;
    ensure!(
        fmt_str(formatted, options)? == formatted,
        "formatting isn't idempotent"
    );
    Ok(())
}

#[track_caller]
pub fn assert_valid_format(before: &str, formatted: &str, options: FormatOptions) {
    if let Err(error) = validate_format(before, formatted, options) {
        panic!("{error}");
    }
}

fn validate_parses(formatted: &str) -> Result<()> {
    let parse = squawk_syntax::ast::SourceFile::parse(formatted);
    ensure!(
        parse.errors().is_empty(),
        "formatted output has syntax errors:\n{}\n\nformatted output:\n{formatted}",
        parse
            .errors()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
    Ok(())
}

fn validate_no_extra_spaces(formatted: &str) -> Result<()> {
    let mut offset = 0;
    for Token { kind, len } in tokenize(formatted) {
        let start = offset;
        offset += len as usize;
        if kind != TokenKind::Whitespace {
            continue;
        }
        let text = &formatted[start..offset];
        let at_line_start = start == 0 || formatted[..start].ends_with(['\n', '\r']);
        let at_eof = offset == formatted.len();
        if at_line_start || at_eof || text.contains(['\n', '\r']) || text == " " {
            continue;
        }
        let snippet = Snippet::source(formatted)
            .fold(true)
            .annotation(AnnotationKind::Primary.span(start..offset));
        let group = Level::ERROR
            .primary_title(format!(
                "expected a single space between tokens, found {text:?}"
            ))
            .element(snippet);
        bail!(Renderer::plain().render(&[group]).to_string());
    }
    Ok(())
}

fn validate_indentation(formatted: &str, indent: usize) -> Result<()> {
    ensure!(indent > 0, "indent must be greater than zero");
    let mut offset = 0;
    for Token { kind, len } in tokenize(formatted) {
        let start = offset;
        offset += len as usize;
        if kind != TokenKind::Whitespace {
            continue;
        }
        let text = &formatted[start..offset];
        let mut indentation_start = match text.rfind(['\n', '\r']) {
            Some(newline) => start + newline + 1,
            None if start == 0 => 0,
            None => continue,
        };
        if formatted[indentation_start..offset].starts_with(BOM) {
            indentation_start += BOM.len();
        }
        let indentation = offset - indentation_start;
        ensure!(
            indentation % indent == 0,
            "indentation must be a multiple of {indent}, found {indentation} spaces at byte {indentation_start}"
        );
    }
    Ok(())
}

fn comment_spacing_error(formatted: &str) -> Option<(Range<usize>, &'static str)> {
    let mut offset = 0;
    let mut tokens = tokenize(formatted)
        .map(|Token { kind, len }| {
            let start = offset;
            offset += len as usize;
            (kind, start..offset)
        })
        .filter(|(kind, _)| *kind != TokenKind::Whitespace)
        .peekable();

    let mut previous = None;
    while let Some((kind, span)) = tokens.next() {
        let left = previous.replace((kind, span.end));
        if !matches!(
            kind,
            TokenKind::LineComment | TokenKind::BlockComment { .. }
        ) {
            continue;
        }
        if let Some((previous, previous_end)) = left {
            let touching = previous_end == span.start;
            let own_line = formatted[previous_end..span.start].contains(['\n', '\r']);
            let after_opening_delimiter = matches!(
                previous,
                TokenKind::OpenParen | TokenKind::OpenBracket | TokenKind::OpenCurly
            );
            if after_opening_delimiter && !own_line {
                let wraps = kind == TokenKind::LineComment
                    || tokens.peek().is_some_and(|(_, next_span)| {
                        formatted[span.end..next_span.start].contains(['\n', '\r'])
                    });
                if wraps {
                    return Some((
                        span,
                        "comment after an opening delimiter must be on its own line when the contents wrap",
                    ));
                }
            }
            let must_touch =
                matches!(kind, TokenKind::BlockComment { .. }) && after_opening_delimiter;
            if must_touch && !touching && !own_line {
                return Some((
                    span,
                    "block comment must touch the opening delimiter on its left",
                ));
            }
            if !must_touch && touching {
                return Some((span, "comment must have whitespace on its left"));
            }
        }
        if matches!(kind, TokenKind::BlockComment { .. })
            && let Some((next, next_span)) = tokens.peek()
        {
            let touching = span.end == next_span.start;
            let own_line = formatted[span.end..next_span.start].contains(['\n', '\r']);
            let must_touch = matches!(
                next,
                TokenKind::CloseParen
                    | TokenKind::CloseBracket
                    | TokenKind::CloseCurly
                    | TokenKind::Comma
                    | TokenKind::Semi
            );
            if must_touch && !touching && !own_line {
                return Some((span, "block comment must touch the delimiter on its right"));
            }
            if !must_touch && touching {
                return Some((span, "block comment must have whitespace on its right"));
            }
        }
    }
    None
}

fn comment_spacing_diagnostic(formatted: &str) -> Option<String> {
    let (span, message) = comment_spacing_error(formatted)?;
    let snippet = Snippet::source(formatted)
        .fold(true)
        .annotation(AnnotationKind::Primary.span(span));
    let group = Level::ERROR.primary_title(message).element(snippet);
    Some(Renderer::plain().render(&[group]).to_string())
}

fn validate_comment_spacing(formatted: &str) -> Result<()> {
    if let Some(diagnostic) = comment_spacing_diagnostic(formatted) {
        bail!(diagnostic);
    }
    Ok(())
}

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

fn line_comments_equivalent(before: &str, after: &str) -> bool {
    let before = before.trim_end_matches([' ', '\t']);
    let after = after.trim_end_matches([' ', '\t']);
    if before == after {
        return true;
    }

    let Some(content) = before.strip_prefix("--") else {
        return false;
    };
    if content.is_empty() || content.starts_with(' ') || content.starts_with('\t') {
        return false;
    }

    after.strip_prefix("-- ") == Some(content)
}

fn tokens_equivalent(before: (TokenKind, &str), after: (TokenKind, &str)) -> bool {
    let (before_kind, before_text) = before;
    let (after_kind, after_text) = after;

    if before_kind == after_kind {
        if before_kind == TokenKind::LineComment {
            return line_comments_equivalent(before_text, after_text);
        }
        if matches!(before_kind, TokenKind::BlockComment { .. }) {
            let normalize = |text: &str| {
                let align_stars = text
                    .universal_newlines()
                    .skip(1)
                    .all(|line| line.as_str().trim_start().starts_with('*'));
                text.universal_newlines()
                    .map(|line| {
                        let line = line.as_str().trim_end_matches([' ', '\t']);
                        if align_stars && line.trim_start().starts_with('*') {
                            format!(" {}", line.trim())
                        } else {
                            line.to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            };
            return normalize(before_text) == normalize(after_text);
        }
        if let TokenKind::Literal {
            kind:
                LiteralKind::NationalStr { .. }
                | LiteralKind::ByteStr { .. }
                | LiteralKind::BitStr { .. }
                | LiteralKind::UnicodeEscStr { .. }
                | LiteralKind::EscStr { .. },
        } = before_kind
            && let Some((before_prefix, before_rest)) = before_text.split_once('\'')
            && let Some((after_prefix, after_rest)) = after_text.split_once('\'')
        {
            return before_prefix.eq_ignore_ascii_case(after_prefix) && before_rest == after_rest;
        }
        return if before_kind == TokenKind::Ident {
            before_text.eq_ignore_ascii_case(after_text)
        } else {
            before_text == after_text
        };
    }

    fn unquote(text: &str) -> Option<&str> {
        text.strip_prefix('"')
            .and_then(|text| text.strip_suffix('"'))
    }

    match (before_kind, after_kind) {
        (TokenKind::QuotedIdent { .. }, TokenKind::Ident) => {
            unquote(before_text) == Some(after_text.to_ascii_lowercase().as_str())
        }
        (TokenKind::Ident, TokenKind::QuotedIdent { .. }) => {
            unquote(after_text) == Some(before_text.to_ascii_lowercase().as_str())
        }
        _ => false,
    }
}

fn validate_no_dropped_tokens(before: &str, after: &str) -> Result<()> {
    let before_tokens = meaningful_tokens(before);
    let after_tokens = meaningful_tokens(after);

    let before_len = before_tokens.len();
    let after_len = after_tokens.len();

    for (index, (&before, &after)) in before_tokens.iter().zip(&after_tokens).enumerate() {
        ensure!(
            tokens_equivalent(before, after),
            "token mismatch at position {index}:\n  before: {:?} {:?}\n  after:  {:?} {:?}",
            before.0,
            before.1,
            after.0,
            after.1
        );
    }

    ensure!(
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
    Ok(())
}
