use squawk_diagnostics::{Edit, Fix};
use squawk_syntax::ast;
use squawk_syntax::ast::AstNode;

use crate::{Linter, Rule, Violation};

use crate::visitors::check_not_allowed_types;
use crate::visitors::is_not_valid_int_type;

fn is_int_name(name: &str) -> bool {
    matches!(name, "int" | "integer" | "int4" | "serial" | "serial4")
}

fn int_to_bigint_replacement(int_type: &str) -> BigintType {
    match int_type {
        "int" | "integer" => BigintType::Bigint,
        "int4" => BigintType::Int8,
        "serial" => BigintType::Bigserial,
        "serial4" => BigintType::Serial8,
        _ => BigintType::Bigint,
    }
}

#[derive(Clone, Copy)]
pub(crate) enum BigintType {
    Bigint,
    Int8,
    Bigserial,
    Serial8,
}

impl BigintType {
    pub(crate) fn name(self) -> &'static str {
        match self {
            BigintType::Bigint => "bigint",
            BigintType::Int8 => "int8",
            BigintType::Bigserial => "bigserial",
            BigintType::Serial8 => "serial8",
        }
    }

    pub(crate) fn fix_title(self) -> &'static str {
        match self {
            BigintType::Bigint => "Replace with a 64-bit integer type: `bigint`",
            BigintType::Int8 => "Replace with a 64-bit integer type: `int8`",
            BigintType::Bigserial => "Replace with a 64-bit integer type: `bigserial`",
            BigintType::Serial8 => "Replace with a 64-bit integer type: `serial8`",
        }
    }
}

fn create_bigint_fix(ty: &ast::Type) -> Option<Fix> {
    if let ast::Type::ArrayType(array_type) = ty {
        return create_bigint_fix(&array_type.ty()?);
    }
    let name = ty.path_ref()?.segment()?;
    let int_type = name.text();
    let i64 = int_to_bigint_replacement(&int_type);
    let edit = Edit::replace(name.syntax().text_range(), i64.name());
    Some(Fix::new(i64.fix_title(), vec![edit]))
}

fn check_ty_for_big_int(ctx: &mut Linter, ty: Option<ast::Type>) {
    if let Some(ty) = ty
        && is_not_valid_int_type(&ty, is_int_name)
    {
        let fix = create_bigint_fix(&ty);

        ctx.report(
            Violation::for_node(
                Rule::PreferBigintOverInt,
                "using 32-bit integer fields can result in hitting the max `int` limit".into(),
                ty.syntax(),
            )
            .help("use 64-bit integer values instead to prevent hitting this limit")
            .fix(fix),
        );
    }
}

// TODO: we should have this be a config option instead of having a bunch of prefer_$int rules
pub(crate) fn prefer_bigint_over_int(ctx: &mut Linter, file: &ast::SourceFile) {
    check_not_allowed_types(ctx, file, check_ty_for_big_int);
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
        fix_sql(sql, Rule::PreferBigintOverInt)
    }

    #[test]
    fn fix_int_types() {
        assert_snapshot!(fix("create table users (id int);"), @"create table users (id bigint);");
        assert_snapshot!(fix("create table users (id integer);"), @"create table users (id bigint);");
        assert_snapshot!(fix("create table users (id int4);"), @"create table users (id int8);");
        assert_snapshot!(fix("create table users (id serial);"), @"create table users (id bigserial);");
        assert_snapshot!(fix("create table users (id serial4);"), @"create table users (id serial8);");
    }

    #[test]
    fn fix_mixed_case() {
        assert_snapshot!(fix("create table users (id INT);"), @"create table users (id bigint);");
        assert_snapshot!(fix("create table users (id INTEGER);"), @"create table users (id bigint);");
        assert_snapshot!(fix("create table users (id Int4);"), @"create table users (id int8);");
        assert_snapshot!(fix("create table users (id Serial);"), @"create table users (id bigserial);");
        assert_snapshot!(fix("create table users (id SERIAL4);"), @"create table users (id serial8);");
    }

    #[test]
    fn fix_multiple_columns() {
        assert_snapshot!(fix("create table users (id int, count integer, version serial);"), @"create table users (id bigint, count bigint, version bigserial);");
    }

    #[test]
    fn fix_with_constraints() {
        assert_snapshot!(fix("create table users (id serial primary key, score int not null);"), @"create table users (id bigserial primary key, score bigint not null);");
    }

    #[test]
    fn fix_array_types() {
        assert_snapshot!(fix("create table users (ids int[]);"), @"create table users (ids bigint[]);");
        assert_snapshot!(fix("create table users (ids integer[]);"), @"create table users (ids bigint[]);");
        assert_snapshot!(fix("create table users (ids int4[]);"), @"create table users (ids int8[]);");
        assert_snapshot!(fix("create table users (ids serial[]);"), @"create table users (ids bigserial[]);");
        assert_snapshot!(fix("create table users (ids serial4[]);"), @"create table users (ids serial8[]);");
    }

    #[test]
    fn err() {
        let sql = r#"
create table users (
    id int
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
        assert_snapshot!(lint_errors(sql, Rule::PreferBigintOverInt));
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
        lint_ok(sql, Rule::PreferBigintOverInt);
    }
}
