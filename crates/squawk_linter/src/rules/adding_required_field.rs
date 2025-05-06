use squawk_syntax::{
    ast::{self, AstNode, HasModuleItem},
    Parse, SourceFile,
};

use crate::{Linter, Rule, Violation};

pub(crate) fn adding_required_field(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    for item in file.items() {
        if let ast::Item::AlterTable(alter_table) = item {
            for action in alter_table.actions() {
                if let ast::AlterTableAction::AddColumn(add_column) = action {
                    if has_generated_constrait(add_column.constraints()) {
                        continue;
                    }
                    if has_not_null_and_no_default_constraint(add_column.constraints()) {
                        ctx.report(Violation::new(
                            Rule::AddingRequiredField,
                            "Adding a new column that is `NOT NULL` and has no default value to an existing table effectively makes it required.".into(),
                            add_column.syntax().text_range(),
                            "Make the field nullable or add a non-VOLATILE DEFAULT".to_string(),
                        ));
                    }
                }
            }
        }
    }
}

fn has_generated_constrait(constraints: ast::AstChildren<ast::Constraint>) -> bool {
    for c in constraints {
        if let ast::Constraint::GeneratedConstraint(_) = c {
            return true;
        }
    }
    false
}

fn has_not_null_and_no_default_constraint(constraints: ast::AstChildren<ast::Constraint>) -> bool {
    let mut has_not_null = false;
    let mut has_default = false;
    for c in constraints {
        match c {
            ast::Constraint::NotNullConstraint(_) => {
                has_not_null = true;
            }
            ast::Constraint::DefaultConstraint(_) => {
                has_default = true;
            }
            _ => (),
        }
    }
    has_not_null && !has_default
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::{Linter, Rule};

    #[test]
    fn nullable_ok() {
        let sql = r#"
ALTER TABLE "recipe" ADD COLUMN "public" boolean;
  "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::AddingRequiredField]);
        let errors = linter.lint(file, sql);
        assert!(errors.is_empty());
    }

    #[test]
    fn not_null_with_default() {
        let sql = r#"
ALTER TABLE "recipe" ADD COLUMN "public" boolean NOT NULL DEFAULT true;
  "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::AddingRequiredField]);
        let errors = linter.lint(file, sql);
        assert!(errors.is_empty());
    }

    #[test]
    fn not_null_without_default() {
        let sql = r#"
ALTER TABLE "recipe" ADD COLUMN "public" boolean NOT NULL;
  "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::AddingRequiredField]);
        let errors = linter.lint(file, sql);
        assert!(!errors.is_empty());
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn generated_stored_not_null_ok() {
        let sql = r#"
ALTER TABLE foo
    ADD COLUMN bar numeric GENERATED ALWAYS AS (bar + baz) STORED NOT NULL;
  "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::AddingRequiredField]);
        let errors = linter.lint(file, sql);
        assert!(errors.is_empty());
    }

    #[test]
    fn generated_stored_ok() {
        let sql = r#"
 ALTER TABLE foo
    ADD COLUMN bar numeric GENERATED ALWAYS AS (bar + baz) STORED ;
  "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::AddingRequiredField]);
        let errors = linter.lint(file, sql);
        assert!(errors.is_empty());
    }
}
