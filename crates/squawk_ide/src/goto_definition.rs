use rowan::{TextRange, TextSize};
use squawk_syntax::{
    SyntaxKind, SyntaxToken,
    ast::{self, AstNode},
};

pub fn goto_definition(file: ast::SourceFile, offset: TextSize) -> Option<TextRange> {
    let token = token_from_offset(&file, offset)?;
    let parent = token.parent()?;

    // goto def on case exprs
    if (token.kind() == SyntaxKind::WHEN_KW && parent.kind() == SyntaxKind::WHEN_CLAUSE)
        || (token.kind() == SyntaxKind::ELSE_KW && parent.kind() == SyntaxKind::ELSE_CLAUSE)
        || (token.kind() == SyntaxKind::END_KW && parent.kind() == SyntaxKind::CASE_EXPR)
    {
        for parent in token.parent_ancestors() {
            if let Some(case_expr) = ast::CaseExpr::cast(parent) {
                if let Some(case_token) = case_expr.case_token() {
                    return Some(case_token.text_range());
                }
            }
        }
    }

    return None;
}

fn token_from_offset(file: &ast::SourceFile, offset: TextSize) -> Option<SyntaxToken> {
    let mut token = file.syntax().token_at_offset(offset).right_biased()?;
    // want to be lenient in case someone clicks the trailing `;` of a line
    // instead of an identifier
    if token.kind() == SyntaxKind::SEMICOLON {
        token = token.prev_token()?;
    }
    return Some(token);
}

#[cfg(test)]
mod test {
    use crate::goto_definition::goto_definition;
    use crate::test_utils::fixture;
    use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
    use insta::assert_snapshot;
    use log::info;
    use squawk_syntax::ast;

    #[track_caller]
    fn goto(sql: &str) -> String {
        goto_(sql).expect("should always find a definition")
    }

    #[track_caller]
    fn goto_(sql: &str) -> Option<String> {
        info!("starting");
        let (mut offset, sql) = fixture(sql);
        // For go to def we want the previous character since we usually put the
        // marker after the item we're trying to go to def on.
        offset = offset.checked_sub(1.into()).unwrap_or_default();
        let parse = ast::SourceFile::parse(&sql);
        assert_eq!(parse.errors(), vec![]);
        let file: ast::SourceFile = parse.tree();
        if let Some(result) = goto_definition(file, offset) {
            let offset: usize = offset.into();
            let group = Level::INFO.primary_title("definition").element(
                Snippet::source(&sql)
                    .fold(true)
                    .annotation(
                        AnnotationKind::Context
                            .span(result.into())
                            .label("2. destination"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(offset..offset + 1)
                            .label("1. source"),
                    ),
            );
            let renderer = Renderer::plain().decor_style(DecorStyle::Unicode);
            return Some(
                renderer
                    .render(&[group])
                    .to_string()
                    // hacky cleanup to make the text shorter
                    .replace("info: definition", ""),
            );
        }
        None
    }

    fn goto_not_found(sql: &str) {
        assert!(goto_(sql).is_none(), "Should not find a definition");
    }

    #[test]
    fn goto_case_when() {
        assert_snapshot!(goto("
select case when$0 x > 1 then 1 else 2 end;
"), @r"
          ╭▸ 
        2 │ select case when x > 1 then 1 else 2 end;
          │        ┬───    ─ 1. source
          │        │
          ╰╴       2. destination
        ");
    }

    #[test]
    fn goto_case_else() {
        assert_snapshot!(goto("
select case when x > 1 then 1 else$0 2 end;
"), @r"
          ╭▸ 
        2 │ select case when x > 1 then 1 else 2 end;
          ╰╴       ──── 2. destination       ─ 1. source
        ");
    }

    #[test]
    fn goto_case_end() {
        assert_snapshot!(goto("
select case when x > 1 then 1 else 2 end$0;
"), @r"
          ╭▸ 
        2 │ select case when x > 1 then 1 else 2 end;
          ╰╴       ──── 2. destination             ─ 1. source
        ");
    }

    #[test]
    fn goto_case_end_trailing_semi() {
        assert_snapshot!(goto("
select case when x > 1 then 1 else 2 end;$0
"), @r"
          ╭▸ 
        2 │ select case when x > 1 then 1 else 2 end;
          ╰╴       ──── 2. destination              ─ 1. source
        ");
    }

    #[test]
    fn goto_case_then_not_found() {
        goto_not_found(
            "
select case when x > 1 then$0 1 else 2 end;
",
        )
    }
}
