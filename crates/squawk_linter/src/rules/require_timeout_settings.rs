use rowan::TextSize;
use squawk_syntax::{
    Parse, SourceFile, SyntaxKind,
    ast::{self, AstNode},
    identifier::Identifier,
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
                    if let Some(segment) = path.segment() {
                        if let Some(name_ref) = segment.name_ref() {
                            let name_ident = Identifier::new(name_ref.text().as_str());
                            if name_ident == Identifier::new("lock_timeout") {
                                lock_timeout = ReportOnce::Present;
                            } else if name_ident == Identifier::new("statement_timeout") {
                                stmt_timeout = ReportOnce::Present;
                            }
                        }
                    }
                }
            }
            _ if analyze::possibly_slow_stmt(&stmt) => {
                if lock_timeout == ReportOnce::Missing {
                    ctx.report(
                        Violation::for_node(
                            Rule::RequireTimeoutSettings,
                            "Missing `set lock_timeout` before potentially slow operations"
                                .to_string(),
                            stmt.syntax(),
                        )
                        .help("Configure a `lock_timeout` before this statement.".to_string())
                        .fix(Some(create_lock_timeout_fix(&file))),
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
                        .fix(Some(create_stmt_timeout_fix(&file))),
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
    use insta::{assert_debug_snapshot, assert_snapshot};

    use crate::{
        Rule,
        test_utils::{fix_sql, lint},
    };

    fn fix(sql: &str) -> String {
        fix_sql(sql, Rule::RequireTimeoutSettings)
    }

    #[test]
    fn err_missing_both_timeouts() {
        let sql = r#"
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        let errors = lint(sql, Rule::RequireTimeoutSettings);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors, @r#"
        [
            Violation {
                code: RequireTimeoutSettings,
                message: "Missing `set lock_timeout` before potentially slow operations",
                text_range: 1..35,
                help: Some(
                    "Configure a `lock_timeout` before this statement.",
                ),
                fix: Some(
                    Fix {
                        title: "Add lock timeout",
                        edits: [
                            Edit {
                                text_range: 1..1,
                                text: Some(
                                    "set lock_timeout = '1s';\n",
                                ),
                            },
                        ],
                    },
                ),
            },
            Violation {
                code: RequireTimeoutSettings,
                message: "Missing `set statement_timeout` before potentially slow operations",
                text_range: 1..35,
                help: Some(
                    "Configure a `statement_timeout` before this statement",
                ),
                fix: Some(
                    Fix {
                        title: "Add statement timeout",
                        edits: [
                            Edit {
                                text_range: 1..1,
                                text: Some(
                                    "set statement_timeout = '5s';\n",
                                ),
                            },
                        ],
                    },
                ),
            },
        ]
        "#);
    }

    #[test]
    fn err_missing_lock_timeout() {
        let sql = r#"
SET statement_timeout = '5s';
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        let errors = lint(sql, Rule::RequireTimeoutSettings);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors, @r#"
        [
            Violation {
                code: RequireTimeoutSettings,
                message: "Missing `set lock_timeout` before potentially slow operations",
                text_range: 31..65,
                help: Some(
                    "Configure a `lock_timeout` before this statement.",
                ),
                fix: Some(
                    Fix {
                        title: "Add lock timeout",
                        edits: [
                            Edit {
                                text_range: 1..1,
                                text: Some(
                                    "set lock_timeout = '1s';\n",
                                ),
                            },
                        ],
                    },
                ),
            },
        ]
        "#);
    }

    #[test]
    fn err_missing_statement_timeout() {
        let sql = r#"
SET lock_timeout = '1s';
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        let errors = lint(sql, Rule::RequireTimeoutSettings);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors, @r#"
        [
            Violation {
                code: RequireTimeoutSettings,
                message: "Missing `set statement_timeout` before potentially slow operations",
                text_range: 26..60,
                help: Some(
                    "Configure a `statement_timeout` before this statement",
                ),
                fix: Some(
                    Fix {
                        title: "Add statement timeout",
                        edits: [
                            Edit {
                                text_range: 1..1,
                                text: Some(
                                    "set statement_timeout = '5s';\n",
                                ),
                            },
                        ],
                    },
                ),
            },
        ]
        "#);
    }

    #[test]
    fn ok_both_timeouts_present() {
        let sql = r#"
SET lock_timeout = '1s';
SET statement_timeout = '5s';
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        let errors = lint(sql, Rule::RequireTimeoutSettings);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn ok_both_timeouts_present_casing() {
        let sql = r#"
SET Lock_Timeout = '1s';
SET Statement_Timeout = '5s';
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        let errors = lint(sql, Rule::RequireTimeoutSettings);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn ok_no_ddl_operations() {
        let sql = r#"
SELECT * FROM t;
        "#;
        let errors = lint(sql, Rule::RequireTimeoutSettings);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn err_timeouts_using_schema() {
        let sql = r#"
SET foo.lock_timeout = '1s';
SET foo.statement_timeout = '5s';
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        let errors = lint(sql, Rule::RequireTimeoutSettings);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors, @r#"
        [
            Violation {
                code: RequireTimeoutSettings,
                message: "Missing `set lock_timeout` before potentially slow operations",
                text_range: 64..98,
                help: Some(
                    "Configure a `lock_timeout` before this statement.",
                ),
                fix: Some(
                    Fix {
                        title: "Add lock timeout",
                        edits: [
                            Edit {
                                text_range: 1..1,
                                text: Some(
                                    "set lock_timeout = '1s';\n",
                                ),
                            },
                        ],
                    },
                ),
            },
            Violation {
                code: RequireTimeoutSettings,
                message: "Missing `set statement_timeout` before potentially slow operations",
                text_range: 64..98,
                help: Some(
                    "Configure a `statement_timeout` before this statement",
                ),
                fix: Some(
                    Fix {
                        title: "Add statement timeout",
                        edits: [
                            Edit {
                                text_range: 1..1,
                                text: Some(
                                    "set statement_timeout = '5s';\n",
                                ),
                            },
                        ],
                    },
                ),
            },
        ]
        "#);
    }
    #[test]
    fn err_timeouts_after_ddl() {
        let sql = r#"
ALTER TABLE t ADD COLUMN c BOOLEAN;
SET lock_timeout = '1s';
SET statement_timeout = '5s';
        "#;
        let errors = lint(sql, Rule::RequireTimeoutSettings);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors, @r#"
        [
            Violation {
                code: RequireTimeoutSettings,
                message: "Missing `set lock_timeout` before potentially slow operations",
                text_range: 1..35,
                help: Some(
                    "Configure a `lock_timeout` before this statement.",
                ),
                fix: Some(
                    Fix {
                        title: "Add lock timeout",
                        edits: [
                            Edit {
                                text_range: 1..1,
                                text: Some(
                                    "set lock_timeout = '1s';\n",
                                ),
                            },
                        ],
                    },
                ),
            },
            Violation {
                code: RequireTimeoutSettings,
                message: "Missing `set statement_timeout` before potentially slow operations",
                text_range: 1..35,
                help: Some(
                    "Configure a `statement_timeout` before this statement",
                ),
                fix: Some(
                    Fix {
                        title: "Add statement timeout",
                        edits: [
                            Edit {
                                text_range: 1..1,
                                text: Some(
                                    "set statement_timeout = '5s';\n",
                                ),
                            },
                        ],
                    },
                ),
            },
        ]
        "#);
    }

    #[test]
    fn err_other_ddl_operations() {
        let sql = r#"
CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');
        "#;
        let errors = lint(sql, Rule::RequireTimeoutSettings);
        assert_ne!(errors.len(), 0);
        assert_debug_snapshot!(errors, @r#"
        [
            Violation {
                code: RequireTimeoutSettings,
                message: "Missing `set lock_timeout` before potentially slow operations",
                text_range: 1..48,
                help: Some(
                    "Configure a `lock_timeout` before this statement.",
                ),
                fix: Some(
                    Fix {
                        title: "Add lock timeout",
                        edits: [
                            Edit {
                                text_range: 1..1,
                                text: Some(
                                    "set lock_timeout = '1s';\n",
                                ),
                            },
                        ],
                    },
                ),
            },
            Violation {
                code: RequireTimeoutSettings,
                message: "Missing `set statement_timeout` before potentially slow operations",
                text_range: 1..48,
                help: Some(
                    "Configure a `statement_timeout` before this statement",
                ),
                fix: Some(
                    Fix {
                        title: "Add statement timeout",
                        edits: [
                            Edit {
                                text_range: 1..1,
                                text: Some(
                                    "set statement_timeout = '5s';\n",
                                ),
                            },
                        ],
                    },
                ),
            },
        ]
        "#);
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
        let errors = lint(sql, Rule::RequireTimeoutSettings);
        assert_eq!(errors.len(), 0);
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
