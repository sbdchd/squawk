use std::ops::Range;

use line_index::{LineIndex, TextRange, TextSize};
use log::warn;

fn text_range(index: &LineIndex, range: lsp_types::Range) -> Option<TextRange> {
    let start = offset(index, range.start)?;
    let end = offset(index, range.end)?;
    if end >= start {
        Some(TextRange::new(start, end))
    } else {
        warn!(
            "Invalid range: start {} > end {}",
            u32::from(start),
            u32::from(end)
        );
        None
    }
}
fn offset(index: &LineIndex, position: lsp_types::Position) -> Option<TextSize> {
    let line_range = index.line(position.line)?;

    let col = TextSize::from(position.character);
    let clamped_len = col.min(line_range.len());

    if clamped_len < col {
        warn!(
            "Position line {}, col {} exceeds line length {}, clamping it",
            position.line,
            position.character,
            u32::from(line_range.len())
        );
    }

    Some(line_range.start() + clamped_len)
}

// base on rust-analyzer's
// see: https://github.com/rust-lang/rust-analyzer/blob/3816d0ae53c19fe75532a8b41d8c546d94246b53/crates/rust-analyzer/src/lsp/utils.rs#L168C1-L168C1
pub(crate) fn apply_incremental_changes(
    content: &str,
    mut content_changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
) -> String {
    // If at least one of the changes is a full document change, use the last
    // of them as the starting point and ignore all previous changes.
    let (mut text, content_changes) = match content_changes
        .iter()
        .rposition(|change| change.range.is_none())
    {
        Some(idx) => {
            let text = std::mem::take(&mut content_changes[idx].text);
            (text, &content_changes[idx + 1..])
        }
        None => (content.to_owned(), &content_changes[..]),
    };

    if content_changes.is_empty() {
        return text;
    }

    let mut line_index = LineIndex::new(&text);

    // The changes we got must be applied sequentially, but can cross lines so we
    // have to keep our line index updated.
    // Some clients (e.g. Code) sort the ranges in reverse. As an optimization, we
    // remember the last valid line in the index and only rebuild it if needed.
    let mut index_valid = !0u32;
    for change in content_changes {
        // The None case can't happen as we have handled it above already
        if let Some(range) = change.range {
            if index_valid <= range.end.line {
                line_index = LineIndex::new(&text);
            }
            index_valid = range.start.line;
            if let Some(range) = text_range(&line_index, range) {
                text.replace_range(Range::<usize>::from(range), &change.text);
            }
        }
    }

    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use lsp_types::{Position, Range, TextDocumentContentChangeEvent};

    #[test]
    fn apply_incremental_changes_no_changes() {
        let content = "hello world";
        let changes = vec![];
        let result = apply_incremental_changes(content, changes);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn apply_incremental_changes_full_document_change() {
        let content = "old content";
        let changes = vec![TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: "new content".to_string(),
        }];
        let result = apply_incremental_changes(content, changes);
        assert_eq!(result, "new content");
    }

    #[test]
    fn apply_incremental_changes_single_line_edit() {
        let content = "hello world";
        let changes = vec![TextDocumentContentChangeEvent {
            range: Some(Range::new(Position::new(0, 6), Position::new(0, 11))),
            range_length: None,
            text: "rust".to_string(),
        }];
        let result = apply_incremental_changes(content, changes);
        assert_eq!(result, "hello rust");
    }

    #[test]
    fn apply_incremental_changes_multiple_edits() {
        let content = "line 1\nline 2\nline 3";
        let changes = vec![
            TextDocumentContentChangeEvent {
                range: Some(Range::new(Position::new(0, 4), Position::new(0, 6))),
                range_length: None,
                text: " updated".to_string(),
            },
            TextDocumentContentChangeEvent {
                range: Some(Range::new(Position::new(2, 4), Position::new(2, 6))),
                range_length: None,
                text: " also updated".to_string(),
            },
        ];
        let result = apply_incremental_changes(content, changes);
        assert_eq!(result, "line updated\nline 2\nline also updated");
    }

    #[test]
    fn apply_incremental_changes_insertion() {
        let content = "hello world";
        let changes = vec![TextDocumentContentChangeEvent {
            range: Some(Range::new(Position::new(0, 5), Position::new(0, 5))),
            range_length: None,
            text: " foo".to_string(),
        }];
        let result = apply_incremental_changes(content, changes);
        assert_eq!(result, "hello foo world");
    }

    #[test]
    fn apply_incremental_changes_deletion() {
        let content = "hello foo world";
        let changes = vec![TextDocumentContentChangeEvent {
            range: Some(Range::new(Position::new(0, 5), Position::new(0, 9))),
            range_length: None,
            text: "".to_string(),
        }];
        let result = apply_incremental_changes(content, changes);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn apply_incremental_changes_multiline_edit() {
        let content = "line 1\nline 2\nline 3";
        let changes = vec![TextDocumentContentChangeEvent {
            range: Some(Range::new(Position::new(0, 6), Position::new(1, 6))),
            range_length: None,
            text: " and\nreplaced".to_string(),
        }];
        let result = apply_incremental_changes(content, changes);
        assert_eq!(result, "line 1 and\nreplaced\nline 3");
    }

    #[test]
    fn apply_incremental_changes_full_then_incremental() {
        let content = "original";
        let changes = vec![
            TextDocumentContentChangeEvent {
                range: None,
                range_length: None,
                text: "hello world".to_string(),
            },
            TextDocumentContentChangeEvent {
                range: Some(Range::new(Position::new(0, 6), Position::new(0, 11))),
                range_length: None,
                text: "rust".to_string(),
            },
        ];
        let result = apply_incremental_changes(content, changes);
        assert_eq!(result, "hello rust");
    }

    #[test]
    fn apply_incremental_changes_invalid_range_ignored() {
        let content = "hello";
        let changes = vec![TextDocumentContentChangeEvent {
            range: Some(Range::new(Position::new(10, 0), Position::new(10, 5))),
            range_length: None,
            text: "invalid".to_string(),
        }];
        let result = apply_incremental_changes(content, changes);
        assert_eq!(result, "hello");
    }

    #[test]
    fn apply_incremental_changes_with_invalid_line_no() {
        let content = "hello world";
        let changes = vec![TextDocumentContentChangeEvent {
            range: Some(Range::new(Position::new(10, 0), Position::new(10, 5))),
            range_length: None,
            text: "invalid".to_string(),
        }];
        let result = apply_incremental_changes(content, changes);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn apply_incremental_changes_column_clamping() {
        let content = "short\nlong line";
        let changes = vec![TextDocumentContentChangeEvent {
            range: Some(Range::new(Position::new(0, 3), Position::new(0, 100))),
            range_length: None,
            text: " extended".to_string(),
        }];
        let result = apply_incremental_changes(content, changes);
        assert_eq!(result, "sho extendedlong line");
    }
}
