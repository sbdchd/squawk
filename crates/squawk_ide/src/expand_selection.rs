// via https://github.com/rust-lang/rust-analyzer/blob/8d75311400a108d7ffe17dc9c38182c566952e6e/crates/ide/src/extend_selection.rs#L1C1-L1C1
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

// NOTE: this is pretty much copied as is from rust analyzer with some
// simplifications. I imagine there's more we can do to adapt it for SQL.

use rowan::{Direction, NodeOrToken, TextRange, TextSize};
use squawk_syntax::{
    SyntaxKind, SyntaxNode, SyntaxToken,
    ast::{self, AstToken},
};

const DELIMITED_LIST_KINDS: &[SyntaxKind] = &[
    SyntaxKind::ARG_LIST,
    SyntaxKind::ATTRIBUTE_LIST,
    SyntaxKind::COLUMN_LIST,
    SyntaxKind::CONSTRAINT_EXCLUSION_LIST,
    SyntaxKind::JSON_TABLE_COLUMN_LIST,
    SyntaxKind::OPTIONS_LIST,
    SyntaxKind::PARAM_LIST,
    SyntaxKind::PARTITION_ITEM_LIST,
    SyntaxKind::ROW_LIST,
    SyntaxKind::SET_OPTIONS_LIST,
    SyntaxKind::TABLE_ARG_LIST,
    SyntaxKind::TABLE_LIST,
    SyntaxKind::TARGET_LIST,
    SyntaxKind::TRANSACTION_MODE_LIST,
    SyntaxKind::VACUUM_OPTION_LIST,
    SyntaxKind::VARIANT_LIST,
    SyntaxKind::XML_TABLE_COLUMN_LIST,
];

pub fn extend_selection(root: &SyntaxNode, range: TextRange) -> TextRange {
    try_extend_selection(root, range).unwrap_or(range)
}

fn try_extend_selection(root: &SyntaxNode, range: TextRange) -> Option<TextRange> {
    let string_kinds = [
        SyntaxKind::COMMENT,
        SyntaxKind::STRING,
        SyntaxKind::BYTE_STRING,
        SyntaxKind::BIT_STRING,
        SyntaxKind::DOLLAR_QUOTED_STRING,
        SyntaxKind::ESC_STRING,
    ];

    if range.is_empty() {
        let offset = range.start();
        let mut leaves = root.token_at_offset(offset);
        // Make sure that if we're on the whitespace at the start of a line, we
        // expand to the node on that line instead of the previous one
        if leaves.clone().all(|it| it.kind() == SyntaxKind::WHITESPACE) {
            return Some(extend_ws(root, leaves.next()?, offset));
        }
        let leaf_range = match root.token_at_offset(offset) {
            rowan::TokenAtOffset::None => return None,
            rowan::TokenAtOffset::Single(l) => {
                if string_kinds.contains(&l.kind()) {
                    extend_single_word_in_comment_or_string(&l, offset)
                        .unwrap_or_else(|| l.text_range())
                } else {
                    l.text_range()
                }
            }
            rowan::TokenAtOffset::Between(l, r) => pick_best(l, r).text_range(),
        };
        return Some(leaf_range);
    }

    let node = match root.covering_element(range) {
        NodeOrToken::Token(token) => {
            if token.text_range() != range {
                return Some(token.text_range());
            }
            if let Some(comment) = ast::Comment::cast(token.clone())
                && let Some(range) = extend_comments(comment)
            {
                return Some(range);
            }
            token.parent()?
        }
        NodeOrToken::Node(node) => node,
    };

    if node.text_range() != range {
        return Some(node.text_range());
    }

    let node = shallowest_node(&node);

    if node
        .parent()
        .is_some_and(|n| DELIMITED_LIST_KINDS.contains(&n.kind()))
    {
        if let Some(range) = extend_list_item(&node) {
            return Some(range);
        }
    }

    node.parent().map(|it| it.text_range())
}

/// Find the shallowest node with same range, which allows us to traverse siblings.
fn shallowest_node(node: &SyntaxNode) -> SyntaxNode {
    node.ancestors()
        .take_while(|n| n.text_range() == node.text_range())
        .last()
        .unwrap()
}

/// Expand to the current word instead the full text range of the node.
fn extend_single_word_in_comment_or_string(
    leaf: &SyntaxToken,
    offset: TextSize,
) -> Option<TextRange> {
    let text: &str = leaf.text();
    let cursor_position: u32 = (offset - leaf.text_range().start()).into();

    let (before, after) = text.split_at(cursor_position as usize);

    fn non_word_char(c: char) -> bool {
        !(c.is_alphanumeric() || c == '_')
    }

    let start_idx = before.rfind(non_word_char)? as u32;
    let end_idx = after.find(non_word_char).unwrap_or(after.len()) as u32;

    // FIXME: use `ceil_char_boundary` from `std::str` when it gets stable
    // https://github.com/rust-lang/rust/issues/93743
    fn ceil_char_boundary(text: &str, index: u32) -> u32 {
        (index..)
            .find(|&index| text.is_char_boundary(index as usize))
            .unwrap_or(text.len() as u32)
    }

    let from: TextSize = ceil_char_boundary(text, start_idx + 1).into();
    let to: TextSize = (cursor_position + end_idx).into();

    let range = TextRange::new(from, to);
    if range.is_empty() {
        None
    } else {
        Some(range + leaf.text_range().start())
    }
}

fn extend_comments(comment: ast::Comment) -> Option<TextRange> {
    let prev = adj_comments(&comment, Direction::Prev);
    let next = adj_comments(&comment, Direction::Next);
    if prev != next {
        Some(TextRange::new(
            prev.syntax().text_range().start(),
            next.syntax().text_range().end(),
        ))
    } else {
        None
    }
}

fn adj_comments(comment: &ast::Comment, dir: Direction) -> ast::Comment {
    let mut res = comment.clone();
    for element in comment.syntax().siblings_with_tokens(dir) {
        let Some(token) = element.as_token() else {
            break;
        };
        if let Some(c) = ast::Comment::cast(token.clone()) {
            res = c
        } else if token.kind() != SyntaxKind::WHITESPACE || token.text().contains("\n\n") {
            break;
        }
    }
    res
}

fn extend_ws(root: &SyntaxNode, ws: SyntaxToken, offset: TextSize) -> TextRange {
    let ws_text = ws.text();
    let suffix = TextRange::new(offset, ws.text_range().end()) - ws.text_range().start();
    let prefix = TextRange::new(ws.text_range().start(), offset) - ws.text_range().start();
    let ws_suffix = &ws_text[suffix];
    let ws_prefix = &ws_text[prefix];
    if ws_text.contains('\n')
        && !ws_suffix.contains('\n')
        && let Some(node) = ws.next_sibling_or_token()
    {
        let start = match ws_prefix.rfind('\n') {
            Some(idx) => ws.text_range().start() + TextSize::from((idx + 1) as u32),
            None => node.text_range().start(),
        };
        let end = if root.text().char_at(node.text_range().end()) == Some('\n') {
            node.text_range().end() + TextSize::of('\n')
        } else {
            node.text_range().end()
        };
        return TextRange::new(start, end);
    }
    ws.text_range()
}

fn pick_best(l: SyntaxToken, r: SyntaxToken) -> SyntaxToken {
    return if priority(&r) > priority(&l) { r } else { l };
    fn priority(n: &SyntaxToken) -> usize {
        match n.kind() {
            SyntaxKind::WHITESPACE => 0,
            // TODO: we can probably include more here, rust analyzer includes a
            // handful of keywords
            SyntaxKind::IDENT => 2,
            _ => 1,
        }
    }
}

/// Extend list item selection to include nearby delimiter and whitespace.
fn extend_list_item(node: &SyntaxNode) -> Option<TextRange> {
    fn is_single_line_ws(node: &SyntaxToken) -> bool {
        node.kind() == SyntaxKind::WHITESPACE && !node.text().contains('\n')
    }

    fn nearby_comma(node: &SyntaxNode, dir: Direction) -> Option<SyntaxToken> {
        node.siblings_with_tokens(dir)
            .skip(1)
            .find(|node| match node {
                NodeOrToken::Node(_) => true,
                NodeOrToken::Token(it) => !is_single_line_ws(it),
            })
            .and_then(|it| it.into_token())
            .filter(|node| node.kind() == SyntaxKind::COMMA)
    }

    if let Some(comma) = nearby_comma(node, Direction::Next) {
        // Include any following whitespace when delimiter is after list item.
        let final_node = comma
            .next_sibling_or_token()
            .and_then(|n| n.into_token())
            .filter(is_single_line_ws)
            .unwrap_or(comma);

        return Some(TextRange::new(
            node.text_range().start(),
            final_node.text_range().end(),
        ));
    }

    if let Some(comma) = nearby_comma(node, Direction::Prev) {
        return Some(TextRange::new(
            comma.text_range().start(),
            node.text_range().end(),
        ));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use rowan::TextSize;
    use squawk_syntax::{SourceFile, ast::AstNode};

    fn expand(sql: &str) -> Vec<String> {
        let (offset, sql) = fixture(sql);
        let parse = SourceFile::parse(&sql);
        let file = parse.tree();
        let root = file.syntax();

        let mut range = TextRange::empty(offset);
        let mut results = Vec::new();

        for _ in 0..20 {
            let new_range = extend_selection(root, range);
            if new_range == range {
                break;
            }
            range = new_range;
            results.push(sql[range].to_string());
        }

        results
    }

    fn fixture(sql: &str) -> (TextSize, String) {
        const MARKER: &str = "$0";
        if let Some(pos) = sql.find(MARKER) {
            return (TextSize::new(pos as u32), sql.replace(MARKER, ""));
        }
        panic!("No marker found in test SQL");
    }

    #[test]
    fn simple() {
        assert_debug_snapshot!(expand(r#"select $01 + 1"#), @r#"
        [
            "1",
            "1 + 1",
            "select 1 + 1",
        ]
        "#);
    }

    #[test]
    fn word_in_string_string() {
        assert_debug_snapshot!(expand(r"
select 'some stret$0ched out words in a string'
"), @r#"
        [
            "stretched",
            "'some stretched out words in a string'",
            "select 'some stretched out words in a string'",
            "\nselect 'some stretched out words in a string'\n",
        ]
        "#);
    }

    #[test]
    fn string() {
        assert_debug_snapshot!(expand(r"
select b'foo$0 bar'
'buzz';
"), @r#"
        [
            "foo",
            "b'foo bar'",
            "b'foo bar'\n'buzz'",
            "select b'foo bar'\n'buzz'",
            "\nselect b'foo bar'\n'buzz';\n",
        ]
        "#);
    }

    #[test]
    fn dollar_string() {
        assert_debug_snapshot!(expand(r"
select $$foo$0 bar$$;
"), @r#"
        [
            "foo",
            "$$foo bar$$",
            "select $$foo bar$$",
            "\nselect $$foo bar$$;\n",
        ]
        "#);
    }

    #[test]
    fn comment_muli_line() {
        assert_debug_snapshot!(expand(r"
-- foo bar
-- buzz$0
-- boo
select 1
"), @r#"
        [
            "-- buzz",
            "-- foo bar\n-- buzz\n-- boo",
            "\n-- foo bar\n-- buzz\n-- boo\nselect 1\n",
        ]
        "#);
    }

    #[test]
    fn comment() {
        assert_debug_snapshot!(expand(r"
-- foo bar$0
select 1
"), @r#"
        [
            "-- foo bar",
            "\n-- foo bar\nselect 1\n",
        ]
        "#);

        assert_debug_snapshot!(expand(r"
/* foo bar$0 */
select 1
"), @r#"
        [
            "bar",
            "/* foo bar */",
            "\n/* foo bar */\nselect 1\n",
        ]
        "#);
    }

    #[test]
    fn create_table_with_comment() {
        assert_debug_snapshot!(expand(r"
-- foo bar buzz
create table t(
  x int$0,
  y text
);
"), @r#"
        [
            "int",
            "x int",
            "x int,",
            "(\n  x int,\n  y text\n)",
            "-- foo bar buzz\ncreate table t(\n  x int,\n  y text\n)",
            "\n-- foo bar buzz\ncreate table t(\n  x int,\n  y text\n);\n",
        ]
        "#);
    }

    #[test]
    fn column_list() {
        assert_debug_snapshot!(expand(r#"create table t($0x int)"#), @r#"
        [
            "x",
            "x int",
            "(x int)",
            "create table t(x int)",
        ]
        "#);

        assert_debug_snapshot!(expand(r#"create table t($0x int, y int)"#), @r#"
        [
            "x",
            "x int",
            "x int, ",
            "(x int, y int)",
            "create table t(x int, y int)",
        ]
        "#);

        assert_debug_snapshot!(expand(r#"create table t(x int, $0y int)"#), @r#"
        [
            "y",
            "y int",
            ", y int",
            "(x int, y int)",
            "create table t(x int, y int)",
        ]
        "#);
    }

    #[test]
    fn start_of_line_whitespace_select() {
        assert_debug_snapshot!(expand(r#"    
select 1;

$0    select 2;"#), @r#"
        [
            "    select 2",
            "    \nselect 1;\n\n    select 2;",
        ]
        "#);
    }

    #[test]
    fn select_list() {
        assert_debug_snapshot!(expand(r#"select x$0, y from t"#), @r#"
        [
            "x",
            "x, ",
            "x, y",
            "select x, y",
            "select x, y from t",
        ]
        "#);

        assert_debug_snapshot!(expand(r#"select x, y$0 from t"#), @r#"
        [
            "y",
            ", y",
            "x, y",
            "select x, y",
            "select x, y from t",
        ]
        "#);
    }

    #[test]
    fn expand_whitespace() {
        assert_debug_snapshot!(expand(r#"select 1 + 
$0
1;"#), @r#"
        [
            " \n\n",
            "1 + \n\n1",
            "select 1 + \n\n1",
            "select 1 + \n\n1;",
        ]
        "#);
    }

    #[test]
    fn function_args() {
        assert_debug_snapshot!(expand(r#"select f(1$0, 2)"#), @r#"
        [
            "1",
            "1, ",
            "(1, 2)",
            "f(1, 2)",
            "select f(1, 2)",
        ]
        "#);
    }

    #[test]
    fn prefer_idents() {
        assert_debug_snapshot!(expand(r#"select foo$0+bar"#), @r#"
        [
            "foo",
            "foo+bar",
            "select foo+bar",
        ]
        "#);

        assert_debug_snapshot!(expand(r#"select foo+$0bar"#), @r#"
        [
            "bar",
            "foo+bar",
            "select foo+bar",
        ]
        "#);
    }

    #[test]
    fn list_variants() {
        let delimited_ws_list_kinds = &[
            SyntaxKind::FUNC_OPTION_LIST,
            SyntaxKind::SEQUENCE_OPTION_LIST,
            SyntaxKind::XML_COLUMN_OPTION_LIST,
            SyntaxKind::WHEN_CLAUSE_LIST,
        ];

        let unhandled_list_kinds = (0..SyntaxKind::__LAST as u16)
            .map(SyntaxKind::from)
            .filter(|kind| {
                format!("{:?}", kind).ends_with("_LIST") && !delimited_ws_list_kinds.contains(kind)
            })
            .filter(|kind| !DELIMITED_LIST_KINDS.contains(kind))
            .collect::<Vec<_>>();

        assert_eq!(
            unhandled_list_kinds,
            vec![],
            "We shouldn't have any unhandled list kinds"
        )
    }
}
