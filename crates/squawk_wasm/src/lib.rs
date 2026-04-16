use line_index::LineIndex;
use log::info;
use rowan::{TextRange, TextSize};
use salsa::Setter;
use serde::{Deserialize, Serialize};
use squawk_ide::builtins::builtins_line_index;
use squawk_ide::db::{self, Database, File};
use squawk_ide::folding_ranges::{FoldKind, folding_ranges};
use squawk_ide::goto_definition::FileId;
use squawk_ide::semantic_tokens::{SemanticTokenType, semantic_tokens};
use squawk_syntax::ast::AstNode;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Error;

const SEMANTIC_TOKEN_TYPES: &[&str] = &[
    "comment",
    "function",
    "keyword",
    "namespace",
    "number",
    "operator",
    "parameter",
    "property",
    "string",
    "struct",
    "type",
    "variable",
];

const SEMANTIC_TOKEN_MODIFIERS: &[&str] = &["declaration", "definition", "readonly"];

fn semantic_token_type_name(ty: SemanticTokenType) -> &'static str {
    match ty {
        SemanticTokenType::Bool | SemanticTokenType::Keyword => "keyword",
        SemanticTokenType::Column => "variable",
        SemanticTokenType::Comment => "comment",
        SemanticTokenType::Function => "function",
        SemanticTokenType::Name | SemanticTokenType::NameRef => "variable",
        SemanticTokenType::Number => "number",
        SemanticTokenType::Operator | SemanticTokenType::Punctuation => "operator",
        SemanticTokenType::Parameter | SemanticTokenType::PositionalParam => "parameter",
        SemanticTokenType::Schema => "namespace",
        SemanticTokenType::String => "string",
        SemanticTokenType::Table => "struct",
        SemanticTokenType::Type => "type",
    }
}

fn semantic_token_type_index(ty: SemanticTokenType) -> u32 {
    let name = semantic_token_type_name(ty);
    SEMANTIC_TOKEN_TYPES
        .iter()
        .position(|it| *it == name)
        .unwrap() as u32
}

struct EncodedSemanticToken {
    line: u32,
    start: u32,
    length: u32,
    token_type: SemanticTokenType,
    modifiers: u32,
}

struct SemanticTokenEncoder {
    data: Vec<u32>,
    prev_line: u32,
    prev_start: u32,
}

impl SemanticTokenEncoder {
    fn with_capacity(token_count: usize) -> Self {
        Self {
            data: Vec::with_capacity(token_count * 5),
            prev_line: 0,
            prev_start: 0,
        }
    }

    fn push(&mut self, token: EncodedSemanticToken) {
        let delta_line = token.line - self.prev_line;
        let delta_start = if delta_line == 0 {
            token.start - self.prev_start
        } else {
            token.start
        };

        self.data.extend_from_slice(&[
            delta_line,
            delta_start,
            token.length,
            semantic_token_type_index(token.token_type),
            token.modifiers,
        ]);

        self.prev_line = token.line;
        self.prev_start = token.start;
    }

    fn finish(self) -> Vec<u32> {
        self.data
    }
}

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
pub struct SquawkDatabase {
    db: Database,
    file: Option<File>,
}

#[wasm_bindgen]
#[allow(clippy::new_without_default)]
impl SquawkDatabase {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SquawkDatabase {
        SquawkDatabase {
            db: Database::default(),
            file: None,
        }
    }

    pub fn open_file(&mut self, content: String) {
        let file = File::new(&self.db, content.into());
        self.file = Some(file);
    }

    pub fn update_file(&mut self, content: String) {
        if let Some(file) = self.file {
            file.set_content(&mut self.db).to(content.into());
        }
    }

    fn file(&self) -> Result<File, Error> {
        self.file
            .ok_or_else(|| Error::new("No file open. Call open_file first."))
    }

    pub fn dump_cst(&self) -> Result<String, Error> {
        let file = self.file()?;
        let parse = db::parse(&self.db, file);
        Ok(format!("{:#?}", parse.syntax_node()))
    }

    pub fn dump_tokens(&self) -> Result<String, Error> {
        let file = self.file()?;
        let content = file.content(&self.db);
        let tokens = squawk_lexer::tokenize(content);
        let mut start = 0;
        let mut out = String::new();
        for token in tokens {
            let end = start + token.len;
            let text = &content[start as usize..(end) as usize];
            out += &format!("{:?}@{start}..{end} {:?}\n", token.kind, text);
            start += token.len;
        }
        Ok(out)
    }

    pub fn lint(&self) -> Result<JsValue, Error> {
        let file = self.file()?;
        let content = file.content(&self.db);
        let mut linter = squawk_linter::Linter::with_all_rules();
        let parse = db::parse(&self.db, file);
        let parse_errors = parse.errors();

        let line_index = db::line_index(&self.db, file);

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

        let lint_errors = linter.lint(&parse, content);
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

    pub fn goto_definition(&self, line: u32, col: u32) -> Result<JsValue, Error> {
        let file = self.file()?;
        let current_line_index = db::line_index(&self.db, file);
        let offset = position_to_offset(&current_line_index, line, col)?;
        let builtins_li = builtins_line_index(&self.db);
        let result = squawk_ide::goto_definition::goto_definition(&self.db, file, offset);

        let response: Vec<LocationRange> = result
            .into_iter()
            .map(|location| {
                let range = location.range;
                let (file, line_index) = match location.file {
                    FileId::Current => ("current", &current_line_index),
                    FileId::Builtins => ("builtins", &builtins_li),
                };
                let start = line_index.line_col(range.start());
                let end = line_index.line_col(range.end());
                let start_wide = line_index
                    .to_wide(line_index::WideEncoding::Utf16, start)
                    .unwrap();
                let end_wide = line_index
                    .to_wide(line_index::WideEncoding::Utf16, end)
                    .unwrap();

                LocationRange {
                    file: file.to_string(),
                    start_line: start_wide.line,
                    start_column: start_wide.col,
                    end_line: end_wide.line,
                    end_column: end_wide.col,
                }
            })
            .collect();

        serde_wasm_bindgen::to_value(&response).map_err(into_error)
    }

    pub fn hover(&self, line: u32, col: u32) -> Result<JsValue, Error> {
        let file = self.file()?;
        let line_index = db::line_index(&self.db, file);
        let offset = position_to_offset(&line_index, line, col)?;
        let result = squawk_ide::hover::hover(&self.db, file, offset);

        let converted = result.map(|hover| WasmHover {
            snippet: hover.snippet,
            comment: hover.comment,
        });

        serde_wasm_bindgen::to_value(&converted).map_err(into_error)
    }

    pub fn find_references(&self, line: u32, col: u32) -> Result<JsValue, Error> {
        let file = self.file()?;
        let line_index = db::line_index(&self.db, file);
        let offset = position_to_offset(&line_index, line, col)?;
        let references = squawk_ide::find_references::find_references(&self.db, file, offset);
        let builtins_li = builtins_line_index(&self.db);
        let locations: Vec<LocationRange> = references
            .iter()
            .map(|loc| {
                let (li, file) = match loc.file {
                    FileId::Current => (&line_index, "current"),
                    FileId::Builtins => (&builtins_li, "builtins"),
                };
                let start = li.line_col(loc.range.start());
                let end = li.line_col(loc.range.end());
                let start_wide = li.to_wide(line_index::WideEncoding::Utf16, start).unwrap();
                let end_wide = li.to_wide(line_index::WideEncoding::Utf16, end).unwrap();

                LocationRange {
                    file: file.to_string(),
                    start_line: start_wide.line,
                    start_column: start_wide.col,
                    end_line: end_wide.line,
                    end_column: end_wide.col,
                }
            })
            .collect();

        serde_wasm_bindgen::to_value(&locations).map_err(into_error)
    }

    pub fn document_symbols(&self) -> Result<JsValue, Error> {
        let file = self.file()?;
        let line_index = db::line_index(&self.db, file);
        let symbols = squawk_ide::document_symbols::document_symbols(&self.db, file);

        let converted: Vec<WasmDocumentSymbol> = symbols
            .into_iter()
            .map(|s| convert_document_symbol(&line_index, s))
            .collect();

        serde_wasm_bindgen::to_value(&converted).map_err(into_error)
    }

    pub fn code_actions(&self, line: u32, col: u32) -> Result<JsValue, Error> {
        let file = self.file()?;
        let line_index = db::line_index(&self.db, file);
        let offset = position_to_offset(&line_index, line, col)?;
        let actions = squawk_ide::code_actions::code_actions(&self.db, file, offset);

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
                            squawk_ide::code_actions::ActionKind::RefactorRewrite => {
                                "refactor.rewrite"
                            }
                        }
                        .to_string(),
                    }
                })
                .collect::<Vec<_>>()
        });

        serde_wasm_bindgen::to_value(&converted).map_err(into_error)
    }

    pub fn inlay_hints(&self) -> Result<JsValue, Error> {
        let file = self.file()?;
        let line_index = db::line_index(&self.db, file);
        let hints = squawk_ide::inlay_hints::inlay_hints(&self.db, file);

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

    pub fn folding_ranges(&self) -> Result<JsValue, Error> {
        let file = self.file()?;
        let line_index = db::line_index(&self.db, file);
        let folds = folding_ranges(&self.db, file);

        let converted: Vec<WasmFoldingRange> = folds
            .into_iter()
            .map(|fold| {
                let start = line_index.line_col(fold.range.start());
                let end = line_index.line_col(fold.range.end());
                let start_wide = line_index
                    .to_wide(line_index::WideEncoding::Utf16, start)
                    .unwrap();
                let end_wide = line_index
                    .to_wide(line_index::WideEncoding::Utf16, end)
                    .unwrap();

                WasmFoldingRange {
                    start_line: start_wide.line,
                    end_line: end_wide.line,
                    kind: match fold.kind {
                        FoldKind::Comment => "comment",
                        _ => "region",
                    }
                    .to_string(),
                }
            })
            .collect();

        serde_wasm_bindgen::to_value(&converted).map_err(into_error)
    }

    pub fn selection_ranges(&self, positions: Vec<JsValue>) -> Result<JsValue, Error> {
        let file = self.file()?;
        let parse = db::parse(&self.db, file);
        let line_index = db::line_index(&self.db, file);
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

    pub fn semantic_tokens(&self) -> Result<Vec<u32>, Error> {
        let file = self.file()?;
        let line_index = db::line_index(&self.db, file);
        let content = file.content(&self.db);
        let tokens = semantic_tokens(&self.db, file, None);

        let mut encoder = SemanticTokenEncoder::with_capacity(tokens.len());

        // Duplicated from squawk-server, fyi
        for token in &tokens {
            // Taken from rust-analyzer, this solves the case where we have a
            // multi line semantic token which isn't supported by the LSP spec.
            // see: https://github.com/rust-lang/rust-analyzer/blob/2efc80078029894eec0699f62ec8d5c1a56af763/crates/rust-analyzer/src/lsp/to_proto.rs#L781C28-L781C28
            for mut text_range in line_index.lines(token.range) {
                if content[text_range].ends_with('\n') {
                    text_range =
                        TextRange::new(text_range.start(), text_range.end() - TextSize::of('\n'));
                }
                let start_lc = line_index.line_col(text_range.start());
                let end_lc = line_index.line_col(text_range.end());
                let start_wide = line_index
                    .to_wide(line_index::WideEncoding::Utf16, start_lc)
                    .unwrap();
                let end_wide = line_index
                    .to_wide(line_index::WideEncoding::Utf16, end_lc)
                    .unwrap();

                encoder.push(EncodedSemanticToken {
                    line: start_wide.line,
                    start: start_wide.col,
                    length: end_wide.col - start_wide.col,
                    token_type: token.token_type,
                    // TODO: once we get modifiers going, we'll need to update this
                    modifiers: 0,
                });
            }
        }

        Ok(encoder.finish())
    }

    pub fn semantic_tokens_legend() -> Result<JsValue, Error> {
        let legend = SemanticTokensLegend {
            token_types: SEMANTIC_TOKEN_TYPES.to_vec(),
            token_modifiers: SEMANTIC_TOKEN_MODIFIERS.to_vec(),
        };
        serde_wasm_bindgen::to_value(&legend).map_err(into_error)
    }

    pub fn completion(&self, line: u32, col: u32) -> Result<JsValue, Error> {
        let file = self.file()?;
        let line_index = db::line_index(&self.db, file);
        let offset = position_to_offset(&line_index, line, col)?;
        let items = squawk_ide::completion::completion(&self.db, file, offset);

        let converted: Vec<WasmCompletionItem> = items
            .into_iter()
            .map(|item| WasmCompletionItem {
                label: item.label,
                kind: match item.kind {
                    squawk_ide::completion::CompletionItemKind::Keyword => "keyword",
                    squawk_ide::completion::CompletionItemKind::Table => "table",
                    squawk_ide::completion::CompletionItemKind::Column => "column",
                    squawk_ide::completion::CompletionItemKind::Function => "function",
                    squawk_ide::completion::CompletionItemKind::Schema => "schema",
                    squawk_ide::completion::CompletionItemKind::Type => "type",
                    squawk_ide::completion::CompletionItemKind::Snippet => "snippet",
                    squawk_ide::completion::CompletionItemKind::Operator => "operator",
                }
                .to_string(),
                detail: item.detail,
                insert_text: item.insert_text,
                insert_text_format: item.insert_text_format.map(|fmt| {
                    match fmt {
                        squawk_ide::completion::CompletionInsertTextFormat::PlainText => {
                            "plainText"
                        }
                        squawk_ide::completion::CompletionInsertTextFormat::Snippet => "snippet",
                    }
                    .to_string()
                }),
                trigger_completion_after_insert: item.trigger_completion_after_insert,
            })
            .collect();

        serde_wasm_bindgen::to_value(&converted).map_err(into_error)
    }
}

fn into_error<E: std::fmt::Display>(err: E) -> Error {
    Error::new(&err.to_string())
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
            squawk_ide::document_symbols::DocumentSymbolKind::EventTrigger => "event_trigger",
            squawk_ide::document_symbols::DocumentSymbolKind::Role => "role",
            squawk_ide::document_symbols::DocumentSymbolKind::Policy => "policy",
            squawk_ide::document_symbols::DocumentSymbolKind::Type => "type",
            squawk_ide::document_symbols::DocumentSymbolKind::Enum => "enum",
            squawk_ide::document_symbols::DocumentSymbolKind::Index => "index",
            squawk_ide::document_symbols::DocumentSymbolKind::Domain => "domain",
            squawk_ide::document_symbols::DocumentSymbolKind::Sequence => "sequence",
            squawk_ide::document_symbols::DocumentSymbolKind::Trigger => "trigger",
            squawk_ide::document_symbols::DocumentSymbolKind::Tablespace => "tablespace",
            squawk_ide::document_symbols::DocumentSymbolKind::Database => "database",
            squawk_ide::document_symbols::DocumentSymbolKind::Server => "server",
            squawk_ide::document_symbols::DocumentSymbolKind::Extension => "extension",
            squawk_ide::document_symbols::DocumentSymbolKind::Column => "column",
            squawk_ide::document_symbols::DocumentSymbolKind::Variant => "variant",
            squawk_ide::document_symbols::DocumentSymbolKind::Cursor => "cursor",
            squawk_ide::document_symbols::DocumentSymbolKind::PreparedStatement => {
                "prepared_statement"
            }
            squawk_ide::document_symbols::DocumentSymbolKind::Channel => "channel",
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
    range_start: usize,
    range_end: usize,
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

#[derive(Serialize)]
struct LocationRange {
    file: String,
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
struct WasmHover {
    snippet: String,
    comment: Option<String>,
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

#[derive(Serialize)]
struct WasmInlayHint {
    line: u32,
    column: u32,
    label: String,
    kind: String,
}

#[derive(Serialize)]
struct WasmFoldingRange {
    start_line: u32,
    end_line: u32,
    kind: String,
}

#[derive(Serialize)]
struct WasmSelectionRange {
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
}

#[derive(Serialize)]
struct SemanticTokensLegend {
    #[serde(rename = "tokenTypes")]
    token_types: Vec<&'static str>,
    #[serde(rename = "tokenModifiers")]
    token_modifiers: Vec<&'static str>,
}

#[derive(Serialize)]
struct WasmCompletionItem {
    label: String,
    kind: String,
    detail: Option<String>,
    insert_text: Option<String>,
    insert_text_format: Option<String>,
    trigger_completion_after_insert: bool,
}

#[derive(Deserialize)]
struct Position {
    line: u32,
    column: u32,
}
