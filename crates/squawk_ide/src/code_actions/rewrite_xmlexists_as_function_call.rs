use rowan::TextSize;
use salsa::Database as Db;
use squawk_linter::Edit;
use squawk_syntax::ast::{self, AstNode};

use crate::{file::InFile, offsets::token_from_offset};

use super::{ActionKind, CodeAction};

pub(super) fn rewrite_xmlexists_as_function_call(
    db: &dyn Db,
    position: InFile<TextSize>,
    actions: &mut Vec<CodeAction>,
) -> Option<()> {
    let token = token_from_offset(db, position)?;
    let xml_exists = token.parent_ancestors().find_map(ast::XmlExistsFn::cast)?;
    let passing = xml_exists.xml_row_passing_clause()?;
    let xpath = passing.row()?;
    let xml = passing.xml_passing_doc()?.expr()?;

    actions.push(CodeAction {
        title: "Rewrite as function call `pg_catalog.xmlexists()`".to_owned(),
        edits: vec![Edit::replace(
            xml_exists.syntax().text_range(),
            format!(
                "pg_catalog.xmlexists({}, {})",
                xpath.syntax().text(),
                xml.syntax().text()
            ),
        )],
        kind: ActionKind::RefactorRewrite,
    });

    Some(())
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::code_actions::test_utils::{apply_code_action, code_action_not_applicable};

    use super::rewrite_xmlexists_as_function_call;

    #[test]
    fn rewrites_xmlexists_as_function_call() {
        assert_snapshot!(
            apply_code_action(
                rewrite_xmlexists_as_function_call,
                "select XML$0EXISTS('//x' PASSING XMLPARSE(DOCUMENT '<r><x/></r>'));",
            ),
            @"select pg_catalog.xmlexists('//x', XMLPARSE(DOCUMENT '<r><x/></r>'));"
        );
    }

    #[test]
    fn applies_with_cursor_in_passing_expression() {
        assert_snapshot!(
            apply_code_action(
                rewrite_xmlexists_as_function_call,
                "select xmlexists(xpath passing xmlparse(document xm$0l_source));",
            ),
            @"select pg_catalog.xmlexists(xpath, xmlparse(document xml_source));"
        );
    }

    #[test]
    fn omits_optional_passing_mechanisms() {
        assert_snapshot!(
            apply_code_action(
                rewrite_xmlexists_as_function_call,
                "select xmlexists(path passing by va$0lue document by ref);",
            ),
            @"select pg_catalog.xmlexists(path, document);"
        );
    }

    #[test]
    fn rewrites_innermost_call_and_preserves_aggregate_clause() {
        assert_snapshot!(
            apply_code_action(
                rewrite_xmlexists_as_function_call,
                "select xmlexists(path passing xmlexists(inner_path pass$0ing document)::xml) FILTER (WHERE ok);",
            ),
            @"select xmlexists(path passing pg_catalog.xmlexists(inner_path, document)::xml) FILTER (WHERE ok);"
        );
    }

    #[test]
    fn not_applicable_outside_xmlexists() {
        assert!(code_action_not_applicable(
            rewrite_xmlexists_as_function_call,
            "select xmlparse(document xm$0l_source);"
        ));
    }
}
