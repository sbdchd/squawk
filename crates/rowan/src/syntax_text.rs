use std::fmt;

use crate::{cursor::SyntaxNode, TextRange, TextSize};

#[derive(Clone)]
pub struct SyntaxText {
    node: SyntaxNode,
    range: TextRange,
}

impl SyntaxText {
    pub(crate) fn new(node: SyntaxNode) -> SyntaxText {
        let range = node.text_range();
        SyntaxText { node, range }
    }

    pub fn as_str(&self) -> &str {
        self.node.text_at(self.range)
    }

    pub fn len(&self) -> TextSize {
        self.range.len()
    }

    pub fn is_empty(&self) -> bool {
        self.range.is_empty()
    }

    pub fn contains_char(&self, c: char) -> bool {
        self.as_str().contains(c)
    }

    pub fn find_char(&self, c: char) -> Option<TextSize> {
        let pos = self.as_str().find(c)?;
        Some(TextSize::new(pos as u32))
    }

    pub fn char_at(&self, offset: TextSize) -> Option<char> {
        self.as_str().get(usize::from(offset)..)?.chars().next()
    }

    pub fn slice<R: private::SyntaxTextRange>(&self, range: R) -> SyntaxText {
        let start = range.start().unwrap_or_default();
        let end = range.end().unwrap_or(self.len());
        assert!(start <= end);
        let len = end - start;
        let start = self.range.start() + start;
        let end = start + len;
        assert!(
            start <= end,
            "invalid slice, range: {:?}, slice: {:?}",
            self.range,
            (range.start(), range.end()),
        );
        let range = TextRange::new(start, end);
        assert!(
            self.range.contains_range(range),
            "invalid slice, range: {:?}, slice: {:?}",
            self.range,
            range,
        );
        SyntaxText {
            node: self.node.clone(),
            range,
        }
    }

    pub fn try_fold_chunks<T, F, E>(&self, init: T, mut f: F) -> Result<T, E>
    where
        F: FnMut(T, &str) -> Result<T, E>,
    {
        f(init, self.as_str())
    }

    pub fn try_for_each_chunk<F: FnMut(&str) -> Result<(), E>, E>(
        &self,
        mut f: F,
    ) -> Result<(), E> {
        self.try_fold_chunks((), move |(), chunk| f(chunk))
    }

    pub fn for_each_chunk<F: FnMut(&str)>(&self, mut f: F) {
        f(self.as_str())
    }
}

impl fmt::Debug for SyntaxText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for SyntaxText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<SyntaxText> for String {
    fn from(text: SyntaxText) -> String {
        text.as_str().to_owned()
    }
}

impl PartialEq<str> for SyntaxText {
    fn eq(&self, rhs: &str) -> bool {
        self.as_str() == rhs
    }
}

impl PartialEq<SyntaxText> for str {
    fn eq(&self, rhs: &SyntaxText) -> bool {
        rhs == self
    }
}

impl PartialEq<&'_ str> for SyntaxText {
    fn eq(&self, rhs: &&str) -> bool {
        self == *rhs
    }
}

impl PartialEq<SyntaxText> for &'_ str {
    fn eq(&self, rhs: &SyntaxText) -> bool {
        rhs == self
    }
}

impl PartialEq for SyntaxText {
    fn eq(&self, other: &SyntaxText) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for SyntaxText {}

mod private {
    use std::ops;

    use crate::{TextRange, TextSize};

    pub trait SyntaxTextRange {
        fn start(&self) -> Option<TextSize>;
        fn end(&self) -> Option<TextSize>;
    }

    impl SyntaxTextRange for TextRange {
        fn start(&self) -> Option<TextSize> {
            Some(TextRange::start(*self))
        }
        fn end(&self) -> Option<TextSize> {
            Some(TextRange::end(*self))
        }
    }

    impl SyntaxTextRange for ops::Range<TextSize> {
        fn start(&self) -> Option<TextSize> {
            Some(self.start)
        }
        fn end(&self) -> Option<TextSize> {
            Some(self.end)
        }
    }

    impl SyntaxTextRange for ops::RangeFrom<TextSize> {
        fn start(&self) -> Option<TextSize> {
            Some(self.start)
        }
        fn end(&self) -> Option<TextSize> {
            None
        }
    }

    impl SyntaxTextRange for ops::RangeTo<TextSize> {
        fn start(&self) -> Option<TextSize> {
            None
        }
        fn end(&self) -> Option<TextSize> {
            Some(self.end)
        }
    }

    impl SyntaxTextRange for ops::RangeFull {
        fn start(&self) -> Option<TextSize> {
            None
        }
        fn end(&self) -> Option<TextSize> {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{green::SyntaxKind, GreenNodeBuilder};

    use super::*;

    fn build_tree(chunks: &[&str]) -> SyntaxNode {
        let mut builder = GreenNodeBuilder::new();
        builder.start_node(SyntaxKind(62));
        for &chunk in chunks.iter() {
            builder.token(SyntaxKind(92), chunk)
        }
        builder.finish_node();
        SyntaxNode::new_root(builder.finish())
    }

    #[test]
    fn test_text_equality() {
        fn do_check(t1: &[&str], t2: &[&str]) {
            let t1 = build_tree(t1).text();
            let t2 = build_tree(t2).text();
            let expected = t1.to_string() == t2.to_string();
            let actual = t1 == t2;
            assert_eq!(
                expected, actual,
                "`{}` (SyntaxText) `{}` (SyntaxText)",
                t1, t2
            );
            let actual = t1 == *t2.to_string();
            assert_eq!(expected, actual, "`{}` (SyntaxText) `{}` (&str)", t1, t2);
        }
        fn check(t1: &[&str], t2: &[&str]) {
            do_check(t1, t2);
            do_check(t2, t1)
        }

        check(&[""], &[""]);
        check(&["a"], &[""]);
        check(&["a"], &["a"]);
        check(&["abc"], &["def"]);
        check(&["hello", "world"], &["hello", "world"]);
        check(&["hellowo", "rld"], &["hell", "oworld"]);
        check(&["hel", "lowo", "rld"], &["helloworld"]);
        check(&["{", "abc", "}"], &["{", "123", "}"]);
        check(&["{", "abc", "}", "{"], &["{", "123", "}"]);
        check(&["{", "abc", "}"], &["{", "123", "}", "{"]);
        check(&["{", "abc", "}ab"], &["{", "abc", "}", "ab"]);
    }
}
