use std::ops::Range;

use rustc_hash::FxHashMap;

use ::line_index::{LineIndex, TextRange, TextSize};
use log::warn;
use lsp_types::{
    CodeAction, CodeActionKind, FoldingRange, FoldingRangeKind as LspFoldingRangeKind, Location,
    SemanticToken, Url, WorkspaceEdit,
};
use squawk_ide::code_actions::ActionKind;
use squawk_ide::db::line_index;
use squawk_ide::folding_ranges::{Fold, FoldKind};
use squawk_ide::semantic_tokens::{SemanticTokenModifier, SemanticTokenType};

use crate::global_state::Snapshot;
use crate::semantic_tokens;

pub(crate) fn text_range(index: &LineIndex, range: lsp_types::Range) -> Option<TextRange> {
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

pub(crate) fn offset(index: &LineIndex, position: lsp_types::Position) -> Option<TextSize> {
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

pub(crate) fn code_action(
    line_index: &LineIndex,
    uri: Url,
    action: squawk_ide::code_actions::CodeAction,
) -> lsp_types::CodeAction {
    let kind = match action.kind {
        ActionKind::QuickFix => CodeActionKind::QUICKFIX,
        ActionKind::RefactorRewrite => CodeActionKind::REFACTOR_REWRITE,
    };

    CodeAction {
        title: action.title,
        kind: Some(kind),
        edit: Some(WorkspaceEdit::new({
            let mut changes = FxHashMap::default();
            let edits = action
                .edits
                .into_iter()
                .map(|edit| lsp_types::TextEdit {
                    range: range(line_index, edit.text_range),
                    new_text: edit.text.unwrap_or_default(),
                })
                .collect();
            changes.insert(uri, edits);
            changes.into_iter().collect()
        })),
        is_preferred: Some(true),
        ..Default::default()
    }
}

pub(crate) fn completion_item(
    item: squawk_ide::completion::CompletionItem,
) -> lsp_types::CompletionItem {
    use squawk_ide::completion::{CompletionInsertTextFormat, CompletionItemKind};

    let kind = match item.kind {
        CompletionItemKind::Schema => lsp_types::CompletionItemKind::MODULE,
        CompletionItemKind::Keyword => lsp_types::CompletionItemKind::KEYWORD,
        CompletionItemKind::Table => lsp_types::CompletionItemKind::STRUCT,
        CompletionItemKind::Column => lsp_types::CompletionItemKind::FIELD,
        CompletionItemKind::Function => lsp_types::CompletionItemKind::FUNCTION,
        CompletionItemKind::Type => lsp_types::CompletionItemKind::CLASS,
        CompletionItemKind::Snippet => lsp_types::CompletionItemKind::SNIPPET,
        CompletionItemKind::Operator => lsp_types::CompletionItemKind::OPERATOR,
    };

    let sort_text = Some(item.sort_text());

    let insert_text_format = item.insert_text_format.map(|x| match x {
        CompletionInsertTextFormat::PlainText => lsp_types::InsertTextFormat::PLAIN_TEXT,
        CompletionInsertTextFormat::Snippet => lsp_types::InsertTextFormat::SNIPPET,
    });

    let command = if item.trigger_completion_after_insert {
        Some(lsp_types::Command {
            title: "Trigger Completion".to_owned(),
            command: "editor.action.triggerSuggest".to_owned(),
            arguments: None,
        })
    } else {
        None
    };

    let label_details = item
        .detail
        .map(|detail| lsp_types::CompletionItemLabelDetails {
            detail: None,
            // Use description instead of detail so VSCode puts it to the right
            // of the item's name instead of smushing them together.
            description: Some(detail),
        });

    lsp_types::CompletionItem {
        label: item.label,
        kind: Some(kind),
        // We use label_details instead of detail so that VSCode shows the type
        // info / function signature when the completion list is open, instead
        // of waiting until you select the given field.
        detail: None,
        label_details,
        insert_text: item.insert_text,
        insert_text_format,
        sort_text,
        command,
        ..Default::default()
    }
}

pub(crate) fn range(line_index: &LineIndex, range: TextRange) -> lsp_types::Range {
    let start = line_index.line_col(range.start());
    let end = line_index.line_col(range.end());

    lsp_types::Range::new(
        lsp_types::Position::new(start.line, start.col),
        lsp_types::Position::new(end.line, end.col),
    )
}

pub(crate) fn folding_range(line_index: &LineIndex, fold: Fold) -> FoldingRange {
    let start = line_index.line_col(fold.range.start());
    let end = line_index.line_col(fold.range.end());
    let kind = match fold.kind {
        FoldKind::Comment => Some(LspFoldingRangeKind::Comment),
        _ => Some(LspFoldingRangeKind::Region),
    };
    FoldingRange {
        start_line: start.line,
        start_character: Some(start.col),
        end_line: end.line,
        end_character: Some(end.col),
        kind,
        collapsed_text: None,
    }
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

pub(crate) fn to_location(
    snapshot: &Snapshot,
    loc: squawk_ide::location::Location,
) -> Option<Location> {
    let db = snapshot.db();
    let uri = snapshot.uri(loc.file)?;

    let line_index = line_index(db, loc.file);
    let range = range(&line_index, loc.range);
    Some(Location { uri, range })
}

pub(crate) fn to_semantic_tokens(
    text: &str,
    line_index: LineIndex,
    semantic_tokens: Vec<squawk_ide::semantic_tokens::SemanticToken>,
) -> Vec<lsp_types::SemanticToken> {
    let mut encoder = Encoder {
        tokens: Vec::with_capacity(semantic_tokens.len()),
        prev_line: 0,
        prev_start: 0,
    };

    // Duplicated in squawk-wasm, fyi
    for token in &*semantic_tokens {
        // Taken from rust-analyzer, this solves the case where we have a multi
        // line semantic token which isn't supported by the LSP spec.
        // see: https://github.com/rust-lang/rust-analyzer/blob/2efc80078029894eec0699f62ec8d5c1a56af763/crates/rust-analyzer/src/lsp/to_proto.rs#L781C28-L781C28
        for mut text_range in line_index.lines(token.range) {
            if text[text_range].ends_with('\n') {
                text_range =
                    TextRange::new(text_range.start(), text_range.end() - TextSize::of('\n'));
            }
            // TODO: Convert these UTF-8 line columns to UTF-16 before encoding semantic tokens.
            let lsp_range = range(&line_index, text_range);
            let len = lsp_range.end.character - lsp_range.start.character;
            encoder.push_token_at(lsp_range.start, len, token.token_type, token.modifiers);
        }
    }

    encoder.tokens
}

// Taken from Ty
// see: https://github.com/charliermarsh/ruff/blob/5011b253c1aca2a1906762cf45414d0fda1a088a/crates/ty_server/src/server/api/semantic_tokens.rs#L23C25-L23C25
struct Encoder {
    tokens: Vec<SemanticToken>,
    prev_line: u32,
    prev_start: u32,
}

impl Encoder {
    fn push_token_at(
        &mut self,
        start: lsp_types::Position,
        length: u32,
        ty: SemanticTokenType,
        _modifiers: Option<SemanticTokenModifier>,
    ) {
        // LSP semantic tokens are encoded as deltas
        let delta_line = start.line - self.prev_line;
        let delta_start = if delta_line == 0 {
            start.character - self.prev_start
        } else {
            start.character
        };

        let token_type = to_token_type(ty);

        let token_index = semantic_tokens::type_index(token_type);

        self.tokens.push(SemanticToken {
            delta_line,
            delta_start,
            length,
            token_type: token_index,
            // TODO: once we get modifiers going, we'll need to update this
            token_modifiers_bitset: 0,
        });

        self.prev_line = start.line;
        self.prev_start = start.character;
    }
}

fn to_token_type(ty: SemanticTokenType) -> lsp_types::SemanticTokenType {
    match ty {
        SemanticTokenType::Keyword => lsp_types::SemanticTokenType::KEYWORD,
        SemanticTokenType::String => lsp_types::SemanticTokenType::STRING,
        SemanticTokenType::Bool => lsp_types::SemanticTokenType::KEYWORD,
        SemanticTokenType::Number => lsp_types::SemanticTokenType::NUMBER,
        SemanticTokenType::Function => lsp_types::SemanticTokenType::FUNCTION,
        SemanticTokenType::Operator => lsp_types::SemanticTokenType::OPERATOR,
        SemanticTokenType::Punctuation => lsp_types::SemanticTokenType::OPERATOR,
        SemanticTokenType::Name => lsp_types::SemanticTokenType::VARIABLE,
        SemanticTokenType::NameRef => lsp_types::SemanticTokenType::VARIABLE,
        SemanticTokenType::Comment => lsp_types::SemanticTokenType::COMMENT,
        SemanticTokenType::Type => lsp_types::SemanticTokenType::TYPE,
        SemanticTokenType::PositionalParam | SemanticTokenType::Parameter => {
            lsp_types::SemanticTokenType::PARAMETER
        }
        SemanticTokenType::Column => lsp_types::SemanticTokenType::VARIABLE,
        SemanticTokenType::PropertyGraph | SemanticTokenType::Table => {
            lsp_types::SemanticTokenType::STRUCT
        }
        SemanticTokenType::Schema => lsp_types::SemanticTokenType::NAMESPACE,
    }
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
