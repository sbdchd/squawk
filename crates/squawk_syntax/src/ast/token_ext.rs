use crate::ast::{self, AstToken};

impl ast::Whitespace {
    pub fn spans_multiple_lines(&self) -> bool {
        let text = self.text();
        text.find('\n')
            .is_some_and(|idx| text[idx + 1..].contains('\n'))
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
    const BY_PREFIX: [(&'static str, CommentKind); 3] = [
        ("/**/", CommentKind::Block),
        ("/*", CommentKind::Block),
        ("--", CommentKind::Line),
    ];
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
