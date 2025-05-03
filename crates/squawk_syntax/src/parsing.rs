// via https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/syntax/src/parsing.rs
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

use rowan::{GreenNode, TextRange};

use crate::{syntax_error::SyntaxError, syntax_node::SyntaxTreeBuilder};

// 1. lex input into vec of tokens
// 2. run the parser over the tokens generating an array of events
// 3. intersperse trivia TODO
pub(crate) fn parse_text(text: &str) -> (GreenNode, Vec<SyntaxError>) {
    let lexed = squawk_parser::LexedStr::new(text);
    let parser_input = lexed.to_input();
    let parser_output = squawk_parser::parse(&parser_input);
    let (node, errors, _eof) = build_tree(lexed, parser_output);
    (node, errors)
}

// 5. check for lexer errors and add them to the parser errors
// 6. return
// (node, errors)

// 7. then some other function passes this stuff to SyntaxNode::new_root
// pub fn parse(text: &str, edition: Edition) -> Parse<SourceFile> {
//     let _p = tracing::info_span!("SourceFile::parse").entered();
//     let (green, errors) = parsing::parse_text(text, edition);
//     let root = SyntaxNode::new_root(green.clone());

//     assert_eq!(root.kind(), SyntaxKind::SOURCE_FILE);
//     Parse::new(green, errors)
// }

// 8. rust analyzer has another validation layer that traverses the syntax tree to cover issues not caught by the lexer or parser

// ast nodes wrap
//  - rowan::syntax nodes
//      - rowan::green nodes

//
// they do some stuff to support having proper types for it
//
// also have can_cast and cast methods to support casting between the two
//
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct IfExpr {
//     pub(crate) syntax: SyntaxNode,
// }
// impl ast::HasAttrs for IfExpr {}
// impl IfExpr {
//     #[inline]
//     pub fn else_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![else]) }
//     #[inline]
//     pub fn if_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![if]) }
// }

pub(crate) fn build_tree(
    lexed: squawk_parser::LexedStr<'_>,
    parser_output: squawk_parser::Output,
) -> (GreenNode, Vec<SyntaxError>, bool) {
    let mut builder = SyntaxTreeBuilder::default();

    let is_eof = lexed.intersperse_trivia(&parser_output, &mut |step| match step {
        squawk_parser::StrStep::Token { kind, text } => builder.token(kind, text),
        squawk_parser::StrStep::Enter { kind } => builder.start_node(kind),
        squawk_parser::StrStep::Exit => builder.finish_node(),
        squawk_parser::StrStep::Error { msg, pos } => {
            builder.error(msg.to_owned(), pos.try_into().unwrap())
        }
    });

    let (node, mut errors) = builder.finish_raw();
    for (i, err) in lexed.errors() {
        let text_range = lexed.text_range(i);
        let text_range = TextRange::new(
            text_range.start.try_into().unwrap(),
            text_range.end.try_into().unwrap(),
        );
        errors.push(SyntaxError::new(err, text_range))
    }

    (node, errors, is_eof)
}
