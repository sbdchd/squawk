use crate::binder::{self, Binder};
use crate::offsets::token_from_offset;
use crate::resolve;
use rowan::{TextRange, TextSize};
use smallvec::{SmallVec, smallvec};
use squawk_syntax::SyntaxNode;
use squawk_syntax::{
    SyntaxNodePtr,
    ast::{self, AstNode},
    match_ast,
};

pub fn find_references(file: &ast::SourceFile, offset: TextSize) -> Vec<TextRange> {
    let binder = binder::bind(file);
    let root = file.syntax();
    let Some(targets) = find_targets(file, root, offset, &binder) else {
        return vec![];
    };

    let mut refs = vec![];
    for node in file.syntax().descendants() {
        match_ast! {
            match node {
                ast::NameRef(name_ref) => {
                    if let Some(found_refs) = resolve::resolve_name_ref_ptrs(&binder, root, &name_ref)
                        && found_refs.iter().any(|ptr| targets.contains(ptr))
                    {
                        refs.push(name_ref.syntax().text_range());
                    }
                },
                ast::Name(name) => {
                    let found = SyntaxNodePtr::new(name.syntax());
                    if targets.contains(&found) {
                        refs.push(name.syntax().text_range());
                    }
                },
                _ => (),
            }
        }
    }

    refs.sort_by_key(|range| range.start());
    refs
}

fn find_targets(
    file: &ast::SourceFile,
    root: &SyntaxNode,
    offset: TextSize,
    binder: &Binder,
) -> Option<SmallVec<[SyntaxNodePtr; 1]>> {
    let token = token_from_offset(file, offset)?;
    let parent = token.parent()?;

    if let Some(name) = ast::Name::cast(parent.clone()) {
        return Some(smallvec![SyntaxNodePtr::new(name.syntax())]);
    }

    if let Some(name_ref) = ast::NameRef::cast(parent.clone()) {
        return resolve::resolve_name_ref_ptrs(binder, root, &name_ref);
    }

    None
}

#[cfg(test)]
mod test {
    use crate::find_references::find_references;
    use crate::test_utils::fixture;
    use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
    use insta::assert_snapshot;
    use squawk_syntax::ast;

    #[track_caller]
    fn find_refs(sql: &str) -> String {
        let (mut offset, sql) = fixture(sql);
        offset = offset.checked_sub(1.into()).unwrap_or_default();
        let parse = ast::SourceFile::parse(&sql);
        assert_eq!(parse.errors(), vec![]);
        let file: ast::SourceFile = parse.tree();

        let references = find_references(&file, offset);

        let offset_usize: usize = offset.into();

        let labels: Vec<String> = (1..=references.len())
            .map(|i| format!("{}. reference", i))
            .collect();

        let mut snippet = Snippet::source(&sql).fold(true).annotation(
            AnnotationKind::Context
                .span(offset_usize..offset_usize + 1)
                .label("0. query"),
        );

        for (i, range) in references.iter().enumerate() {
            snippet = snippet.annotation(
                AnnotationKind::Context
                    .span((*range).into())
                    .label(&labels[i]),
            );
        }

        let group = Level::INFO.primary_title("references").element(snippet);
        let renderer = Renderer::plain().decor_style(DecorStyle::Unicode);
        renderer
            .render(&[group])
            .to_string()
            .replace("info: references", "")
    }

    #[test]
    fn simple_table_reference() {
        assert_snapshot!(find_refs("
create table t();
drop table t$0;
"), @r"
          ╭▸ 
        2 │ create table t();
          │              ─ 1. reference
        3 │ drop table t;
          │            ┬
          │            │
          │            0. query
          ╰╴           2. reference
        ");
    }

    #[test]
    fn multiple_references() {
        assert_snapshot!(find_refs("
create table users();
drop table users$0;
table users;
"), @r"
          ╭▸ 
        2 │ create table users();
          │              ───── 1. reference
        3 │ drop table users;
          │            ┬───┬
          │            │   │
          │            │   0. query
          │            2. reference
        4 │ table users;
          ╰╴      ───── 3. reference
        ");
    }

    #[test]
    fn join_using_column() {
        assert_snapshot!(find_refs("
create table t(id int);
create table u(id int);
select * from t join u using (id$0);
"), @r"
          ╭▸ 
        2 │ create table t(id int);
          │                ── 1. reference
        3 │ create table u(id int);
          │                ── 2. reference
        4 │ select * from t join u using (id);
          │                               ┬┬
          │                               ││
          │                               │0. query
          ╰╴                              3. reference
        ");
    }

    #[test]
    fn find_from_definition() {
        assert_snapshot!(find_refs("
create table t$0();
drop table t;
"), @r"
          ╭▸ 
        2 │ create table t();
          │              ┬
          │              │
          │              0. query
          │              1. reference
        3 │ drop table t;
          ╰╴           ─ 2. reference
        ");
    }

    #[test]
    fn with_schema_qualified() {
        assert_snapshot!(find_refs("
create table public.users();
drop table public.users$0;
table users;
"), @r"
          ╭▸ 
        2 │ create table public.users();
          │                     ───── 1. reference
        3 │ drop table public.users;
          │                   ┬───┬
          │                   │   │
          │                   │   0. query
          │                   2. reference
        4 │ table users;
          ╰╴      ───── 3. reference
        ");
    }

    #[test]
    fn temp_table_do_not_shadows_public() {
        assert_snapshot!(find_refs("
create table t();
create temp table t$0();
drop table t;
"), @r"
          ╭▸ 
        3 │ create temp table t();
          │                   ┬
          │                   │
          │                   0. query
          ╰╴                  1. reference
        ");
    }

    #[test]
    fn different_schema_no_match() {
        assert_snapshot!(find_refs("
create table foo.t();
create table bar.t$0();
"), @r"
          ╭▸ 
        3 │ create table bar.t();
          │                  ┬
          │                  │
          │                  0. query
          ╰╴                 1. reference
        ");
    }

    #[test]
    fn with_search_path() {
        assert_snapshot!(find_refs("
set search_path to myschema;
create table myschema.users$0();
drop table users;
"), @r"
          ╭▸ 
        3 │ create table myschema.users();
          │                       ┬───┬
          │                       │   │
          │                       │   0. query
          │                       1. reference
        4 │ drop table users;
          ╰╴           ───── 2. reference
        ");
    }

    #[test]
    fn temp_table_with_pg_temp_schema() {
        assert_snapshot!(find_refs("
create temp table t();
drop table pg_temp.t$0;
"), @r"
          ╭▸ 
        2 │ create temp table t();
          │                   ─ 1. reference
        3 │ drop table pg_temp.t;
          │                    ┬
          │                    │
          │                    0. query
          ╰╴                   2. reference
        ");
    }

    #[test]
    fn case_insensitive() {
        assert_snapshot!(find_refs("
create table Users();
drop table USERS$0;
table users;
"), @r"
          ╭▸ 
        2 │ create table Users();
          │              ───── 1. reference
        3 │ drop table USERS;
          │            ┬───┬
          │            │   │
          │            │   0. query
          │            2. reference
        4 │ table users;
          ╰╴      ───── 3. reference
        ");
    }
    #[test]
    fn case_insensitive_part_2() {
        // we should see refs for `drop table` and `table`
        assert_snapshot!(find_refs(r#"
create table actors();
create table "Actors"();
drop table ACTORS$0;
table actors;
"#), @r#"
          ╭▸ 
        2 │ create table actors();
          │              ────── 1. reference
        3 │ create table "Actors"();
        4 │ drop table ACTORS;
          │            ┬────┬
          │            │    │
          │            │    0. query
          │            2. reference
        5 │ table actors;
          ╰╴      ────── 3. reference
        "#);
    }

    #[test]
    fn case_insensitive_with_schema() {
        assert_snapshot!(find_refs("
create table Public.Users();
drop table PUBLIC.USERS$0;
table public.users;
"), @r"
          ╭▸ 
        2 │ create table Public.Users();
          │                     ───── 1. reference
        3 │ drop table PUBLIC.USERS;
          │                   ┬───┬
          │                   │   │
          │                   │   0. query
          │                   2. reference
        4 │ table public.users;
          ╰╴             ───── 3. reference
        ");
    }

    #[test]
    fn no_partial_match() {
        assert_snapshot!(find_refs("
create table t$0();
create table temp_t();
"), @r"
          ╭▸ 
        2 │ create table t();
          │              ┬
          │              │
          │              0. query
          ╰╴             1. reference
        ");
    }

    #[test]
    fn identifier_boundaries() {
        assert_snapshot!(find_refs("
create table foo$0();
drop table foo;
drop table foo1;
drop table barfoo;
drop table foo_bar;
"), @r"
          ╭▸ 
        2 │ create table foo();
          │              ┬─┬
          │              │ │
          │              │ 0. query
          │              1. reference
        3 │ drop table foo;
          ╰╴           ─── 2. reference
        ");
    }
}
