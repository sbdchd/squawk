use rowan::GreenNode;

use crate::{
    ast,
    ast::AstNode,
    body::{Body, BodyLanguage},
    parsing,
    syntax_error::SyntaxError,
};

pub type SqlBody = Body<ast::SourceFile>;

impl BodyLanguage for ast::SourceFile {
    const LANGUAGE: &'static str = "sql";

    fn parse_text(text: &str) -> (GreenNode, Vec<SyntaxError>) {
        parsing::parse_text(text)
    }
}

impl ast::SourceFile {
    pub fn sql_body_errors(&self) -> Vec<SyntaxError> {
        self.syntax()
            .descendants()
            .filter_map(|node| {
                ast::CreateFunction::cast(node.clone())
                    .and_then(|function| function.sql_body())
                    .or_else(|| {
                        ast::CreateProcedure::cast(node).and_then(|procedure| procedure.sql_body())
                    })
            })
            .flat_map(|body| body.errors())
            .collect()
    }
}

impl ast::Literal {
    pub fn sql_body(&self) -> Option<SqlBody> {
        let definition = ast::AsDefinition::cast(self.syntax().parent()?)?;
        if definition.literal()? != *self {
            return None;
        }
        let options = definition
            .syntax()
            .ancestors()
            .find_map(ast::FuncOptionList::cast)?;
        SqlBody::from_options(options)
    }
}

impl ast::CreateFunction {
    pub fn sql_body(&self) -> Option<SqlBody> {
        SqlBody::from_options(self.option_list()?)
    }
}

impl ast::CreateProcedure {
    pub fn sql_body(&self) -> Option<SqlBody> {
        SqlBody::from_options(self.option_list()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SourceFile;
    use crate::test::render_errors;
    use insta::assert_snapshot;
    use rowan::{TextRange, TextSize};

    fn find(sql: &str) -> Option<SqlBody> {
        let parse = SourceFile::parse(sql);

        parse.tree().syntax().descendants().find_map(|node| {
            ast::CreateFunction::cast(node.clone())
                .and_then(|it| it.sql_body())
                .or_else(|| ast::CreateProcedure::cast(node).and_then(|it| it.sql_body()))
        })
    }

    fn body(sql: &str) -> String {
        let body = find(sql).expect("no SQL body");

        let range = body.source_range(TextRange::up_to(TextSize::of(body.text())));
        let start = usize::from(range.start());
        let end = usize::from(range.end());

        let mut out = format!("{:#?}", body.syntax());
        out.push_str(&format!("---\nsource {range:?} {:?}\n", &sql[start..end]));
        out.push_str(&render_errors(sql, &body.errors()));
        out
    }

    #[test]
    fn function_language_before_as() {
        assert_snapshot!(body(
            "\
create function f(int) returns int
  language sql
  as $$ select $1 + 1 $$;"
        ));
    }

    #[test]
    fn function_language_after_as() {
        assert_snapshot!(body(
            "\
create function f(int) returns int
  as 'select $1'
  language sql;"
        ));
    }

    #[test]
    fn procedure() {
        assert_snapshot!(body(
            "\
create procedure p()
  language sql
  as $$ select 1; select 2 $$;"
        ));
    }

    #[test]
    fn other_language_is_not_sql() {
        assert!(
            find(
                "\
create function f() returns int
  as $$ select 1 $$
  language plpgsql;"
            )
            .is_none()
        );
    }

    #[test]
    fn string_language_is_case_sensitive() {
        assert!(
            find(
                "\
create function f() returns int
  as 'not even sql'
  language 'SQL';"
            )
            .is_none()
        );
    }

    #[test]
    fn inline_body_is_not_a_string_body() {
        assert!(
            find(
                "\
create function f() returns int
  language sql
  return 1;"
            )
            .is_none()
        );
    }

    #[test]
    fn errors_map_into_the_containing_file() {
        assert_snapshot!(body(
            "\
create function f() returns int
  language sql
  as $$ select from $$;"
        ));
    }
}
