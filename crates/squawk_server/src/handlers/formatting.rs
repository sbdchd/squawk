use anyhow::Result;
use gen_lsp_types::{DocumentFormattingParams, TextEdit};
use rowan::{TextRange, TextSize};
use squawk_ide::db::{line_index, parse};
use squawk_line_index::find_newline;

use crate::global_state::Snapshot;
use crate::lsp_utils;

pub(crate) fn handle_formatting(
    snapshot: &Snapshot,
    params: DocumentFormattingParams,
) -> Result<Option<Vec<TextEdit>>> {
    let db = snapshot.db();
    let file = snapshot.file(&params.text_document.uri).unwrap();
    let content = file.content(db);
    let line_ending = find_newline(content)
        .map(|(_, ending)| ending)
        .unwrap_or_default();
    let parse = parse(db, file);
    if !parse.errors().is_empty() {
        return Ok(Some(Vec::new()));
    }
    let formatted = squawk_fmt::fmt(&parse.tree(), line_ending)?;

    if formatted == content.as_ref() {
        return Ok(Some(Vec::new()));
    }

    let range = TextRange::new(TextSize::default(), TextSize::try_from(content.len())?);
    let range = lsp_utils::range(&line_index(db, file), range);

    Ok(Some(vec![TextEdit {
        range,
        new_text: formatted,
    }]))
}

#[cfg(test)]
mod tests {
    use crossbeam_channel::unbounded;
    use gen_lsp_types::{
        FormattingOptions, Position, Range, TextDocumentIdentifier, WorkDoneProgressParams,
    };
    use url::Url;

    use super::*;
    use crate::global_state::GlobalState;

    #[test]
    fn formats_the_entire_document() {
        let (sender, _receiver) = unbounded();
        let mut state = GlobalState::new(sender);
        let uri = Url::parse("file:///migration.sql").unwrap();
        state.set(
            uri.clone(),
            "select foo,bar from users where id=1;\n".to_owned(),
        );

        let params = DocumentFormattingParams::new(
            TextDocumentIdentifier::new(uri),
            FormattingOptions::new(4, true, None, None, None),
            WorkDoneProgressParams::default(),
        );
        let edits = handle_formatting(&state.snapshot(), params)
            .unwrap()
            .unwrap();

        assert_eq!(
            edits,
            vec![TextEdit {
                range: Range::new(Position::new(0, 0), Position::new(1, 0)),
                new_text: "select foo, bar from users where id = 1;\n".to_owned(),
            }]
        );
    }
}
