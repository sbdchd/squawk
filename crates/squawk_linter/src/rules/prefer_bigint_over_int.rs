use std::collections::HashSet;

use squawk_syntax::ast::AstNode;
use squawk_syntax::{Parse, SourceFile, ast};

use crate::identifier::Identifier;
use crate::{Edit, Fix, Linter, Rule, Violation};

use crate::visitors::check_not_allowed_types;
use crate::visitors::is_not_valid_int_type;

use lazy_static::lazy_static;

lazy_static! {
    static ref INT_TYPES: HashSet<Identifier> = HashSet::from([
        Identifier::new("int"),
        Identifier::new("integer"),
        Identifier::new("int4"),
        Identifier::new("serial"),
        Identifier::new("serial4"),
    ]);
}

fn int_to_bigint_replacement(int_type: &str) -> &'static str {
    match int_type.to_lowercase().as_str() {
        "int" | "integer" => "bigint",
        "int4" => "int8",
        "serial" => "bigserial",
        "serial4" => "serial8",
        _ => "bigint",
    }
}

fn create_bigint_fix(ty: &ast::Type) -> Option<Fix> {
    let type_name = ty.syntax().first_token()?;
    let i64 = int_to_bigint_replacement(type_name.text());
    let edit = Edit::replace(ty.syntax().text_range(), i64);
    Some(Fix::new(
        format!("Replace with a 64-bit integer type: `{i64}`"),
        vec![edit],
    ))
}

fn check_ty_for_big_int(ctx: &mut Linter, ty: Option<ast::Type>) {
    if let Some(ty) = ty {
        if is_not_valid_int_type(&ty, &INT_TYPES) {
            let fix = create_bigint_fix(&ty);

            ctx.report(
                Violation::for_node(
                    Rule::PreferBigintOverInt,
                    "Using 32-bit integer fields can result in hitting the max `int` limit.".into(),
                    ty.syntax(),
                )
                .help("Use 64-bit integer values instead to prevent hitting this limit.")
                .fix(fix),
            );
        };
    }
}

// TODO: we should have this be a config option instead of having a bunch of prefer_$int rules
pub(crate) fn prefer_bigint_over_int(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    check_not_allowed_types(ctx, &file, check_ty_for_big_int);
}

#[cfg(test)]
mod test {
    use insta::{assert_debug_snapshot, assert_snapshot};

    use crate::{
        Rule,
        test_utils::{fix_sql, lint},
    };

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
        let errors = lint(sql, Rule::PreferBigintOverInt);
        assert_ne!(errors.len(), 0);
        assert_eq!(errors.len(), 5);
        assert_eq!(
            errors
                .iter()
                .filter(|x| x.code == Rule::PreferBigintOverInt)
                .count(),
            5
        );
        assert_debug_snapshot!(errors);
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
        let errors = lint(sql, Rule::PreferBigintOverInt);
        assert_eq!(errors.len(), 0);
    }
}
