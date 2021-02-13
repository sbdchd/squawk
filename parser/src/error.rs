#[derive(Debug, PartialEq)]
pub enum PGQueryError {
    ParsingCString,
    JsonParse(String),
    QueryToCString,
    PGParseError,
}

impl std::fmt::Display for PGQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::ParsingCString => write!(
                f,
                "Could not convert Postgres response from CString to String"
            ),
            Self::JsonParse(ref err) => write!(
                f,
                "{}",
                format!("Squawk schema failed to parse Postgres response. This indicates a bug with Squawk. Please report this error to https://github.com/sbdchd/squawk. Schema error: {}", err)
            ),
            Self::QueryToCString => write!(f, "Could not encode query into CString"),
            Self::PGParseError => write!(f, "Postgres failed to parse query"),
        }
    }
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

impl std::convert::From<std::str::Utf8Error> for PGQueryError {
    fn from(_: std::str::Utf8Error) -> Self {
        Self::ParsingCString
    }
}
