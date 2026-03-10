use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

use crate::{Edit, Fix, Linter, Rule, Violation};

fn create_fix(add_value: &ast::AddValue) -> Option<Fix> {
    let literal = add_value.literal()?;
    let insert_at = literal.syntax().text_range().end();
    let edit = Edit::insert(" BEFORE 'existing_value'", insert_at);
    Some(Fix::new("Insert `BEFORE` clause", vec![edit]))
}

pub(crate) fn require_enum_value_ordering(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for stmt in file.stmts() {
        if let ast::Stmt::AlterType(alter_type) = stmt
            && let Some(add_value) = alter_type.add_value()
            && add_value.value_position().is_none()
        {
            let fix = create_fix(&add_value);
            ctx.report(
                    Violation::for_node(
                        Rule::RequireEnumValueOrdering,
                        "ADD VALUE without BEFORE or AFTER appends the value to the end of the enum, which may result in unexpected ordering.".into(),
                        add_value.syntax(),
                    )
                    .help("Add `BEFORE` or `AFTER` to specify the position of the new enum value.")
                    .fix(fix),
                );
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::Rule;
    use crate::test_utils::{fix_sql, lint_errors, lint_ok};

    #[test]
    fn err_add_value_without_ordering() {
        let sql = r#"
ALTER TYPE my_enum ADD VALUE 'new_value';
"#;
        assert_snapshot!(lint_errors(sql, Rule::RequireEnumValueOrdering));
    }

    #[test]
    fn err_add_value_if_not_exists_without_ordering() {
        let sql = r#"
ALTER TYPE my_enum ADD VALUE IF NOT EXISTS 'new_value';
"#;
        assert_snapshot!(lint_errors(sql, Rule::RequireEnumValueOrdering));
    }

    #[test]
    fn fix_add_value_without_ordering() {
        let sql = r#"
ALTER TYPE my_enum ADD VALUE 'new_value';
"#;
        assert_snapshot!(fix_sql(sql, Rule::RequireEnumValueOrdering), @"ALTER TYPE my_enum ADD VALUE 'new_value' BEFORE 'existing_value';");
    }

    #[test]
    fn ok_add_value_before() {
        let sql = r#"
ALTER TYPE my_enum ADD VALUE 'new_value' BEFORE 'existing_value';
"#;
        lint_ok(sql, Rule::RequireEnumValueOrdering);
    }

    #[test]
    fn ok_add_value_after() {
        let sql = r#"
ALTER TYPE my_enum ADD VALUE 'new_value' AFTER 'existing_value';
"#;
        lint_ok(sql, Rule::RequireEnumValueOrdering);
    }
}
