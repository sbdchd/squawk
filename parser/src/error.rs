#[derive(Debug, PartialEq)]
pub enum PgQueryError {
    ParsingCString,
    JsonParse(String),
    QueryToCString,
    // The only useful field on PgQueryError is `message`.
    // The other `lineo` and `filename` fields are just references to the Postgres parser code.
    //
    // https://cs.github.com/pganalyze/libpg_query/blob/4b30b03cb3944f01d4807ee89532549ccf115a44/pg_query.h?q=PgQueryError#L6-L13
    PgParseError(Option<String>),
}

impl std::fmt::Display for PgQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::ParsingCString => write!(
                f,
                "Could not convert Postgres response from CString to String"
            ),
            Self::JsonParse(ref err) => write!(
                f,
                "Squawk schema failed to parse Postgres response. This indicates a bug with Squawk. Please report this error to https://github.com/sbdchd/squawk. Schema error: {err}"
            ),
            Self::QueryToCString => write!(f, "Could not encode query into CString"),
            Self::PgParseError(ref err) => {
                if let Some(err) = err {
                    write!(f, "Postgres failed to parse query: {err}")
                } else {
                    write!(f, "Postgres failed to parse query.")
                }
            },
        }
    }
}

impl std::convert::From<std::ffi::NulError> for PgQueryError {
    fn from(_: std::ffi::NulError) -> Self {
        Self::QueryToCString
    }
}

impl std::convert::From<serde_json::error::Error> for PgQueryError {
    fn from(e: serde_json::error::Error) -> Self {
        Self::JsonParse(e.to_string())
    }
}

impl std::convert::From<std::str::Utf8Error> for PgQueryError {
    fn from(_: std::str::Utf8Error) -> Self {
        Self::ParsingCString
    }
}
