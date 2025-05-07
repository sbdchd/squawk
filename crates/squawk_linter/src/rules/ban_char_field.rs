use squawk_syntax::{
    ast::{self, AstNode},
    Parse, SourceFile, TokenText,
};

use crate::visitors::check_not_allowed_types;
use crate::{Linter, Rule, Violation};

fn is_char_type(x: TokenText<'_>) -> bool {
    if x == "char" || x == "character" || x == "bpchar" {
        return true;
    }
    false
}

fn check_path_type(ctx: &mut Linter, path_type: ast::PathType) {
    if let Some(name_ref) = path_type
        .path()
        .and_then(|x| x.segment())
        .and_then(|x| x.name_ref())
    {
        if is_char_type(name_ref.text()) {
            ctx.report(Violation::new(
                Rule::BanCharField,
                "Using `character` is likely a mistake and should almost always be replaced by `text` or `varchar`.".into(),
                path_type.syntax().text_range(),
                None,
            ));
        }
    }
}

fn check_char_type(ctx: &mut Linter, char_type: ast::CharType) {
    if is_char_type(char_type.text()) {
        ctx.report(Violation::new(
            Rule::BanCharField,
            "Using `character` is likey a mistake and should almost always be replaced by `text` or `varchar`.".into(),
            char_type.syntax().text_range(),
            None,
        ));
    }
}

fn check_ty(ctx: &mut Linter, ty: Option<ast::Type>) {
    match ty {
        Some(ast::Type::ArrayType(array_type)) => match array_type.ty() {
            Some(ast::Type::CharType(char_type)) => {
                check_char_type(ctx, char_type);
            }
            Some(ast::Type::PathType(path_type)) => {
                check_path_type(ctx, path_type);
            }
            _ => (),
        },
        Some(ast::Type::PathType(path_type)) => {
            check_path_type(ctx, path_type);
        }
        Some(ast::Type::CharType(char_type)) => {
            check_char_type(ctx, char_type);
        }
        _ => (),
    }
}

pub(crate) fn ban_char_field(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();
    check_not_allowed_types(ctx, &file, check_ty);
}

#[cfg(test)]
mod test {
    use insta::assert_debug_snapshot;

    use crate::{Linter, Rule};

    #[test]
    fn creating_table_with_char_errors() {
        let sql = r#"
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "alpha" char(100) NOT NULL,
    "beta" character(100) NOT NULL,
    "charlie" char NOT NULL,
    "delta" character NOT NULL
);
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanCharField]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn creating_table_with_var_char_and_text_okay() {
        let sql = r#"
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "alpha" varchar(100) NOT NULL,
    "beta" text NOT NULL
);
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanCharField]);
        let errors = linter.lint(file, sql);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn all_the_types() {
        let sql = r#"
create table t (
    a serial not null primary key,
    b char(100),
    c character(100),
    d char,
    e character,
    f double precision,
    g time with time zone,
    h interval,
    j int[5][10],
    k bar(10),
    l bit varying,
    m int array[],
    o pg_catalog.char,
    p char[]
);
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanCharField]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn array_char_type_err() {
        let sql = r#"
create table t (
  a char[]
);
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanCharField]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn alter_table_err() {
        let sql = r#"
alter table t add column c char;
        "#;
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanCharField]);
        let errors = linter.lint(file, sql);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }
}
