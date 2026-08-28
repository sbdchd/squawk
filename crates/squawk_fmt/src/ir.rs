//! Rendering of the formatter's pretty printing IR.
//!
//! The formatter builds a `tiny_pretty::Doc` tree and then prints it to text.
//! When a statement formats in a surprising way, it's useful to see that tree
//! instead of the text it printed to, so this renders it in a compact,
//! indented form:
//!
//! ```text
//! list [
//!   group [
//!     text "select"
//!     nest 2 [
//!       line_or_space
//!       text "1"
//!     ]
//!   ]
//!   text ";"
//! ]
//! ```
//!
//! `nil` is a no-op when printing, so we leave it out of a group's or list's
//! children. It's only shown where it's positional, i.e. as a branch of
//! `flat_or_break` or `union`.
//!
//! `Doc` derives `Debug`, but `Doc::Nest`'s payload type isn't exported by
//! `tiny_pretty`, so we can't walk the tree ourselves. Instead we parse the
//! `Debug` output, which is a small, regular grammar, and re-render it. If
//! that parse ever fails we fall back to the raw `{:#?}` output.

use tiny_pretty::Doc;

pub(crate) fn render(doc: &Doc<'_>) -> String {
    let Some(node) = Parser::new(&format!("{doc:?}")).parse() else {
        return format!("{doc:#?}\n");
    };
    let mut out = String::new();
    write_doc(&mut out, &node, 0);
    out.push('\n');
    out
}

#[derive(Clone)]
enum Node {
    /// A unit variant, e.g. `Nil`, or a bare `true`.
    Name(String),
    /// A tuple variant, e.g. `Text("select")`.
    Call { name: String, args: Vec<Node> },
    /// A bracketed list of docs, e.g. `[Nil, Text("select")]`.
    Seq(Vec<Node>),
    /// A literal, e.g. `"select"`, `' '`, or `2`.
    Atom(String),
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(text: &str) -> Self {
        Self {
            chars: text.chars().collect(),
            pos: 0,
        }
    }

    fn parse(mut self) -> Option<Node> {
        let node = self.node()?;
        self.skip_whitespace();
        if self.pos == self.chars.len() {
            Some(node)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn eat(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(char::is_whitespace) {
            self.pos += 1;
        }
    }

    fn node(&mut self) -> Option<Node> {
        self.skip_whitespace();
        match self.peek()? {
            '[' => {
                self.pos += 1;
                let items = self.items(']')?;
                Some(Node::Seq(items))
            }
            '"' => Some(Node::Atom(self.quoted('"')?)),
            '\'' => Some(Node::Atom(self.quoted('\'')?)),
            c if c.is_ascii_digit() => {
                let start = self.pos;
                while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                    self.pos += 1;
                }
                Some(Node::Atom(self.chars[start..self.pos].iter().collect()))
            }
            c if c.is_alphabetic() || c == '_' => {
                let start = self.pos;
                while self.peek().is_some_and(|c| c.is_alphanumeric() || c == '_') {
                    self.pos += 1;
                }
                let name: String = self.chars[start..self.pos].iter().collect();
                self.skip_whitespace();
                if self.eat('(') {
                    let args = self.items(')')?;
                    Some(Node::Call { name, args })
                } else {
                    Some(Node::Name(name))
                }
            }
            _ => None,
        }
    }

    /// Comma separated nodes up to, and including, `close`.
    fn items(&mut self, close: char) -> Option<Vec<Node>> {
        let mut items = vec![];
        loop {
            self.skip_whitespace();
            if self.eat(close) {
                return Some(items);
            }
            items.push(self.node()?);
            self.skip_whitespace();
            // a trailing comma before the closer is allowed
            self.eat(',');
        }
    }

    /// A string or char literal, quotes and escapes left as they are.
    fn quoted(&mut self, quote: char) -> Option<String> {
        let start = self.pos;
        self.pos += 1;
        loop {
            match self.peek()? {
                '\\' => self.pos += 2,
                c if c == quote => {
                    self.pos += 1;
                    return Some(self.chars[start..self.pos].iter().collect());
                }
                _ => self.pos += 1,
            }
        }
    }
}

fn write_doc(out: &mut String, node: &Node, level: usize) {
    for _ in 0..level {
        out.push_str("  ");
    }
    match node {
        Node::Name(name) => out.push_str(match name.as_str() {
            "Nil" => "nil",
            "NewLine" => "hard_line",
            "EmptyLine" => "empty_line",
            other => other,
        }),
        Node::Call { name, args } => match (name.as_str(), args.as_slice()) {
            ("Text", [Node::Atom(text)]) => {
                out.push_str("text ");
                out.push_str(text);
            }
            ("Char", [Node::Atom(literal)]) => {
                if literal == "' '" {
                    out.push_str("space");
                } else {
                    out.push_str("char ");
                    out.push_str(literal);
                }
            }
            ("Break", [Node::Name(space), Node::Atom(offset)]) => {
                out.push_str(if space == "true" {
                    "line_or_space"
                } else {
                    "line_or_nil"
                });
                if offset != "0" {
                    out.push_str(" +");
                    out.push_str(offset);
                }
            }
            ("Group", [Node::Seq(docs)]) => {
                if is_soft_line(docs) {
                    out.push_str("soft_line");
                } else {
                    write_children(out, "group", &without_nil(docs), level);
                }
            }
            ("List", [Node::Seq(docs)]) => write_children(out, "list", &without_nil(docs), level),
            ("Slice", [Node::Seq(docs)]) => {
                write_children(out, "slice", &without_nil(docs), level);
            }
            ("Nest", [Node::Atom(size), inner]) => {
                let header = format!("nest {size}");
                match inner {
                    Node::Call {
                        name,
                        args: inner_args,
                    } => match (name.as_str(), inner_args.as_slice()) {
                        ("Vec" | "Slice", [Node::Seq(docs)]) => {
                            write_children(out, &header, &without_nil(docs), level);
                        }
                        ("Box", [doc]) => {
                            write_children(
                                out,
                                &header,
                                &without_nil(std::slice::from_ref(doc)),
                                level,
                            );
                        }
                        _ => write_raw(out, node),
                    },
                    _ => write_raw(out, node),
                }
            }
            ("Alt", [flat, broken]) => {
                write_children(out, "flat_or_break", &[flat.clone(), broken.clone()], level);
            }
            ("Union", [doc, alternate]) => {
                write_children(out, "union", &[doc.clone(), alternate.clone()], level);
            }
            _ => write_raw(out, node),
        },
        Node::Seq(docs) => write_children(out, "list", &without_nil(docs), level),
        Node::Atom(atom) => out.push_str(atom),
    }
}

/// Children are written one per line, unless they all fit on a single line.
fn write_children(out: &mut String, header: &str, docs: &[Node], level: usize) {
    out.push_str(header);
    if docs.is_empty() {
        out.push_str(" []");
        return;
    }
    if let Some(inline) = inline_children(docs, level * 2 + header.len()) {
        out.push_str(&inline);
        return;
    }
    out.push_str(" [\n");
    for doc in docs {
        write_doc(out, doc, level + 1);
        out.push('\n');
    }
    for _ in 0..level {
        out.push_str("  ");
    }
    out.push(']');
}

/// The single line form, e.g. `list [ text "," line_or_space ]`, when every
/// child fits on one line and the whole thing isn't too wide.
fn inline_children(docs: &[Node], used: usize) -> Option<String> {
    const MAX_WIDTH: usize = 60;

    let mut inline = String::from(" [");
    for doc in docs {
        let mut child = String::new();
        write_doc(&mut child, doc, 0);
        if child.contains('\n') {
            return None;
        }
        inline.push(' ');
        inline.push_str(&child);
        if used + inline.len() > MAX_WIDTH {
            return None;
        }
    }
    inline.push_str(" ]");
    Some(inline)
}

/// `nil` prints nothing, so it's noise in a group's or list's children.
fn without_nil(docs: &[Node]) -> Vec<Node> {
    docs.iter()
        .filter(|doc| !matches!(doc, Node::Name(name) if name == "Nil"))
        .cloned()
        .collect()
}

/// `Doc::soft_line()` is a group wrapping a single `line_or_space`.
fn is_soft_line(docs: &[Node]) -> bool {
    match docs {
        [Node::Call { name, args }] if name == "Break" => {
            matches!(args.as_slice(), [Node::Name(space), Node::Atom(offset)] if space == "true" && offset == "0")
        }
        _ => false,
    }
}

/// Anything we don't have a nicer name for, written back out as is.
fn write_raw(out: &mut String, node: &Node) {
    match node {
        Node::Name(name) => out.push_str(name),
        Node::Atom(atom) => out.push_str(atom),
        Node::Call { name, args } => {
            out.push_str(name);
            out.push('(');
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                write_raw(out, arg);
            }
            out.push(')');
        }
        Node::Seq(docs) => {
            out.push('[');
            for (i, doc) in docs.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                write_raw(out, doc);
            }
            out.push(']');
        }
    }
}

#[cfg(test)]
mod tests {
    use super::render;
    use insta::assert_snapshot;
    use tiny_pretty::Doc;

    #[test]
    fn leaves() {
        let doc = Doc::list(vec![
            Doc::nil(),
            Doc::text("select"),
            Doc::space(),
            Doc::char('*'),
            Doc::hard_line(),
            Doc::empty_line(),
            Doc::line_or_space(),
            Doc::line_or_nil(),
            Doc::soft_line(),
            Doc::line_or_space().nest(4),
        ]);
        assert_snapshot!(render(&doc), @r#"
        list [
          text "select"
          space
          char '*'
          hard_line
          empty_line
          line_or_space
          line_or_nil
          soft_line
          line_or_space +4
        ]
        "#);
    }

    #[test]
    fn nesting() {
        let docs = [Doc::text("a"), Doc::text("b")];
        let doc = Doc::text("select")
            .append(Doc::line_or_space().append(Doc::text("1")).nest(2))
            .append(Doc::text("2").nest(2))
            .append(Doc::slice(&docs))
            .group();
        assert_snapshot!(render(&doc), @r#"
        group [
          text "select"
          nest 2 [ line_or_space text "1" ]
          nest 2 [ text "2" ]
          slice [ text "a" text "b" ]
        ]
        "#);
    }

    #[test]
    fn alternatives() {
        let doc = Doc::flat_or_break(Doc::nil(), Doc::text(","))
            .append(Doc::text("a").union(Doc::text("b")));
        assert_snapshot!(render(&doc), @r#"
        list [
          flat_or_break [ nil text "," ]
          union [ text "a" text "b" ]
        ]
        "#);
    }

    #[test]
    fn text_with_quotes_and_escapes() {
        let doc = Doc::list(vec![
            Doc::text("'it''s'"),
            Doc::text("\"col\""),
            Doc::text("-- a\ncomment"),
            Doc::char('\''),
        ]);
        assert_snapshot!(render(&doc), @r#"
        list [
          text "'it''s'"
          text "\"col\""
          text "-- a\ncomment"
          char '\''
        ]
        "#);
    }

    #[test]
    fn empty_containers() {
        let doc = Doc::list(vec![Doc::list(vec![]), Doc::list(vec![Doc::nil()]).group()]);
        assert_snapshot!(render(&doc), @"list [ list [] group [] ]");
    }

    #[test]
    fn wide_children_break_onto_their_own_lines() {
        let doc = Doc::list(vec![
            Doc::text("a really quite long bit of text that won't fit inline"),
            Doc::text("b"),
        ]);
        assert_snapshot!(render(&doc), @r#"
        list [
          text "a really quite long bit of text that won't fit inline"
          text "b"
        ]
        "#);
    }
}
