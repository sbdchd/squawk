use line_index::LineIndex;
use log::info;
use rowan::TextRange;
use serde::{Deserialize, Serialize};
use squawk_syntax::ast::AstNode;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Error;

#[wasm_bindgen(start)]
pub fn run() {
    use log::Level;

    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).expect("Initializing logger went wrong.");
    info!("init!");
}

#[wasm_bindgen]
pub fn dump_cst(text: String) -> String {
    let parse = squawk_syntax::SourceFile::parse(&text);
    format!("{:#?}", parse.syntax_node())
}

#[wasm_bindgen]
pub fn dump_tokens(text: String) -> String {
    let tokens = squawk_lexer::tokenize(&text);
    let mut start = 0;
    let mut out = String::new();
    for token in tokens {
        let end = start + token.len;
        let content = &text[start as usize..(end) as usize];
        out += &format!("{:?}@{start}..{end} {:?}\n", token.kind, content);
        start += token.len;
    }
    out
}

#[expect(unused)]
#[derive(Serialize)]
enum Severity {
    Hint,
    Info,
    Warning,
    Error,
}

#[derive(Serialize)]
struct LintError {
    severity: Severity,
    code: String,
    message: String,
    start_line_number: u32,
    start_column: u32,
    end_line_number: u32,
    end_column: u32,
    // used for the linter tab
    range_start: usize,
    // used for the linter tab
    range_end: usize,
    // used for the linter tab
    messages: Vec<String>,
    fix: Option<Fix>,
}

#[derive(Serialize)]
struct Fix {
    title: String,
    edits: Vec<TextEdit>,
}

#[derive(Serialize)]
struct TextEdit {
    start_line_number: u32,
    start_column: u32,
    end_line_number: u32,
    end_column: u32,
    text: String,
}

#[wasm_bindgen]
pub fn lint(text: String) -> Result<JsValue, Error> {
    let mut linter = squawk_linter::Linter::with_all_rules();
    let parse = squawk_syntax::SourceFile::parse(&text);
    let parse_errors = parse.errors();

    let line_index = LineIndex::new(&text);

    // TODO: chain these with other stuff
    let parse_errors = parse_errors.iter().map(|x| {
        let range_start = x.range().start();
        let range_end = x.range().end();
        let start = line_index.line_col(range_start);
        let end = line_index.line_col(range_end);
        let start = line_index
            .to_wide(line_index::WideEncoding::Utf16, start)
            .unwrap();
        let end = line_index
            .to_wide(line_index::WideEncoding::Utf16, end)
            .unwrap();
        LintError {
            severity: Severity::Error,
            code: "syntax-error".to_string(),
            message: x.message().to_string(),
            start_line_number: start.line,
            start_column: start.col,
            end_line_number: end.line,
            end_column: end.col,
            range_start: range_start.into(),
            range_end: range_end.into(),
            messages: vec![],
            fix: None,
        }
    });

    let lint_errors = linter.lint(&parse, &text);
    let errors = lint_errors.into_iter().map(|x| {
        let start = line_index.line_col(x.text_range.start());
        let end = line_index.line_col(x.text_range.end());
        let start = line_index
            .to_wide(line_index::WideEncoding::Utf16, start)
            .unwrap();
        let end = line_index
            .to_wide(line_index::WideEncoding::Utf16, end)
            .unwrap();

        let messages = x.help.into_iter().collect();

        let fix = x.fix.map(|fix| {
            let edits = fix
                .edits
                .into_iter()
                .map(|edit| {
                    let start_pos = line_index.line_col(edit.text_range.start());
                    let end_pos = line_index.line_col(edit.text_range.end());
                    let start_wide = line_index
                        .to_wide(line_index::WideEncoding::Utf16, start_pos)
                        .unwrap();
                    let end_wide = line_index
                        .to_wide(line_index::WideEncoding::Utf16, end_pos)
                        .unwrap();

                    TextEdit {
                        start_line_number: start_wide.line,
                        start_column: start_wide.col,
                        end_line_number: end_wide.line,
                        end_column: end_wide.col,
                        text: edit.text.unwrap_or_default(),
                    }
                })
                .collect();

            Fix {
                title: fix.title,
                edits,
            }
        });

        LintError {
            code: x.code.to_string(),
            range_start: x.text_range.start().into(),
            range_end: x.text_range.end().into(),
            message: x.message.clone(),
            messages,
            // parser errors should be error
            severity: Severity::Warning,
            start_line_number: start.line,
            start_column: start.col,
            end_line_number: end.line,
            end_column: end.col,
            fix,
        }
    });

    let mut errors_to_dump = errors.chain(parse_errors).collect::<Vec<_>>();
    errors_to_dump.sort_by_key(|k| (k.start_line_number, k.start_column));

    serde_wasm_bindgen::to_value(&errors_to_dump).map_err(into_error)
}

fn into_error<E: std::fmt::Display>(err: E) -> Error {
    Error::new(&err.to_string())
}

#[wasm_bindgen]
pub fn goto_definition(content: String, line: u32, col: u32) -> Result<JsValue, Error> {
    let parse = squawk_syntax::SourceFile::parse(&content);
    let line_index = LineIndex::new(&content);
    let offset = position_to_offset(&line_index, line, col)?;
    let result = squawk_ide::goto_definition::goto_definition(parse.tree(), offset);

    let response: Vec<LocationRange> = result
        .into_iter()
        .map(|range| {
            let start = line_index.line_col(range.start());
            let end = line_index.line_col(range.end());
            let start_wide = line_index
                .to_wide(line_index::WideEncoding::Utf16, start)
                .unwrap();
            let end_wide = line_index
                .to_wide(line_index::WideEncoding::Utf16, end)
                .unwrap();

            LocationRange {
                start_line: start_wide.line,
                start_column: start_wide.col,
                end_line: end_wide.line,
                end_column: end_wide.col,
            }
        })
        .collect();

    serde_wasm_bindgen::to_value(&response).map_err(into_error)
}

#[wasm_bindgen]
pub fn hover(content: String, line: u32, col: u32) -> Result<JsValue, Error> {
    let parse = squawk_syntax::SourceFile::parse(&content);
    let line_index = LineIndex::new(&content);
    let offset = position_to_offset(&line_index, line, col)?;
    let result = squawk_ide::hover::hover(&parse.tree(), offset);

    serde_wasm_bindgen::to_value(&result).map_err(into_error)
}

#[wasm_bindgen]
pub fn find_references(content: String, line: u32, col: u32) -> Result<JsValue, Error> {
    let parse = squawk_syntax::SourceFile::parse(&content);
    let line_index = LineIndex::new(&content);
    let offset = position_to_offset(&line_index, line, col)?;
    let references = squawk_ide::find_references::find_references(&parse.tree(), offset);

    let locations: Vec<LocationRange> = references
        .iter()
        .map(|range| {
            let start = line_index.line_col(range.start());
            let end = line_index.line_col(range.end());
            let start_wide = line_index
                .to_wide(line_index::WideEncoding::Utf16, start)
                .unwrap();
            let end_wide = line_index
                .to_wide(line_index::WideEncoding::Utf16, end)
                .unwrap();

            LocationRange {
                start_line: start_wide.line,
                start_column: start_wide.col,
                end_line: end_wide.line,
                end_column: end_wide.col,
            }
        })
        .collect();

    serde_wasm_bindgen::to_value(&locations).map_err(into_error)
}

#[wasm_bindgen]
pub fn document_symbols(content: String) -> Result<JsValue, Error> {
    let parse = squawk_syntax::SourceFile::parse(&content);
    let line_index = LineIndex::new(&content);
    let symbols = squawk_ide::document_symbols::document_symbols(&parse.tree());

    let converted: Vec<WasmDocumentSymbol> = symbols
        .into_iter()
        .map(|s| convert_document_symbol(&line_index, s))
        .collect();

    serde_wasm_bindgen::to_value(&converted).map_err(into_error)
}

#[wasm_bindgen]
pub fn code_actions(content: String, line: u32, col: u32) -> Result<JsValue, Error> {
    let parse = squawk_syntax::SourceFile::parse(&content);
    let line_index = LineIndex::new(&content);
    let offset = position_to_offset(&line_index, line, col)?;
    let actions = squawk_ide::code_actions::code_actions(parse.tree(), offset);

    let converted = actions.map(|actions| {
        actions
            .into_iter()
            .map(|action| {
                let edits = action
                    .edits
                    .into_iter()
                    .map(|edit| {
                        let start_pos = line_index.line_col(edit.text_range.start());
                        let end_pos = line_index.line_col(edit.text_range.end());
                        let start_wide = line_index
                            .to_wide(line_index::WideEncoding::Utf16, start_pos)
                            .unwrap();
                        let end_wide = line_index
                            .to_wide(line_index::WideEncoding::Utf16, end_pos)
                            .unwrap();

                        TextEdit {
                            start_line_number: start_wide.line,
                            start_column: start_wide.col,
                            end_line_number: end_wide.line,
                            end_column: end_wide.col,
                            text: edit.text.unwrap_or_default(),
                        }
                    })
                    .collect();

                WasmCodeAction {
                    title: action.title,
                    edits,
                    kind: match action.kind {
                        squawk_ide::code_actions::ActionKind::QuickFix => "quickfix",
                        squawk_ide::code_actions::ActionKind::RefactorRewrite => "refactor.rewrite",
                    }
                    .to_string(),
                }
            })
            .collect::<Vec<_>>()
    });

    serde_wasm_bindgen::to_value(&converted).map_err(into_error)
}

fn position_to_offset(
    line_index: &LineIndex,
    line: u32,
    col: u32,
) -> Result<rowan::TextSize, Error> {
    let wide_pos = line_index::WideLineCol { line, col };

    let pos = line_index
        .to_utf8(line_index::WideEncoding::Utf16, wide_pos)
        .ok_or_else(|| Error::new("Invalid position"))?;

    line_index
        .offset(pos)
        .ok_or_else(|| Error::new("Invalid position offset"))
}

#[derive(Serialize)]
struct LocationRange {
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
}

#[derive(Serialize)]
struct WasmCodeAction {
    title: String,
    edits: Vec<TextEdit>,
    kind: String,
}

#[derive(Serialize)]
struct WasmDocumentSymbol {
    name: String,
    detail: Option<String>,
    kind: String,
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
    selection_start_line: u32,
    selection_start_column: u32,
    selection_end_line: u32,
    selection_end_column: u32,
    children: Vec<WasmDocumentSymbol>,
}

fn convert_document_symbol(
    line_index: &LineIndex,
    symbol: squawk_ide::document_symbols::DocumentSymbol,
) -> WasmDocumentSymbol {
    let full_start = line_index.line_col(symbol.full_range.start());
    let full_end = line_index.line_col(symbol.full_range.end());
    let full_start_wide = line_index
        .to_wide(line_index::WideEncoding::Utf16, full_start)
        .unwrap();
    let full_end_wide = line_index
        .to_wide(line_index::WideEncoding::Utf16, full_end)
        .unwrap();

    let focus_start = line_index.line_col(symbol.focus_range.start());
    let focus_end = line_index.line_col(symbol.focus_range.end());
    let focus_start_wide = line_index
        .to_wide(line_index::WideEncoding::Utf16, focus_start)
        .unwrap();
    let focus_end_wide = line_index
        .to_wide(line_index::WideEncoding::Utf16, focus_end)
        .unwrap();

    WasmDocumentSymbol {
        name: symbol.name,
        detail: symbol.detail,
        kind: match symbol.kind {
            squawk_ide::document_symbols::DocumentSymbolKind::Schema => "schema",
            squawk_ide::document_symbols::DocumentSymbolKind::Table => "table",
            squawk_ide::document_symbols::DocumentSymbolKind::View => "view",
            squawk_ide::document_symbols::DocumentSymbolKind::MaterializedView => {
                "materialized_view"
            }
            squawk_ide::document_symbols::DocumentSymbolKind::Function => "function",
            squawk_ide::document_symbols::DocumentSymbolKind::Aggregate => "aggregate",
            squawk_ide::document_symbols::DocumentSymbolKind::Procedure => "procedure",
            squawk_ide::document_symbols::DocumentSymbolKind::Type => "type",
            squawk_ide::document_symbols::DocumentSymbolKind::Enum => "enum",
            squawk_ide::document_symbols::DocumentSymbolKind::Column => "column",
            squawk_ide::document_symbols::DocumentSymbolKind::Variant => "variant",
            squawk_ide::document_symbols::DocumentSymbolKind::Cursor => "cursor",
        }
        .to_string(),
        start_line: full_start_wide.line,
        start_column: full_start_wide.col,
        end_line: full_end_wide.line,
        end_column: full_end_wide.col,
        selection_start_line: focus_start_wide.line,
        selection_start_column: focus_start_wide.col,
        selection_end_line: focus_end_wide.line,
        selection_end_column: focus_end_wide.col,
        children: symbol
            .children
            .into_iter()
            .map(|child| convert_document_symbol(line_index, child))
            .collect(),
    }
}

#[wasm_bindgen]
pub fn inlay_hints(content: String) -> Result<JsValue, Error> {
    let parse = squawk_syntax::SourceFile::parse(&content);
    let line_index = LineIndex::new(&content);
    let hints = squawk_ide::inlay_hints::inlay_hints(&parse.tree());

    let converted: Vec<WasmInlayHint> = hints
        .into_iter()
        .map(|hint| {
            let position = line_index.line_col(hint.position);
            let position_wide = line_index
                .to_wide(line_index::WideEncoding::Utf16, position)
                .unwrap();

            WasmInlayHint {
                line: position_wide.line,
                column: position_wide.col,
                label: hint.label,
                kind: match hint.kind {
                    squawk_ide::inlay_hints::InlayHintKind::Type => "type",
                    squawk_ide::inlay_hints::InlayHintKind::Parameter => "parameter",
                }
                .to_string(),
            }
        })
        .collect();

    serde_wasm_bindgen::to_value(&converted).map_err(into_error)
}

#[derive(Deserialize)]
struct Position {
    line: u32,
    column: u32,
}

#[wasm_bindgen]
pub fn selection_ranges(content: String, positions: Vec<JsValue>) -> Result<JsValue, Error> {
    let parse = squawk_syntax::SourceFile::parse(&content);
    let line_index = LineIndex::new(&content);
    let tree = parse.tree();
    let root = tree.syntax();

    let mut results: Vec<Vec<WasmSelectionRange>> = vec![];

    for pos in positions {
        let pos: Position = serde_wasm_bindgen::from_value(pos).map_err(into_error)?;
        let offset = position_to_offset(&line_index, pos.line, pos.column)?;

        let mut ranges = vec![];
        let mut range = TextRange::new(offset, offset);

        for _ in 0..20 {
            let next = squawk_ide::expand_selection::extend_selection(root, range);
            if next == range {
                break;
            }

            let start = line_index.line_col(next.start());
            let end = line_index.line_col(next.end());
            let start_wide = line_index
                .to_wide(line_index::WideEncoding::Utf16, start)
                .unwrap();
            let end_wide = line_index
                .to_wide(line_index::WideEncoding::Utf16, end)
                .unwrap();

            ranges.push(WasmSelectionRange {
                start_line: start_wide.line,
                start_column: start_wide.col,
                end_line: end_wide.line,
                end_column: end_wide.col,
            });

            range = next;
        }

        results.push(ranges);
    }

    serde_wasm_bindgen::to_value(&results).map_err(into_error)
}

#[derive(Serialize)]
struct WasmInlayHint {
    line: u32,
    column: u32,
    label: String,
    kind: String,
}

#[derive(Serialize)]
struct WasmSelectionRange {
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
}
