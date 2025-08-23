use std::collections::HashSet;

use rowan::TextRange;
use squawk_syntax::{
    Parse, SourceFile, TokenText,
    ast::{self, AstNode},
};

use crate::visitors::check_not_allowed_types;
use crate::{Edit, Fix, Linter, Rule, Violation, identifier::Identifier};

use lazy_static::lazy_static;

lazy_static! {
    static ref CHAR_TYPES: HashSet<Identifier> = HashSet::from([
        Identifier::new("char"),
        Identifier::new("character"),
        Identifier::new("bpchar"),
    ]);
}

fn is_char_type(x: TokenText<'_>) -> bool {
    CHAR_TYPES.contains(&Identifier::new(&x.to_string()))
}

fn create_fix(range: TextRange, args: Option<ast::ArgList>) -> Fix {
    if let Some(args_list) = args {
        let end = args_list.syntax().text_range().start();
        let edit = Edit::replace(TextRange::new(range.start(), end), "varchar");
        Fix::new(format!("Replace with `varchar`"), vec![edit])
    } else {
        let edit = Edit::replace(range, "text");
        Fix::new(format!("Replace with `text`"), vec![edit])
    }
}

fn check_path_type(ctx: &mut Linter, path_type: ast::PathType) {
    if let Some(name_ref) = path_type
        .path()
        .and_then(|x| x.segment())
        .and_then(|x| x.name_ref())
    {
        if is_char_type(name_ref.text()) {
            let fix = create_fix(name_ref.syntax().text_range(), path_type.arg_list());
            ctx.report(Violation::for_node(
                Rule::BanCharField,
                "Using `character` is likely a mistake and should almost always be replaced by `text` or `varchar`.".into(),
                path_type.syntax(),
            ).fix(Some(fix)));
        }
    }
}

fn check_char_type(ctx: &mut Linter, char_type: ast::CharType) {
    if is_char_type(char_type.text()) {
        let fix = create_fix(char_type.syntax().text_range(), char_type.arg_list());
        ctx.report(Violation::for_node(
            Rule::BanCharField,
            "Using `character` is likely a mistake and should almost always be replaced by `text` or `varchar`.".into(),
            char_type.syntax(),
        ).fix(Some(fix)));
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
    use insta::{assert_debug_snapshot, assert_snapshot};

    use crate::{
        Rule,
        test_utils::{fix_sql, lint},
    };

    fn fix(sql: &str) -> String {
        fix_sql(sql, Rule::BanCharField)
    }

    #[test]
    fn fix_char_without_length() {
        assert_snapshot!(fix("CREATE TABLE t (c char);"), @"CREATE TABLE t (c text);");
        assert_snapshot!(fix("CREATE TABLE t (c character);"), @"CREATE TABLE t (c text);");
        assert_snapshot!(fix("CREATE TABLE t (c bpchar);"), @"CREATE TABLE t (c text);");
    }

    #[test]
    fn fix_char_with_length() {
        assert_snapshot!(fix("CREATE TABLE t (c char(100));"), @"CREATE TABLE t (c varchar(100));");
        assert_snapshot!(fix("CREATE TABLE t (c character(255));"), @"CREATE TABLE t (c varchar(255));");
        assert_snapshot!(fix("CREATE TABLE t (c bpchar(50));"), @"CREATE TABLE t (c varchar(50));");

        assert_snapshot!(fix(r#"CREATE TABLE t (c "char"(100));"#), @"CREATE TABLE t (c varchar(100));");
        assert_snapshot!(fix(r#"CREATE TABLE t (c "character"(255));"#), @"CREATE TABLE t (c varchar(255));");
        assert_snapshot!(fix(r#"CREATE TABLE t (c "bpchar"(50));"#), @"CREATE TABLE t (c varchar(50));");
    }

    #[test]
    fn fix_mixed_case() {
        assert_snapshot!(fix("CREATE TABLE t (c CHAR);"), @"CREATE TABLE t (c text);");
        assert_snapshot!(fix("CREATE TABLE t (c CHARACTER(100));"), @"CREATE TABLE t (c varchar(100));");
        assert_snapshot!(fix("CREATE TABLE t (c Char(50));"), @"CREATE TABLE t (c varchar(50));");
    }

    #[test]
    fn fix_array_types() {
        assert_snapshot!(fix("CREATE TABLE t (c char[]);"), @"CREATE TABLE t (c text[]);");
        assert_snapshot!(fix("CREATE TABLE t (c character(100)[]);"), @"CREATE TABLE t (c varchar(100)[]);");
    }

    #[test]
    fn fix_alter_table() {
        assert_snapshot!(fix("ALTER TABLE t ADD COLUMN c char;"), @"ALTER TABLE t ADD COLUMN c text;");
        assert_snapshot!(fix("ALTER TABLE t ADD COLUMN c character(100);"), @"ALTER TABLE t ADD COLUMN c varchar(100);");
    }

    #[test]
    fn fix_multiple_columns() {
        assert_snapshot!(fix("CREATE TABLE t (a char, b character(100), c bpchar(50));"), @"CREATE TABLE t (a text, b varchar(100), c varchar(50));");
    }

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
        let errors = lint(sql, Rule::BanCharField);
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
        let errors = lint(sql, Rule::BanCharField);
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
        let errors = lint(sql, Rule::BanCharField);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn case_insensitive() {
        let sql = r#"
create table t (
  a Char
);
        "#;
        let errors = lint(sql, Rule::BanCharField);
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
        let errors = lint(sql, Rule::BanCharField);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }

    #[test]
    fn alter_table_err() {
        let sql = r#"
alter table t add column c char;
        "#;
        let errors = lint(sql, Rule::BanCharField);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors);
    }
}
