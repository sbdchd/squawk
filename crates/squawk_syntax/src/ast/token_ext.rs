use squawk_line_index::find_newline;

use crate::ast::{self, AstToken};

impl ast::Whitespace {
    pub fn spans_multiple_lines(&self) -> bool {
        let text = self.text();
        find_newline(text).is_some_and(|(idx, line_ending)| {
            find_newline(&text[idx + line_ending.as_str().len()..]).is_some()
        })
    }
}

impl ast::Comment {
    pub fn kind(&self) -> CommentKind {
        CommentKind::from_text(self.text())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CommentKind {
    Line,
    Block,
}

impl CommentKind {
    const BY_PREFIX: [(&'static str, CommentKind); 2] =
        [("/*", CommentKind::Block), ("--", CommentKind::Line)];
    pub(crate) fn from_text(text: &str) -> CommentKind {
        let &(_prefix, kind) = CommentKind::BY_PREFIX
            .iter()
            .find(|&(prefix, _kind)| text.starts_with(prefix))
            .unwrap();
        kind
    }

    pub fn is_line(self) -> bool {
        self == CommentKind::Line
    }

    pub fn is_block(self) -> bool {
        self == CommentKind::Block
    }
}
