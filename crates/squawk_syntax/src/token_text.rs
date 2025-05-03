// via https://github.com/rust-lang/rust-analyzer/blob/d8887c0758bbd2d5f752d5bd405d4491e90e7ed6/crates/syntax/src/token_text.rs
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

//! Yet another version of owned string, backed by a syntax tree token.

use std::{cmp::Ordering, fmt, ops};

use rowan::GreenToken;
use smol_str::SmolStr;

pub struct TokenText<'a>(pub(crate) Repr<'a>);

pub(crate) enum Repr<'a> {
    Borrowed(&'a str),
    Owned(GreenToken),
}

impl<'a> TokenText<'a> {
    pub fn borrowed(text: &'a str) -> Self {
        TokenText(Repr::Borrowed(text))
    }

    pub(crate) fn owned(green: GreenToken) -> Self {
        TokenText(Repr::Owned(green))
    }

    pub fn as_str(&self) -> &str {
        match &self.0 {
            &Repr::Borrowed(it) => it,
            Repr::Owned(green) => green.text(),
        }
    }
}

impl ops::Deref for TokenText<'_> {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}
impl AsRef<str> for TokenText<'_> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<TokenText<'_>> for String {
    fn from(token_text: TokenText<'_>) -> Self {
        token_text.as_str().into()
    }
}

impl From<TokenText<'_>> for SmolStr {
    fn from(token_text: TokenText<'_>) -> Self {
        SmolStr::new(token_text.as_str())
    }
}

impl PartialEq<&'_ str> for TokenText<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl PartialEq<TokenText<'_>> for &'_ str {
    fn eq(&self, other: &TokenText<'_>) -> bool {
        other == self
    }
}
impl PartialEq<String> for TokenText<'_> {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}
impl PartialEq<TokenText<'_>> for String {
    fn eq(&self, other: &TokenText<'_>) -> bool {
        other == self
    }
}
impl PartialEq for TokenText<'_> {
    fn eq(&self, other: &TokenText<'_>) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for TokenText<'_> {}
impl Ord for TokenText<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}
impl PartialOrd for TokenText<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl fmt::Display for TokenText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}
impl fmt::Debug for TokenText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}
