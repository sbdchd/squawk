use line_index::LineIndex;
use log::info;
use serde::Serialize;
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

#[allow(dead_code)]
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
        }
    });

    let lint_errors = linter.lint(parse, &text);
    let errors = lint_errors.into_iter().map(|x| {
        let start = line_index.line_col(x.text_range.start());
        let end = line_index.line_col(x.text_range.end());
        let start = line_index
            .to_wide(line_index::WideEncoding::Utf16, start)
            .unwrap();
        let end = line_index
            .to_wide(line_index::WideEncoding::Utf16, end)
            .unwrap();
        LintError {
            code: x.code.to_string(),
            range_start: x.text_range.start().into(),
            range_end: x.text_range.end().into(),
            message: x.message.clone(),
            messages: x.messages.clone(),
            // parser errors should be error
            severity: Severity::Warning,
            start_line_number: start.line,
            start_column: start.col,
            end_line_number: end.line,
            end_column: end.col,
        }
    });

    let mut errors_to_dump = errors.chain(parse_errors).collect::<Vec<_>>();
    errors_to_dump.sort_by_key(|k| (k.start_line_number, k.start_column));

    serde_wasm_bindgen::to_value(&errors_to_dump).map_err(into_error)
}

fn into_error<E: std::fmt::Display>(err: E) -> Error {
    Error::new(&err.to_string())
}
