pub const BUILTINS_SQL: &str = include_str!("builtins.sql");

#[cfg(test)]
mod test {
    use squawk_syntax::ast;

    use crate::builtins::BUILTINS_SQL;

    #[test]
    fn no_errors() {
        let parse = ast::SourceFile::parse(BUILTINS_SQL);
        assert_eq!(parse.errors(), vec![]);
    }
}
