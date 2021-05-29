use squawk_parser::error::PgQueryError;

#[derive(Debug, PartialEq)]
pub enum CheckSqlError {
    ParsingSql(PgQueryError),
}

impl std::fmt::Display for CheckSqlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::ParsingSql(ref err) => err.fmt(f),
        }
    }
}

impl std::convert::From<PgQueryError> for CheckSqlError {
    fn from(err: PgQueryError) -> Self {
        Self::ParsingSql(err)
    }
}
