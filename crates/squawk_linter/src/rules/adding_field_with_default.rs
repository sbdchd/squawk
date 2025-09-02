use lazy_static::lazy_static;
use std::collections::HashSet;

use squawk_syntax::ast;
use squawk_syntax::ast::AstNode;
use squawk_syntax::{Parse, SourceFile};

use crate::identifier::Identifier;
use crate::{Linter, Rule, Version, Violation};

fn is_const_expr(expr: &ast::Expr) -> bool {
    match expr {
        ast::Expr::Literal(_) => true,
        ast::Expr::CastExpr(cast) => matches!(cast.expr(), Some(ast::Expr::Literal(_))),
        _ => false,
    }
}

lazy_static! {
    static ref NON_VOLATILE_FUNCS: HashSet<Identifier> = {
        NON_VOLATILE_BUILT_IN_FUNCTIONS
            .split('\n')
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(Identifier::new)
            .collect()
    };
}

fn is_non_volatile(expr: &ast::Expr) -> bool {
    match expr {
        ast::Expr::CallExpr(call_expr) => {
            if let Some(arglist) = call_expr.arg_list() {
                let no_args = arglist.args().count() == 0;

                // TODO: what about FieldExpr? like, pg_catalog.uuid()
                let Some(ast::Expr::NameRef(name_ref)) = call_expr.expr() else {
                    return false;
                };

                let non_volatile_name =
                    NON_VOLATILE_FUNCS.contains(&Identifier::new(name_ref.text().as_str()));

                no_args && non_volatile_name
            } else {
                false
            }
        }
        _ => false,
    }
}

// Generated via the following Postgres query:
//      select proname from pg_proc where provolatile <> 'v';
const NON_VOLATILE_BUILT_IN_FUNCTIONS: &str = include_str!("non_volatile_built_in_functions.txt");

pub(crate) fn adding_field_with_default(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let message = "Adding a generated column requires a table rewrite with an `ACCESS EXCLUSIVE` lock. In Postgres versions 11+, non-VOLATILE DEFAULTs can be added without a rewrite.";
    let help = "Add the column as nullable, backfill existing rows, and add a trigger to update the column on write instead.";
    let file = parse.tree();
    // TODO: use match_ast! like in #api_walkthrough
    for stmt in file.stmts() {
        if let ast::Stmt::AlterTable(alter_table) = stmt {
            for action in alter_table.actions() {
                if let ast::AlterTableAction::AddColumn(add_column) = action {
                    for constraint in add_column.constraints() {
                        match constraint {
                            ast::Constraint::DefaultConstraint(default) => {
                                let Some(expr) = default.expr() else {
                                    continue;
                                };
                                if ctx.settings.pg_version > Version::new(11, None, None)
                                    && (is_const_expr(&expr) || is_non_volatile(&expr))
                                {
                                    continue;
                                }
                                ctx.report(
                                    Violation::for_node(
                                        Rule::AddingFieldWithDefault,
                                        message.into(),
                                        expr.syntax(),
                                    )
                                    .help(help),
                                )
                            }
                            ast::Constraint::GeneratedConstraint(generated) => {
                                ctx.report(
                                    Violation::for_node(
                                        Rule::AddingFieldWithDefault,
                                        message.into(),
                                        generated.syntax(),
                                    )
                                    .help(help),
                                );
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::Rule;
    use crate::test_utils::{lint, lint_with_postgres_version};

    #[test]
    fn docs_example_ok_post_pg_11() {
        let sql = r#"
-- instead of
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
        "#;

        let errors = lint(sql, Rule::AddingFieldWithDefault);
        assert!(errors.is_empty());
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn docs_example_ok() {
        let sql = r#"
-- use
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer;
ALTER TABLE "core_recipe" ALTER COLUMN "foo" SET DEFAULT 10;
-- backfill
-- remove nullability
            "#;

        let errors = lint(sql, Rule::AddingFieldWithDefault);
        assert!(errors.is_empty());
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn default_uuid_error_multi_stmt() {
        let sql = r#"
alter table t set logged, add column c integer default uuid();
        "#;

        let errors = lint(sql, Rule::AddingFieldWithDefault);
        assert!(!errors.is_empty());
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn default_uuid_error() {
        let sql = r#"
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT uuid();
        "#;

        let errors = lint(sql, Rule::AddingFieldWithDefault);
        assert!(!errors.is_empty());
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn default_volatile_func_err() {
        let sql = r#"
    -- VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" boolean DEFAULT random();
            "#;

        let errors = lint(sql, Rule::AddingFieldWithDefault);
        assert!(!errors.is_empty());
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn default_bool_ok() {
        let sql = r#"
    -- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" boolean DEFAULT true;
            "#;

        let errors = lint(sql, Rule::AddingFieldWithDefault);
        assert!(errors.is_empty());
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn default_str_ok() {
        let sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" text DEFAULT 'some-str';
            "#;

        let errors = lint(sql, Rule::AddingFieldWithDefault);
        assert!(errors.is_empty());
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn default_enum_ok() {
        let sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" some_enum_type DEFAULT 'my-enum-variant';
        "#;

        let errors = lint(sql, Rule::AddingFieldWithDefault);
        assert!(errors.is_empty());
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn default_jsonb_ok() {
        let sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" jsonb DEFAULT '{}'::jsonb;
        "#;

        let errors = lint(sql, Rule::AddingFieldWithDefault);
        assert!(errors.is_empty());
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn arbitrary_func_err() {
        let sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" jsonb DEFAULT myjsonb();
        "#;

        let errors = lint(sql, Rule::AddingFieldWithDefault);
        assert!(!errors.is_empty());
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn default_random_with_args_err() {
        let sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" timestamptz DEFAULT now(123);
        "#;

        let errors = lint(sql, Rule::AddingFieldWithDefault);
        assert!(!errors.is_empty());
        assert_debug_snapshot!(errors);
    }
    #[test]
    fn default_func_now_ok() {
        let sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" timestamptz DEFAULT now();
        "#;

        let errors = lint(sql, Rule::AddingFieldWithDefault);
        assert!(errors.is_empty());
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn add_numbers_ok() {
        // This should be okay, but we don't handle expressions like this at the moment.
        let sql = r#"
alter table account_metadata add column blah integer default 2 + 2;
        "#;

        let errors = lint(sql, Rule::AddingFieldWithDefault);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn generated_stored_err() {
        let sql = r#"
ALTER TABLE foo
ADD COLUMN bar numeric GENERATED ALWAYS AS (bar + baz) STORED;
        "#;

        let errors = lint(sql, Rule::AddingFieldWithDefault);
        assert!(!errors.is_empty());
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn docs_example_error_on_pg_11() {
        let sql = r#"
-- instead of
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
        "#;

        let errors = lint_with_postgres_version(sql, Rule::AddingFieldWithDefault, "11");
        assert!(!errors.is_empty());
        assert_debug_snapshot!(errors);
    }
}
