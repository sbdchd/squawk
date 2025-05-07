use std::collections::HashSet;

use squawk_syntax::ast::AstNode;
use squawk_syntax::{ast, Parse, SourceFile};

use crate::{Linter, Rule, Violation};

use crate::visitors::check_not_allowed_types;
use crate::visitors::is_not_valid_int_type;

use lazy_static::lazy_static;

lazy_static! {
    static ref INT_TYPES: HashSet<&'static str> =
        HashSet::from(["integer", "int4", "serial", "serial4",]);
}

fn check_ty_for_big_int(ctx: &mut Linter, ty: Option<ast::Type>) {
    if let Some(ty) = ty {
        if is_not_valid_int_type(&ty, &INT_TYPES) {
            ctx.report(Violation::new(
                Rule::PreferBigintOverInt,
                "Using 32-bit integer fields can result in hitting the max `int` limit.".into(),
                ty.syntax().text_range(),
                "Use 64-bit integer values instead to prevent hitting this limit.".to_string(),
            ));
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
    use insta::assert_debug_snapshot;

    use crate::{Linter, Rule};

    #[test]
    fn err() {
        let sql = r#"
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
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferBigintOverInt]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_eq!(errors.len(), 4);
        assert_eq!(
            errors
                .iter()
                .filter(|x| x.code == Rule::PreferBigintOverInt)
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
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::PreferBigintOverInt]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }
}
