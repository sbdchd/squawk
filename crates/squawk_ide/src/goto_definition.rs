use crate::binder;
use crate::offsets::token_from_offset;
use crate::resolve;
use rowan::{TextRange, TextSize};
use squawk_syntax::{
    SyntaxKind,
    ast::{self, AstNode},
};

pub fn goto_definition(file: ast::SourceFile, offset: TextSize) -> Option<TextRange> {
    let token = token_from_offset(&file, offset)?;
    let parent = token.parent()?;

    // goto def on case exprs
    if (token.kind() == SyntaxKind::WHEN_KW && parent.kind() == SyntaxKind::WHEN_CLAUSE)
        || (token.kind() == SyntaxKind::ELSE_KW && parent.kind() == SyntaxKind::ELSE_CLAUSE)
        || (token.kind() == SyntaxKind::END_KW && parent.kind() == SyntaxKind::CASE_EXPR)
    {
        for parent in token.parent_ancestors() {
            if let Some(case_expr) = ast::CaseExpr::cast(parent)
                && let Some(case_token) = case_expr.case_token()
            {
                return Some(case_token.text_range());
            }
        }
    }

    // goto def on COMMIT -> BEGIN/START TRANSACTION
    if ast::Commit::can_cast(parent.kind()) {
        if let Some(begin_range) = find_preceding_begin(&file, token.text_range().start()) {
            return Some(begin_range);
        }
    }

    // goto def on ROLLBACK -> BEGIN/START TRANSACTION
    if ast::Rollback::can_cast(parent.kind()) {
        if let Some(begin_range) = find_preceding_begin(&file, token.text_range().start()) {
            return Some(begin_range);
        }
    }

    // goto def on BEGIN/START TRANSACTION -> COMMIT or ROLLBACK
    if ast::Begin::can_cast(parent.kind()) {
        if let Some(end_range) = find_following_commit_or_rollback(&file, token.text_range().end())
        {
            return Some(end_range);
        }
    }

    if let Some(name) = ast::Name::cast(parent.clone()) {
        return Some(name.syntax().text_range());
    }

    if let Some(name_ref) = ast::NameRef::cast(parent.clone()) {
        let binder_output = binder::bind(&file);
        if let Some(ptr) = resolve::resolve_name_ref(&binder_output, &name_ref) {
            let node = ptr.to_node(file.syntax());
            return Some(node.text_range());
        }
    }

    return None;
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
        if let Some(result) = goto_definition(file, offset) {
            let offset: usize = offset.into();
            let group = Level::INFO.primary_title("definition").element(
                Snippet::source(&sql)
                    .fold(true)
                    .annotation(
                        AnnotationKind::Context
                            .span(result.into())
                            .label("2. destination"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(offset..offset + 1)
                            .label("1. source"),
                    ),
            );
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
}
