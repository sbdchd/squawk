use rowan::{GreenNode, TextRange};

use crate::{
    SyntaxNode, ast, ast::AstNode, decoded_text::DecodedText, parsing, syntax_error::SyntaxError,
};

pub struct Plpgsql {
    green: GreenNode,
    errors: Vec<SyntaxError>,
    decoded: DecodedText,
}

impl Plpgsql {
    pub(crate) fn parse(decoded: DecodedText) -> Self {
        let (green, errors) = parsing::parse_plpgsql_text(decoded.text());
        let errors = errors
            .into_iter()
            .map(|error| {
                let range = decoded.source_range(error.range());
                error.with_range(range)
            })
            .collect();

        Self {
            green,
            errors,
            decoded,
        }
    }

    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }

    pub fn tree(&self) -> ast::Plpgsql {
        ast::Plpgsql::cast(self.syntax()).expect("root is always a Plpgsql")
    }

    pub fn text(&self) -> &str {
        self.decoded.text()
    }

    pub fn source_range(&self, range: TextRange) -> TextRange {
        self.decoded.source_range(range)
    }

    pub fn errors(&self) -> &[SyntaxError] {
        &self.errors
    }
}

fn is_plpgsql(language_ref: Option<ast::LanguageRef>, literal: Option<ast::Literal>) -> bool {
    let name = match (language_ref, literal) {
        (Some(language_ref), _) => language_ref.syntax().text().to_string(),
        (_, Some(literal)) => literal.string_value().unwrap_or_default(),
        _ => return false,
    };
    name.eq_ignore_ascii_case("plpgsql")
}

fn from_options(options: ast::FuncOptionList) -> Option<Plpgsql> {
    let mut plpgsql = false;
    let mut body = None;

    for option in options.options() {
        match option {
            ast::FuncOption::LanguageFuncOption(option) => {
                plpgsql = is_plpgsql(option.language_ref(), option.literal());
            }
            ast::FuncOption::AsFuncOption(option) => {
                if let Some(ast::AsFuncTarget::AsDefinition(definition)) = option.as_func_target() {
                    body = definition.literal();
                }
            }
            _ => (),
        }
    }

    plpgsql.then_some(())?;
    Some(Plpgsql::parse(body?.decoded_value()?))
}

impl ast::CreateFunction {
    pub fn plpgsql(&self) -> Option<Plpgsql> {
        from_options(self.option_list()?)
    }
}

impl ast::CreateProcedure {
    pub fn plpgsql(&self) -> Option<Plpgsql> {
        from_options(self.option_list()?)
    }
}

impl ast::Do {
    pub fn plpgsql(&self) -> Option<Plpgsql> {
        if let Some(language) = self.do_language()
            && !is_plpgsql(language.language_ref(), language.literal())
        {
            return None;
        }
        Some(Plpgsql::parse(self.body()?.decoded_value()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SourceFile;
    use insta::assert_snapshot;
    use rowan::{TextRange, TextSize};

    fn find(sql: &str) -> Option<Plpgsql> {
        let parse = SourceFile::parse(sql);
        assert!(parse.errors().is_empty(), "{:?}", parse.errors());

        parse.tree().syntax().descendants().find_map(|node| {
            ast::CreateFunction::cast(node.clone())
                .and_then(|it| it.plpgsql())
                .or_else(|| ast::CreateProcedure::cast(node.clone()).and_then(|it| it.plpgsql()))
                .or_else(|| ast::Do::cast(node.clone()).and_then(|it| it.plpgsql()))
        })
    }

    fn body(sql: &str) -> String {
        let body = find(sql).expect("no plpgsql body");

        let range = body.source_range(TextRange::up_to(TextSize::of(body.text())));
        let start = usize::from(range.start());
        let end = usize::from(range.end());

        let mut out = format!("{:#?}", body.syntax());
        out.push_str(&format!("---\nsource {range:?} {:?}\n", &sql[start..end]));
        for error in body.errors() {
            out.push_str(&format!("error {:?} {}\n", error.range(), error.message()));
        }
        out
    }

    #[test]
    fn language_after_as() {
        assert_snapshot!(
            body("create function f() returns int as $$ null; $$ language plpgsql;"),
            @r#"
        PLPGSQL@0..7
          WHITESPACE@0..1 " "
          PLPGSQL_NULL_STMT@1..6
            NULL_KW@1..5 "null"
            SEMICOLON@5..6 ";"
          WHITESPACE@6..7 " "
        ---
        source 37..44 " null; "
        "#
        );
    }

    #[test]
    fn language_before_as() {
        assert_snapshot!(
            body("create function f() returns int language plpgsql as $$ null; $$;"),
            @r#"
        PLPGSQL@0..7
          WHITESPACE@0..1 " "
          PLPGSQL_NULL_STMT@1..6
            NULL_KW@1..5 "null"
            SEMICOLON@5..6 ";"
          WHITESPACE@6..7 " "
        ---
        source 54..61 " null; "
        "#
        );
    }

    #[test]
    fn other_language_is_not_a_body() {
        assert!(find("create function f() returns int as $$ select 1 $$ language sql;").is_none());
    }

    #[test]
    fn procedure() {
        assert_snapshot!(
            body("create procedure p() as $$ null; $$ language plpgsql;"),
            @r#"
        PLPGSQL@0..7
          WHITESPACE@0..1 " "
          PLPGSQL_NULL_STMT@1..6
            NULL_KW@1..5 "null"
            SEMICOLON@5..6 ";"
          WHITESPACE@6..7 " "
        ---
        source 26..33 " null; "
        "#
        );
    }

    #[test]
    fn do_defaults_to_plpgsql() {
        assert_snapshot!(body("do $$ null; $$;"), @r#"
        PLPGSQL@0..7
          WHITESPACE@0..1 " "
          PLPGSQL_NULL_STMT@1..6
            NULL_KW@1..5 "null"
            SEMICOLON@5..6 ";"
          WHITESPACE@6..7 " "
        ---
        source 5..12 " null; "
        "#
        );
    }

    #[test]
    fn do_with_other_language_is_not_a_body() {
        assert!(find("do language sql $$ select 1 $$;").is_none());
    }

    #[test]
    fn escaped_body_maps_back_through_the_escapes() {
        assert_snapshot!(
            body(r"create function f() returns int as E'null;\n' language plpgsql;"),
            @r#"
        PLPGSQL@0..6
          PLPGSQL_NULL_STMT@0..5
            NULL_KW@0..4 "null"
            SEMICOLON@4..5 ";"
          WHITESPACE@5..6 "\n"
        ---
        source 37..44 "null;\\n"
        "#
        );
    }

    #[test]
    fn unparsed_tokens_are_errors() {
        assert_snapshot!(body("do $$ begin null; end $$;"), @r#"
        PLPGSQL@0..17
          WHITESPACE@0..1 " "
          ERROR@1..6
            BEGIN_KW@1..6 "begin"
          WHITESPACE@6..7 " "
          PLPGSQL_NULL_STMT@7..12
            NULL_KW@7..11 "null"
            SEMICOLON@11..12 ";"
          WHITESPACE@12..13 " "
          ERROR@13..16
            END_KW@13..16 "end"
          WHITESPACE@16..17 " "
        ---
        source 5..22 " begin null; end "
        error 6..6 expected a statement, found BEGIN_KW
        error 18..18 expected a statement, found END_KW
        "#);
    }

    #[test]
    fn errors_in_an_escaped_body_map_into_the_file() {
        assert_snapshot!(body(r"do E'begin\n null;\n end';"), @r#"
        PLPGSQL@0..17
          ERROR@0..5
            BEGIN_KW@0..5 "begin"
          WHITESPACE@5..7 "\n "
          PLPGSQL_NULL_STMT@7..12
            NULL_KW@7..11 "null"
            SEMICOLON@11..12 ";"
          WHITESPACE@12..14 "\n "
          ERROR@14..17
            END_KW@14..17 "end"
        ---
        source 5..24 "begin\\n null;\\n end"
        error 5..5 expected a statement, found BEGIN_KW
        error 21..21 expected a statement, found END_KW
        "#);
    }
}
