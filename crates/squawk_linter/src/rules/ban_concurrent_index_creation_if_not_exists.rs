use std::iter;

use rustc_hash::FxHashSet;
use smallvec::SmallVec;
use squawk_diagnostics::{Edit, Fix};
use squawk_rowan::TextSize;
use squawk_syntax::{
    SyntaxKind, SyntaxNode, SyntaxToken,
    ast::{self, AstNode},
};

use crate::{
    Linter, Rule, Violation,
    name::{Name, path_name},
};

#[derive(Default)]
pub(crate) struct DroppedIndexes {
    names: FxHashSet<SmallVec<[Name; 2]>>,
}

impl DroppedIndexes {
    pub(crate) fn update(&mut self, stmt: &ast::Stmt) {
        if let ast::Stmt::DropIndex(drop_index) = stmt {
            for path in drop_index.index_refs().filter_map(|index| index.path_ref()) {
                self.names.insert(path_name(path.segments()));
            }
        }
    }

    pub(crate) fn take(&mut self, create_index: &ast::CreateIndex) -> bool {
        index_name(create_index).is_some_and(|name| self.names.remove(&name))
    }
}

fn table_schema(create_index: &ast::CreateIndex) -> Option<ast::PathSegmentRef> {
    create_index
        .table_relation_name()?
        .table_name_ref()?
        .path_ref()?
        .qualifier()?
        .segment()
}

// An index is always created in the schema of its table, so that is the
// schema a matching `drop index` has to name.
fn index_name(create_index: &ast::CreateIndex) -> Option<SmallVec<[Name; 2]>> {
    let index = create_index.index()?.path()?.segment()?;
    Some(path_name(
        table_schema(create_index)
            .map(ast::AnyName::PathSegmentRef)
            .into_iter()
            .chain(iter::once(ast::AnyName::PathSegment(index))),
    ))
}

fn line_breaks(text: &str) -> usize {
    text.matches('\n').count() + text.matches('\r').count() - text.matches("\r\n").count()
}

fn starts_line(token: &SyntaxToken) -> bool {
    match token.prev_token() {
        None => true,
        Some(prev) => {
            prev.kind() == SyntaxKind::WHITESPACE
                && (line_breaks(prev.text()) > 0 || prev.prev_token().is_none())
        }
    }
}

// Comment lines directly above a statement, like `squawk-ignore`, belong to
// it, so the drop goes above them.
fn insert_position(stmt: &SyntaxNode) -> Option<(TextSize, String)> {
    let first = stmt.first_token()?;
    let mut at = first.text_range().start();
    let mut before = first.prev_token();
    while let Some(whitespace) = &before
        && whitespace.kind() == SyntaxKind::WHITESPACE
        && line_breaks(whitespace.text()) == 1
        && let Some(comment) = whitespace.prev_token()
        && comment.kind() == SyntaxKind::COMMENT
        && starts_line(&comment)
    {
        at = comment.text_range().start();
        before = comment.prev_token();
    }
    let indent = match &before {
        Some(whitespace) if whitespace.kind() == SyntaxKind::WHITESPACE => {
            let text = whitespace.text();
            match text.rfind(['\n', '\r']) {
                Some(i) => text[i + 1..].to_owned(),
                None if whitespace.prev_token().is_none() => text.to_owned(),
                None => String::new(),
            }
        }
        _ => String::new(),
    };
    Some((at, indent))
}

fn drop_index_fix(
    file: &ast::SourceFile,
    create_index: &ast::CreateIndex,
    assume_in_transaction: bool,
) -> Option<Fix> {
    // Migration tools that wrap each file in a transaction run a file with a
    // single `create index concurrently` outside of it, but wrap a file with a
    // second statement, which makes `concurrently` fail.
    if assume_in_transaction && file.stmts().nth(1).is_none() {
        return None;
    }
    let index = create_index.index()?.path()?.segment()?;
    let schema = table_schema(create_index)
        .map(|schema| format!("{}.", schema.syntax().text()))
        .unwrap_or_default();
    let (at, indent) = insert_position(create_index.syntax())?;
    let edit = Edit::insert(
        format!(
            "drop index concurrently if exists {schema}{};{}{indent}",
            index.syntax().text(),
            file.line_ending().as_str()
        ),
        at,
    );
    Some(Fix::new(
        "Add `drop index concurrently if exists`",
        vec![edit],
    ))
}

pub(crate) fn ban_concurrent_index_creation_if_not_exists(
    ctx: &mut Linter,
    file: &ast::SourceFile,
) {
    let mut dropped_indexes = DroppedIndexes::default();
    for stmt in file.stmts() {
        dropped_indexes.update(&stmt);
        if let ast::Stmt::CreateIndex(create_index) = stmt
            && create_index.concurrently_token().is_some()
            && let Some(if_not_exists) = create_index.if_not_exists()
            && !dropped_indexes.take(&create_index)
        {
            ctx.report(
                Violation::for_node(
                    Rule::BanConcurrentIndexCreationIfNotExists,
                    "if `create index concurrently` fails, it leaves behind an invalid index, which `if not exists` skips when the migration is rerun".into(),
                    if_not_exists.syntax(),
                )
                .help("drop the index with `drop index concurrently if exists` before creating it")
                .fix(drop_index_fix(
                    file,
                    &create_index,
                    ctx.settings.assume_in_transaction,
                )),
            );
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::{
        Linter, LinterSettings, Rule,
        test_utils::{
            fix_sql, fix_sql_with, format_violations, lint_errors, lint_errors_with, lint_ok,
        },
    };

    fn assume_in_transaction() -> LinterSettings {
        LinterSettings {
            assume_in_transaction: true,
            ..Default::default()
        }
    }

    #[must_use]
    fn fix(sql: &str) -> String {
        fix_sql(sql, Rule::BanConcurrentIndexCreationIfNotExists)
    }

    #[test]
    fn if_not_exists_err() {
        let sql = "
create index concurrently if not exists email_idx on app_user (email);
";
        assert_snapshot!(lint_errors(sql, Rule::BanConcurrentIndexCreationIfNotExists), @"
        warning[ban-concurrent-index-creation-if-not-exists]: if `create index concurrently` fails, it leaves behind an invalid index, which `if not exists` skips when the migration is rerun
          ╭▸ 
        2 │ create index concurrently if not exists email_idx on app_user (email);
          │                           ━━━━━━━━━━━━━
          │
          ├ help: drop the index with `drop index concurrently if exists` before creating it
          ╭╴
        2 + drop index concurrently if exists email_idx;
          ╰╴
        ");
    }

    #[test]
    fn unique_if_not_exists_err() {
        let sql = r#"
CREATE UNIQUE INDEX CONCURRENTLY IF NOT EXISTS "email_idx" ON "app_user" ("email");
"#;
        assert_snapshot!(lint_errors(sql, Rule::BanConcurrentIndexCreationIfNotExists), @r#"
        warning[ban-concurrent-index-creation-if-not-exists]: if `create index concurrently` fails, it leaves behind an invalid index, which `if not exists` skips when the migration is rerun
          ╭▸ 
        2 │ CREATE UNIQUE INDEX CONCURRENTLY IF NOT EXISTS "email_idx" ON "app_user" ("email");
          │                                  ━━━━━━━━━━━━━
          │
          ├ help: drop the index with `drop index concurrently if exists` before creating it
          ╭╴
        2 + drop index concurrently if exists "email_idx";
          ╰╴
        "#);
    }

    #[test]
    fn drop_after_create_err() {
        let sql = "
create index concurrently if not exists email_idx on app_user (email);
drop index concurrently if exists email_idx;
";
        assert_snapshot!(lint_errors(sql, Rule::BanConcurrentIndexCreationIfNotExists), @"
        warning[ban-concurrent-index-creation-if-not-exists]: if `create index concurrently` fails, it leaves behind an invalid index, which `if not exists` skips when the migration is rerun
          ╭▸ 
        2 │ create index concurrently if not exists email_idx on app_user (email);
          │                           ━━━━━━━━━━━━━
          │
          ├ help: drop the index with `drop index concurrently if exists` before creating it
          ╭╴
        2 + drop index concurrently if exists email_idx;
          ╰╴
        ");
    }

    #[test]
    fn drop_other_index_err() {
        let sql = "
drop index concurrently if exists name_idx;
create index concurrently if not exists email_idx on app_user (email);
";
        assert_snapshot!(lint_errors(sql, Rule::BanConcurrentIndexCreationIfNotExists), @"
        warning[ban-concurrent-index-creation-if-not-exists]: if `create index concurrently` fails, it leaves behind an invalid index, which `if not exists` skips when the migration is rerun
          ╭▸ 
        3 │ create index concurrently if not exists email_idx on app_user (email);
          │                           ━━━━━━━━━━━━━
          │
          ├ help: drop the index with `drop index concurrently if exists` before creating it
          ╭╴
        3 + drop index concurrently if exists email_idx;
          ╰╴
        ");
    }

    #[test]
    fn drop_index_in_other_schema_err() {
        let sql = "
drop index concurrently if exists email_idx;
create index concurrently if not exists email_idx on app.app_user (email);
";
        assert_snapshot!(lint_errors(sql, Rule::BanConcurrentIndexCreationIfNotExists), @"
        warning[ban-concurrent-index-creation-if-not-exists]: if `create index concurrently` fails, it leaves behind an invalid index, which `if not exists` skips when the migration is rerun
          ╭▸ 
        3 │ create index concurrently if not exists email_idx on app.app_user (email);
          │                           ━━━━━━━━━━━━━
          │
          ├ help: drop the index with `drop index concurrently if exists` before creating it
          ╭╴
        3 + drop index concurrently if exists app.email_idx;
          ╰╴
        ");
    }

    #[test]
    fn second_create_after_single_drop_err() {
        let sql = "
drop index concurrently if exists email_idx;
create index concurrently if not exists email_idx on app_user (email);
create index concurrently if not exists email_idx on app_user (email);
";
        assert_snapshot!(lint_errors(sql, Rule::BanConcurrentIndexCreationIfNotExists), @"
        warning[ban-concurrent-index-creation-if-not-exists]: if `create index concurrently` fails, it leaves behind an invalid index, which `if not exists` skips when the migration is rerun
          ╭▸ 
        4 │ create index concurrently if not exists email_idx on app_user (email);
          │                           ━━━━━━━━━━━━━
          │
          ├ help: drop the index with `drop index concurrently if exists` before creating it
          ╭╴
        4 + drop index concurrently if exists email_idx;
          ╰╴
        ");
    }

    #[test]
    fn create_index_concurrently_ok() {
        let sql = "
create index concurrently email_idx on app_user (email);
";
        lint_ok(sql, Rule::BanConcurrentIndexCreationIfNotExists);
    }

    #[test]
    fn create_index_if_not_exists_without_concurrently_ok() {
        let sql = "
create index if not exists email_idx on app_user (email);
";
        lint_ok(sql, Rule::BanConcurrentIndexCreationIfNotExists);
    }

    #[test]
    fn drop_before_create_ok() {
        let sql = "
drop index concurrently if exists email_idx;
create index concurrently email_idx on app_user (email);
";
        lint_ok(sql, Rule::BanConcurrentIndexCreationIfNotExists);
    }

    #[test]
    fn drop_before_create_if_not_exists_ok() {
        let sql = "
drop index concurrently if exists email_idx;
create unique index concurrently if not exists email_idx on app_user (email);
";
        lint_ok(sql, Rule::BanConcurrentIndexCreationIfNotExists);
    }

    #[test]
    fn drop_before_create_mixed_case_ok() {
        let sql = r#"
DROP INDEX CONCURRENTLY IF EXISTS "email_idx";
CREATE INDEX CONCURRENTLY IF NOT EXISTS Email_Idx ON app_user (email);
"#;
        lint_ok(sql, Rule::BanConcurrentIndexCreationIfNotExists);
    }

    #[test]
    fn drop_before_create_schema_qualified_ok() {
        let sql = r#"
drop index concurrently if exists APP."email_idx";
create index concurrently if not exists email_idx on app.app_user (email);
drop index concurrently if exists name_idx;
create index concurrently if not exists name_idx on public.app_user (name);
"#;
        lint_ok(sql, Rule::BanConcurrentIndexCreationIfNotExists);
    }

    #[test]
    fn drop_multiple_before_create_ok() {
        let sql = "
drop index if exists email_idx, name_idx;
create index concurrently if not exists email_idx on app_user (email);
create index concurrently if not exists name_idx on app_user (name);
";
        lint_ok(sql, Rule::BanConcurrentIndexCreationIfNotExists);
    }

    #[test]
    fn fix_add_drop_index() {
        assert_snapshot!(fix(r#"
create index concurrently if not exists email_idx on app_user (email);
CREATE UNIQUE INDEX CONCURRENTLY IF NOT EXISTS "Name_Idx" ON app.app_user (name);
"#), @r#"
        drop index concurrently if exists email_idx;
        create index concurrently if not exists email_idx on app_user (email);
        drop index concurrently if exists app."Name_Idx";
        CREATE UNIQUE INDEX CONCURRENTLY IF NOT EXISTS "Name_Idx" ON app.app_user (name);
        "#);
    }

    #[test]
    fn unnamed_index_no_fix() {
        let sql = "create index concurrently if not exists on t (c);";
        let file = squawk_syntax::SourceFile::parse(sql);
        let mut linter = Linter::from([Rule::BanConcurrentIndexCreationIfNotExists]);
        assert_snapshot!(format_violations(sql, &linter.lint(&file, sql)), @"
        warning[ban-concurrent-index-creation-if-not-exists]: if `create index concurrently` fails, it leaves behind an invalid index, which `if not exists` skips when the migration is rerun
          ╭▸ 
        1 │ create index concurrently if not exists on t (c);
          │                           ━━━━━━━━━━━━━
          │
          ╰ help: drop the index with `drop index concurrently if exists` before creating it
        ");
    }

    #[test]
    fn assume_in_transaction_single_statement_no_fix() {
        let sql = "create index concurrently if not exists email_idx on app_user (email);";
        assert_snapshot!(lint_errors_with(
            sql,
            assume_in_transaction(),
            Rule::BanConcurrentIndexCreationIfNotExists
        ), @"
        warning[ban-concurrent-index-creation-if-not-exists]: if `create index concurrently` fails, it leaves behind an invalid index, which `if not exists` skips when the migration is rerun
          ╭▸ 
        1 │ create index concurrently if not exists email_idx on app_user (email);
          │                           ━━━━━━━━━━━━━
          │
          ╰ help: drop the index with `drop index concurrently if exists` before creating it
        ");
    }

    #[test]
    fn fix_assume_in_transaction_multiple_statements() {
        assert_snapshot!(fix_sql_with(
            "set lock_timeout = '1s';
create index concurrently if not exists email_idx on app_user (email);",
            assume_in_transaction(),
            Rule::BanConcurrentIndexCreationIfNotExists
        ), @"
        set lock_timeout = '1s';
        drop index concurrently if exists email_idx;
        create index concurrently if not exists email_idx on app_user (email);
        ");
    }

    #[test]
    fn fix_inserts_above_comments() {
        assert_snapshot!(fix("set lock_timeout = '1s';
-- squawk-ignore ban-concurrent-index-creation-in-transaction
-- second comment line
create index concurrently if not exists email_idx on app_user (email);
"), @"
        set lock_timeout = '1s';
        drop index concurrently if exists email_idx;
        -- squawk-ignore ban-concurrent-index-creation-in-transaction
        -- second comment line
        create index concurrently if not exists email_idx on app_user (email);
        ");
    }

    #[test]
    fn fix_skips_comment_above_blank_line() {
        assert_snapshot!(fix("set lock_timeout = '1s';
-- unrelated comment

create index concurrently if not exists email_idx on app_user (email);
"), @"
        set lock_timeout = '1s';
        -- unrelated comment

        drop index concurrently if exists email_idx;
        create index concurrently if not exists email_idx on app_user (email);
        ");
    }

    #[test]
    fn fix_skips_trailing_comment_of_previous_statement() {
        assert_snapshot!(fix("set lock_timeout = '1s'; -- trailing comment
create index concurrently if not exists email_idx on app_user (email);
"), @"
        set lock_timeout = '1s'; -- trailing comment
        drop index concurrently if exists email_idx;
        create index concurrently if not exists email_idx on app_user (email);
        ");
    }

    #[test]
    fn fix_keeps_indentation() {
        assert_snapshot!(fix("select 1;
    -- comment
    create index concurrently if not exists i on t (c);
"), @"
        select 1;
            drop index concurrently if exists i;
            -- comment
            create index concurrently if not exists i on t (c);
        ");
    }

    #[test]
    fn fixes_preserve_crlf_line_endings() {
        assert_snapshot!(
            format!(
                "{:?}",
                fix("select 1;\r\n  create index concurrently if not exists i on t (c);\r\n")
            ),
            @r#""select 1;\r\n  drop index concurrently if exists i;\r\n  create index concurrently if not exists i on t (c);\r\n""#
        );
    }

    #[test]
    fn fixes_preserve_cr_line_endings() {
        assert_snapshot!(
            format!(
                "{:?}",
                fix("select 1;\r-- comment\rcreate index concurrently if not exists i on t (c);")
            ),
            @r#""select 1;\rdrop index concurrently if exists i;\r-- comment\rcreate index concurrently if not exists i on t (c);""#
        );
    }
}
