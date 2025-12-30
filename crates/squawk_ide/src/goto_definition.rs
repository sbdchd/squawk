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
}
