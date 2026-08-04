//! Changes mostly ported from Ruff that aren't part of upstream `line_index` crate in Rust Analyzer
#![allow(missing_docs)]

use std::iter::FusedIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineEnding {
    /// Classic Mac, `\r`.
    Cr,
    /// Windows, `\r\n`.
    CrLf,
    /// Unix, `\n`.
    Lf,
}

impl Default for LineEnding {
    fn default() -> Self {
        if cfg!(windows) {
            LineEnding::CrLf
        } else {
            LineEnding::Lf
        }
    }
}

impl LineEnding {
    pub const fn as_str(self) -> &'static str {
        match self {
            LineEnding::Cr => "\r",
            LineEnding::CrLf => "\r\n",
            LineEnding::Lf => "\n",
        }
    }
}

/// Finds the next newline character. Returns its position and the [`LineEnding`].
// via: https://github.com/astral-sh/ruff/blob/2742f956d87314cf1522c69c9a95a71d972bd76c/crates/ruff_source_file/src/newlines.rs#L58
#[inline]
pub fn find_newline(text: &str) -> Option<(usize, LineEnding)> {
    let bytes = text.as_bytes();
    let position = memchr::memchr2(b'\n', b'\r', bytes)?;
    let line_ending = match bytes[position] {
        // explicit branch for `\n` since it's the most likely path
        b'\n' => LineEnding::Lf,
        b'\r' if bytes.get(position + 1) == Some(&b'\n') => LineEnding::CrLf,
        _ => LineEnding::Cr,
    };
    Some((position, line_ending))
}

// via: https://github.com/astral-sh/ruff/blob/2742f956d87314cf1522c69c9a95a71d972bd76c/crates/ruff_source_file/src/newlines.rs#L7
/// Extension trait for [`str`] that provides a [`UniversalNewlineIterator`].
pub trait UniversalNewlines {
    fn universal_newlines(&self) -> UniversalNewlineIterator<'_>;
}

impl UniversalNewlines for str {
    fn universal_newlines(&self) -> UniversalNewlineIterator<'_> {
        UniversalNewlineIterator::new(self)
    }
}

/// Like [`str::lines`], but accommodates LF, CRLF, and CR line endings,
/// the latter of which are not supported by [`str::lines`].
#[derive(Debug, Clone)]
pub struct UniversalNewlineIterator<'a> {
    text: &'a str,
}

impl<'a> UniversalNewlineIterator<'a> {
    pub fn new(text: &'a str) -> UniversalNewlineIterator<'a> {
        UniversalNewlineIterator { text }
    }
}

impl<'a> Iterator for UniversalNewlineIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.text.is_empty() {
            return None;
        }
        match find_newline(self.text) {
            Some((position, line_ending)) => {
                let line = &self.text[..position];
                self.text = &self.text[position + line_ending.as_str().len()..];
                Some(line)
            }
            None => Some(std::mem::take(&mut self.text)),
        }
    }
}

impl FusedIterator for UniversalNewlineIterator<'_> {}

#[cfg(test)]
mod tests {
    use super::{LineEnding, UniversalNewlines, find_newline};

    #[test]
    fn finding_the_newline() {
        assert_eq!(find_newline("a\nb"), Some((1, LineEnding::Lf)));
        assert_eq!(find_newline("a\r\nb"), Some((1, LineEnding::CrLf)));
        assert_eq!(find_newline("a\rb"), Some((1, LineEnding::Cr)));
        // a lone `\r` at the very end still reads as CR
        assert_eq!(find_newline("a\r"), Some((1, LineEnding::Cr)));
        // we report the first line break, so mixed endings pick the first one
        assert_eq!(find_newline("a\rb\nc"), Some((1, LineEnding::Cr)));
        // no line breaks at all
        assert_eq!(find_newline("a"), None);
        assert_eq!(find_newline(""), None);
    }

    #[test]
    fn platform_default() {
        #[cfg(windows)]
        assert_eq!(LineEnding::default(), LineEnding::CrLf);
        #[cfg(windows)]
        assert_eq!(LineEnding::default().as_str(), "\r\n");

        #[cfg(not(windows))]
        assert_eq!(LineEnding::default(), LineEnding::Lf);
        #[cfg(not(windows))]
        assert_eq!(LineEnding::default().as_str(), "\n");
    }

    #[test]
    fn universal_newlines_matches_str_lines() {
        let lines = |text: &'static str| text.universal_newlines().collect::<Vec<_>>();

        assert_eq!(lines(""), Vec::<&str>::new());
        assert_eq!(lines("a"), vec!["a"]);
        assert_eq!(lines("a\nb"), vec!["a", "b"]);
        assert_eq!(lines("a\r\nb"), vec!["a", "b"]);

        // `str::lines` doesn't support `\r`
        assert_eq!(lines("a\rb"), vec!["a", "b"]);
        assert_eq!(lines("a\rb\r\nc\nd"), vec!["a", "b", "c", "d"]);

        // trailing new lines
        assert_eq!(lines("a\n"), vec!["a"]);
        assert_eq!(lines("a\r\n"), vec!["a"]);
        assert_eq!(lines("a\r"), vec!["a"]);

        assert_eq!(lines("a\n\nb"), vec!["a", "", "b"]);
        assert_eq!(lines("\n"), vec![""]);
    }

    #[test]
    fn newline_type_default() {
        let detect = |text| {
            find_newline(text)
                .map(|(_, ending)| ending)
                .unwrap_or_default()
        };

        assert_eq!(detect("select 1;"), LineEnding::default());
        assert_eq!(detect(""), LineEnding::default());

        assert_eq!(detect("select 1;\n"), LineEnding::Lf);
        assert_eq!(detect("select 1;\r\n"), LineEnding::CrLf);
        assert_eq!(detect("select 1;\r"), LineEnding::Cr);
    }
}
