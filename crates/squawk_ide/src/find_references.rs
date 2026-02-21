use crate::binder::{self, Binder};
use crate::builtins::BUILTINS_SQL;
use crate::goto_definition::{FileId, Location};
use crate::offsets::token_from_offset;
use crate::resolve;
use rowan::TextSize;
use smallvec::{SmallVec, smallvec};
use squawk_syntax::{
    SyntaxNodePtr,
    ast::{self, AstNode},
    match_ast,
};

pub fn find_references(file: &ast::SourceFile, offset: TextSize) -> Vec<Location> {
    let current_binder = binder::bind(file);

    let builtins_tree = ast::SourceFile::parse(BUILTINS_SQL).tree();
    let builtins_binder = binder::bind(&builtins_tree);

    let Some((target_file, target_defs)) = find_target_defs(
        file,
        offset,
        &current_binder,
        &builtins_tree,
        &builtins_binder,
    ) else {
        return vec![];
    };

    let (binder, root) = match target_file {
        FileId::Current => (&current_binder, file.syntax()),
        FileId::Builtins => (&builtins_binder, builtins_tree.syntax()),
    };

    let mut refs: Vec<Location> = vec![];

    if target_file == FileId::Builtins {
        for ptr in &target_defs {
            refs.push(Location {
                file: FileId::Builtins,
                range: ptr.to_node(builtins_tree.syntax()).text_range(),
            });
        }
    }

    for node in file.syntax().descendants() {
        match_ast! {
            match node {
                ast::NameRef(name_ref) => {
                    // Check if the ref matches one of the defs
                    if let Some(found_defs) = resolve::resolve_name_ref_ptrs(binder, root, &name_ref)
                        && found_defs.iter().any(|def| target_defs.contains(def))
                    {
                        refs.push(Location {
                            file: FileId::Current,
                            range: name_ref.syntax().text_range(),
                        });
                    }
                },
                ast::Name(name) => {
                    // Find refs also includes the defs so we have to check.
                    let found = SyntaxNodePtr::new(name.syntax());
                    if target_defs.contains(&found) {
                        refs.push(Location {
                            file: FileId::Current,
                            range: name.syntax().text_range(),
                        });
                    }
                },
                _ => (),
            }
        }
    }

    refs.sort_by_key(|loc| (loc.file, loc.range.start()));
    refs
}

fn find_target_defs(
    file: &ast::SourceFile,
    offset: TextSize,
    current_binder: &Binder,
    builtins_tree: &ast::SourceFile,
    builtins_binder: &Binder,
) -> Option<(FileId, SmallVec<[SyntaxNodePtr; 1]>)> {
    let token = token_from_offset(file, offset)?;
    let parent = token.parent()?;

    if let Some(name) = ast::Name::cast(parent.clone()) {
        return Some((
            FileId::Current,
            smallvec![SyntaxNodePtr::new(name.syntax())],
        ));
    }

    if let Some(name_ref) = ast::NameRef::cast(parent.clone()) {
        for file_id in [FileId::Current, FileId::Builtins] {
            let (binder, root) = match file_id {
                FileId::Current => (current_binder, file.syntax()),
                FileId::Builtins => (builtins_binder, builtins_tree.syntax()),
            };
            if let Some(ptrs) = resolve::resolve_name_ref_ptrs(binder, root, &name_ref) {
                return Some((file_id, ptrs));
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use crate::builtins::BUILTINS_SQL;
    use crate::find_references::find_references;
    use crate::goto_definition::FileId;
    use crate::test_utils::fixture;
    use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
    use insta::assert_snapshot;
    use rowan::TextRange;
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

        let mut current_refs = vec![];
        let mut builtin_refs = vec![];
        for (i, location) in references.iter().enumerate() {
            let label_index = i + 1;
            match location.file {
                FileId::Current => current_refs.push((label_index, location.range)),
                FileId::Builtins => builtin_refs.push((label_index, location.range)),
            }
        }

        let has_builtins = !builtin_refs.is_empty();

        let mut snippet = Snippet::source(&sql).fold(true);
        if has_builtins {
            snippet = snippet.path("current.sql");
        }
        snippet = snippet.annotation(
            AnnotationKind::Context
                .span(offset_usize..offset_usize + 1)
                .label("0. query"),
        );
        snippet = annotate_refs(snippet, current_refs);

        let mut groups = vec![Level::INFO.primary_title("references").element(snippet)];

        if has_builtins {
            let builtins_snippet = Snippet::source(BUILTINS_SQL).path("builtin.sql").fold(true);
            let builtins_snippet = annotate_refs(builtins_snippet, builtin_refs);
            groups.push(
                Level::INFO
                    .primary_title("references")
                    .element(builtins_snippet),
            );
        }

        let renderer = Renderer::plain().decor_style(DecorStyle::Unicode);
        renderer
            .render(&groups)
            .to_string()
            .replace("info: references", "")
    }

    fn annotate_refs<'a>(
        mut snippet: Snippet<'a, annotate_snippets::Annotation<'a>>,
        refs: Vec<(usize, TextRange)>,
    ) -> Snippet<'a, annotate_snippets::Annotation<'a>> {
        for (label_index, range) in refs {
            snippet = snippet.annotation(
                AnnotationKind::Context
                    .span(range.into())
                    .label(format!("{}. reference", label_index)),
            );
        }
        snippet
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

    #[test]
    fn builtin_function_references() {
        assert_snapshot!(find_refs("
select now$0();
select now();
"), @r"
              ╭▸ current.sql:2:8
              │
            2 │ select now();
              │        ┬─┬
              │        │ │
              │        │ 0. query
              │        1. reference
            3 │ select now();
              │        ─── 2. reference
              ╰╴

              ╭▸ builtin.sql:10798:28
              │
        10798 │ create function pg_catalog.now() returns timestamp with time zone
              ╰╴                           ─── 3. reference
        ");
    }
}
