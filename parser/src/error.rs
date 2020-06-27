use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct ParseError {
    pub message: Option<String>,
    pub funcname: Option<String>,
    pub filename: Option<String>,
    pub lineno: i32,
    pub cursorpos: i32,
    pub context: Option<String>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum PGQueryError {
    ParsingCString,
    JsonParse(String),
    QueryToCString,
    PGParseError(ParseError),
}

impl std::convert::From<std::ffi::NulError> for PGQueryError {
    fn from(_: std::ffi::NulError) -> Self {
        Self::QueryToCString
    }
}

impl std::convert::From<serde_json::error::Error> for PGQueryError {
    fn from(e: serde_json::error::Error) -> Self {
        Self::JsonParse(e.to_string())
    }
}

impl std::convert::From<std::ffi::IntoStringError> for PGQueryError {
    fn from(_: std::ffi::IntoStringError) -> Self {
        Self::ParsingCString
    }
}

impl std::convert::From<std::str::Utf8Error> for PGQueryError {
    fn from(_: std::str::Utf8Error) -> Self {
        Self::ParsingCString
    }
}
