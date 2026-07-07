use std::io::{BufRead, BufReader, Read, Write};
use std::ops::Range;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet};
use anyhow::{Context, Result, bail};
use serde_json::{Value, json};

use crate::LspEvalArgs;

const MARKER: &str = "$0";
const DOC_URI: &str = "file:///eval.sql";

/// Start the real LSP server (`squawk server`), open a snippet with a `$0`
/// marker, ask it for the goto-definition at the marker, and render the
/// response with annotate-snippets, the same way the `goto_definition.rs` tests
/// do.
pub(crate) fn lsp_eval(args: LspEvalArgs) -> Result<()> {
    let raw = match args.sql {
        Some(sql) => sql,
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    let Some(marker_offset) = raw.find(MARKER) else {
        bail!("No marker found in SQL. Did you forget to add a marker `{MARKER}`?");
    };
    let content = raw.replacen(MARKER, "", 1);

    // For goto def we want the previous character since we usually put the
    // marker after the item we're trying to go to def on, matching the tests.
    let source_span = char_span_before(&content, marker_offset);
    let position = byte_to_position(&content, source_span.start);

    let mut server = Server::spawn()?;
    server.initialize()?;
    server.did_open(&content)?;
    let response = server.goto_definition(position)?;
    server.shutdown()?;

    let output = render(&content, source_span, &response);
    print!("{output}");

    Ok(())
}

fn render(content: &str, source_span: Range<usize>, locations: &[Location]) -> String {
    let mut snippet = Snippet::source(content).fold(true);

    let mut other_files: Vec<&Location> = vec![];
    for (i, location) in locations.iter().enumerate() {
        let label = format!("{}. destination", i + 2);
        if location.uri == DOC_URI {
            let range =
                position_to_byte(content, location.start)..position_to_byte(content, location.end);
            snippet = snippet.annotation(AnnotationKind::Context.span(range).label(label));
        } else {
            other_files.push(location);
        }
    }

    snippet = snippet.annotation(AnnotationKind::Context.span(source_span).label("1. source"));

    let groups = vec![Level::INFO.primary_title("definition").element(snippet)];

    let renderer = Renderer::plain().decor_style(DecorStyle::Unicode);
    let mut out = renderer
        .render(&groups)
        .to_string()
        .replace("info: definition", "");

    if locations.is_empty() {
        out.push_str("\nno definition found\n");
    }
    for location in other_files {
        out.push_str(&format!(
            "\ndestination in {} at {:?}..{:?}\n",
            location.uri, location.start, location.end
        ));
    }

    out
}

struct Location {
    uri: String,
    start: (u32, u32),
    end: (u32, u32),
}

struct Server {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next_id: i64,
}

impl Server {
    fn spawn() -> Result<Self> {
        // Prefer a prebuilt binary if one is provided, otherwise build & run via
        // cargo. stderr is inherited so cargo build progress is visible.
        let mut command = if let Ok(bin) = std::env::var("SQUAWK_BIN") {
            let mut command = Command::new(bin);
            command.arg("server");
            command
        } else {
            let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
            let mut command = Command::new(cargo);
            command.args(["run", "--quiet", "-p", "squawk", "--", "server"]);
            command
        };

        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .context("failed to spawn the squawk server")?;

        let stdin = child.stdin.take().context("missing server stdin")?;
        let stdout = BufReader::new(child.stdout.take().context("missing server stdout")?);

        Ok(Self {
            child,
            stdin,
            stdout,
            next_id: 0,
        })
    }

    fn request(&mut self, method: &str, params: Value) -> Result<Value> {
        self.next_id += 1;
        let id = self.next_id;
        write_message(
            &mut self.stdin,
            &json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}),
        )?;
        self.wait_for_response(id)
    }

    fn notify(&mut self, method: &str, params: Value) -> Result<()> {
        write_message(
            &mut self.stdin,
            &json!({"jsonrpc": "2.0", "method": method, "params": params}),
        )
    }

    fn wait_for_response(&mut self, id: i64) -> Result<Value> {
        loop {
            let message = read_message(&mut self.stdout)?
                .context("server closed the connection before responding")?;
            if message.get("id").and_then(Value::as_i64) == Some(id) {
                if let Some(error) = message.get("error") {
                    bail!("server returned an error: {error}");
                }
                return Ok(message.get("result").cloned().unwrap_or(Value::Null));
            }
            // Ignore server-initiated notifications and requests.
        }
    }

    fn initialize(&mut self) -> Result<()> {
        self.request(
            "initialize",
            json!({"processId": null, "rootUri": null, "capabilities": {}}),
        )?;
        self.notify("initialized", json!({}))?;
        Ok(())
    }

    fn did_open(&mut self, content: &str) -> Result<()> {
        self.notify(
            "textDocument/didOpen",
            json!({
                "textDocument": {
                    "uri": DOC_URI,
                    "languageId": "sql",
                    "version": 1,
                    "text": content,
                }
            }),
        )
    }

    fn goto_definition(&mut self, position: (u32, u32)) -> Result<Vec<Location>> {
        let result = self.request(
            "textDocument/definition",
            json!({
                "textDocument": {"uri": DOC_URI},
                "position": {"line": position.0, "character": position.1},
            }),
        )?;
        Ok(parse_locations(&result))
    }

    fn shutdown(&mut self) -> Result<()> {
        self.request("shutdown", Value::Null)?;
        self.notify("exit", Value::Null)?;
        self.child.wait()?;
        Ok(())
    }
}

fn parse_locations(result: &Value) -> Vec<Location> {
    let items = match result {
        Value::Array(items) => items.clone(),
        Value::Object(_) => vec![result.clone()],
        _ => vec![],
    };
    items
        .iter()
        .filter_map(|item| {
            let range = item.get("range")?;
            Some(Location {
                uri: item.get("uri")?.as_str()?.to_owned(),
                start: position(range.get("start")?),
                end: position(range.get("end")?),
            })
        })
        .collect()
}

fn position(value: &Value) -> (u32, u32) {
    let line = value.get("line").and_then(Value::as_u64).unwrap_or(0) as u32;
    let character = value.get("character").and_then(Value::as_u64).unwrap_or(0) as u32;
    (line, character)
}

// The server encodes columns as utf-8 byte offsets within the line, so we match
// that here instead of utf-16.
fn position_to_byte(text: &str, position: (u32, u32)) -> usize {
    let (line, col) = position;
    let line_start = text
        .split_inclusive('\n')
        .take(line as usize)
        .map(str::len)
        .sum::<usize>();
    (line_start + col as usize).min(text.len())
}

fn byte_to_position(text: &str, byte: usize) -> (u32, u32) {
    let before = &text[..byte];
    let line = before.matches('\n').count() as u32;
    let line_start = before.rfind('\n').map_or(0, |i| i + 1);
    let col = (byte - line_start) as u32;
    (line, col)
}

fn char_span_before(content: &str, marker_offset: usize) -> Range<usize> {
    if marker_offset == 0 {
        let len = content[..].chars().next().map_or(0, char::len_utf8);
        return 0..len;
    }
    let start = content[..marker_offset]
        .char_indices()
        .last()
        .map_or(0, |(offset, _)| offset);
    let len = content[start..].chars().next().map_or(0, char::len_utf8);
    start..start + len
}

fn read_message(reader: &mut impl BufRead) -> Result<Option<Value>> {
    let mut content_length: Option<usize> = None;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            return Ok(None);
        }
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }
        if let Some(rest) = line.strip_prefix("Content-Length:") {
            content_length = Some(rest.trim().parse()?);
        }
    }
    let len = content_length.context("missing Content-Length header")?;
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;
    Ok(Some(serde_json::from_slice(&buf)?))
}

fn write_message(writer: &mut impl Write, value: &Value) -> Result<()> {
    let body = serde_json::to_vec(value)?;
    write!(writer, "Content-Length: {}\r\n\r\n", body.len())?;
    writer.write_all(&body)?;
    writer.flush()?;
    Ok(())
}
