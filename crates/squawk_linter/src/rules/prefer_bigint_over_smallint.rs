use std::collections::HashSet;

use squawk_syntax::ast::AstNode;
use squawk_syntax::{Parse, SourceFile, ast};

use crate::identifier::Identifier;
use crate::{Edit, Fix, Linter, Rule, Violation};

use crate::visitors::check_not_allowed_types;
use crate::visitors::is_not_valid_int_type;

use lazy_static::lazy_static;

lazy_static! {
    static ref SMALL_INT_TYPES: HashSet<Identifier> = HashSet::from([
        Identifier::new("smallint"),
        Identifier::new("int2"),
        Identifier::new("smallserial"),
        Identifier::new("serial2"),
    ]);
}

fn smallint_to_bigint(smallint_type: &str) -> &'static str {
    match smallint_type.to_lowercase().as_str() {
        "smallint" => "bigint",
        "int2" => "int8",
        "smallserial" => "bigserial",
        "serial2" => "serial8",
        _ => "bigint",
    }
}

fn create_bigint_fix(ty: &ast::Type) -> Option<Fix> {
    let type_name = ty.syntax().first_token()?;
    let i64 = smallint_to_bigint(type_name.text());
    let edit = Edit::replace(ty.syntax().text_range(), i64);
    Some(Fix::new(
        format!("Replace with a 64-bit integer type: `{i64}`"),
        vec![edit],
    ))
}

fn check_ty_for_small_int(ctx: &mut Linter, ty: Option<ast::Type>) {
    if let Some(ty) = ty {
        if is_not_valid_int_type(&ty, &SMALL_INT_TYPES) {
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
    use insta::{assert_debug_snapshot, assert_snapshot};

    use crate::{
        Rule,
        test_utils::{fix_sql, lint},
    };

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
        let errors = lint(sql, Rule::PreferBigintOverSmallint);
        assert_ne!(errors.len(), 0);
        assert_eq!(errors.len(), 4);
        assert_eq!(
            errors
                .iter()
                .filter(|x| x.code == Rule::PreferBigintOverSmallint)
                .count(),
            4
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
        let errors = lint(sql, Rule::PreferBigintOverSmallint);
        assert_eq!(errors.len(), 0);
    }
}
