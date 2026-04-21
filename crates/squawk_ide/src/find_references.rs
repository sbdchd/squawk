use crate::builtins::builtins_file;
use crate::db::{File, parse};
use crate::location::{Location, LocationKind};
use crate::offsets::token_from_offset;
use crate::resolve;
use rowan::TextSize;
use salsa::Database as Db;
use smallvec::{SmallVec, smallvec};
use squawk_syntax::{
    SyntaxNodePtr,
    ast::{self, AstNode},
};

#[salsa::tracked]
pub fn find_references(db: &dyn Db, file: File, offset: TextSize) -> Vec<Location> {
    let Some((target_file, target_defs, target_kind)) = find_target_defs(db, offset, file) else {
        return vec![];
    };

    let mut refs: Vec<Location> = vec![];

    for ptr in &target_defs {
        refs.push(Location {
            file: target_file,
            range: ptr.text_range(),
            kind: target_kind,
        });
    }

    for node in parse(db, file).tree().syntax().descendants() {
        if let Some(name_ref) = ast::NameRef::cast(node) {
            // Check if the ref matches one of the defs
            if let Some(found_defs) = resolve::resolve_name_ref_ptrs(db, target_file, &name_ref)
                && found_defs.iter().any(|def| target_defs.contains(def))
            {
                refs.push(Location {
                    file,
                    range: name_ref.syntax().text_range(),
                    kind: target_kind,
                });
            }
        }
    }

    refs.sort_by_key(|loc| (loc.file != file, loc.range.start()));
    refs
}

fn find_target_defs(
    db: &dyn Db,
    offset: TextSize,
    current_file: File,
) -> Option<(File, SmallVec<[SyntaxNodePtr; 1]>, LocationKind)> {
    let token = token_from_offset(db, current_file, offset)?;
    let parent = token.parent()?;

    if let Some(name) = ast::Name::cast(parent.clone())
        && let Some(kind) = LocationKind::from_node(name.syntax())
    {
        return Some((
            current_file,
            smallvec![SyntaxNodePtr::new(name.syntax())],
            kind,
        ));
    }

    if let Some(name_ref) = ast::NameRef::cast(parent.clone()) {
        for target_file in [current_file, builtins_file(db)] {
            if let Some((ptrs, kind)) = resolve::resolve_name_ref(db, target_file, &name_ref) {
                return Some((target_file, ptrs, kind));
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use crate::builtins::builtins_file;
    use crate::db::{Database, File};
    use crate::find_references::find_references;
    use crate::test_utils::fixture;
    use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
    use insta::assert_snapshot;
    use rowan::TextRange;
    use rustc_hash::FxHashMap;

    #[track_caller]
    fn find_refs(sql: &str) -> String {
        let (mut offset, sql) = fixture(sql);
        offset = offset.checked_sub(1.into()).unwrap_or_default();
        let db = Database::default();
        let current_file = File::new(&db, sql.clone().into());
        assert_eq!(crate::db::parse(&db, current_file).errors(), vec![]);

        let references = find_references(&db, current_file, offset);
        let offset_usize: usize = offset.into();

        let mut file_paths = FxHashMap::default();
        file_paths.insert(current_file, "current.sql");
        file_paths.insert(builtins_file(&db), "builtins.sql");

        let mut refs_by_file: FxHashMap<File, Vec<(usize, TextRange)>> = FxHashMap::default();
        for (i, location) in references.iter().enumerate() {
            refs_by_file
                .entry(location.file)
                .or_default()
                .push((i + 1, location.range));
        }

        let multi_file = refs_by_file.len() > 1 || !refs_by_file.contains_key(&current_file);

        let mut snippet = Snippet::source(&sql).fold(true);
        if multi_file {
            snippet = snippet.path(*file_paths.get(&current_file).unwrap());
        }
        snippet = snippet.annotation(
            AnnotationKind::Context
                .span(offset_usize..offset_usize + 1)
                .label("0. query"),
        );
        if let Some(current_refs) = refs_by_file.remove(&current_file) {
            snippet = annotate_refs(snippet, current_refs);
        }

        let mut groups = vec![Level::INFO.primary_title("references").element(snippet)];

        for (ref_file, refs) in refs_by_file {
            let path = file_paths.get(&ref_file).unwrap();
            let other_snippet = Snippet::source(ref_file.content(&db).as_ref())
                .path(*path)
                .fold(true);
            let other_snippet = annotate_refs(other_snippet, refs);
            groups.push(
                Level::INFO
                    .primary_title("references")
                    .element(other_snippet),
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
"), @"
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

              ╭▸ builtins.sql:11089:28
              │
        11089 │ create function pg_catalog.now() returns timestamp with time zone
              ╰╴                           ─── 3. reference
        ");
    }
}
