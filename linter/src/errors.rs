use squawk_parser::error::PGQueryError;

#[derive(Debug, PartialEq)]
pub enum CheckSQLError {
    ParsingSQL(PGQueryError),
}

impl std::fmt::Display for CheckSQLError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::ParsingSQL(ref err) => write!(f, "{}", format!("Failed to parse query: {}", err)),
        }
    }
}

impl std::convert::From<PGQueryError> for CheckSQLError {
    fn from(err: PGQueryError) -> Self {
        Self::ParsingSQL(err)
    }
}
