use std::collections::HashSet;

use squawk_syntax::ast::AstNode;
use squawk_syntax::{Parse, SourceFile, ast};

use crate::identifier::Identifier;
use crate::{Linter, Rule, Violation};

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

fn check_ty_for_small_int(ctx: &mut Linter, ty: Option<ast::Type>) {
    if let Some(ty) = ty {
        if is_not_valid_int_type(&ty, &SMALL_INT_TYPES) {
            ctx.report(Violation::new(
                Rule::PreferBigintOverSmallint,
                "Using 16-bit integer fields can result in hitting the max `int` limit.".into(),
                ty.syntax().text_range(),
                "Use 64-bit integer values instead to prevent hitting this limit.".to_string(),
            ));
        };
    }
}

pub(crate) fn prefer_bigint_over_smallint(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    check_not_allowed_types(ctx, &file, check_ty_for_small_int);
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::Rule;
    use crate::test_utils::lint;

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
