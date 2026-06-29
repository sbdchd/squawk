use rowan::TextSize;
use squawk_syntax::{
    Parse, SourceFile, SyntaxKind,
    ast::{self, AstNode},
};

use crate::{Edit, Fix, Linter, Rule, Violation, analyze};

fn find_insert_pos(file: &SourceFile) -> TextSize {
    for child in file.syntax().children_with_tokens() {
        match child.kind() {
            SyntaxKind::COMMENT | SyntaxKind::WHITESPACE => continue,
            _ => return child.text_range().start(),
        }
    }
    TextSize::from(0)
}

fn create_stmt_timeout_fix(file: &SourceFile) -> Fix {
    let at = find_insert_pos(file);
    Fix::new(
        "Add statement timeout",
        vec![Edit::insert("set statement_timeout = '5s';\n", at)],
    )
}

fn create_lock_timeout_fix(file: &SourceFile) -> Fix {
    let at = find_insert_pos(file);
    Fix::new(
        "Add lock timeout",
        vec![Edit::insert("set lock_timeout = '1s';\n", at)],
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReportOnce {
    Missing,
    Present,
    Reported,
}

/// The lock a statement takes. `Unknown` covers statements we don't have a
/// specific note for; more get added over time (e.g. the ACCESS EXCLUSIVE
/// `ALTER TABLE` variants).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LockMode {
    Unknown,
    ShareUpdateExclusive,
}

impl LockMode {
    /// The lock's name, or `None` when we can't classify the statement.
    fn name(self) -> Option<&'static str> {
        match self {
            LockMode::Unknown => None,
            LockMode::ShareUpdateExclusive => Some("SHARE UPDATE EXCLUSIVE"),
        }
    }

    /// The warning message, naming the lock when we can classify it.
    fn violation_message(self) -> String {
        match self.name() {
            Some(name) => {
                format!("Missing `set lock_timeout` before potentially slow {name} lock operations")
            }
            None => "Missing `set lock_timeout` before potentially slow operations".to_string(),
        }
    }

    /// The `lock_timeout` help text, ending with what this lock does and doesn't
    /// block so a reader can judge how much the timeout matters.
    fn help(self) -> &'static str {
        match self {
            LockMode::Unknown => "Configure a `lock_timeout` before this statement.",
            LockMode::ShareUpdateExclusive => {
                "Configure a `lock_timeout` before this statement. It takes a SHARE UPDATE \
                 EXCLUSIVE lock. It will block other schema changes (VACUUM, ANALYZE, index \
                 builds) but not reads or writes."
            }
        }
    }
}

fn statement_lock(stmt: &ast::Stmt) -> LockMode {
    match stmt {
        ast::Stmt::CommentOn(_) => LockMode::ShareUpdateExclusive,
        _ => LockMode::Unknown,
    }
}

pub(crate) fn require_timeout_settings(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();

    let mut lock_timeout = ReportOnce::Missing;
    let mut stmt_timeout = ReportOnce::Missing;

    for stmt in file.stmts() {
        // stop early if both are reported
        if lock_timeout == ReportOnce::Reported && stmt_timeout == ReportOnce::Reported {
            break;
        }

        match stmt {
            ast::Stmt::Set(set) => {
                if let Some(path) = set.path() {
                    // only want to check for `set lock_timeout = '1s'`, not `set foo.lock_timeout = '1s'`
                    if path.qualifier().is_some() {
                        continue;
                    }
                    if let Some(segment) = path.segment()
                        && let Some(name_ref) = segment.name_ref()
                    {
                        let name = name_ref.text();
                        if name == "lock_timeout" {
                            lock_timeout = ReportOnce::Present;
                        } else if name == "statement_timeout" {
                            stmt_timeout = ReportOnce::Present;
                        }
                    }
                }
            }
            _ if analyze::possibly_slow_stmt(&stmt) => {
                let lock = statement_lock(&stmt);
                if lock_timeout == ReportOnce::Missing {
                    ctx.report(
                        Violation::for_node(
                            Rule::RequireTimeoutSettings,
                            lock.violation_message(),
                            stmt.syntax(),
                        )
                        .help(lock.help().to_string())
                        .fix(create_lock_timeout_fix(&file)),
                    );
                    lock_timeout = ReportOnce::Reported;
                }
                if stmt_timeout == ReportOnce::Missing {
                    ctx.report(
                        Violation::for_node(
                            Rule::RequireTimeoutSettings,
                            "Missing `set statement_timeout` before potentially slow operations"
                                .to_string(),
                            stmt.syntax(),
                        )
                        .help("Configure a `statement_timeout` before this statement".to_string())
                        .fix(create_stmt_timeout_fix(&file)),
                    );
                    stmt_timeout = ReportOnce::Reported;
                }
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;

    use crate::{
        Rule,
        test_utils::{fix_sql, lint_errors, lint_ok},
    };

    #[must_use]
    fn fix(sql: &str) -> String {
        fix_sql(sql, Rule::RequireTimeoutSettings)
    }

    #[test]
    fn err_missing_both_timeouts() {
        let sql = r#"
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RequireTimeoutSettings), @"
        warning[require-timeout-settings]: Missing `set lock_timeout` before potentially slow operations
          ╭▸ 
        2 │ ALTER TABLE t ADD COLUMN c BOOLEAN;
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `lock_timeout` before this statement.
          ╭╴
        2 + set lock_timeout = '1s';
          ╰╴
        warning[require-timeout-settings]: Missing `set statement_timeout` before potentially slow operations
          ╭▸ 
        2 │ ALTER TABLE t ADD COLUMN c BOOLEAN;
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `statement_timeout` before this statement
          ╭╴
        2 + set statement_timeout = '5s';
          ╰╴
        ");
    }

    #[test]
    fn err_missing_lock_timeout() {
        let sql = r#"
SET statement_timeout = '5s';
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RequireTimeoutSettings), @"
        warning[require-timeout-settings]: Missing `set lock_timeout` before potentially slow operations
          ╭▸ 
        3 │ ALTER TABLE t ADD COLUMN c BOOLEAN;
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `lock_timeout` before this statement.
          ╭╴
        2 + set lock_timeout = '1s';
          ╰╴
        ");
    }

    #[test]
    fn err_missing_statement_timeout() {
        let sql = r#"
SET lock_timeout = '1s';
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RequireTimeoutSettings), @"
        warning[require-timeout-settings]: Missing `set statement_timeout` before potentially slow operations
          ╭▸ 
        3 │ ALTER TABLE t ADD COLUMN c BOOLEAN;
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `statement_timeout` before this statement
          ╭╴
        2 + set statement_timeout = '5s';
          ╰╴
        ");
    }

    #[test]
    fn ok_both_timeouts_present() {
        let sql = r#"
SET lock_timeout = '1s';
SET statement_timeout = '5s';
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        lint_ok(sql, Rule::RequireTimeoutSettings);
    }

    #[test]
    fn ok_both_timeouts_present_casing() {
        let sql = r#"
SET Lock_Timeout = '1s';
SET Statement_Timeout = '5s';
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        lint_ok(sql, Rule::RequireTimeoutSettings);
    }

    #[test]
    fn ok_no_ddl_operations() {
        let sql = r#"
SELECT * FROM t;
        "#;
        lint_ok(sql, Rule::RequireTimeoutSettings);
    }

    #[test]
    fn err_timeouts_using_schema() {
        let sql = r#"
SET foo.lock_timeout = '1s';
SET foo.statement_timeout = '5s';
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RequireTimeoutSettings), @"
        warning[require-timeout-settings]: Missing `set lock_timeout` before potentially slow operations
          ╭▸ 
        4 │ ALTER TABLE t ADD COLUMN c BOOLEAN;
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `lock_timeout` before this statement.
          ╭╴
        2 + set lock_timeout = '1s';
          ╰╴
        warning[require-timeout-settings]: Missing `set statement_timeout` before potentially slow operations
          ╭▸ 
        4 │ ALTER TABLE t ADD COLUMN c BOOLEAN;
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `statement_timeout` before this statement
          ╭╴
        2 + set statement_timeout = '5s';
          ╰╴
        ");
    }
    #[test]
    fn err_timeouts_after_ddl() {
        let sql = r#"
ALTER TABLE t ADD COLUMN c BOOLEAN;
SET lock_timeout = '1s';
SET statement_timeout = '5s';
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RequireTimeoutSettings), @"
        warning[require-timeout-settings]: Missing `set lock_timeout` before potentially slow operations
          ╭▸ 
        2 │ ALTER TABLE t ADD COLUMN c BOOLEAN;
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `lock_timeout` before this statement.
          ╭╴
        2 + set lock_timeout = '1s';
          ╰╴
        warning[require-timeout-settings]: Missing `set statement_timeout` before potentially slow operations
          ╭▸ 
        2 │ ALTER TABLE t ADD COLUMN c BOOLEAN;
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `statement_timeout` before this statement
          ╭╴
        2 + set statement_timeout = '5s';
          ╰╴
        ");
    }

    #[test]
    fn err_other_ddl_operations() {
        let sql = r#"
CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RequireTimeoutSettings), @"
        warning[require-timeout-settings]: Missing `set lock_timeout` before potentially slow operations
          ╭▸ 
        2 │ CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `lock_timeout` before this statement.
          ╭╴
        2 + set lock_timeout = '1s';
          ╰╴
        warning[require-timeout-settings]: Missing `set statement_timeout` before potentially slow operations
          ╭▸ 
        2 │ CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `statement_timeout` before this statement
          ╭╴
        2 + set statement_timeout = '5s';
          ╰╴
        ");
    }

    #[test]
    fn err_comment_on_explains_lock_impact() {
        // COMMENT ON takes a SHARE UPDATE EXCLUSIVE lock, so the help should
        // name the lock mode.
        let sql = r#"
COMMENT ON COLUMN t.c IS 'an opaque id';
        "#;
        let out = lint_errors(sql, Rule::RequireTimeoutSettings);
        assert!(
            out.contains("EXCLUSIVE"),
            "expected the lock mode to appear in the help text, got:\n{out}"
        );
    }

    #[test]
    fn ok_other_ddl_with_timeouts() {
        let sql = r#"
SET lock_timeout = '1s';
SET statement_timeout = '5s';
CREATE FUNCTION add(integer, integer) RETURNS integer
    AS 'select $1 + $2;'
    LANGUAGE SQL;
        "#;
        lint_ok(sql, Rule::RequireTimeoutSettings);
    }

    #[test]
    fn fix_add_both_timeouts() {
        let sql = "ALTER TABLE t ADD COLUMN c BOOLEAN;";
        let result = fix(sql);
        assert_snapshot!(result, @r"
        set statement_timeout = '5s';
        set lock_timeout = '1s';
        ALTER TABLE t ADD COLUMN c BOOLEAN;
        ");
    }

    #[test]
    fn fix_add_lock_timeout_only() {
        let sql = r#"
SET statement_timeout = '5s';
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        let result = fix(sql);
        assert_snapshot!(result, @r"
        set lock_timeout = '1s';
        SET statement_timeout = '5s';
        ALTER TABLE t ADD COLUMN c BOOLEAN;
        ");
    }

    #[test]
    fn fix_add_statement_timeout_only() {
        let sql = r#"
SET lock_timeout = '1s';
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        let result = fix(sql);
        assert_snapshot!(result, @r"
        set statement_timeout = '5s';
        SET lock_timeout = '1s';
        ALTER TABLE t ADD COLUMN c BOOLEAN;
        ");
    }

    #[test]
    fn fix_with_leading_comments() {
        let sql = r#"-- leading comment
-- should be okay

ALTER TABLE users ADD COLUMN email TEXT;"#;
        let result = fix(sql);
        assert_snapshot!(result, @r"
        -- leading comment
        -- should be okay

        set statement_timeout = '5s';
        set lock_timeout = '1s';
        ALTER TABLE users ADD COLUMN email TEXT;
        ");
    }

    #[test]
    fn fix_with_leading_comment_c_style() {
        let sql = r#"/* foo bar */
ALTER TABLE users ADD COLUMN email TEXT;"#;
        let result = fix(sql);
        assert_snapshot!(result, @r"
        /* foo bar */
        set statement_timeout = '5s';
        set lock_timeout = '1s';
        ALTER TABLE users ADD COLUMN email TEXT;
        ");
    }

    #[test]
    fn fix_with_prefix_comment_c_style() {
        let sql = r#"/* boo */ALTER TABLE users ADD COLUMN email TEXT;"#;
        let result = fix(sql);
        assert_snapshot!(result, @r"
        /* boo */set statement_timeout = '5s';
        set lock_timeout = '1s';
        ALTER TABLE users ADD COLUMN email TEXT;
        ");
    }

    #[test]
    fn fix_multiple_ddl_statements() {
        let sql = r#"
CREATE TABLE users (id SERIAL);
ALTER TABLE users ADD COLUMN email TEXT;
        "#;
        let result = fix(sql);
        assert_snapshot!(result, @r"
        set statement_timeout = '5s';
        set lock_timeout = '1s';
        CREATE TABLE users (id SERIAL);
        ALTER TABLE users ADD COLUMN email TEXT;
        ");
    }
}
