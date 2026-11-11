use std::sync::LazyLock;

use rustc_hash::FxHashSet;

use squawk_syntax::ast;
use squawk_syntax::ast::AstNode;

use crate::{Linter, Rule, Version, Violation};

static NON_VOLATILE_FUNCS: LazyLock<FxHashSet<&'static str>> = LazyLock::new(|| {
    NON_VOLATILE_BUILT_IN_FUNCTIONS
        .split('\n')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect()
});

fn is_non_volatile_or_const(expr: &ast::Expr) -> bool {
    match expr.unparen() {
        ast::Expr::Literal(_) => true,
        ast::Expr::PrefixExpr(prefix_expr) => {
            prefix_expr.op().is_some()
                && prefix_expr
                    .expr()
                    .is_some_and(|expr| is_non_volatile_or_const(&expr))
        }
        ast::Expr::ArrayExpr(array_expr) => array_expr
            .exprs()
            .all(|expr| is_non_volatile_or_const(&expr)),
        ast::Expr::BinExpr(bin_expr) => {
            if let Some(lhs) = bin_expr.lhs()
                && let Some(rhs) = bin_expr.rhs()
            {
                return is_non_volatile_or_const(&lhs) && is_non_volatile_or_const(&rhs);
            }
            false
        }
        ast::Expr::CallExpr(call_expr) => {
            let Some(arg_list) = call_expr.arg_list() else {
                return false;
            };

            // TODO: what about FieldExpr? like, pg_catalog.uuid()
            let Some(name_ref) = call_expr.expr().and_then(|expr| expr.as_name_ref()) else {
                return false;
            };

            let non_volatile_name = name_ref.sql_value_function().is_some()
                || NON_VOLATILE_FUNCS.contains(name_ref.text().as_ref());

            non_volatile_name
                && arg_list.args().all(|arg| {
                    arg.func_arg_expr()
                        .and_then(|arg| arg.expr())
                        .is_some_and(|expr| is_non_volatile_or_const(&expr))
                })
        }
        // array[]::t[] is non-volatile. We don't check for a plain array expr
        // since postgres will reject it as a default unless it's cast to a type.
        ast::Expr::CastExpr(cast_expr) => {
            if let Some(inner_expr) = cast_expr.expr() {
                is_non_volatile_or_const(&inner_expr)
            } else {
                false
            }
        }
        ast::Expr::Collate(collate) => collate
            .expr()
            .is_some_and(|expr| is_non_volatile_or_const(&expr)),
        ast::Expr::NameRef(name_ref) => name_ref.sql_value_function().is_some(),
        _ => false,
    }
}

fn is_virtual(generated: &ast::GeneratedConstraint, pg_version: Version) -> bool {
    let Some(ast::GeneratedAs::GeneratedStored(generated_stored)) = generated.generated_as() else {
        return false;
    };
    match generated_stored.generated_kind() {
        Some(ast::GeneratedKind::Virtual(_)) => true,
        Some(ast::GeneratedKind::Stored(_)) => false,
        None => pg_version >= Version::major(18),
    }
}

// Generated via the following Postgres query:
//      select proname from pg_proc where provolatile <> 'v';
const NON_VOLATILE_BUILT_IN_FUNCTIONS: &str = include_str!("non_volatile_built_in_functions.txt");

pub(crate) fn adding_field_with_default(ctx: &mut Linter, file: &ast::SourceFile) {
    let default_message = "adding a column with a volatile default rewrites the table under an `access exclusive` lock";
    let default_note = "in Postgres 11+, non-volatile defaults can be added without a rewrite";
    let generated_message =
        "adding a generated column rewrites the table under an `access exclusive` lock";
    let serial_message =
        "adding a `serial` column rewrites the table under an `access exclusive` lock";
    let serial_note = "`serial` columns have an implicit `nextval()` default, which is volatile";
    let help = "add the column as nullable, backfill existing rows, and add a trigger to update the column on write instead";
    // TODO: use match_ast! like in #api_walkthrough
    for stmt in file.stmts() {
        if let ast::Stmt::AlterTable(alter_table) = stmt {
            for action in alter_table.actions() {
                if let ast::AlterTableAction::AddColumn(add_column) = action {
                    if let Some(ty) = add_column.ty()
                        && ty.serial_type_name().is_some()
                    {
                        ctx.report(
                            Violation::for_node(
                                Rule::AddingFieldWithDefault,
                                serial_message.into(),
                                ty.syntax(),
                            )
                            .note(serial_note)
                            .help(help),
                        );
                    }
                    for constraint in add_column.constraints() {
                        match constraint {
                            ast::Constraint::DefaultConstraint(default) => {
                                let Some(expr) = default.expr() else {
                                    continue;
                                };
                                if ctx.settings.pg_version >= Version::major(11)
                                    && is_non_volatile_or_const(&expr)
                                {
                                    continue;
                                }
                                ctx.report(
                                    Violation::for_node(
                                        Rule::AddingFieldWithDefault,
                                        default_message.into(),
                                        expr.syntax(),
                                    )
                                    .note(default_note)
                                    .help(help),
                                );
                            }
                            ast::Constraint::GeneratedConstraint(generated) => {
                                if is_virtual(&generated, ctx.settings.pg_version) {
                                    continue;
                                }
                                ctx.report(
                                    Violation::for_node(
                                        Rule::AddingFieldWithDefault,
                                        generated_message.into(),
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
    use insta::assert_snapshot;

    use crate::test_utils::{lint_errors, lint_ok, lint_ok_with};
    use crate::{LinterSettings, Rule};

    #[must_use]
    fn lint_errors_with(sql: &str, settings: LinterSettings) -> String {
        crate::test_utils::lint_errors_with(sql, settings, Rule::AddingFieldWithDefault)
    }

    #[test]
    fn docs_example_ok_post_pg_11() {
        let sql = r#"
-- instead of
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
        "#;

        lint_ok(sql, Rule::AddingFieldWithDefault);
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

        lint_ok(sql, Rule::AddingFieldWithDefault);
    }

    #[test]
    fn default_uuid_error_multi_stmt() {
        let sql = r#"
alter table t set logged, add column c integer default uuid();
        "#;

        assert_snapshot!(lint_errors(sql, Rule::AddingFieldWithDefault));
    }

    #[test]
    fn default_uuid_error() {
        let sql = r#"
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT uuid();
        "#;

        assert_snapshot!(lint_errors(sql, Rule::AddingFieldWithDefault));
    }

    #[test]
    fn default_volatile_func_err() {
        let sql = r#"
    -- VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" boolean DEFAULT random();
            "#;

        assert_snapshot!(lint_errors(sql, Rule::AddingFieldWithDefault));
    }

    #[test]
    fn default_bool_ok() {
        let sql = r#"
    -- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" boolean DEFAULT true;
            "#;

        lint_ok(sql, Rule::AddingFieldWithDefault);
    }

    #[test]
    fn default_empty_array_ok() {
        let sql = r#"
alter table t add column a double precision[] default array[]::double precision[];

alter table t add column b bigint[] default cast(array[] as bigint[]);

alter table t add column c text[] default array['foo', 'bar']::text[];

alter table t add column d int[] default array[[1, 2], [3, 4]]::int[];
            "#;

        lint_ok(sql, Rule::AddingFieldWithDefault);
    }

    #[test]
    fn default_array_with_volatile_element_err() {
        let sql = r#"
alter table t add column a float8[] default array[random()];

alter table t add column b float8[] default array[random()]::float8[];
            "#;

        assert_snapshot!(lint_errors(sql, Rule::AddingFieldWithDefault));
    }

    #[test]
    fn default_with_const_bin_expr() {
        let sql = r#"
ALTER TABLE assessments
ADD COLUMN statistics_last_updated_at timestamptz NOT NULL DEFAULT now() - interval '100 years';
            "#;

        lint_ok(sql, Rule::AddingFieldWithDefault);
    }

    #[test]
    fn default_str_ok() {
        let sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" text DEFAULT 'some-str';
            "#;

        lint_ok(sql, Rule::AddingFieldWithDefault);
    }

    #[test]
    fn default_enum_ok() {
        let sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" some_enum_type DEFAULT 'my-enum-variant';
        "#;

        lint_ok(sql, Rule::AddingFieldWithDefault);
    }

    #[test]
    fn default_jsonb_ok() {
        let sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" jsonb DEFAULT '{}'::jsonb;
        "#;

        lint_ok(sql, Rule::AddingFieldWithDefault);
    }

    #[test]
    fn arbitrary_func_err() {
        let sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" jsonb DEFAULT myjsonb();
        "#;

        assert_snapshot!(lint_errors(sql, Rule::AddingFieldWithDefault));
    }

    #[test]
    fn default_random_with_args_err() {
        let sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" float8 DEFAULT random_normal(0, 1);
        "#;

        assert_snapshot!(lint_errors(sql, Rule::AddingFieldWithDefault));
    }

    #[test]
    fn default_func_now_ok() {
        let sql = r#"
-- NON-VOLATILE
ALTER TABLE "core_recipe" ADD COLUMN "foo" timestamptz DEFAULT now();
        "#;

        lint_ok(sql, Rule::AddingFieldWithDefault);
    }

    #[test]
    fn default_func_current_timestamp_ok() {
        let sql = r#"
alter table t add column c timestamptz default current_timestamp;
        "#;

        lint_ok(sql, Rule::AddingFieldWithDefault);
    }

    #[test]
    fn default_sql_value_functions_ok() {
        let sql = r#"
alter table t add column a date default current_date;

alter table t add column b timestamp default localtimestamp;

alter table t add column c timetz default current_time;

alter table t add column d text default current_user;

alter table t add column e timestamptz default current_timestamp(2);
        "#;

        lint_ok(sql, Rule::AddingFieldWithDefault);
    }

    #[test]
    fn default_non_volatile_func_with_args_ok() {
        let sql = r#"
alter table t add column a text default lower('A');
        "#;

        lint_ok(sql, Rule::AddingFieldWithDefault);
    }

    #[test]
    fn default_non_volatile_func_with_volatile_arg_err() {
        let sql = r#"
alter table t add column a float8 default abs(random());
        "#;

        assert_snapshot!(lint_errors(sql, Rule::AddingFieldWithDefault));
    }

    #[test]
    fn default_paren_and_prefix_ok() {
        let sql = r#"
alter table t add column a int default (1);

alter table t add column b timestamptz default (now() - interval '1 day');

alter table t add column c int default -1;

alter table t add column d bool default not (true);
        "#;

        lint_ok(sql, Rule::AddingFieldWithDefault);
    }

    #[test]
    fn default_paren_and_prefix_volatile_err() {
        let sql = r#"
alter table t add column a float8 default (random());

alter table t add column b float8 default -random();
        "#;

        assert_snapshot!(lint_errors(sql, Rule::AddingFieldWithDefault));
    }

    #[test]
    fn add_numbers_ok() {
        let sql = r#"
alter table account_metadata add column blah integer default 2 + 2;
        "#;

        lint_ok(sql, Rule::AddingFieldWithDefault);
    }

    #[test]
    fn generated_stored_err() {
        let sql = r#"
ALTER TABLE foo
ADD COLUMN bar numeric GENERATED ALWAYS AS (bar + baz) STORED;
        "#;

        assert_snapshot!(lint_errors(sql, Rule::AddingFieldWithDefault));
    }

    #[test]
    fn generated_virtual_ok() {
        let sql = r#"
alter table t add column c int generated always as (x + 1) virtual;
        "#;

        lint_ok(sql, Rule::AddingFieldWithDefault);
    }

    #[test]
    fn generated_without_kind_ok_on_pg_18() {
        let sql = r#"
alter table t add column d int generated always as (x + 1);
        "#;

        lint_ok_with(
            sql,
            LinterSettings {
                pg_version: "18".parse().expect("Invalid PostgreSQL version"),
                ..Default::default()
            },
            Rule::AddingFieldWithDefault,
        );
    }

    #[test]
    fn generated_without_kind_err_before_pg_18() {
        let sql = r#"
alter table t add column d int generated always as (x + 1);
        "#;

        assert_snapshot!(lint_errors_with(
            sql,
            LinterSettings {
                pg_version: "17".parse().expect("Invalid PostgreSQL version"),
                ..Default::default()
            },
        ));
    }

    #[test]
    fn add_serial_column_err() {
        let sql = r#"
alter table t add column e bigserial;
        "#;

        assert_snapshot!(lint_errors(sql, Rule::AddingFieldWithDefault));
    }

    #[test]
    fn docs_example_error_on_pg_10() {
        let sql = r#"
-- instead of
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
        "#;

        assert_snapshot!(lint_errors_with(
            sql,
            LinterSettings {
                pg_version: "10".parse().expect("Invalid PostgreSQL version"),
                ..Default::default()
            },
        ));
    }

    #[test]
    fn docs_example_ok_on_pg_11() {
        let sql = r#"
alter table t add column a int default 1;
        "#;

        lint_ok_with(
            sql,
            LinterSettings {
                pg_version: "11".parse().expect("Invalid PostgreSQL version"),
                ..Default::default()
            },
            Rule::AddingFieldWithDefault,
        );
    }
}
