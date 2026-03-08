use squawk_syntax::{
    Parse, SourceFile, SyntaxKind,
    ast::{self, AstNode},
};

use crate::{Linter, Rule, Violation};

pub(crate) fn require_enum_value_ordering(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for stmt in file.stmts() {
        if let ast::Stmt::AlterType(alter_type) = stmt {
            let syntax = alter_type.syntax();

            let mut has_add = false;
            let mut has_value = false;
            let mut has_before_or_after = false;

            for child in syntax.children_with_tokens() {
                match child.kind() {
                    SyntaxKind::ADD_KW => has_add = true,
                    SyntaxKind::VALUE_KW if has_add => has_value = true,
                    SyntaxKind::BEFORE_KW | SyntaxKind::AFTER_KW if has_value => {
                        has_before_or_after = true;
                    }
                    _ => {}
                }
            }

            if has_add && has_value && !has_before_or_after {
                ctx.report(
                    Violation::for_node(
                        Rule::RequireEnumValueOrdering,
                        "ALTER TYPE ... ADD VALUE without BEFORE or AFTER appends the value to the end of the enum, which may result in unexpected ordering.".into(),
                        syntax,
                    )
                    .help("Add BEFORE or AFTER to specify the position of the new enum value. Example: ALTER TYPE my_enum ADD VALUE 'new_value' BEFORE 'existing_value';"),
                );
            }
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::Rule;
    use crate::test_utils::{lint_errors, lint_ok};

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

    #[test]
    fn ok_add_value_if_not_exists_before() {
        let sql = r#"
ALTER TYPE my_enum ADD VALUE IF NOT EXISTS 'new_value' BEFORE 'existing_value';
"#;
        lint_ok(sql, Rule::RequireEnumValueOrdering);
    }

    #[test]
    fn ok_rename_value() {
        let sql = r#"
ALTER TYPE my_enum RENAME VALUE 'old' TO 'new';
"#;
        lint_ok(sql, Rule::RequireEnumValueOrdering);
    }

    #[test]
    fn ok_add_attribute() {
        let sql = r#"
ALTER TYPE my_type ADD ATTRIBUTE name text;
"#;
        lint_ok(sql, Rule::RequireEnumValueOrdering);
    }
}
