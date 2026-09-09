// based on https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/parser/src/syntax_kind.rs

pub use crate::generated::syntax_kind::SyntaxKind;

impl From<u16> for SyntaxKind {
    #[inline]
    fn from(d: u16) -> SyntaxKind {
        assert!(d <= (SyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, SyntaxKind>(d) }
    }
}

impl From<SyntaxKind> for u16 {
    #[inline]
    fn from(k: SyntaxKind) -> u16 {
        k as u16
    }
}

impl SyntaxKind {
    #[inline]
    pub fn is_trivia(self) -> bool {
        matches!(self, SyntaxKind::WHITESPACE | SyntaxKind::COMMENT)
    }
}

#[inline]
pub fn is_reserved_keyword(kind: SyntaxKind) -> bool {
    kind <= SyntaxKind::WHITESPACE && crate::generated::token_sets::RESERVED_KEYWORDS.contains(kind)
}

#[inline]
pub fn is_col_name_keyword(kind: SyntaxKind) -> bool {
    kind <= SyntaxKind::WHITESPACE
        && crate::generated::token_sets::COL_NAME_KEYWORD_FIRST.contains(kind)
}

#[inline]
pub fn is_type_func_name_keyword(kind: SyntaxKind) -> bool {
    kind <= SyntaxKind::WHITESPACE
        && crate::generated::token_sets::TYPE_FUNC_NAME_KEYWORDS.contains(kind)
}

#[cfg(test)]
mod tests {
    use super::{SyntaxKind, is_col_name_keyword, is_reserved_keyword, is_type_func_name_keyword};

    #[test]
    fn keyword_categories() {
        assert!(is_reserved_keyword(SyntaxKind::SELECT_KW));
        assert!(is_col_name_keyword(SyntaxKind::BETWEEN_KW));
        assert!(is_type_func_name_keyword(SyntaxKind::LEFT_KW));

        assert!(!is_reserved_keyword(SyntaxKind::IDENT));
        assert!(!is_reserved_keyword(SyntaxKind::SELECT));
    }
}
