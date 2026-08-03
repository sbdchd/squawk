//! Changes mostly ported from Ruff that aren't part of upstream line_index crate in Rust Analyzer
#![allow(missing_docs)]

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

#[cfg(test)]
mod tests {
    use super::{LineEnding, find_newline};

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
    fn newline_type_default() {
        let detect = |text| {
            find_newline(text)
                .map(|(_, ending)| ending)
                .unwrap_or_default()
        };

        assert_eq!(detect("select 1;"), LineEnding::default());
        assert_eq!(detect(""), LineEnding::default());
        // anything with a break is detected, never defaulted
        assert_eq!(detect("select 1;\n"), LineEnding::Lf);
        assert_eq!(detect("select 1;\r\n"), LineEnding::CrLf);
        assert_eq!(detect("select 1;\r"), LineEnding::Cr);
    }
}
