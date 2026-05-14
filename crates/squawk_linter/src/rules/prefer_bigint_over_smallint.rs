use rustc_hash::FxHashSet;

use squawk_syntax::ast::AstNode;
use squawk_syntax::{Parse, SourceFile, ast};

use crate::{Edit, Fix, Linter, Rule, Violation};

use crate::visitors::check_not_allowed_types;
use crate::visitors::is_not_valid_int_type;

use std::sync::OnceLock;

fn small_int_types() -> &'static FxHashSet<&'static str> {
    static SMALL_INT_TYPES: OnceLock<FxHashSet<&'static str>> = OnceLock::new();
    SMALL_INT_TYPES
        .get_or_init(|| FxHashSet::from_iter(["smallint", "int2", "smallserial", "serial2"]))
}

fn smallint_to_bigint(smallint_type: &str) -> &'static str {
    match smallint_type {
        "smallint" => "bigint",
        "int2" => "int8",
        "smallserial" => "bigserial",
        "serial2" => "serial8",
        _ => "bigint",
    }
}

fn create_bigint_fix(ty: &ast::Type) -> Option<Fix> {
    let name = match ty {
        ast::Type::ArrayType(array_type) => return create_bigint_fix(&array_type.ty()?),
        ast::Type::PathType(path_type) => path_type.path()?.segment()?.name_ref()?,
        ast::Type::BitType(_)
        | ast::Type::CharType(_)
        | ast::Type::DoubleType(_)
        | ast::Type::ExprType(_)
        | ast::Type::PercentType(_)
        | ast::Type::TimeType(_)
        | ast::Type::IntervalType(_) => return None,
    };
    let i64 = smallint_to_bigint(&name.text());
    let edit = Edit::replace(name.syntax().text_range(), i64);
    Some(Fix::new(
        format!("Replace with a 64-bit integer type: `{i64}`"),
        vec![edit],
    ))
}

fn check_ty_for_small_int(ctx: &mut Linter, ty: Option<ast::Type>) {
    if let Some(ty) = ty {
        if is_not_valid_int_type(&ty, small_int_types()) {
            let fix = create_bigint_fix(&ty);

            ctx.report(
                Violation::for_node(
                    Rule::PreferBigintOverSmallint,
                    "Using 16-bit integer fields can result in hitting the max `int` limit.".into(),
                    ty.syntax(),
                )
                .help("Use 64-bit integer values instead to prevent hitting this limit.")
                .fix(fix),
            );
        };
    }
}

pub(crate) fn prefer_bigint_over_smallint(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    check_not_allowed_types(ctx, &file, check_ty_for_small_int);
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::{
        Rule,
        test_utils::{fix_sql, lint_errors, lint_ok},
    };

    #[must_use]
    fn fix(sql: &str) -> String {
        fix_sql(sql, Rule::PreferBigintOverSmallint)
    }

    #[test]
    fn fix_smallint_types() {
        assert_snapshot!(fix("create table users (id smallint);"), @"create table users (id bigint);");
        assert_snapshot!(fix("create table users (id int2);"), @"create table users (id int8);");
        assert_snapshot!(fix("create table users (id smallserial);"), @"create table users (id bigserial);");
        assert_snapshot!(fix("create table users (id serial2);"), @"create table users (id serial8);");
    }

    #[test]
    fn fix_mixed_case() {
        assert_snapshot!(fix("create table users (id SMALLINT);"), @"create table users (id bigint);");
        assert_snapshot!(fix("create table users (id Int2);"), @"create table users (id int8);");
        assert_snapshot!(fix("create table users (id SmallSerial);"), @"create table users (id bigserial);");
        assert_snapshot!(fix("create table users (id SERIAL2);"), @"create table users (id serial8);");
    }

    #[test]
    fn fix_multiple_columns() {
        assert_snapshot!(fix("create table users (id smallint, count int2, version smallserial);"), @"create table users (id bigint, count int8, version bigserial);");
    }

    #[test]
    fn fix_with_constraints() {
        assert_snapshot!(fix("create table users (id smallserial primary key, score smallint not null);"), @"create table users (id bigserial primary key, score bigint not null);");
    }

    #[test]
    fn fix_array_types() {
        assert_snapshot!(fix("create table users (ids smallint[]);"), @"create table users (ids bigint[]);");
        assert_snapshot!(fix("create table users (ids int2[]);"), @"create table users (ids int8[]);");
        assert_snapshot!(fix("create table users (ids smallserial[]);"), @"create table users (ids bigserial[]);");
        assert_snapshot!(fix("create table users (ids serial2[]);"), @"create table users (ids serial8[]);");
    }

    #[test]
    fn err() {
        let sql = r#"
create table users (
    id smallint
);
create table users (
    id int2
);
create table users (
    id smallserial
);
create table users (
    id serial2
);
        "#;
        assert_snapshot!(lint_errors(sql, Rule::PreferBigintOverSmallint));
    }

    #[test]
    fn ok() {
        let sql = r#"
create table users (
    id bigint
);
create table users (
    id int8
);
create table users (
    id bigserial
);
create table users (
    id serial8
);
create table users (
    id integer
);
create table users (
    id int4
);
create table users (
    id serial
);
create table users (
    id serial4
);
        "#;
        lint_ok(sql, Rule::PreferBigintOverSmallint);
    }
}
