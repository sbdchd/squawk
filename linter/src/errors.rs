use squawk_parser::error::PGQueryError;

#[derive(Debug, PartialEq)]
pub enum CheckSQLError {
    ParsingSQL(PGQueryError),
}

impl std::convert::From<PGQueryError> for CheckSQLError {
    fn from(err: PGQueryError) -> Self {
        Self::ParsingSQL(err)
    }
}
