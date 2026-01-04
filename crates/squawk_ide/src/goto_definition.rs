use crate::binder;
use crate::offsets::token_from_offset;
use crate::resolve;
use rowan::{TextRange, TextSize};
use smallvec::{SmallVec, smallvec};
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

pub fn goto_definition(file: ast::SourceFile, offset: TextSize) -> SmallVec<[TextRange; 1]> {
    let Some(token) = token_from_offset(&file, offset) else {
        return smallvec![];
    };
    let Some(parent) = token.parent() else {
        return smallvec![];
    };

    // goto def on case exprs
    if (token.kind() == SyntaxKind::WHEN_KW && parent.kind() == SyntaxKind::WHEN_CLAUSE)
        || (token.kind() == SyntaxKind::ELSE_KW && parent.kind() == SyntaxKind::ELSE_CLAUSE)
        || (token.kind() == SyntaxKind::END_KW && parent.kind() == SyntaxKind::CASE_EXPR)
    {
        for parent in token.parent_ancestors() {
            if let Some(case_expr) = ast::CaseExpr::cast(parent)
                && let Some(case_token) = case_expr.case_token()
            {
                return smallvec![case_token.text_range()];
            }
        }
    }

    // goto def on COMMIT -> BEGIN/START TRANSACTION
    if ast::Commit::can_cast(parent.kind()) {
        if let Some(begin_range) = find_preceding_begin(&file, token.text_range().start()) {
            return smallvec![begin_range];
        }
    }

    // goto def on ROLLBACK -> BEGIN/START TRANSACTION
    if ast::Rollback::can_cast(parent.kind()) {
        if let Some(begin_range) = find_preceding_begin(&file, token.text_range().start()) {
            return smallvec![begin_range];
        }
    }

    // goto def on BEGIN/START TRANSACTION -> COMMIT or ROLLBACK
    if ast::Begin::can_cast(parent.kind()) {
        if let Some(end_range) = find_following_commit_or_rollback(&file, token.text_range().end())
        {
            return smallvec![end_range];
        }
    }

    if let Some(name) = ast::Name::cast(parent.clone()) {
        return smallvec![name.syntax().text_range()];
    }

    if let Some(name_ref) = ast::NameRef::cast(parent.clone()) {
        let binder_output = binder::bind(&file);
        let root = file.syntax();
        if let Some(ptrs) = resolve::resolve_name_ref(&binder_output, root, &name_ref) {
            return ptrs
                .iter()
                .map(|ptr| ptr.to_node(file.syntax()).text_range())
                .collect();
        }
    }

    smallvec![]
}

fn find_preceding_begin(file: &ast::SourceFile, before: TextSize) -> Option<TextRange> {
    let mut last_begin: Option<TextRange> = None;
    for stmt in file.stmts() {
        if let ast::Stmt::Begin(begin) = stmt {
            let range = begin.syntax().text_range();
            if range.end() <= before {
                last_begin = Some(range);
            }
        }
    }
    last_begin
}

fn find_following_commit_or_rollback(file: &ast::SourceFile, after: TextSize) -> Option<TextRange> {
    for stmt in file.stmts() {
        let range = match &stmt {
            ast::Stmt::Commit(commit) => commit.syntax().text_range(),
            ast::Stmt::Rollback(rollback) => rollback.syntax().text_range(),
            _ => continue,
        };
        if range.start() >= after {
            return Some(range);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use crate::goto_definition::goto_definition;
    use crate::test_utils::fixture;
    use annotate_snippets::{AnnotationKind, Level, Renderer, Snippet, renderer::DecorStyle};
    use insta::assert_snapshot;
    use log::info;
    use squawk_syntax::ast;

    #[track_caller]
    fn goto(sql: &str) -> String {
        goto_(sql).expect("should always find a definition")
    }

    #[track_caller]
    fn goto_(sql: &str) -> Option<String> {
        info!("starting");
        let (mut offset, sql) = fixture(sql);
        // For go to def we want the previous character since we usually put the
        // marker after the item we're trying to go to def on.
        offset = offset.checked_sub(1.into()).unwrap_or_default();
        let parse = ast::SourceFile::parse(&sql);
        assert_eq!(parse.errors(), vec![]);
        let file: ast::SourceFile = parse.tree();
        let results = goto_definition(file, offset);
        if !results.is_empty() {
            let offset: usize = offset.into();
            let mut snippet = Snippet::source(&sql).fold(true);

            for (i, result) in results.iter().enumerate() {
                snippet = snippet.annotation(
                    AnnotationKind::Context
                        .span((*result).into())
                        .label(format!("{}. destination", i + 2)),
                );
            }

            snippet = snippet.annotation(
                AnnotationKind::Context
                    .span(offset..offset + 1)
                    .label("1. source"),
            );

            let group = Level::INFO.primary_title("definition").element(snippet);
            let renderer = Renderer::plain().decor_style(DecorStyle::Unicode);
            return Some(
                renderer
                    .render(&[group])
                    .to_string()
                    // hacky cleanup to make the text shorter
                    .replace("info: definition", ""),
            );
        }
        None
    }

    fn goto_not_found(sql: &str) {
        assert!(goto_(sql).is_none(), "Should not find a definition");
    }

    #[test]
    fn goto_case_when() {
        assert_snapshot!(goto("
select case when$0 x > 1 then 1 else 2 end;
"), @r"
          ╭▸ 
        2 │ select case when x > 1 then 1 else 2 end;
          │        ┬───    ─ 1. source
          │        │
          ╰╴       2. destination
        ");
    }

    #[test]
    fn goto_case_else() {
        assert_snapshot!(goto("
select case when x > 1 then 1 else$0 2 end;
"), @r"
          ╭▸ 
        2 │ select case when x > 1 then 1 else 2 end;
          ╰╴       ──── 2. destination       ─ 1. source
        ");
    }

    #[test]
    fn goto_case_end() {
        assert_snapshot!(goto("
select case when x > 1 then 1 else 2 end$0;
"), @r"
          ╭▸ 
        2 │ select case when x > 1 then 1 else 2 end;
          ╰╴       ──── 2. destination             ─ 1. source
        ");
    }

    #[test]
    fn goto_case_end_trailing_semi() {
        assert_snapshot!(goto("
select case when x > 1 then 1 else 2 end;$0
"), @r"
          ╭▸ 
        2 │ select case when x > 1 then 1 else 2 end;
          ╰╴       ──── 2. destination              ─ 1. source
        ");
    }

    #[test]
    fn goto_case_then_not_found() {
        goto_not_found(
            "
select case when x > 1 then$0 1 else 2 end;
",
        )
    }

    #[test]
    fn rollback_to_begin() {
        assert_snapshot!(goto(
            "
begin;
select 1;
rollback$0;
",
        ), @r"
          ╭▸ 
        2 │ begin;
          │ ───── 2. destination
        3 │ select 1;
        4 │ rollback;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_table() {
        assert_snapshot!(goto("
create table t();
drop table t$0;
"), @r"
          ╭▸ 
        2 │ create table t();
          │              ─ 2. destination
        3 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_definition_prefers_previous_token() {
        assert_snapshot!(goto("
create table t(a int);
select t.$0a from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int);
          │              ─ 2. destination
        3 │ select t.a from t;
          ╰╴        ─ 1. source
        ");

        assert_snapshot!(goto("
create type ty as (a int, b int);
with t as (select '(1,2)'::ty c)
select (c)$0.a from t;
"), @r"
          ╭▸ 
        3 │ with t as (select '(1,2)'::ty c)
          │                               ─ 2. destination
        4 │ select (c).a from t;
          ╰╴         ─ 1. source
        ");
        assert_snapshot!(goto("
create function f() returns int as 'select 1' language sql;
select f($0);
"), @r"
          ╭▸ 
        2 │ create function f() returns int as 'select 1' language sql;
          │                 ─ 2. destination
        3 │ select f();
          ╰╴        ─ 1. source
        ");

        assert_snapshot!(goto("
with t as (select array[1,2,3]::int[] c)
select c[$01] from t;
"), @r"
          ╭▸ 
        2 │ with t as (select array[1,2,3]::int[] c)
          │                                       ─ 2. destination
        3 │ select c[1] from t;
          ╰╴        ─ 1. source
        ");

        assert_snapshot!(goto("
with t as (select array[1,2,3]::int[] c, 1 b)
select c[b]$0 from t;
"), @r"
          ╭▸ 
        2 │ with t as (select array[1,2,3]::int[] c, 1 b)
          │                                            ─ 2. destination
        3 │ select c[b] from t;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_with_table_star() {
        assert_snapshot!(goto("
with t as (select 1 a)
select t$0.* from t;
"), @r"
          ╭▸ 
        2 │ with t as (select 1 a)
          │      ─ 2. destination
        3 │ select t.* from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_sequence() {
        assert_snapshot!(goto("
create sequence s;
drop sequence s$0;
"), @r"
          ╭▸ 
        2 │ create sequence s;
          │                 ─ 2. destination
        3 │ drop sequence s;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_sequence_owned_by() {
        assert_snapshot!(goto("
create table t(c serial);
create sequence s
  owned by t.c$0;
"), @r"
          ╭▸ 
        2 │ create table t(c serial);
          │                ─ 2. destination
        3 │ create sequence s
        4 │   owned by t.c;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_tablespace() {
        assert_snapshot!(goto("
create tablespace ts location '/tmp/ts';
drop tablespace ts$0;
"), @r"
          ╭▸ 
        2 │ create tablespace ts location '/tmp/ts';
          │                   ── 2. destination
        3 │ drop tablespace ts;
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_tablespace() {
        assert_snapshot!(goto("
create tablespace bar location '/tmp/ts';
create table t (a int) tablespace b$0ar;
"), @r"
          ╭▸ 
        2 │ create tablespace bar location '/tmp/ts';
          │                   ─── 2. destination
        3 │ create table t (a int) tablespace bar;
          ╰╴                                  ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_database() {
        assert_snapshot!(goto("
create database mydb;
drop database my$0db;
"), @r"
          ╭▸ 
        2 │ create database mydb;
          │                 ──── 2. destination
        3 │ drop database mydb;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_database_defined_after() {
        assert_snapshot!(goto("
drop database my$0db;
create database mydb;
"), @r"
          ╭▸ 
        2 │ drop database mydb;
          │                ─ 1. source
        3 │ create database mydb;
          ╰╴                ──── 2. destination
        ");
    }

    #[test]
    fn goto_database_definition_returns_self() {
        assert_snapshot!(goto("
create database my$0db;
"), @r"
          ╭▸ 
        2 │ create database mydb;
          │                 ┬┬──
          │                 ││
          │                 │1. source
          ╰╴                2. destination
        ");
    }

    #[test]
    fn goto_drop_sequence_with_schema() {
        assert_snapshot!(goto("
create sequence foo.s;
drop sequence foo.s$0;
"), @r"
          ╭▸ 
        2 │ create sequence foo.s;
          │                     ─ 2. destination
        3 │ drop sequence foo.s;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_table_with_schema() {
        assert_snapshot!(goto("
create table public.t();
drop table t$0;
"), @r"
          ╭▸ 
        2 │ create table public.t();
          │                     ─ 2. destination
        3 │ drop table t;
          ╰╴           ─ 1. source
        ");

        assert_snapshot!(goto("
create table foo.t();
drop table foo.t$0;
"), @r"
          ╭▸ 
        2 │ create table foo.t();
          │                  ─ 2. destination
        3 │ drop table foo.t;
          ╰╴               ─ 1. source
        ");

        goto_not_found(
            "
-- defaults to public schema
create table t();
drop table foo.t$0;
",
        );
    }

    #[test]
    fn goto_drop_temp_table() {
        assert_snapshot!(goto("
create temp table t();
drop table t$0;
"), @r"
          ╭▸ 
        2 │ create temp table t();
          │                   ─ 2. destination
        3 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_temporary_table() {
        assert_snapshot!(goto("
create temporary table t();
drop table t$0;
"), @r"
          ╭▸ 
        2 │ create temporary table t();
          │                        ─ 2. destination
        3 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_temp_table_with_pg_temp_schema() {
        assert_snapshot!(goto("
create temp table t();
drop table pg_temp.t$0;
"), @r"
          ╭▸ 
        2 │ create temp table t();
          │                   ─ 2. destination
        3 │ drop table pg_temp.t;
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_table_definition_returns_self() {
        assert_snapshot!(goto("
create table t$0(x bigint, y bigint);
"), @r"
          ╭▸ 
        2 │ create table t(x bigint, y bigint);
          │              ┬
          │              │
          │              2. destination
          ╰╴             1. source
        ");
    }

    #[test]
    fn goto_foreign_table_column() {
        assert_snapshot!(goto("
create foreign table ft(a int)
  server s;

select a$0 from ft;
"), @r"
          ╭▸ 
        2 │ create foreign table ft(a int)
          │                         ─ 2. destination
          ‡
        5 │ select a from ft;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_foreign_table_definition() {
        assert_snapshot!(goto("
create foreign table ft(a int)
  server s;

select a from ft$0;
"), @r"
          ╭▸ 
        2 │ create foreign table ft(a int)
          │                      ── 2. destination
          ‡
        5 │ select a from ft;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_foreign_key_references_table() {
        assert_snapshot!(goto("
create table foo(id int);
create table bar(
  id int,
  foo_id int,
  foreign key (foo_id) references foo$0(id)
);
"), @r"
          ╭▸ 
        2 │ create table foo(id int);
          │              ─── 2. destination
          ‡
        6 │   foreign key (foo_id) references foo(id)
          ╰╴                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_foreign_key_on_delete_set_null_column() {
        assert_snapshot!(goto("
create table users (
  user_id integer not null,
  primary key (user_id)
);

create table posts (
  post_id integer not null,
  author_id integer,
  primary key (post_id),
  foreign key (author_id) references users on delete set null (author_id$0)
);
"), @r"
           ╭▸ 
         9 │   author_id integer,
           │   ───────── 2. destination
        10 │   primary key (post_id),
        11 │   foreign key (author_id) references users on delete set null (author_id)
           ╰╴                                                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_references_constraint_table() {
        assert_snapshot!(goto("
create table t (
  id serial primary key
);

create table u (
  id serial primary key,
  t_id int references t$0
);
"), @r"
          ╭▸ 
        2 │ create table t (
          │              ─ 2. destination
          ‡
        8 │   t_id int references t
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_references_constraint_column() {
        assert_snapshot!(goto("
create table t (
  id serial primary key
);

create table u (
  id serial primary key,
  t_id int references t(id$0)
);
"), @r"
          ╭▸ 
        3 │   id serial primary key
          │   ── 2. destination
          ‡
        8 │   t_id int references t(id)
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_foreign_key_references_column() {
        assert_snapshot!(goto("
create table foo(id int);
create table bar(
  id int,
  foo_id int,
  foreign key (foo_id) references foo(id$0)
);
"), @r"
          ╭▸ 
        2 │ create table foo(id int);
          │                  ── 2. destination
          ‡
        6 │   foreign key (foo_id) references foo(id)
          ╰╴                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_foreign_key_local_column() {
        assert_snapshot!(goto("
create table bar(
  id int,
  foo_id int,
  foreign key (foo_id$0) references foo(id)
);
"), @r"
          ╭▸ 
        4 │   foo_id int,
          │   ────── 2. destination
        5 │   foreign key (foo_id) references foo(id)
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_check_constraint_column() {
        assert_snapshot!(goto("
create table t (
  b int check (b > 10),
  c int check (c$0 > 10) no inherit
);
"), @r"
          ╭▸ 
        4 │   c int check (c > 10) no inherit
          │   ┬            ─ 1. source
          │   │
          ╰╴  2. destination
        ");
    }

    #[test]
    fn goto_generated_column() {
        assert_snapshot!(goto("
create table t (
  a int,
  b int generated always as (
    a$0 * 2
  ) stored
);
"), @r"
          ╭▸ 
        3 │   a int,
          │   ─ 2. destination
        4 │   b int generated always as (
        5 │     a * 2
          ╰╴    ─ 1. source
        ");
    }

    #[test]
    fn goto_table_check_constraint_column() {
        assert_snapshot!(goto("
create table t (
  a int,
  b text,
  check (a$0 > b)
);
"), @r"
          ╭▸ 
        3 │   a int,
          │   ─ 2. destination
        4 │   b text,
        5 │   check (a > b)
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_table_unique_constraint_column() {
        assert_snapshot!(goto("
create table t (
  a int,
  b text,
  unique (a$0)
);
"), @r"
          ╭▸ 
        3 │   a int,
          │   ─ 2. destination
        4 │   b text,
        5 │   unique (a)
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_table_primary_key_constraint_column() {
        assert_snapshot!(goto("
create table t (
  id bigint generated always as identity,
  inserted_at timestamptz not null default now(),
  primary key (id, inserted_at$0)
);
"), @r"
          ╭▸ 
        4 │   inserted_at timestamptz not null default now(),
          │   ─────────── 2. destination
        5 │   primary key (id, inserted_at)
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_table_not_null_constraint_column() {
        assert_snapshot!(goto("
create table t (
  id integer,
  name text,
  not null name$0
);
"), @r"
          ╭▸ 
        4 │   name text,
          │   ──── 2. destination
        5 │   not null name
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_table_exclude_constraint_column() {
        assert_snapshot!(goto("
create table circles (
  c circle,
  exclude using gist (c$0 with &&)
);
"), @r"
          ╭▸ 
        3 │   c circle,
          │   ─ 2. destination
        4 │   exclude using gist (c with &&)
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_table_exclude_constraint_include_column() {
        assert_snapshot!(goto("
create table t (
  a int,
  b text,
  exclude using btree ( a with > ) 
    include (a$0, b)
);
"), @r"
          ╭▸ 
        3 │   a int,
          │   ─ 2. destination
          ‡
        6 │     include (a, b)
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_table_exclude_constraint_where_column() {
        assert_snapshot!(goto("
create table t (
  a int,
  b text,
  exclude using btree ( a with > ) 
    where ( a$0 > 10 and b like '%foo' )
);
"), @r"
          ╭▸ 
        3 │   a int,
          │   ─ 2. destination
          ‡
        6 │     where ( a > 10 and b like '%foo' )
          ╰╴            ─ 1. source
        ");
    }

    #[test]
    fn goto_table_partition_by_column() {
        assert_snapshot!(goto("
create table t (
  id bigint generated always as identity,
  inserted_at timestamptz not null default now()
) partition by range (inserted_at$0);
"), @r"
          ╭▸ 
        4 │   inserted_at timestamptz not null default now()
          │   ─────────── 2. destination
        5 │ ) partition by range (inserted_at);
          ╰╴                                ─ 1. source
        ");
    }

    #[test]
    fn goto_table_partition_of_table() {
        assert_snapshot!(goto("
create table t ();
create table t_2026_01_02 partition of t$0
    for values from ('2026-01-02') to ('2026-01-03');
"), @r"
          ╭▸ 
        2 │ create table t ();
          │              ─ 2. destination
        3 │ create table t_2026_01_02 partition of t
          ╰╴                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_like_clause() {
        assert_snapshot!(goto("
create table large_data_table(a text);
create table t (
  a text,
  like large_data_table$0,
  b integer
);
"), @r"
          ╭▸ 
        2 │ create table large_data_table(a text);
          │              ──────────────── 2. destination
          ‡
        5 │   like large_data_table,
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_inherits() {
        assert_snapshot!(goto("
create table bar(a int);
create table t (a int)
inherits (foo.bar, bar$0, buzz);
"), @r"
          ╭▸ 
        2 │ create table bar(a int);
          │              ─── 2. destination
        3 │ create table t (a int)
        4 │ inherits (foo.bar, bar, buzz);
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_temp_table_shadows_public() {
        // temp tables shadow public tables when no schema is specified
        assert_snapshot!(goto("
create table t();
create temp table t();
drop table t$0;
"), @r"
          ╭▸ 
        2 │ create table t();
          │              ─ 2. destination
        3 │ create temp table t();
        4 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_public_table_when_temp_exists() {
        // can still access public table explicitly
        assert_snapshot!(goto("
create table t();
create temp table t();
drop table public.t$0;
"), @r"
          ╭▸ 
        2 │ create table t();
          │              ─ 2. destination
        3 │ create temp table t();
        4 │ drop table public.t;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_table_defined_after() {
        assert_snapshot!(goto("
drop table t$0;
create table t();
"), @r"
          ╭▸ 
        2 │ drop table t;
          │            ─ 1. source
        3 │ create table t();
          ╰╴             ─ 2. destination
        ");
    }

    #[test]
    fn goto_drop_type() {
        assert_snapshot!(goto("
create type t as enum ('a', 'b');
drop type t$0;
"), @r"
          ╭▸ 
        2 │ create type t as enum ('a', 'b');
          │             ─ 2. destination
        3 │ drop type t;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_type_with_schema() {
        assert_snapshot!(goto("
create type public.t as enum ('a', 'b');
drop type t$0;
"), @r"
          ╭▸ 
        2 │ create type public.t as enum ('a', 'b');
          │                    ─ 2. destination
        3 │ drop type t;
          ╰╴          ─ 1. source
        ");

        assert_snapshot!(goto("
create type foo.t as enum ('a', 'b');
drop type foo.t$0;
"), @r"
          ╭▸ 
        2 │ create type foo.t as enum ('a', 'b');
          │                 ─ 2. destination
        3 │ drop type foo.t;
          ╰╴              ─ 1. source
        ");

        goto_not_found(
            "
create type t as enum ('a', 'b');
drop type foo.t$0;
",
        );
    }

    #[test]
    fn goto_drop_type_defined_after() {
        assert_snapshot!(goto("
drop type t$0;
create type t as enum ('a', 'b');
"), @r"
          ╭▸ 
        2 │ drop type t;
          │           ─ 1. source
        3 │ create type t as enum ('a', 'b');
          ╰╴            ─ 2. destination
        ");
    }

    #[test]
    fn goto_drop_type_composite() {
        assert_snapshot!(goto("
create type person as (name text, age int);
drop type person$0;
"), @r"
          ╭▸ 
        2 │ create type person as (name text, age int);
          │             ────── 2. destination
        3 │ drop type person;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_type_reference() {
        assert_snapshot!(goto("
create type person_info as (name text, email text);
create table user(id int, member person_info$0);
"), @r"
          ╭▸ 
        2 │ create type person_info as (name text, email text);
          │             ─────────── 2. destination
        3 │ create table user(id int, member person_info);
          ╰╴                                           ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_type_reference_enum() {
        assert_snapshot!(goto("
create type mood as enum ('sad', 'ok', 'happy');
create table users(id int, mood mood$0);
"), @r"
          ╭▸ 
        2 │ create type mood as enum ('sad', 'ok', 'happy');
          │             ──── 2. destination
        3 │ create table users(id int, mood mood);
          ╰╴                                   ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_type_reference_range() {
        assert_snapshot!(goto("
create type int4_range as range (subtype = int4);
create table metrics(id int, span int4_range$0);
"), @r"
          ╭▸ 
        2 │ create type int4_range as range (subtype = int4);
          │             ────────── 2. destination
        3 │ create table metrics(id int, span int4_range);
          ╰╴                                           ─ 1. source
        ");
    }

    #[test]
    fn goto_create_table_type_reference_input_output() {
        assert_snapshot!(goto("
create type myint (input = myintin, output = myintout, like = int4);
create table data(id int, value myint$0);
"), @r"
          ╭▸ 
        2 │ create type myint (input = myintin, output = myintout, like = int4);
          │             ───── 2. destination
        3 │ create table data(id int, value myint);
          ╰╴                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_composite_type_field() {
        assert_snapshot!(goto("
create type person_info as (name text, email text);
create table user(id int, member person_info);
select (member).name$0 from user;
"), @r"
          ╭▸ 
        2 │ create type person_info as (name text, email text);
          │                             ──── 2. destination
        3 │ create table user(id int, member person_info);
        4 │ select (member).name from user;
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_type_range() {
        assert_snapshot!(goto("
create type int4_range as range (subtype = int4);
drop type int4_range$0;
"), @r"
          ╭▸ 
        2 │ create type int4_range as range (subtype = int4);
          │             ────────── 2. destination
        3 │ drop type int4_range;
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_view() {
        assert_snapshot!(goto("
create view v as select 1;
drop view v$0;
"), @r"
          ╭▸ 
        2 │ create view v as select 1;
          │             ─ 2. destination
        3 │ drop view v;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_materialized_view() {
        assert_snapshot!(goto("
create materialized view v as select 1;
drop materialized view v$0;
"), @r"
          ╭▸ 
        2 │ create materialized view v as select 1;
          │                          ─ 2. destination
        3 │ drop materialized view v;
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_view_with_schema() {
        assert_snapshot!(goto("
create view public.v as select 1;
drop view v$0;
"), @r"
          ╭▸ 
        2 │ create view public.v as select 1;
          │                    ─ 2. destination
        3 │ drop view v;
          ╰╴          ─ 1. source
        ");

        assert_snapshot!(goto("
create view foo.v as select 1;
drop view foo.v$0;
"), @r"
          ╭▸ 
        2 │ create view foo.v as select 1;
          │                 ─ 2. destination
        3 │ drop view foo.v;
          ╰╴              ─ 1. source
        ");

        goto_not_found(
            "
create view v as select 1;
drop view foo.v$0;
",
        );
    }

    #[test]
    fn goto_drop_temp_view() {
        assert_snapshot!(goto("
create temp view v as select 1;
drop view v$0;
"), @r"
          ╭▸ 
        2 │ create temp view v as select 1;
          │                  ─ 2. destination
        3 │ drop view v;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_select_from_view() {
        assert_snapshot!(goto("
create view v as select 1;
select * from v$0;
"), @r"
          ╭▸ 
        2 │ create view v as select 1;
          │             ─ 2. destination
        3 │ select * from v;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_select_from_materialized_view() {
        assert_snapshot!(goto("
create materialized view v as select 1;
select * from v$0;
"), @r"
          ╭▸ 
        2 │ create materialized view v as select 1;
          │                          ─ 2. destination
        3 │ select * from v;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_select_from_view_with_schema() {
        assert_snapshot!(goto("
create view public.v as select 1;
select * from public.v$0;
"), @r"
          ╭▸ 
        2 │ create view public.v as select 1;
          │                    ─ 2. destination
        3 │ select * from public.v;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_view_column() {
        assert_snapshot!(goto("
create view v as select 1 as a;
select a$0 from v;
"), @r"
          ╭▸ 
        2 │ create view v as select 1 as a;
          │                              ─ 2. destination
        3 │ select a from v;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_view_column_qualified() {
        assert_snapshot!(goto("
create view v as select 1 as a;
select v.a$0 from v;
"), @r"
          ╭▸ 
        2 │ create view v as select 1 as a;
          │                              ─ 2. destination
        3 │ select v.a from v;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_view_with_explicit_column_list() {
        assert_snapshot!(goto("
create view v(col1) as select 1;
select * from v$0;
"), @r"
          ╭▸ 
        2 │ create view v(col1) as select 1;
          │             ─ 2. destination
        3 │ select * from v;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_view_column_with_explicit_column_list() {
        assert_snapshot!(goto("
    create view v(col1) as select 1;
    select col1$0 from v;
    "), @r"
          ╭▸ 
        2 │     create view v(col1) as select 1;
          │                   ──── 2. destination
        3 │     select col1 from v;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_view_column_with_schema() {
        assert_snapshot!(goto("
create view public.v as select 1 as a;
select a$0 from public.v;
"), @r"
          ╭▸ 
        2 │ create view public.v as select 1 as a;
          │                                     ─ 2. destination
        3 │ select a from public.v;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_view_multiple_columns() {
        assert_snapshot!(goto("
create view v as select 1 as a, 2 as b;
select b$0 from v;
"), @r"
          ╭▸ 
        2 │ create view v as select 1 as a, 2 as b;
          │                                      ─ 2. destination
        3 │ select b from v;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_view_column_from_table() {
        assert_snapshot!(goto("
create table t(x int, y int);
create view v as select x, y from t;
select x$0 from v;
"), @r"
          ╭▸ 
        3 │ create view v as select x, y from t;
          │                         ─ 2. destination
        4 │ select x from v;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_view_column_with_table_preference() {
        assert_snapshot!(goto("
create table v(a int);
create view vw as select 1 as a;
select a$0 from v;
"), @r"
          ╭▸ 
        2 │ create table v(a int);
          │                ─ 2. destination
        3 │ create view vw as select 1 as a;
        4 │ select a from v;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_operator() {
        assert_snapshot!(goto("
create type foo as enum ('a', 'b');
select x::foo$0;
"), @r"
          ╭▸ 
        2 │ create type foo as enum ('a', 'b');
          │             ─── 2. destination
        3 │ select x::foo;
          ╰╴            ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_function() {
        assert_snapshot!(goto("
create type bar as enum ('x', 'y');
select cast(x as bar$0);
"), @r"
          ╭▸ 
        2 │ create type bar as enum ('x', 'y');
          │             ─── 2. destination
        3 │ select cast(x as bar);
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_with_schema() {
        assert_snapshot!(goto("
create type public.baz as enum ('m', 'n');
select x::public.baz$0;
"), @r"
          ╭▸ 
        2 │ create type public.baz as enum ('m', 'n');
          │                    ─── 2. destination
        3 │ select x::public.baz;
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_composite_type() {
        assert_snapshot!(goto("
create type person_info as (name varchar(50), age int);
select ('Alice', 30)::person_info$0;
"), @r"
          ╭▸ 
        2 │ create type person_info as (name varchar(50), age int);
          │             ─────────── 2. destination
        3 │ select ('Alice', 30)::person_info;
          ╰╴                                ─ 1. source
        ");
    }

    #[test]
    fn goto_cast_composite_type_in_cte() {
        assert_snapshot!(goto("
create type person_info as (name varchar(50), age int);
with team as (
    select 1 as id, ('Alice', 30)::person_info$0 as member
)
select * from team;
"), @r"
          ╭▸ 
        2 │ create type person_info as (name varchar(50), age int);
          │             ─────────── 2. destination
        3 │ with team as (
        4 │     select 1 as id, ('Alice', 30)::person_info as member
          ╰╴                                             ─ 1. source
        ");
    }

    #[test]
    fn goto_composite_type_field_name() {
        assert_snapshot!(goto("
create type person_info as (name varchar(50), age int);
with team as (
    select 1 as id, ('Alice', 30)::person_info as member
)
select (member).name$0, (member).age from team;
"), @r"
          ╭▸ 
        2 │ create type person_info as (name varchar(50), age int);
          │                             ──── 2. destination
          ‡
        6 │ select (member).name, (member).age from team;
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_composite_type_field_in_where() {
        assert_snapshot!(goto("
create type person_info as (name varchar(50), age int);
with team as (
    select 1 as id, ('Alice', 30)::person_info as member
    union all
    select 2, ('Bob', 25)::person_info
)
select (member).name, (member).age
from team
where (member).age$0 >= 18;
"), @r"
           ╭▸ 
         2 │ create type person_info as (name varchar(50), age int);
           │                                               ─── 2. destination
           ‡
        10 │ where (member).age >= 18;
           ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_composite_type_field_base() {
        assert_snapshot!(goto("
create type person_info as (name varchar(50), age int);
with team as (
    select 1 as id, ('Alice', 30)::person_info as member
)
select (member$0).age from team;
"), @r"
          ╭▸ 
        4 │     select 1 as id, ('Alice', 30)::person_info as member
          │                                                   ────── 2. destination
        5 │ )
        6 │ select (member).age from team;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_composite_type_field_nested_parens() {
        assert_snapshot!(goto("
create type person_info as (name varchar(50), age int);
with team as (
    select 1 as id, ('Alice', 30)::person_info as member
)
select ((((member))).name$0) from team;
"), @r"
          ╭▸ 
        2 │ create type person_info as (name varchar(50), age int);
          │                             ──── 2. destination
          ‡
        6 │ select ((((member))).name) from team;
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn begin_to_rollback() {
        assert_snapshot!(goto(
            "
begin$0;
select 1;
rollback;
commit;
",
        ), @r"
          ╭▸ 
        2 │ begin;
          │     ─ 1. source
        3 │ select 1;
        4 │ rollback;
          ╰╴──────── 2. destination
        ");
    }

    #[test]
    fn commit_to_begin() {
        assert_snapshot!(goto(
            "
begin;
select 1;
commit$0;
",
        ), @r"
          ╭▸ 
        2 │ begin;
          │ ───── 2. destination
        3 │ select 1;
        4 │ commit;
          ╰╴     ─ 1. source
        ");
    }

    #[test]
    fn begin_to_commit() {
        assert_snapshot!(goto(
            "
begin$0;
select 1;
commit;
",
        ), @r"
          ╭▸ 
        2 │ begin;
          │     ─ 1. source
        3 │ select 1;
        4 │ commit;
          ╰╴────── 2. destination
        ");
    }

    #[test]
    fn commit_to_start_transaction() {
        assert_snapshot!(goto(
            "
start transaction;
select 1;
commit$0;
",
        ), @r"
          ╭▸ 
        2 │ start transaction;
          │ ───────────────── 2. destination
        3 │ select 1;
        4 │ commit;
          ╰╴     ─ 1. source
        ");
    }

    #[test]
    fn start_transaction_to_commit() {
        assert_snapshot!(goto(
            "
start$0 transaction;
select 1;
commit;
",
        ), @r"
          ╭▸ 
        2 │ start transaction;
          │     ─ 1. source
        3 │ select 1;
        4 │ commit;
          ╰╴────── 2. destination
        ");
    }

    #[test]
    fn goto_with_search_path() {
        assert_snapshot!(goto(r#"
set search_path to "foo", public;
create table foo.t();
drop table t$0;
"#), @r"
          ╭▸ 
        3 │ create table foo.t();
          │                  ─ 2. destination
        4 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_with_search_path_and_unspecified_table() {
        assert_snapshot!(goto(r#"
set search_path to foo,bar;
create table t();
drop table foo.t$0;
"#), @r"
          ╭▸ 
        3 │ create table t();
          │              ─ 2. destination
        4 │ drop table foo.t;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_with_search_path_empty() {
        goto_not_found(
            r#"
set search_path = '';
create table t();
drop table t$0;
"#,
        );
    }

    #[test]
    fn goto_with_search_path_like_variable() {
        // not actually search path
        goto_not_found(
            "
set bar.search_path to foo, public;
create table foo.t();
drop table t$0;
",
        )
    }

    #[test]
    fn goto_with_search_path_second_schema() {
        assert_snapshot!(goto("
set search_path to foo, bar, public;
create table bar.t();
drop table t$0;
"), @r"
          ╭▸ 
        3 │ create table bar.t();
          │                  ─ 2. destination
        4 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_with_search_path_skips_first() {
        assert_snapshot!(goto("
set search_path to foo, bar, public;
create table foo.t();
create table bar.t();
drop table t$0;
"), @r"
          ╭▸ 
        3 │ create table foo.t();
          │                  ─ 2. destination
        4 │ create table bar.t();
        5 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_without_search_path_uses_default() {
        assert_snapshot!(goto("
create table foo.t();
create table public.t();
drop table t$0;
"), @r"
          ╭▸ 
        3 │ create table public.t();
          │                     ─ 2. destination
        4 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_with_set_schema() {
        assert_snapshot!(goto("
set schema 'myschema';
create table myschema.t();
drop table t$0;
"), @r"
          ╭▸ 
        3 │ create table myschema.t();
          │                       ─ 2. destination
        4 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_with_set_schema_ignores_other_schemas() {
        assert_snapshot!(goto("
set schema 'myschema';
create table public.t();
create table myschema.t();
drop table t$0;
"), @r"
          ╭▸ 
        4 │ create table myschema.t();
          │                       ─ 2. destination
        5 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_with_search_path_changed_twice() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.t();
set search_path to bar;
create table bar.t();
drop table t$0;
"), @r"
          ╭▸ 
        5 │ create table bar.t();
          │                  ─ 2. destination
        6 │ drop table t;
          ╰╴           ─ 1. source
        ");

        assert_snapshot!(goto("
set search_path to foo;
create table foo.t();
drop table t$0;
set search_path to bar;
create table bar.t();
drop table t;
"), @r"
          ╭▸ 
        3 │ create table foo.t();
          │                  ─ 2. destination
        4 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_with_empty_search_path() {
        goto_not_found(
            "
set search_path to '';
create table public.t();
drop table t$0;
",
        )
    }

    #[test]
    fn goto_with_search_path_uppercase() {
        assert_snapshot!(goto("
SET SEARCH_PATH TO foo;
create table foo.t();
drop table t$0;
"), @r"
          ╭▸ 
        3 │ create table foo.t();
          │                  ─ 2. destination
        4 │ drop table t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_table_stmt() {
        assert_snapshot!(goto("
create table t();
table t$0;
"), @r"
          ╭▸ 
        2 │ create table t();
          │              ─ 2. destination
        3 │ table t;
          ╰╴      ─ 1. source
        ");
    }

    #[test]
    fn goto_table_stmt_with_schema() {
        assert_snapshot!(goto("
create table public.t();
table public.t$0;
"), @r"
          ╭▸ 
        2 │ create table public.t();
          │                     ─ 2. destination
        3 │ table public.t;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_table_stmt_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.t();
table t$0;
"), @r"
          ╭▸ 
        3 │ create table foo.t();
          │                  ─ 2. destination
        4 │ table t;
          ╰╴      ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_index() {
        assert_snapshot!(goto("
create index idx_name on t(x);
drop index idx_name$0;
"), @r"
          ╭▸ 
        2 │ create index idx_name on t(x);
          │              ──────── 2. destination
        3 │ drop index idx_name;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_index_with_schema() {
        assert_snapshot!(goto(r#"
set search_path to public;
create index idx_name on t(x);
drop index public.idx_name$0;
"#), @r"
          ╭▸ 
        3 │ create index idx_name on t(x);
          │              ──────── 2. destination
        4 │ drop index public.idx_name;
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_index_defined_after() {
        assert_snapshot!(goto("
drop index idx_name$0;
create index idx_name on t(x);
"), @r"
          ╭▸ 
        2 │ drop index idx_name;
          │                   ─ 1. source
        3 │ create index idx_name on t(x);
          ╰╴             ──────── 2. destination
        ");
    }

    #[test]
    fn goto_index_definition_returns_self() {
        assert_snapshot!(goto("
create index idx_name$0 on t(x);
"), @r"
          ╭▸ 
        2 │ create index idx_name on t(x);
          │              ┬──────┬
          │              │      │
          │              │      1. source
          ╰╴             2. destination
        ");
    }

    #[test]
    fn goto_drop_index_with_search_path() {
        assert_snapshot!(goto(r#"
create index idx_name on t(x);
set search_path to bar;
create index idx_name on f(x);
set search_path to default;
drop index idx_name$0;
"#), @r"
          ╭▸ 
        2 │ create index idx_name on t(x);
          │              ──────── 2. destination
          ‡
        6 │ drop index idx_name;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_index_multiple() {
        assert_snapshot!(goto("
create index idx1 on t(x);
create index idx2 on t(y);
drop index idx1, idx2$0;
"), @r"
          ╭▸ 
        3 │ create index idx2 on t(y);
          │              ──── 2. destination
        4 │ drop index idx1, idx2;
          ╰╴                    ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_table() {
        assert_snapshot!(goto("
create table users(id int);
create index idx_users on users$0(id);
"), @r"
          ╭▸ 
        2 │ create table users(id int);
          │              ───── 2. destination
        3 │ create index idx_users on users(id);
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_table_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int);
create index idx_users on public.users$0(id);
"), @r"
          ╭▸ 
        2 │ create table public.users(id int);
          │                     ───── 2. destination
        3 │ create index idx_users on public.users(id);
          ╰╴                                     ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_table_with_search_path() {
        assert_snapshot!(goto(r#"
set search_path to foo;
create table foo.users(id int);
create index idx_users on users$0(id);
"#), @r"
          ╭▸ 
        3 │ create table foo.users(id int);
          │                  ───── 2. destination
        4 │ create index idx_users on users(id);
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_temp_table() {
        assert_snapshot!(goto("
create temp table users(id int);
create index idx_users on users$0(id);
"), @r"
          ╭▸ 
        2 │ create temp table users(id int);
          │                   ───── 2. destination
        3 │ create index idx_users on users(id);
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
create index idx_email on users(email$0);
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                            ───── 2. destination
        3 │ create index idx_email on users(email);
          ╰╴                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_first_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
create index idx_id on users(id$0);
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                    ── 2. destination
        3 │ create index idx_id on users(id);
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_multiple_columns() {
        assert_snapshot!(goto("
create table users(id int, email text, name text);
create index idx_users on users(id, email$0, name);
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text, name text);
          │                            ───── 2. destination
        3 │ create index idx_users on users(id, email, name);
          ╰╴                                        ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_column_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
create index idx_email on public.users(email$0);
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                                   ───── 2. destination
        3 │ create index idx_email on public.users(email);
          ╰╴                                           ─ 1. source
        ");
    }

    #[test]
    fn goto_create_index_column_temp_table() {
        assert_snapshot!(goto("
create temp table users(id int, email text);
create index idx_email on users(email$0);
"), @r"
          ╭▸ 
        2 │ create temp table users(id int, email text);
          │                                 ───── 2. destination
        3 │ create index idx_email on users(email);
          ╰╴                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_function() {
        assert_snapshot!(goto("
create function foo() returns int as $$ select 1 $$ language sql;
drop function foo$0();
"), @r"
          ╭▸ 
        2 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        3 │ drop function foo();
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_function_with_schema() {
        assert_snapshot!(goto("
set search_path to public;
create function foo() returns int as $$ select 1 $$ language sql;
drop function public.foo$0();
"), @r"
          ╭▸ 
        3 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        4 │ drop function public.foo();
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_function_defined_after() {
        assert_snapshot!(goto("
drop function foo$0();
create function foo() returns int as $$ select 1 $$ language sql;
"), @r"
          ╭▸ 
        2 │ drop function foo();
          │                 ─ 1. source
        3 │ create function foo() returns int as $$ select 1 $$ language sql;
          ╰╴                ─── 2. destination
        ");
    }

    #[test]
    fn goto_function_definition_returns_self() {
        assert_snapshot!(goto("
create function foo$0() returns int as $$ select 1 $$ language sql;
"), @r"
          ╭▸ 
        2 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ┬─┬
          │                 │ │
          │                 │ 1. source
          ╰╴                2. destination
        ");
    }

    #[test]
    fn goto_drop_function_with_search_path() {
        assert_snapshot!(goto("
create function foo() returns int as $$ select 1 $$ language sql;
set search_path to bar;
create function foo() returns int as $$ select 1 $$ language sql;
set search_path to default;
drop function foo$0();
"), @r"
          ╭▸ 
        2 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
          ‡
        6 │ drop function foo();
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_function_multiple() {
        assert_snapshot!(goto("
create function foo() returns int as $$ select 1 $$ language sql;
create function bar() returns int as $$ select 1 $$ language sql;
drop function foo(), bar$0();
"), @r"
          ╭▸ 
        3 │ create function bar() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        4 │ drop function foo(), bar();
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_function_overloaded() {
        assert_snapshot!(goto("
create function add(complex) returns complex as $$ select null $$ language sql;
create function add(bigint) returns bigint as $$ select 1 $$ language sql;
drop function add$0(complex);
"), @r"
          ╭▸ 
        2 │ create function add(complex) returns complex as $$ select null $$ language sql;
          │                 ─── 2. destination
        3 │ create function add(bigint) returns bigint as $$ select 1 $$ language sql;
        4 │ drop function add(complex);
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_function_second_overload() {
        assert_snapshot!(goto("
create function add(complex) returns complex as $$ select null $$ language sql;
create function add(bigint) returns bigint as $$ select 1 $$ language sql;
drop function add$0(bigint);
"), @r"
          ╭▸ 
        3 │ create function add(bigint) returns bigint as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        4 │ drop function add(bigint);
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_select_function_call() {
        assert_snapshot!(goto("
create function foo() returns int as $$ select 1 $$ language sql;
select foo$0();
"), @r"
          ╭▸ 
        2 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        3 │ select foo();
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_select_aggregate_call() {
        assert_snapshot!(goto("
create aggregate foo(int) (
  sfunc = int4pl,
  stype = int,
  initcond = '0'
);

select foo$0(1);
"), @r"
          ╭▸ 
        2 │ create aggregate foo(int) (
          │                  ─── 2. destination
          ‡
        8 │ select foo(1);
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_default_constraint_function_call() {
        assert_snapshot!(goto("
create function f() returns int as 'select 1' language sql;
create table t(
  a int default f$0()
);
"), @r"
          ╭▸ 
        2 │ create function f() returns int as 'select 1' language sql;
          │                 ─ 2. destination
        3 │ create table t(
        4 │   a int default f()
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_select_function_call_with_schema() {
        assert_snapshot!(goto("
create function public.foo() returns int as $$ select 1 $$ language sql;
select public.foo$0();
"), @r"
          ╭▸ 
        2 │ create function public.foo() returns int as $$ select 1 $$ language sql;
          │                        ─── 2. destination
        3 │ select public.foo();
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_select_function_call_with_search_path() {
        assert_snapshot!(goto("
set search_path to myschema;
create function foo() returns int as $$ select 1 $$ language sql;
select myschema.foo$0();
"), @r"
          ╭▸ 
        3 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        4 │ select myschema.foo();
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_column_access() {
        assert_snapshot!(goto("
create table t(a int, b int);
select a$0(t) from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ select a(t) from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_column_access_with_function_precedence() {
        assert_snapshot!(goto("
create table t(a int, b int);
create function b(t) returns int as 'select 1' LANGUAGE sql;
select b$0(t) from t;
"), @r"
          ╭▸ 
        3 │ create function b(t) returns int as 'select 1' LANGUAGE sql;
          │                 ─ 2. destination
        4 │ select b(t) from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_column_access_table_arg() {
        assert_snapshot!(goto("
create table t(a int, b int);
select a(t$0) from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │              ─ 2. destination
        3 │ select a(t) from t;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_column_access_table_arg_with_function() {
        assert_snapshot!(goto("
create table t(a int, b int);
create function b(t) returns int as 'select 1' LANGUAGE sql;
select b(t$0) from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │              ─ 2. destination
        3 │ create function b(t) returns int as 'select 1' LANGUAGE sql;
        4 │ select b(t) from t;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_multiple_args_not_column_access() {
        goto_not_found(
            "
create table t(a int, b int);
select a$0(t, 1) from t;
",
        );
    }

    #[test]
    fn goto_field_style_function_call() {
        assert_snapshot!(goto("
create table t(a int);
create function b(t) returns int as 'select 1' language sql;
select t.b$0 from t;
"), @r"
          ╭▸ 
        3 │ create function b(t) returns int as 'select 1' language sql;
          │                 ─ 2. destination
        4 │ select t.b from t;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_field_style_function_call_column_precedence() {
        assert_snapshot!(goto("
create table t(a int, b int);
create function b(t) returns int as 'select 1' language sql;
select t.b$0 from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                       ─ 2. destination
        3 │ create function b(t) returns int as 'select 1' language sql;
        4 │ select t.b from t;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_field_style_function_call_table_ref() {
        assert_snapshot!(goto("
create table t(a int);
create function b(t) returns int as 'select 1' language sql;
select t$0.b from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int);
          │              ─ 2. destination
        3 │ create function b(t) returns int as 'select 1' language sql;
        4 │ select t.b from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_in_where() {
        assert_snapshot!(goto("
create table t(a int, b int);
select * from t where a$0(t) > 0;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ select * from t where a(t) > 0;
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_in_where_function_precedence() {
        assert_snapshot!(goto("
create table t(a int, b int);
create function b(t) returns int as 'select 1' language sql;
select * from t where b$0(t) > 0;
"), @r"
          ╭▸ 
        3 │ create function b(t) returns int as 'select 1' language sql;
          │                 ─ 2. destination
        4 │ select * from t where b(t) > 0;
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_field_style_function_call_in_where() {
        assert_snapshot!(goto("
create table t(a int);
create function b(t) returns int as 'select 1' language sql;
select * from t where t.b$0 > 0;
"), @r"
          ╭▸ 
        3 │ create function b(t) returns int as 'select 1' language sql;
          │                 ─ 2. destination
        4 │ select * from t where t.b > 0;
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_field_style_in_where_column_precedence() {
        assert_snapshot!(goto("
create table t(a int, b int);
create function b(t) returns int as 'select 1' language sql;
select * from t where t.b$0 > 0;
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                       ─ 2. destination
        3 │ create function b(t) returns int as 'select 1' language sql;
        4 │ select * from t where t.b > 0;
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_table_arg_in_where() {
        assert_snapshot!(goto("
create table t(a int);
select * from t where a(t$0) > 2;
"), @r"
          ╭▸ 
        2 │ create table t(a int);
          │              ─ 2. destination
        3 │ select * from t where a(t) > 2;
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_qualified_table_ref_in_where() {
        assert_snapshot!(goto("
create table t(a int);
create function b(t) returns int as 'select 1' language sql;
select * from t where t$0.b > 2;
"), @r"
          ╭▸ 
        2 │ create table t(a int);
          │              ─ 2. destination
        3 │ create function b(t) returns int as 'select 1' language sql;
        4 │ select * from t where t.b > 2;
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_in_order_by() {
        assert_snapshot!(goto("
create table t(a int, b int);
create function b(t) returns int as 'select 1' language sql;
select * from t order by b$0(t);
"), @r"
          ╭▸ 
        3 │ create function b(t) returns int as 'select 1' language sql;
          │                 ─ 2. destination
        4 │ select * from t order by b(t);
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_field_style_in_order_by() {
        assert_snapshot!(goto("
create table t(a int);
create function b(t) returns int as 'select 1' language sql;
select * from t order by t.b$0;
"), @r"
          ╭▸ 
        3 │ create function b(t) returns int as 'select 1' language sql;
          │                 ─ 2. destination
        4 │ select * from t order by t.b;
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_function_call_style_in_group_by() {
        assert_snapshot!(goto("
create table t(a int, b int);
select * from t group by a$0(t);
"), @r"
          ╭▸ 
        2 │ create table t(a int, b int);
          │                ─ 2. destination
        3 │ select * from t group by a(t);
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_field_style_in_group_by() {
        assert_snapshot!(goto("
create table t(a int);
create function b(t) returns int as 'select 1' language sql;
select * from t group by t.b$0;
"), @r"
          ╭▸ 
        3 │ create function b(t) returns int as 'select 1' language sql;
          │                 ─ 2. destination
        4 │ select * from t group by t.b;
          ╰╴                           ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_table() {
        assert_snapshot!(goto("
with x as (select 1 as a)
select a from x$0;
"), @r"
          ╭▸ 
        2 │ with x as (select 1 as a)
          │      ─ 2. destination
        3 │ select a from x;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_column() {
        assert_snapshot!(goto("
with x as (select 1 as a)
select a$0 from x;
"), @r"
          ╭▸ 
        2 │ with x as (select 1 as a)
          │                        ─ 2. destination
        3 │ select a from x;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_qualified_column_prefers_cte_over_table() {
        assert_snapshot!(goto("
create table u(id int, b int);
with u as (select 1 id, 2 b)
select u.id$0 from u;
"), @r"
          ╭▸ 
        3 │ with u as (select 1 id, 2 b)
          │                     ── 2. destination
        4 │ select u.id from u;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_subquery_qualified_column() {
        assert_snapshot!(goto("
select t.a$0 from (select 1 a) t;
"), @r"
          ╭▸ 
        2 │ select t.a from (select 1 a) t;
          ╰╴         ─ 1. source      ─ 2. destination
        ");
    }

    #[test]
    fn goto_cte_multiple_columns() {
        assert_snapshot!(goto("
with x as (select 1 as a, 2 as b)
select b$0 from x;
"), @r"
          ╭▸ 
        2 │ with x as (select 1 as a, 2 as b)
          │                                ─ 2. destination
        3 │ select b from x;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_nested() {
        assert_snapshot!(goto("
with x as (select 1 as a),
     y as (select a from x)
select a$0 from y;
"), @r"
          ╭▸ 
        3 │      y as (select a from x)
          │                   ─ 2. destination
        4 │ select a from y;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_unnamed_column() {
        assert_snapshot!(goto(r#"
with x as (select 1)
select "?column?"$0 from x;
"#), @r#"
          ╭▸ 
        2 │ with x as (select 1)
          │                   ─ 2. destination
        3 │ select "?column?" from x;
          ╰╴                ─ 1. source
        "#);
    }

    #[test]
    fn goto_cte_star_expansion() {
        assert_snapshot!(goto("
with t as (select 1 a),
     y as (select * from t)
select a$0 from y;
"), @r"
          ╭▸ 
        2 │ with t as (select 1 a),
          │                     ─ 2. destination
        3 │      y as (select * from t)
        4 │ select a from y;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_qualified_star_join_column() {
        assert_snapshot!(goto("
create table u(id int, b int);
create table t(id int, a int);

with k as (
    select u.* from t join u on a = b
)
select b$0 from k;
"), @r"
          ╭▸ 
        2 │ create table u(id int, b int);
          │                        ─ 2. destination
          ‡
        8 │ select b from k;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_qualified_star_join_column_with_partial_column_list() {
        assert_snapshot!(goto("
with
  u as (
    select 1 id, 2 b
  ),
  t as (
    select 1 id, 2 a
  ),
  k(x) as (
    select u.* from t join u on a = b
  )
select b$0 from k;
"), @r"
          ╭▸ 
        4 │     select 1 id, 2 b
          │                    ─ 2. destination
          ‡
       12 │ select b from k;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_reference_inside_cte() {
        assert_snapshot!(goto("
with t as (select 1 a),
     y as (select a$0 from t)
select a from y;
"), @r"
          ╭▸ 
        2 │ with t as (select 1 a),
          │                     ─ 2. destination
        3 │      y as (select a from t)
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_with_column_list() {
        assert_snapshot!(goto("
with t(a) as (select 1)
select a$0 from t;
"), @r"
          ╭▸ 
        2 │ with t(a) as (select 1)
          │        ─ 2. destination
        3 │ select a from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_with_partial_column_list() {
        assert_snapshot!(goto("
with t(x) as (select 1 as a, 2 as b)
select b$0 from t;
"), @r"
          ╭▸ 
        2 │ with t(x) as (select 1 as a, 2 as b)
          │                                   ─ 2. destination
        3 │ select b from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_with_partial_column_list_renamed() {
        assert_snapshot!(goto("
with t(x) as (select 1 as a, 2 as b)
select x$0 from t;
"), @r"
          ╭▸ 
        2 │ with t(x) as (select 1 as a, 2 as b)
          │        ─ 2. destination
        3 │ select x from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_column_list_overwrites_column() {
        goto_not_found(
            "
with t(x) as (select 1 as a)
select a$0 from t;
",
        );
    }

    #[test]
    fn goto_cte_shadows_table() {
        assert_snapshot!(goto("
create table t(a int);
with t as (select a$0 from t)
select a from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ with t as (select a from t)
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_subquery_column() {
        assert_snapshot!(goto("
select a$0 from (select 1 a);
"), @r"
          ╭▸ 
        2 │ select a from (select 1 a);
          ╰╴       ─ 1. source      ─ 2. destination
        ");
    }

    #[test]
    fn goto_subquery_column_with_as() {
        assert_snapshot!(goto("
select a$0 from (select 1 as a);
"), @r"
          ╭▸ 
        2 │ select a from (select 1 as a);
          ╰╴       ─ 1. source         ─ 2. destination
        ");
    }

    #[test]
    fn goto_subquery_column_multiple_columns() {
        assert_snapshot!(goto("
select b$0 from (select 1 a, 2 b);
"), @r"
          ╭▸ 
        2 │ select b from (select 1 a, 2 b);
          ╰╴       ─ 1. source           ─ 2. destination
        ");
    }

    #[test]
    fn goto_subquery_column_nested_parens() {
        assert_snapshot!(goto("
select a$0 from ((select 1 a));
"), @r"
          ╭▸ 
        2 │ select a from ((select 1 a));
          ╰╴       ─ 1. source       ─ 2. destination
        ");
    }

    #[test]
    fn goto_subquery_column_star_table() {
        assert_snapshot!(goto("
create table foo.t(a int);
select a$0 from (select * from foo.t);
"), @r"
          ╭▸ 
        2 │ create table foo.t(a int);
          │                    ─ 2. destination
        3 │ select a from (select * from foo.t);
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_subquery_column_qualified_star_join() {
        assert_snapshot!(goto("
create table t(a int);
create table u(b int);
select b$0 from (select u.* from t join u on a = b);
"), @r"
          ╭▸ 
        3 │ create table u(b int);
          │                ─ 2. destination
        4 │ select b from (select u.* from t join u on a = b);
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_subquery_column_qualified_star_join_not_found() {
        goto_not_found(
            "
create table t(a int);
create table u(b int);
select a$0 from (select u.* from t join u on a = b);
",
        );
    }

    #[test]
    fn goto_insert_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
insert into users$0(id, email) values (1, 'test@example.com');
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │              ───── 2. destination
        3 │ insert into users(id, email) values (1, 'test@example.com');
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_table_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
insert into public.users$0(id, email) values (1, 'test@example.com');
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                     ───── 2. destination
        3 │ insert into public.users(id, email) values (1, 'test@example.com');
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
insert into users(id$0, email) values (1, 'test@example.com');
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                    ── 2. destination
        3 │ insert into users(id, email) values (1, 'test@example.com');
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_column_second() {
        assert_snapshot!(goto("
create table users(id int, email text);
insert into users(id, email$0) values (1, 'test@example.com');
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                            ───── 2. destination
        3 │ insert into users(id, email) values (1, 'test@example.com');
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_column_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
insert into public.users(email$0) values ('test@example.com');
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                                   ───── 2. destination
        3 │ insert into public.users(email) values ('test@example.com');
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_table_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.users(id int, email text);
insert into users$0(id, email) values (1, 'test@example.com');
"), @r"
          ╭▸ 
        3 │ create table foo.users(id int, email text);
          │                  ───── 2. destination
        4 │ insert into users(id, email) values (1, 'test@example.com');
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_column_with_search_path() {
        assert_snapshot!(goto("
set search_path to myschema;
create table myschema.users(id int, email text, name text);
insert into users(email$0, name) values ('test@example.com', 'Test');
"), @r"
          ╭▸ 
        3 │ create table myschema.users(id int, email text, name text);
          │                                     ───── 2. destination
        4 │ insert into users(email, name) values ('test@example.com', 'Test');
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
delete from users$0 where id = 1;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │              ───── 2. destination
        3 │ delete from users where id = 1;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_table_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
delete from public.users$0 where id = 1;
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                     ───── 2. destination
        3 │ delete from public.users where id = 1;
          ╰╴                       ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_table_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.users(id int, email text);
delete from users$0 where id = 1;
"), @r"
          ╭▸ 
        3 │ create table foo.users(id int, email text);
          │                  ───── 2. destination
        4 │ delete from users where id = 1;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_temp_table() {
        assert_snapshot!(goto("
create temp table users(id int, email text);
delete from users$0 where id = 1;
"), @r"
          ╭▸ 
        2 │ create temp table users(id int, email text);
          │                   ───── 2. destination
        3 │ delete from users where id = 1;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_where_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
delete from users where id$0 = 1;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                    ── 2. destination
        3 │ delete from users where id = 1;
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_where_column_second() {
        assert_snapshot!(goto("
create table users(id int, email text);
delete from users where email$0 = 'test@example.com';
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                            ───── 2. destination
        3 │ delete from users where email = 'test@example.com';
          ╰╴                            ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_where_column_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text, name text);
delete from public.users where name$0 = 'Test';
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text, name text);
          │                                               ──── 2. destination
        3 │ delete from public.users where name = 'Test';
          ╰╴                                  ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_where_column_with_search_path() {
        assert_snapshot!(goto("
set search_path to myschema;
create table myschema.users(id int, email text, active boolean);
delete from users where active$0 = true;
"), @r"
          ╭▸ 
        3 │ create table myschema.users(id int, email text, active boolean);
          │                                                 ────── 2. destination
        4 │ delete from users where active = true;
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_where_multiple_columns() {
        assert_snapshot!(goto("
create table users(id int, email text, active boolean);
delete from users where id$0 = 1 and active = true;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text, active boolean);
          │                    ── 2. destination
        3 │ delete from users where id = 1 and active = true;
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_select_from_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
select * from users$0;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │              ───── 2. destination
        3 │ select * from users;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_select_from_table_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
select * from public.users$0;
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                     ───── 2. destination
        3 │ select * from public.users;
          ╰╴                         ─ 1. source
        ");
    }

    #[test]
    fn goto_select_from_table_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.users(id int, email text);
select * from users$0;
"), @r"
          ╭▸ 
        3 │ create table foo.users(id int, email text);
          │                  ───── 2. destination
        4 │ select * from users;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_select_from_temp_table() {
        assert_snapshot!(goto("
create temp table users(id int, email text);
select * from users$0;
"), @r"
          ╭▸ 
        2 │ create temp table users(id int, email text);
          │                   ───── 2. destination
        3 │ select * from users;
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_select_from_table_defined_after() {
        assert_snapshot!(goto("
select * from users$0;
create table users(id int, email text);
"), @r"
          ╭▸ 
        2 │ select * from users;
          │                   ─ 1. source
        3 │ create table users(id int, email text);
          ╰╴             ───── 2. destination
        ");
    }

    #[test]
    fn goto_select_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
select id$0 from users;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                    ── 2. destination
        3 │ select id from users;
          ╰╴        ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_second() {
        assert_snapshot!(goto("
create table users(id int, email text);
select id, email$0 from users;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                            ───── 2. destination
        3 │ select id, email from users;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
select email$0 from public.users;
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                                   ───── 2. destination
        3 │ select email from public.users;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.users(id int, email text);
select id$0 from users;
"), @r"
          ╭▸ 
        3 │ create table foo.users(id int, email text);
          │                        ── 2. destination
        4 │ select id from users;
          ╰╴        ─ 1. source
        ");
    }

    #[test]
    fn goto_select_table_as_column() {
        assert_snapshot!(goto("
create table t(x bigint, y bigint);
select t$0 from t;
"), @r"
          ╭▸ 
        2 │ create table t(x bigint, y bigint);
          │              ─ 2. destination
        3 │ select t from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_select_table_star_expansion() {
        assert_snapshot!(goto("
create table t(id int, a int);
select t$0.* from t;
"), @r"
          ╭▸ 
        2 │ create table t(id int, a int);
          │              ─ 2. destination
        3 │ select t.* from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_select_table_as_column_with_schema() {
        assert_snapshot!(goto("
create table public.t(x bigint, y bigint);
select t$0 from public.t;
"), @r"
          ╭▸ 
        2 │ create table public.t(x bigint, y bigint);
          │                     ─ 2. destination
        3 │ select t from public.t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_select_table_as_column_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.users(id int, email text);
select users$0 from users;
"), @r"
          ╭▸ 
        3 │ create table foo.users(id int, email text);
          │                  ───── 2. destination
        4 │ select users from users;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_select_column_with_same_name_as_table() {
        assert_snapshot!(goto("
create table t(t int);
select t$0 from t;
"), @r"
          ╭▸ 
        2 │ create table t(t int);
          │                ─ 2. destination
        3 │ select t from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_schema() {
        assert_snapshot!(goto("
create schema foo;
drop schema foo$0;
"), @r"
          ╭▸ 
        2 │ create schema foo;
          │               ─── 2. destination
        3 │ drop schema foo;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_create_schema_authorization() {
        assert_snapshot!(goto("
create schema authorization foo$0;
"), @r"
          ╭▸ 
        2 │ create schema authorization foo;
          │                             ┬─┬
          │                             │ │
          │                             │ 1. source
          ╰╴                            2. destination
        ");
    }

    #[test]
    fn goto_drop_schema_authorization() {
        assert_snapshot!(goto("
create schema authorization foo;
drop schema foo$0;
"), @r"
          ╭▸ 
        2 │ create schema authorization foo;
          │                             ─── 2. destination
        3 │ drop schema foo;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_schema_defined_after() {
        assert_snapshot!(goto("
drop schema foo$0;
create schema foo;
"), @r"
          ╭▸ 
        2 │ drop schema foo;
          │               ─ 1. source
        3 │ create schema foo;
          ╰╴              ─── 2. destination
        ");
    }

    #[test]
    fn goto_schema_qualifier_in_table() {
        assert_snapshot!(goto("
create schema foo;
create table foo$0.t(a int);
"), @r"
          ╭▸ 
        2 │ create schema foo;
          │               ─── 2. destination
        3 │ create table foo.t(a int);
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_schema_qualifier_in_drop_table() {
        assert_snapshot!(goto("
create schema foo;
create table foo.t(a int);
drop table foo$0.t;
"), @r"
          ╭▸ 
        2 │ create schema foo;
          │               ─── 2. destination
        3 │ create table foo.t(a int);
        4 │ drop table foo.t;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_schema_qualifier_multiple_schemas() {
        assert_snapshot!(goto("
create schema foo;
create schema bar;
create table bar$0.t(a int);
"), @r"
          ╭▸ 
        3 │ create schema bar;
          │               ─── 2. destination
        4 │ create table bar.t(a int);
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_schema_qualifier_in_function_call() {
        assert_snapshot!(goto(r#"
create schema foo;
create function foo.bar() returns int as $$ begin return 1; end; $$ language plpgsql;
select foo$0.bar();
"#), @r"
          ╭▸ 
        2 │ create schema foo;
          │               ─── 2. destination
        3 │ create function foo.bar() returns int as $$ begin return 1; end; $$ language plpgsql;
        4 │ select foo.bar();
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_schema_qualifier_in_function_call_from_clause() {
        assert_snapshot!(goto(r#"
create schema myschema;
create function myschema.get_data() returns table(id int) as $$ begin return query select 1; end; $$ language plpgsql;
select * from myschema$0.get_data();
"#), @r"
          ╭▸ 
        2 │ create schema myschema;
          │               ──────── 2. destination
        3 │ create function myschema.get_data() returns table(id int) as $$ begin return query select 1; end; $$ language plpgsql;
        4 │ select * from myschema.get_data();
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_schema_qualifier_in_select_from() {
        assert_snapshot!(goto("
create schema foo;
create table foo.t(x int);
select x from foo$0.t;
"), @r"
          ╭▸ 
        2 │ create schema foo;
          │               ─── 2. destination
        3 │ create table foo.t(x int);
        4 │ select x from foo.t;
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_qualified_column_table() {
        assert_snapshot!(goto("
create table t(a int);
select t$0.a from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int);
          │              ─ 2. destination
        3 │ select t.a from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_qualified_column_column() {
        assert_snapshot!(goto("
create table t(a int);
select t.a$0 from t;
"), @r"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ select t.a from t;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_three_part_qualified_column_schema() {
        assert_snapshot!(goto("
create schema foo;
create table foo.t(a int);
select foo$0.t.a from t;
"), @r"
          ╭▸ 
        2 │ create schema foo;
          │               ─── 2. destination
        3 │ create table foo.t(a int);
        4 │ select foo.t.a from t;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_three_part_qualified_column_table() {
        assert_snapshot!(goto("
create schema foo;
create table foo.t(a int);
select foo.t$0.a from t;
"), @r"
          ╭▸ 
        3 │ create table foo.t(a int);
          │                  ─ 2. destination
        4 │ select foo.t.a from t;
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_three_part_qualified_column_column() {
        assert_snapshot!(goto("
create schema foo;
create table foo.t(a int);
select foo.t.a$0 from t;
"), @r"
          ╭▸ 
        3 │ create table foo.t(a int);
          │                    ─ 2. destination
        4 │ select foo.t.a from t;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_values_column1() {
        assert_snapshot!(goto("
with t as (
    values (1, 2), (3, 4)
)
select column1$0, column2 from t;
"), @r"
          ╭▸ 
        3 │     values (1, 2), (3, 4)
          │             ─ 2. destination
        4 │ )
        5 │ select column1, column2 from t;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_values_column2() {
        assert_snapshot!(goto("
with t as (
    values (1, 2), (3, 4)
)
select column1, column2$0 from t;
"), @r"
          ╭▸ 
        3 │     values (1, 2), (3, 4)
          │                ─ 2. destination
        4 │ )
        5 │ select column1, column2 from t;
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_values_single_column() {
        assert_snapshot!(goto("
with t as (
    values (1), (2), (3)
)
select column1$0 from t;
"), @r"
          ╭▸ 
        3 │     values (1), (2), (3)
          │             ─ 2. destination
        4 │ )
        5 │ select column1 from t;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_values_multiple_rows() {
        assert_snapshot!(goto("
with t as (
    values
        (1, 2, 3),
        (4, 5, 6),
        (7, 8, 9)
)
select column3$0 from t;
"), @r"
          ╭▸ 
        4 │         (1, 2, 3),
          │                ─ 2. destination
          ‡
        8 │ select column3 from t;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_values_uppercase_column_names() {
        assert_snapshot!(goto("
with t as (
    values (1, 2), (3, 4)
)
select COLUMN1$0, COLUMN2 from t;
"), @r"
          ╭▸ 
        3 │     values (1, 2), (3, 4)
          │             ─ 2. destination
        4 │ )
        5 │ select COLUMN1, COLUMN2 from t;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_qualified_column_with_schema_in_from_table() {
        assert_snapshot!(goto("
create table foo.t(a int, b int);
select t$0.a from foo.t;
"), @r"
          ╭▸ 
        2 │ create table foo.t(a int, b int);
          │                  ─ 2. destination
        3 │ select t.a from foo.t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_qualified_column_with_schema_in_from_column() {
        assert_snapshot!(goto("
create table foo.t(a int, b int);
select t.a$0 from foo.t;
"), @r"
          ╭▸ 
        2 │ create table foo.t(a int, b int);
          │                    ─ 2. destination
        3 │ select t.a from foo.t;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_union_all_column() {
        assert_snapshot!(goto("
with t as (
    select 1 as a, 2 as b
    union all
    select 3, 4
)
select a$0, b from t;
"), @r"
          ╭▸ 
        3 │     select 1 as a, 2 as b
          │                 ─ 2. destination
          ‡
        7 │ select a, b from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_union_all_column_second() {
        assert_snapshot!(goto("
with t as (
    select 1 as a, 2 as b
    union all
    select 3, 4
)
select a, b$0 from t;
"), @r"
          ╭▸ 
        3 │     select 1 as a, 2 as b
          │                         ─ 2. destination
          ‡
        7 │ select a, b from t;
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_union_column() {
        assert_snapshot!(goto("
with t as (
    select 1 as a, 2 as b
    union
    select 3, 4
)
select a$0 from t;
"), @r"
          ╭▸ 
        3 │     select 1 as a, 2 as b
          │                 ─ 2. destination
          ‡
        7 │ select a from t;
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_aggregate() {
        assert_snapshot!(goto("
create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
drop aggregate myavg$0(int);
"), @r"
          ╭▸ 
        2 │ create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
          │                  ───── 2. destination
        3 │ drop aggregate myavg(int);
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_aggregate_with_schema() {
        assert_snapshot!(goto("
set search_path to public;
create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
drop aggregate public.myavg$0(int);
"), @r"
          ╭▸ 
        3 │ create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
          │                  ───── 2. destination
        4 │ drop aggregate public.myavg(int);
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_aggregate_defined_after() {
        assert_snapshot!(goto("
drop aggregate myavg$0(int);
create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
"), @r"
          ╭▸ 
        2 │ drop aggregate myavg(int);
          │                    ─ 1. source
        3 │ create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
          ╰╴                 ───── 2. destination
        ");
    }

    #[test]
    fn goto_aggregate_definition_returns_self() {
        assert_snapshot!(goto("
create aggregate myavg$0(int) (sfunc = int4_avg_accum, stype = _int8);
"), @r"
          ╭▸ 
        2 │ create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
          │                  ┬───┬
          │                  │   │
          │                  │   1. source
          ╰╴                 2. destination
        ");
    }

    #[test]
    fn goto_drop_aggregate_with_search_path() {
        assert_snapshot!(goto("
create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
set search_path to bar;
create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
set search_path to default;
drop aggregate myavg$0(int);
"), @r"
          ╭▸ 
        2 │ create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
          │                  ───── 2. destination
          ‡
        6 │ drop aggregate myavg(int);
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_aggregate_multiple() {
        assert_snapshot!(goto("
create aggregate avg1(int) (sfunc = int4_avg_accum, stype = _int8);
create aggregate avg2(int) (sfunc = int4_avg_accum, stype = _int8);
drop aggregate avg1(int), avg2$0(int);
"), @r"
          ╭▸ 
        3 │ create aggregate avg2(int) (sfunc = int4_avg_accum, stype = _int8);
          │                  ──── 2. destination
        4 │ drop aggregate avg1(int), avg2(int);
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_aggregate_overloaded() {
        assert_snapshot!(goto("
create aggregate sum(complex) (sfunc = complex_add, stype = complex, initcond = '(0,0)');
create aggregate sum(bigint) (sfunc = bigint_add, stype = bigint, initcond = '0');
drop aggregate sum$0(complex);
"), @r"
          ╭▸ 
        2 │ create aggregate sum(complex) (sfunc = complex_add, stype = complex, initcond = '(0,0)');
          │                  ─── 2. destination
        3 │ create aggregate sum(bigint) (sfunc = bigint_add, stype = bigint, initcond = '0');
        4 │ drop aggregate sum(complex);
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_aggregate_second_overload() {
        assert_snapshot!(goto("
create aggregate sum(complex) (sfunc = complex_add, stype = complex, initcond = '(0,0)');
create aggregate sum(bigint) (sfunc = bigint_add, stype = bigint, initcond = '0');
drop aggregate sum$0(bigint);
"), @r"
          ╭▸ 
        3 │ create aggregate sum(bigint) (sfunc = bigint_add, stype = bigint, initcond = '0');
          │                  ─── 2. destination
        4 │ drop aggregate sum(bigint);
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_function() {
        assert_snapshot!(goto("
create function foo() returns int as $$ select 1 $$ language sql;
drop routine foo$0();
"), @r"
          ╭▸ 
        2 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        3 │ drop routine foo();
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_aggregate() {
        assert_snapshot!(goto("
create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
drop routine myavg$0(int);
"), @r"
          ╭▸ 
        2 │ create aggregate myavg(int) (sfunc = int4_avg_accum, stype = _int8);
          │                  ───── 2. destination
        3 │ drop routine myavg(int);
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_with_schema() {
        assert_snapshot!(goto("
set search_path to public;
create function foo() returns int as $$ select 1 $$ language sql;
drop routine public.foo$0();
"), @r"
          ╭▸ 
        3 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        4 │ drop routine public.foo();
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_defined_after() {
        assert_snapshot!(goto("
drop routine foo$0();
create function foo() returns int as $$ select 1 $$ language sql;
"), @r"
          ╭▸ 
        2 │ drop routine foo();
          │                ─ 1. source
        3 │ create function foo() returns int as $$ select 1 $$ language sql;
          ╰╴                ─── 2. destination
        ");
    }

    #[test]
    fn goto_drop_routine_with_search_path() {
        assert_snapshot!(goto("
create function foo() returns int as $$ select 1 $$ language sql;
set search_path to bar;
create function foo() returns int as $$ select 1 $$ language sql;
set search_path to default;
drop routine foo$0();
"), @r"
          ╭▸ 
        2 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
          ‡
        6 │ drop routine foo();
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_overloaded() {
        assert_snapshot!(goto("
create function add(complex) returns complex as $$ select null $$ language sql;
create function add(bigint) returns bigint as $$ select 1 $$ language sql;
drop routine add$0(complex);
"), @r"
          ╭▸ 
        2 │ create function add(complex) returns complex as $$ select null $$ language sql;
          │                 ─── 2. destination
        3 │ create function add(bigint) returns bigint as $$ select 1 $$ language sql;
        4 │ drop routine add(complex);
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_second_overload() {
        assert_snapshot!(goto("
create function add(complex) returns complex as $$ select null $$ language sql;
create function add(bigint) returns bigint as $$ select 1 $$ language sql;
drop routine add$0(bigint);
"), @r"
          ╭▸ 
        3 │ create function add(bigint) returns bigint as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        4 │ drop routine add(bigint);
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_aggregate_overloaded() {
        assert_snapshot!(goto("
create aggregate sum(complex) (sfunc = complex_add, stype = complex, initcond = '(0,0)');
create aggregate sum(bigint) (sfunc = bigint_add, stype = bigint, initcond = '0');
drop routine sum$0(complex);
"), @r"
          ╭▸ 
        2 │ create aggregate sum(complex) (sfunc = complex_add, stype = complex, initcond = '(0,0)');
          │                  ─── 2. destination
        3 │ create aggregate sum(bigint) (sfunc = bigint_add, stype = bigint, initcond = '0');
        4 │ drop routine sum(complex);
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_multiple() {
        assert_snapshot!(goto("
create function foo() returns int as $$ select 1 $$ language sql;
create function bar() returns int as $$ select 1 $$ language sql;
drop routine foo(), bar$0();
"), @r"
          ╭▸ 
        3 │ create function bar() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        4 │ drop routine foo(), bar();
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_procedure() {
        assert_snapshot!(goto("
create procedure foo() language sql as $$ select 1 $$;
drop procedure foo$0();
"), @r"
          ╭▸ 
        2 │ create procedure foo() language sql as $$ select 1 $$;
          │                  ─── 2. destination
        3 │ drop procedure foo();
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_procedure_with_schema() {
        assert_snapshot!(goto("
set search_path to public;
create procedure foo() language sql as $$ select 1 $$;
drop procedure public.foo$0();
"), @r"
          ╭▸ 
        3 │ create procedure foo() language sql as $$ select 1 $$;
          │                  ─── 2. destination
        4 │ drop procedure public.foo();
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_procedure_defined_after() {
        assert_snapshot!(goto("
drop procedure foo$0();
create procedure foo() language sql as $$ select 1 $$;
"), @r"
          ╭▸ 
        2 │ drop procedure foo();
          │                  ─ 1. source
        3 │ create procedure foo() language sql as $$ select 1 $$;
          ╰╴                 ─── 2. destination
        ");
    }

    #[test]
    fn goto_drop_procedure_with_search_path() {
        assert_snapshot!(goto("
create procedure foo() language sql as $$ select 1 $$;
set search_path to bar;
create procedure foo() language sql as $$ select 1 $$;
set search_path to default;
drop procedure foo$0();
"), @r"
          ╭▸ 
        2 │ create procedure foo() language sql as $$ select 1 $$;
          │                  ─── 2. destination
          ‡
        6 │ drop procedure foo();
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_procedure_overloaded() {
        assert_snapshot!(goto("
create procedure add(complex) language sql as $$ select null $$;
create procedure add(bigint) language sql as $$ select 1 $$;
drop procedure add$0(complex);
"), @r"
          ╭▸ 
        2 │ create procedure add(complex) language sql as $$ select null $$;
          │                  ─── 2. destination
        3 │ create procedure add(bigint) language sql as $$ select 1 $$;
        4 │ drop procedure add(complex);
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_procedure_second_overload() {
        assert_snapshot!(goto("
create procedure add(complex) language sql as $$ select null $$;
create procedure add(bigint) language sql as $$ select 1 $$;
drop procedure add$0(bigint);
"), @r"
          ╭▸ 
        3 │ create procedure add(bigint) language sql as $$ select 1 $$;
          │                  ─── 2. destination
        4 │ drop procedure add(bigint);
          ╰╴                 ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_procedure_multiple() {
        assert_snapshot!(goto("
create procedure foo() language sql as $$ select 1 $$;
create procedure bar() language sql as $$ select 1 $$;
drop procedure foo(), bar$0();
"), @r"
          ╭▸ 
        3 │ create procedure bar() language sql as $$ select 1 $$;
          │                  ─── 2. destination
        4 │ drop procedure foo(), bar();
          ╰╴                        ─ 1. source
        ");
    }

    #[test]
    fn goto_procedure_definition_returns_self() {
        assert_snapshot!(goto("
create procedure foo$0() language sql as $$ select 1 $$;
"), @r"
          ╭▸ 
        2 │ create procedure foo() language sql as $$ select 1 $$;
          │                  ┬─┬
          │                  │ │
          │                  │ 1. source
          ╰╴                 2. destination
        ");
    }

    #[test]
    fn goto_call_procedure() {
        assert_snapshot!(goto("
create procedure foo() language sql as $$ select 1 $$;
call foo$0();
"), @r"
          ╭▸ 
        2 │ create procedure foo() language sql as $$ select 1 $$;
          │                  ─── 2. destination
        3 │ call foo();
          ╰╴       ─ 1. source
        ");
    }

    #[test]
    fn goto_call_procedure_with_schema() {
        assert_snapshot!(goto("
create procedure public.foo() language sql as $$ select 1 $$;
call public.foo$0();
"), @r"
          ╭▸ 
        2 │ create procedure public.foo() language sql as $$ select 1 $$;
          │                         ─── 2. destination
        3 │ call public.foo();
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_call_procedure_with_search_path() {
        assert_snapshot!(goto("
set search_path to myschema;
create procedure foo() language sql as $$ select 1 $$;
call myschema.foo$0();
"), @r"
          ╭▸ 
        3 │ create procedure foo() language sql as $$ select 1 $$;
          │                  ─── 2. destination
        4 │ call myschema.foo();
          ╰╴                ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_procedure() {
        assert_snapshot!(goto("
create procedure foo() language sql as $$ select 1 $$;
drop routine foo$0();
"), @r"
          ╭▸ 
        2 │ create procedure foo() language sql as $$ select 1 $$;
          │                  ─── 2. destination
        3 │ drop routine foo();
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_prefers_function_over_procedure() {
        assert_snapshot!(goto("
create function foo() returns int as $$ select 1 $$ language sql;
create procedure foo() language sql as $$ select 1 $$;
drop routine foo$0();
"), @r"
          ╭▸ 
        2 │ create function foo() returns int as $$ select 1 $$ language sql;
          │                 ─── 2. destination
        3 │ create procedure foo() language sql as $$ select 1 $$;
        4 │ drop routine foo();
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_drop_routine_prefers_aggregate_over_procedure() {
        assert_snapshot!(goto("
create aggregate foo(int) (sfunc = int4_avg_accum, stype = _int8);
create procedure foo(int) language sql as $$ select 1 $$;
drop routine foo$0(int);
"), @r"
          ╭▸ 
        2 │ create aggregate foo(int) (sfunc = int4_avg_accum, stype = _int8);
          │                  ─── 2. destination
        3 │ create procedure foo(int) language sql as $$ select 1 $$;
        4 │ drop routine foo(int);
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_table_alias_in_qualified_column() {
        assert_snapshot!(goto("
create table t(a int8, b text);
select f$0.a from t as f;
"), @r"
          ╭▸ 
        3 │ select f.a from t as f;
          ╰╴       ─ 1. source   ─ 2. destination
        ");
    }

    #[test]
    fn goto_column_through_table_alias() {
        assert_snapshot!(goto("
create table t(a int8, b text);
select f.a$0 from t as f;
"), @r"
          ╭▸ 
        2 │ create table t(a int8, b text);
          │                ─ 2. destination
        3 │ select f.a from t as f;
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_cte_alias_renamed_column() {
        assert_snapshot!(goto("
with t as (select 1 a, 2 b)
select f.x$0 from t as f(x);
"), @r"
          ╭▸ 
        3 │ select f.x from t as f(x);
          ╰╴         ─ 1. source   ─ 2. destination
        ");
    }

    #[test]
    fn goto_cte_alias_unrenamed_column() {
        assert_snapshot!(goto("
with t as (select 1 a, 2 b)
select f.b$0 from t as f(x);
"), @r"
          ╭▸ 
        2 │ with t as (select 1 a, 2 b)
          │                          ─ 2. destination
        3 │ select f.b from t as f(x);
          ╰╴         ─ 1. source
        ");
    }

    #[test]
    fn goto_join_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
create table messages(id int, user_id int, message text);
select * from users join messages$0 on users.id = messages.user_id;
"), @r"
          ╭▸ 
        3 │ create table messages(id int, user_id int, message text);
          │              ──────── 2. destination
        4 │ select * from users join messages on users.id = messages.user_id;
          ╰╴                                ─ 1. source
        ");
    }

    #[test]
    fn goto_join_qualified_column_from_joined_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
create table messages(id int, user_id int, message text);
select messages.user_id$0 from users join messages on users.id = messages.user_id;
"), @r"
          ╭▸ 
        3 │ create table messages(id int, user_id int, message text);
          │                               ─────── 2. destination
        4 │ select messages.user_id from users join messages on users.id = messages.user_id;
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_join_qualified_column_from_base_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
create table messages(id int, user_id int, message text);
select users.id$0 from users join messages on users.id = messages.user_id;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                    ── 2. destination
        3 │ create table messages(id int, user_id int, message text);
        4 │ select users.id from users join messages on users.id = messages.user_id;
          ╰╴              ─ 1. source
        ");
    }

    #[test]
    fn goto_join_multiple_joins() {
        assert_snapshot!(goto("
create table users(id int, name text);
create table messages(id int, user_id int, message text);
create table comments(id int, message_id int, text text);
select comments.text$0 from users
  join messages on users.id = messages.user_id
  join comments on messages.id = comments.message_id;
"), @r"
          ╭▸ 
        4 │ create table comments(id int, message_id int, text text);
          │                                               ──── 2. destination
        5 │ select comments.text from users
          ╰╴                   ─ 1. source
        ");
    }

    #[test]
    fn goto_join_with_aliases() {
        assert_snapshot!(goto("
create table users(id int, name text);
create table messages(id int, user_id int, message text);
select m.message$0 from users as u join messages as m on u.id = m.user_id;
"), @r"
          ╭▸ 
        3 │ create table messages(id int, user_id int, message text);
          │                                            ─────── 2. destination
        4 │ select m.message from users as u join messages as m on u.id = m.user_id;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_join_unqualified_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
create table messages(id int, user_id int, message text);
select message$0 from users join messages on users.id = messages.user_id;
"), @r"
          ╭▸ 
        3 │ create table messages(id int, user_id int, message text);
          │                                            ─────── 2. destination
        4 │ select message from users join messages on users.id = messages.user_id;
          ╰╴             ─ 1. source
        ");
    }

    #[test]
    fn goto_join_with_many_tables() {
        assert_snapshot!(goto("
create table users(id int, email text);
create table messages(id int, user_id int, message text);
create table logins(id int, user_id int, at timestamptz);
create table posts(id int, user_id int, post text);

select post$0 
  from users
    join messages 
      on users.id = messages.user_id
      join logins
        on users.id = logins.user_id
        join posts
          on users.id = posts.user_id
"), @r"
          ╭▸ 
        5 │ create table posts(id int, user_id int, post text);
          │                                         ──── 2. destination
        6 │
        7 │ select post 
          ╰╴          ─ 1. source
        ");
    }

    #[test]
    fn goto_join_with_schema() {
        assert_snapshot!(goto("
create schema foo;
create table foo.users(id int, email text);
create table foo.messages(id int, user_id int, message text);
select foo.messages.message$0 from foo.users join foo.messages on foo.users.id = foo.messages.user_id;
"), @r"
          ╭▸ 
        4 │ create table foo.messages(id int, user_id int, message text);
          │                                                ─────── 2. destination
        5 │ select foo.messages.message from foo.users join foo.messages on foo.users.id = foo.messages.user_id;
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_join_left_join() {
        assert_snapshot!(goto("
create table users(id int, email text);
create table messages(id int, user_id int, message text);
select messages.message$0 from users left join messages on users.id = messages.user_id;
"), @r"
          ╭▸ 
        3 │ create table messages(id int, user_id int, message text);
          │                                            ─────── 2. destination
        4 │ select messages.message from users left join messages on users.id = messages.user_id;
          ╰╴                      ─ 1. source
        ");
    }

    #[test]
    fn goto_join_on_table_qualifier() {
        assert_snapshot!(goto("
create table t(a int);
create table u(a int);
select * from t join u on u$0.a = t.a;
"), @r"
          ╭▸ 
        3 │ create table u(a int);
          │              ─ 2. destination
        4 │ select * from t join u on u.a = t.a;
          ╰╴                          ─ 1. source
        ");
    }

    #[test]
    fn goto_join_on_column() {
        assert_snapshot!(goto("
create table t(a int);
create table u(a int);
select * from t join u on u.a$0 = t.a;
"), @r"
          ╭▸ 
        3 │ create table u(a int);
          │                ─ 2. destination
        4 │ select * from t join u on u.a = t.a;
          ╰╴                            ─ 1. source
        ");
    }

    #[test]
    fn goto_join_using_column() {
        assert_snapshot!(goto("
create table t(a int);
create table u(a int);
select * from t join u using (a$0);
"), @r"
          ╭▸ 
        2 │ create table t(a int);
          │                ─ 2. destination
        3 │ create table u(a int);
          │                ─ 3. destination
        4 │ select * from t join u using (a);
          ╰╴                              ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_select_cte_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
with new_data as (
    select 1 as id, 'test@example.com' as email
)
insert into users (id, email)
select id$0, email from new_data;
"), @r"
          ╭▸ 
        4 │     select 1 as id, 'test@example.com' as email
          │                 ── 2. destination
          ‡
        7 │ select id, email from new_data;
          ╰╴        ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_select_cte_column_second() {
        assert_snapshot!(goto("
create table users(id int, email text);
with new_data as (
    select 1 as id, 'test@example.com' as email
)
insert into users (id, email)
select id, email$0 from new_data;
"), @r"
          ╭▸ 
        4 │     select 1 as id, 'test@example.com' as email
          │                                           ───── 2. destination
          ‡
        7 │ select id, email from new_data;
          ╰╴               ─ 1. source
        ");
    }

    #[test]
    fn goto_insert_select_cte_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
with new_data as (
    select 1 as id, 'test@example.com' as email
)
insert into users (id, email)
select id, email from new_data$0;
"), @r"
          ╭▸ 
        3 │ with new_data as (
          │      ──────── 2. destination
          ‡
        7 │ select id, email from new_data;
          ╰╴                             ─ 1. source
        ");
    }

    #[test]
    fn goto_delete_cte_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
with old_data as (
    select 1 as id
)
delete from users where id in (select id$0 from old_data);
"), @r"
          ╭▸ 
        4 │     select 1 as id
          │                 ── 2. destination
        5 │ )
        6 │ delete from users where id in (select id from old_data);
          ╰╴                                       ─ 1. source
        ");
    }

    #[test]
    fn goto_update_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
update users$0 set email = 'new@example.com';
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │              ───── 2. destination
        3 │ update users set email = 'new@example.com';
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_update_table_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
update public.users$0 set email = 'new@example.com';
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                     ───── 2. destination
        3 │ update public.users set email = 'new@example.com';
          ╰╴                  ─ 1. source
        ");
    }

    #[test]
    fn goto_update_table_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.users(id int, email text);
update users$0 set email = 'new@example.com';
"), @r"
          ╭▸ 
        3 │ create table foo.users(id int, email text);
          │                  ───── 2. destination
        4 │ update users set email = 'new@example.com';
          ╰╴           ─ 1. source
        ");
    }

    #[test]
    fn goto_update_where_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
update users set email = 'new@example.com' where id$0 = 1;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                    ── 2. destination
        3 │ update users set email = 'new@example.com' where id = 1;
          ╰╴                                                  ─ 1. source
        ");
    }

    #[test]
    fn goto_update_where_column_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
update public.users set email = 'new@example.com' where id$0 = 1;
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                           ── 2. destination
        3 │ update public.users set email = 'new@example.com' where id = 1;
          ╰╴                                                         ─ 1. source
        ");
    }

    #[test]
    fn goto_update_where_column_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.users(id int, email text);
update users set email = 'new@example.com' where id$0 = 1;
"), @r"
          ╭▸ 
        3 │ create table foo.users(id int, email text);
          │                        ── 2. destination
        4 │ update users set email = 'new@example.com' where id = 1;
          ╰╴                                                  ─ 1. source
        ");
    }

    #[test]
    fn goto_update_set_column() {
        assert_snapshot!(goto("
create table users(id int, email text);
update users set email$0 = 'new@example.com' where id = 1;
"), @r"
          ╭▸ 
        2 │ create table users(id int, email text);
          │                            ───── 2. destination
        3 │ update users set email = 'new@example.com' where id = 1;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_update_set_column_with_schema() {
        assert_snapshot!(goto("
create table public.users(id int, email text);
update public.users set email$0 = 'new@example.com' where id = 1;
"), @r"
          ╭▸ 
        2 │ create table public.users(id int, email text);
          │                                   ───── 2. destination
        3 │ update public.users set email = 'new@example.com' where id = 1;
          ╰╴                            ─ 1. source
        ");
    }

    #[test]
    fn goto_update_set_column_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table foo.users(id int, email text);
update users set email$0 = 'new@example.com' where id = 1;
"), @r"
          ╭▸ 
        3 │ create table foo.users(id int, email text);
          │                                ───── 2. destination
        4 │ update users set email = 'new@example.com' where id = 1;
          ╰╴                     ─ 1. source
        ");
    }

    #[test]
    fn goto_update_from_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
create table messages(id int, user_id int, email text);
update users set email = messages.email from messages$0 where users.id = messages.user_id;
"), @r"
          ╭▸ 
        3 │ create table messages(id int, user_id int, email text);
          │              ──────── 2. destination
        4 │ update users set email = messages.email from messages where users.id = messages.user_id;
          ╰╴                                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_update_from_table_with_schema() {
        assert_snapshot!(goto("
create table users(id int, email text);
create table public.messages(id int, user_id int, email text);
update users set email = messages.email from public.messages$0 where users.id = messages.user_id;
"), @r"
          ╭▸ 
        3 │ create table public.messages(id int, user_id int, email text);
          │                     ──────── 2. destination
        4 │ update users set email = messages.email from public.messages where users.id = messages.user_id;
          ╰╴                                                           ─ 1. source
        ");
    }

    #[test]
    fn goto_update_from_table_with_search_path() {
        assert_snapshot!(goto("
set search_path to foo;
create table users(id int, email text);
create table foo.messages(id int, user_id int, email text);
update users set email = messages.email from messages$0 where users.id = messages.user_id;
"), @r"
          ╭▸ 
        4 │ create table foo.messages(id int, user_id int, email text);
          │                  ──────── 2. destination
        5 │ update users set email = messages.email from messages where users.id = messages.user_id;
          ╰╴                                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_update_with_cte_table() {
        assert_snapshot!(goto("
create table users(id int, email text);
with new_data as (
    select 1 as id, 'new@example.com' as email
)
update users set email = new_data.email from new_data$0 where users.id = new_data.id;
"), @r"
          ╭▸ 
        3 │ with new_data as (
          │      ──────── 2. destination
          ‡
        6 │ update users set email = new_data.email from new_data where users.id = new_data.id;
          ╰╴                                                    ─ 1. source
        ");
    }

    #[test]
    fn goto_update_with_cte_column_in_set() {
        assert_snapshot!(goto("
create table users(id int, email text);
with new_data as (
    select 1 as id, 'new@example.com' as email
)
update users set email = new_data.email$0 from new_data where users.id = new_data.id;
"), @r"
          ╭▸ 
        4 │     select 1 as id, 'new@example.com' as email
          │                                          ───── 2. destination
        5 │ )
        6 │ update users set email = new_data.email from new_data where users.id = new_data.id;
          ╰╴                                      ─ 1. source
        ");
    }

    #[test]
    fn goto_update_with_cte_column_in_where() {
        assert_snapshot!(goto("
create table users(id int, email text);
with new_data as (
    select 1 as id, 'new@example.com' as email
)
update users set email = new_data.email from new_data where new_data.id$0 = users.id;
"), @r"
          ╭▸ 
        4 │     select 1 as id, 'new@example.com' as email
          │                 ── 2. destination
        5 │ )
        6 │ update users set email = new_data.email from new_data where new_data.id = users.id;
          ╰╴                                                                      ─ 1. source
        ");
    }

    #[test]
    fn goto_update_with_cte_values() {
        assert_snapshot!(goto("
create table users(id int, email text);
with new_data as (
    values (1, 'new@example.com')
)
update users set email = new_data.column2$0 from new_data where users.id = new_data.column1;
"), @r"
          ╭▸ 
        4 │     values (1, 'new@example.com')
          │                ───────────────── 2. destination
        5 │ )
        6 │ update users set email = new_data.column2 from new_data where users.id = new_data.column1;
          ╰╴                                        ─ 1. source
        ");
    }
}
