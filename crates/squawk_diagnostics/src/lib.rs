use std::borrow::Cow;
use std::cmp::Ordering;

use text_size::{TextRange, TextSize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fix {
    pub title: Cow<'static, str>,
    pub edits: Vec<Edit>,
}

impl Fix {
    pub fn new<T: Into<Cow<'static, str>>>(title: T, edits: Vec<Edit>) -> Fix {
        Fix {
            title: title.into(),
            edits,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edit {
    pub text_range: TextRange,
    // TODO: does this need to be an Option?
    pub text: Option<Cow<'static, str>>,
}
impl Ord for Edit {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.text_range.start(), self.text_range.end(), &self.text).cmp(&(
            other.text_range.start(),
            other.text_range.end(),
            &other.text,
        ))
    }
}

impl PartialOrd for Edit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Edit {
    pub fn insert<T: Into<Cow<'static, str>>>(text: T, at: TextSize) -> Self {
        Self {
            text_range: TextRange::new(at, at),
            text: Some(text.into()),
        }
    }
    pub fn replace<T: Into<Cow<'static, str>>>(text_range: TextRange, text: T) -> Self {
        Self {
            text_range,
            text: Some(text.into()),
        }
    }
    pub fn delete(text_range: TextRange) -> Self {
        Self {
            text_range,
            text: None,
        }
    }
}
