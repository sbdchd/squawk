use std::fmt;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LockImpact(u8);

impl LockImpact {
    const READS: u8 = 0b001;
    const SCHEMA_CHANGES: u8 = 0b100;
    const WRITES: u8 = 0b010;

    const fn new(blocks: u8) -> Self {
        Self(blocks)
    }

    fn blocked(self) -> Vec<&'static str> {
        let mut items = vec![];
        for (bit, label) in [
            (Self::READS, "reads"),
            (Self::WRITES, "writes"),
            (Self::SCHEMA_CHANGES, "schema changes"),
        ] {
            if self.0 & bit != 0 {
                items.push(label);
            }
        }
        items
    }
}

/// The lock a statement takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LockKind {
    AccessExclusive,
    AccessShare,
    Exclusive,
    RowExclusive,
    RowShare,
    Share,
    ShareRowExclusive,
    ShareUpdateExclusive,
    Unknown,
}

impl fmt::Display for LockKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LockKind::AccessExclusive => f.write_str("ACCESS EXCLUSIVE"),
            LockKind::AccessShare => f.write_str("ACCESS SHARE"),
            LockKind::Exclusive => f.write_str("EXCLUSIVE"),
            LockKind::RowExclusive => f.write_str("ROW EXCLUSIVE"),
            LockKind::RowShare => f.write_str("ROW SHARE"),
            LockKind::Share => f.write_str("SHARE"),
            LockKind::ShareRowExclusive => f.write_str("SHARE ROW EXCLUSIVE"),
            LockKind::ShareUpdateExclusive => f.write_str("SHARE UPDATE EXCLUSIVE"),
            LockKind::Unknown => Ok(()),
        }
    }
}

impl LockKind {
    fn from_lock_mode(lock_mode: ast::LockMode) -> LockKind {
        match lock_mode {
            ast::LockMode::AccessExclusive(_) => LockKind::AccessExclusive,
            ast::LockMode::AccessShare(_) => LockKind::AccessShare,
            ast::LockMode::Exclusive(_) => LockKind::Exclusive,
            ast::LockMode::RowExclusive(_) => LockKind::RowExclusive,
            ast::LockMode::RowShare(_) => LockKind::RowShare,
            ast::LockMode::Share(_) => LockKind::Share,
            ast::LockMode::ShareRowExclusive(_) => LockKind::ShareRowExclusive,
            ast::LockMode::ShareUpdateExclusive(_) => LockKind::ShareUpdateExclusive,
        }
    }

    fn strength(self) -> u8 {
        match self {
            LockKind::Unknown => 0,
            LockKind::AccessShare => 1,
            LockKind::RowShare => 2,
            LockKind::RowExclusive => 3,
            LockKind::ShareUpdateExclusive => 4,
            LockKind::Share => 5,
            LockKind::ShareRowExclusive => 6,
            LockKind::Exclusive => 7,
            LockKind::AccessExclusive => 8,
        }
    }

    fn from_alter_table_action(action: &ast::AlterTableAction) -> LockKind {
        match action {
            ast::AlterTableAction::ClusterOn(_)
            | ast::AlterTableAction::ResetOptions(_)
            | ast::AlterTableAction::SetOptions(_)
            | ast::AlterTableAction::SetWithoutCluster(_)
            | ast::AlterTableAction::ValidateConstraint(_) => LockKind::ShareUpdateExclusive,
            ast::AlterTableAction::DetachPartition(_) => LockKind::AccessExclusive,
            ast::AlterTableAction::AddConstraint(add_constraint) => {
                if let Some(ast::Constraint::ForeignKeyConstraint(_)) = add_constraint.constraint()
                {
                    LockKind::ShareRowExclusive
                } else {
                    LockKind::AccessExclusive
                }
            }
            ast::AlterTableAction::DisableTrigger(_)
            | ast::AlterTableAction::EnableAlwaysTrigger(_)
            | ast::AlterTableAction::EnableReplicaTrigger(_)
            | ast::AlterTableAction::EnableTrigger(_) => LockKind::ShareRowExclusive,
            ast::AlterTableAction::AlterColumn(alter_column) => match alter_column.option() {
                Some(
                    ast::AlterColumnOption::ResetOptions(_)
                    | ast::AlterColumnOption::SetOptions(_)
                    | ast::AlterColumnOption::SetStatistics(_),
                ) => LockKind::ShareUpdateExclusive,
                Some(
                    ast::AlterColumnOption::AddGenerated(_)
                    | ast::AlterColumnOption::DropDefault(_)
                    | ast::AlterColumnOption::DropExpression(_)
                    | ast::AlterColumnOption::DropIdentity(_)
                    | ast::AlterColumnOption::DropNotNull(_)
                    | ast::AlterColumnOption::Inherit(_)
                    | ast::AlterColumnOption::NoInherit(_)
                    | ast::AlterColumnOption::Restart(_)
                    | ast::AlterColumnOption::SetCompression(_)
                    | ast::AlterColumnOption::SetDefault(_)
                    | ast::AlterColumnOption::SetExpression(_)
                    | ast::AlterColumnOption::SetGenerated(_)
                    | ast::AlterColumnOption::SetGeneratedOptions(_)
                    | ast::AlterColumnOption::SetNotNull(_)
                    | ast::AlterColumnOption::SetOptionsList(_)
                    | ast::AlterColumnOption::SetSequenceOption(_)
                    | ast::AlterColumnOption::SetStorage(_)
                    | ast::AlterColumnOption::SetType(_),
                )
                | None => LockKind::AccessExclusive,
            },
            ast::AlterTableAction::AddColumn(_)
            | ast::AlterTableAction::AlterConstraint(_)
            | ast::AlterTableAction::AttachPartition(_)
            | ast::AlterTableAction::DisableRls(_)
            | ast::AlterTableAction::DisableRule(_)
            | ast::AlterTableAction::DropColumn(_)
            | ast::AlterTableAction::DropConstraint(_)
            | ast::AlterTableAction::EnableAlwaysRule(_)
            | ast::AlterTableAction::EnableReplicaRule(_)
            | ast::AlterTableAction::EnableRls(_)
            | ast::AlterTableAction::EnableRule(_)
            | ast::AlterTableAction::ForceRls(_)
            | ast::AlterTableAction::InheritTable(_)
            | ast::AlterTableAction::MergePartitions(_)
            | ast::AlterTableAction::NoForceRls(_)
            | ast::AlterTableAction::NoInheritTable(_)
            | ast::AlterTableAction::NotOf(_)
            | ast::AlterTableAction::OfType(_)
            | ast::AlterTableAction::OptionItemList(_)
            | ast::AlterTableAction::OwnerTo(_)
            | ast::AlterTableAction::RenameColumn(_)
            | ast::AlterTableAction::RenameConstraint(_)
            | ast::AlterTableAction::RenameTo(_)
            | ast::AlterTableAction::ReplicaIdentity(_)
            | ast::AlterTableAction::SetAccessMethod(_)
            | ast::AlterTableAction::SetLogged(_)
            | ast::AlterTableAction::SetSchema(_)
            | ast::AlterTableAction::SetTablespace(_)
            | ast::AlterTableAction::SetUnlogged(_)
            | ast::AlterTableAction::SetWithoutOids(_)
            | ast::AlterTableAction::SplitPartition(_) => LockKind::AccessExclusive,
        }
    }

    fn from(stmt: &ast::Stmt) -> LockKind {
        match stmt {
            ast::Stmt::AlterTable(alter_table) => alter_table
                .actions()
                .map(|action| Self::from_alter_table_action(&action))
                .max_by_key(|lock| lock.strength())
                .unwrap_or(LockKind::AccessExclusive),
            ast::Stmt::Cluster(_) => LockKind::AccessExclusive,
            ast::Stmt::CommentOn(_) => LockKind::ShareUpdateExclusive,
            ast::Stmt::CreateIndex(create_index) => {
                if create_index.concurrently_token().is_some() {
                    LockKind::ShareUpdateExclusive
                } else {
                    LockKind::Share
                }
            }
            ast::Stmt::DropIndex(drop_index) => {
                if drop_index.concurrently_token().is_some() {
                    LockKind::ShareUpdateExclusive
                } else {
                    LockKind::AccessExclusive
                }
            }
            ast::Stmt::DropTable(_) | ast::Stmt::DropView(_) => LockKind::AccessExclusive,
            ast::Stmt::Lock(lock) => lock
                .lock_mode()
                .map(Self::from_lock_mode)
                .unwrap_or(LockKind::AccessExclusive),
            ast::Stmt::Refresh(refresh) => {
                if refresh.concurrently_token().is_some() {
                    LockKind::Exclusive
                } else {
                    LockKind::AccessExclusive
                }
            }
            ast::Stmt::Reindex(reindex) => {
                if reindex.is_concurrently() {
                    LockKind::ShareUpdateExclusive
                } else {
                    LockKind::AccessExclusive
                }
            }
            ast::Stmt::Truncate(_) => LockKind::AccessExclusive,
            ast::Stmt::Vacuum(vacuum) => {
                if vacuum.is_full() {
                    LockKind::AccessExclusive
                } else {
                    LockKind::ShareUpdateExclusive
                }
            }
            _ => LockKind::Unknown,
        }
    }

    fn violation_message(self) -> String {
        let name = self.to_string();
        if name.is_empty() {
            "Missing `set lock_timeout` before potentially slow operations".to_string()
        } else {
            format!("Missing `set lock_timeout` before potentially slow {name} lock operations")
        }
    }

    fn impact(self) -> Option<LockImpact> {
        match self {
            LockKind::AccessExclusive => Some(LockImpact::new(
                LockImpact::READS | LockImpact::WRITES | LockImpact::SCHEMA_CHANGES,
            )),
            LockKind::AccessShare => Some(LockImpact::new(LockImpact::SCHEMA_CHANGES)),
            LockKind::Exclusive | LockKind::Share | LockKind::ShareRowExclusive => Some(
                LockImpact::new(LockImpact::WRITES | LockImpact::SCHEMA_CHANGES),
            ),
            LockKind::RowExclusive | LockKind::RowShare | LockKind::ShareUpdateExclusive => {
                Some(LockImpact::new(LockImpact::SCHEMA_CHANGES))
            }
            LockKind::Unknown => None,
        }
    }

    fn help(self) -> String {
        let help = "Configure a `lock_timeout` before this statement.";
        let name = self.to_string();
        let Some(impact) = self.impact() else {
            return help.to_string();
        };

        let blocked = impact.blocked();
        if blocked.is_empty() {
            format!("{help} Statement requires: {name} lock.")
        } else {
            format!(
                "{help} Statement requires: {name} lock; blocking: {}.",
                blocked.join(", ")
            )
        }
    }
}

pub(crate) fn require_timeout_settings(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    let file = parse.tree();

    // treat a disabled rule as already reported so it never reports
    let mut lock_timeout = if ctx.rules.contains(&Rule::RequireLockTimeout) {
        ReportOnce::Missing
    } else {
        ReportOnce::Reported
    };
    let mut stmt_timeout = if ctx.rules.contains(&Rule::RequireStatementTimeout) {
        ReportOnce::Missing
    } else {
        ReportOnce::Reported
    };

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
                let lock = LockKind::from(&stmt);
                if lock_timeout == ReportOnce::Missing {
                    ctx.report(
                        Violation::for_node(
                            Rule::RequireLockTimeout,
                            lock.violation_message(),
                            stmt.syntax(),
                        )
                        .help(lock.help())
                        .fix(create_lock_timeout_fix(&file)),
                    );
                    lock_timeout = ReportOnce::Reported;
                }
                if stmt_timeout == ReportOnce::Missing {
                    ctx.report(
                        Violation::for_node(
                            Rule::RequireStatementTimeout,
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
    use squawk_syntax::SourceFile;
    use tabled::{builder::Builder, settings::Style};

    use crate::{
        Rule,
        test_utils::{fix_sql, lint_errors, lint_ok},
    };

    #[must_use]
    fn fix(sql: &str) -> String {
        fix_sql(sql, Rule::RequireTimeoutSettings)
    }

    fn lock_kinds(cases: &[&str]) -> String {
        let mut builder = Builder::default();
        builder.push_record(["sql", "lock"]);

        for sql in cases {
            let file = SourceFile::parse(sql);
            assert_eq!(file.errors(), vec![]);
            let stmt = file.tree().stmts().next().expect("expected statement");
            let lock = super::LockKind::from(&stmt);
            builder.push_record([sql.to_string(), format!("{lock:?}")]);
        }

        let mut table = builder.build();
        table.with(Style::psql());
        table.to_string()
    }

    #[test]
    fn lock_kind_for_statement_variants() {
        let cases = [
            "ALTER TABLE t ADD COLUMN c int;",
            "CLUSTER t;",
            "COMMENT ON TABLE t IS 'x';",
            "CREATE INDEX idx ON t (c);",
            "CREATE INDEX CONCURRENTLY idx ON t (c);",
            "DROP INDEX idx;",
            "DROP INDEX CONCURRENTLY idx;",
            "DROP TABLE t;",
            "DROP VIEW v;",
            "REFRESH MATERIALIZED VIEW mv;",
            "REFRESH MATERIALIZED VIEW CONCURRENTLY mv;",
            "REINDEX INDEX idx;",
            "REINDEX INDEX CONCURRENTLY idx;",
            "TRUNCATE t;",
            "CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');",
        ];

        assert_snapshot!(lock_kinds(&cases), @"
         sql                                              | lock                 
        --------------------------------------------------+----------------------
         ALTER TABLE t ADD COLUMN c int;                  | AccessExclusive      
         CLUSTER t;                                       | AccessExclusive      
         COMMENT ON TABLE t IS 'x';                       | ShareUpdateExclusive 
         CREATE INDEX idx ON t (c);                       | Share                
         CREATE INDEX CONCURRENTLY idx ON t (c);          | ShareUpdateExclusive 
         DROP INDEX idx;                                  | AccessExclusive      
         DROP INDEX CONCURRENTLY idx;                     | ShareUpdateExclusive 
         DROP TABLE t;                                    | AccessExclusive      
         DROP VIEW v;                                     | AccessExclusive      
         REFRESH MATERIALIZED VIEW mv;                    | AccessExclusive      
         REFRESH MATERIALIZED VIEW CONCURRENTLY mv;       | Exclusive            
         REINDEX INDEX idx;                               | AccessExclusive      
         REINDEX INDEX CONCURRENTLY idx;                  | ShareUpdateExclusive 
         TRUNCATE t;                                      | AccessExclusive      
         CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy'); | Unknown
        ");
    }

    #[test]
    fn lock_kind_for_alter_table_variants() {
        let cases = [
            "ALTER TABLE t ADD COLUMN c int;",
            "ALTER TABLE t DROP COLUMN c;",
            "ALTER TABLE t ALTER COLUMN c TYPE bigint;",
            "ALTER TABLE t VALIDATE CONSTRAINT c;",
            "ALTER TABLE t ALTER COLUMN c SET STATISTICS 100;",
            "ALTER TABLE t ALTER COLUMN c SET (n_distinct = 5);",
            "ALTER TABLE t SET (fillfactor = 70);",
            "ALTER TABLE t RESET (fillfactor);",
            "ALTER TABLE t CLUSTER ON idx;",
            "ALTER TABLE t SET WITHOUT CLUSTER;",
            "ALTER TABLE t DETACH PARTITION p;",
            "ALTER TABLE t DETACH PARTITION p CONCURRENTLY;",
            "ALTER TABLE t ADD CONSTRAINT fk FOREIGN KEY (a) REFERENCES o (id);",
            "ALTER TABLE t ADD CONSTRAINT ck CHECK (a > 0);",
            "ALTER TABLE t DISABLE TRIGGER trg;",
            "ALTER TABLE t ENABLE TRIGGER trg;",
            "ALTER TABLE t ENABLE REPLICA TRIGGER trg;",
            "ALTER TABLE t ENABLE ALWAYS TRIGGER trg;",
            "ALTER TABLE t VALIDATE CONSTRAINT c, ADD COLUMN d int;",
        ];

        assert_snapshot!(lock_kinds(&cases), @"
         sql                                                                | lock                 
        --------------------------------------------------------------------+----------------------
         ALTER TABLE t ADD COLUMN c int;                                    | AccessExclusive      
         ALTER TABLE t DROP COLUMN c;                                       | AccessExclusive      
         ALTER TABLE t ALTER COLUMN c TYPE bigint;                          | AccessExclusive      
         ALTER TABLE t VALIDATE CONSTRAINT c;                               | ShareUpdateExclusive 
         ALTER TABLE t ALTER COLUMN c SET STATISTICS 100;                   | ShareUpdateExclusive 
         ALTER TABLE t ALTER COLUMN c SET (n_distinct = 5);                 | ShareUpdateExclusive 
         ALTER TABLE t SET (fillfactor = 70);                               | ShareUpdateExclusive 
         ALTER TABLE t RESET (fillfactor);                                  | ShareUpdateExclusive 
         ALTER TABLE t CLUSTER ON idx;                                      | ShareUpdateExclusive 
         ALTER TABLE t SET WITHOUT CLUSTER;                                 | ShareUpdateExclusive 
         ALTER TABLE t DETACH PARTITION p;                                  | AccessExclusive      
         ALTER TABLE t DETACH PARTITION p CONCURRENTLY;                     | AccessExclusive      
         ALTER TABLE t ADD CONSTRAINT fk FOREIGN KEY (a) REFERENCES o (id); | ShareRowExclusive    
         ALTER TABLE t ADD CONSTRAINT ck CHECK (a > 0);                     | AccessExclusive      
         ALTER TABLE t DISABLE TRIGGER trg;                                 | ShareRowExclusive    
         ALTER TABLE t ENABLE TRIGGER trg;                                  | ShareRowExclusive    
         ALTER TABLE t ENABLE REPLICA TRIGGER trg;                          | ShareRowExclusive    
         ALTER TABLE t ENABLE ALWAYS TRIGGER trg;                           | ShareRowExclusive    
         ALTER TABLE t VALIDATE CONSTRAINT c, ADD COLUMN d int;             | AccessExclusive
        ");
    }

    #[test]
    fn lock_kind_for_explicit_lock_modes() {
        let cases = [
            "LOCK TABLE t;",
            "LOCK TABLE t IN ACCESS EXCLUSIVE MODE;",
            "LOCK TABLE t IN ACCESS SHARE MODE;",
            "LOCK TABLE t IN EXCLUSIVE MODE;",
            "LOCK TABLE t IN ROW EXCLUSIVE MODE;",
            "LOCK TABLE t IN ROW SHARE MODE;",
            "LOCK TABLE t IN SHARE MODE;",
            "LOCK TABLE t IN SHARE ROW EXCLUSIVE MODE;",
            "LOCK TABLE t IN SHARE UPDATE EXCLUSIVE MODE;",
        ];

        assert_snapshot!(lock_kinds(&cases), @"
         sql                                          | lock                 
        ----------------------------------------------+----------------------
         LOCK TABLE t;                                | AccessExclusive      
         LOCK TABLE t IN ACCESS EXCLUSIVE MODE;       | AccessExclusive      
         LOCK TABLE t IN ACCESS SHARE MODE;           | AccessShare          
         LOCK TABLE t IN EXCLUSIVE MODE;              | Exclusive            
         LOCK TABLE t IN ROW EXCLUSIVE MODE;          | RowExclusive         
         LOCK TABLE t IN ROW SHARE MODE;              | RowShare             
         LOCK TABLE t IN SHARE MODE;                  | Share                
         LOCK TABLE t IN SHARE ROW EXCLUSIVE MODE;    | ShareRowExclusive    
         LOCK TABLE t IN SHARE UPDATE EXCLUSIVE MODE; | ShareUpdateExclusive
        ");
    }

    #[test]
    fn lock_kind_for_reindex_concurrently_options() {
        let cases = [
            "REINDEX (CONCURRENTLY) INDEX idx;",
            "REINDEX (CONCURRENTLY true) INDEX idx;",
            "REINDEX (CONCURRENTLY false) INDEX idx;",
            "REINDEX (CONCURRENTLY 'false') INDEX idx;",
            "REINDEX (CONCURRENTLY E'false') INDEX idx;",
            "REINDEX (CONCURRENTLY U&'false') INDEX idx;",
            "REINDEX (CONCURRENTLY off) INDEX idx;",
            "REINDEX (CONCURRENTLY 0) INDEX idx;",
        ];

        assert_snapshot!(lock_kinds(&cases), @"
         sql                                         | lock                 
        ---------------------------------------------+----------------------
         REINDEX (CONCURRENTLY) INDEX idx;           | ShareUpdateExclusive 
         REINDEX (CONCURRENTLY true) INDEX idx;      | ShareUpdateExclusive 
         REINDEX (CONCURRENTLY false) INDEX idx;     | AccessExclusive      
         REINDEX (CONCURRENTLY 'false') INDEX idx;   | AccessExclusive      
         REINDEX (CONCURRENTLY E'false') INDEX idx;  | AccessExclusive      
         REINDEX (CONCURRENTLY U&'false') INDEX idx; | AccessExclusive      
         REINDEX (CONCURRENTLY off) INDEX idx;       | AccessExclusive      
         REINDEX (CONCURRENTLY 0) INDEX idx;         | AccessExclusive
        ");
    }

    #[test]
    fn lock_kind_for_vacuum_full_options() {
        let cases = [
            "VACUUM t;",
            "VACUUM FULL t;",
            "VACUUM (FULL) t;",
            "VACUUM (FULL true) t;",
            "VACUUM (FULL false) t;",
            "VACUUM (FULL 'false') t;",
            "VACUUM (FULL E'false') t;",
            "VACUUM (FULL U&'false') t;",
            "VACUUM (FULL off) t;",
            "VACUUM (FULL 0) t;",
        ];

        assert_snapshot!(lock_kinds(&cases), @"
         sql                        | lock                 
        ----------------------------+----------------------
         VACUUM t;                  | ShareUpdateExclusive 
         VACUUM FULL t;             | AccessExclusive      
         VACUUM (FULL) t;           | AccessExclusive      
         VACUUM (FULL true) t;      | AccessExclusive      
         VACUUM (FULL false) t;     | ShareUpdateExclusive 
         VACUUM (FULL 'false') t;   | ShareUpdateExclusive 
         VACUUM (FULL E'false') t;  | ShareUpdateExclusive 
         VACUUM (FULL U&'false') t; | ShareUpdateExclusive 
         VACUUM (FULL off) t;       | ShareUpdateExclusive 
         VACUUM (FULL 0) t;         | ShareUpdateExclusive
        ");
    }

    #[test]
    fn err_missing_both_timeouts() {
        let sql = r#"
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RequireTimeoutSettings), @"
        warning[require-lock-timeout]: Missing `set lock_timeout` before potentially slow ACCESS EXCLUSIVE lock operations
          ╭▸ 
        2 │ ALTER TABLE t ADD COLUMN c BOOLEAN;
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `lock_timeout` before this statement. Statement requires: ACCESS EXCLUSIVE lock; blocking: reads, writes, schema changes.
          ╭╴
        2 + set lock_timeout = '1s';
          ╰╴
        warning[require-statement-timeout]: Missing `set statement_timeout` before potentially slow operations
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
        warning[require-lock-timeout]: Missing `set lock_timeout` before potentially slow ACCESS EXCLUSIVE lock operations
          ╭▸ 
        3 │ ALTER TABLE t ADD COLUMN c BOOLEAN;
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `lock_timeout` before this statement. Statement requires: ACCESS EXCLUSIVE lock; blocking: reads, writes, schema changes.
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
        warning[require-statement-timeout]: Missing `set statement_timeout` before potentially slow operations
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
    fn err_only_lock_timeout_rule_enabled() {
        let sql = r#"
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RequireLockTimeout), @"
        warning[require-lock-timeout]: Missing `set lock_timeout` before potentially slow ACCESS EXCLUSIVE lock operations
          ╭▸ 
        2 │ ALTER TABLE t ADD COLUMN c BOOLEAN;
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `lock_timeout` before this statement. Statement requires: ACCESS EXCLUSIVE lock; blocking: reads, writes, schema changes.
          ╭╴
        2 + set lock_timeout = '1s';
          ╰╴
        ");
    }

    #[test]
    fn err_only_statement_timeout_rule_enabled() {
        let sql = r#"
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RequireStatementTimeout), @"
        warning[require-statement-timeout]: Missing `set statement_timeout` before potentially slow operations
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
    fn ok_only_lock_timeout_rule_enabled() {
        let sql = r#"
SET lock_timeout = '1s';
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        lint_ok(sql, Rule::RequireLockTimeout);
    }

    #[test]
    fn ok_only_statement_timeout_rule_enabled() {
        let sql = r#"
SET statement_timeout = '5s';
ALTER TABLE t ADD COLUMN c BOOLEAN;
        "#;
        lint_ok(sql, Rule::RequireStatementTimeout);
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
        warning[require-lock-timeout]: Missing `set lock_timeout` before potentially slow ACCESS EXCLUSIVE lock operations
          ╭▸ 
        4 │ ALTER TABLE t ADD COLUMN c BOOLEAN;
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `lock_timeout` before this statement. Statement requires: ACCESS EXCLUSIVE lock; blocking: reads, writes, schema changes.
          ╭╴
        2 + set lock_timeout = '1s';
          ╰╴
        warning[require-statement-timeout]: Missing `set statement_timeout` before potentially slow operations
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
        warning[require-lock-timeout]: Missing `set lock_timeout` before potentially slow ACCESS EXCLUSIVE lock operations
          ╭▸ 
        2 │ ALTER TABLE t ADD COLUMN c BOOLEAN;
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `lock_timeout` before this statement. Statement requires: ACCESS EXCLUSIVE lock; blocking: reads, writes, schema changes.
          ╭╴
        2 + set lock_timeout = '1s';
          ╰╴
        warning[require-statement-timeout]: Missing `set statement_timeout` before potentially slow operations
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
        warning[require-lock-timeout]: Missing `set lock_timeout` before potentially slow operations
          ╭▸ 
        2 │ CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `lock_timeout` before this statement.
          ╭╴
        2 + set lock_timeout = '1s';
          ╰╴
        warning[require-statement-timeout]: Missing `set statement_timeout` before potentially slow operations
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
        assert_snapshot!(out, @"
        warning[require-lock-timeout]: Missing `set lock_timeout` before potentially slow SHARE UPDATE EXCLUSIVE lock operations
          ╭▸ 
        2 │ COMMENT ON COLUMN t.c IS 'an opaque id';
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `lock_timeout` before this statement. Statement requires: SHARE UPDATE EXCLUSIVE lock; blocking: schema changes.
          ╭╴
        2 + set lock_timeout = '1s';
          ╰╴
        warning[require-statement-timeout]: Missing `set statement_timeout` before potentially slow operations
          ╭▸ 
        2 │ COMMENT ON COLUMN t.c IS 'an opaque id';
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `statement_timeout` before this statement
          ╭╴
        2 + set statement_timeout = '5s';
          ╰╴
        ");
    }

    #[test]
    fn err_reindex_concurrently_false_uses_non_concurrent_lock() {
        let sql = r#"
REINDEX (CONCURRENTLY false) INDEX idx;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RequireTimeoutSettings), @"
        warning[require-lock-timeout]: Missing `set lock_timeout` before potentially slow ACCESS EXCLUSIVE lock operations
          ╭▸ 
        2 │ REINDEX (CONCURRENTLY false) INDEX idx;
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `lock_timeout` before this statement. Statement requires: ACCESS EXCLUSIVE lock; blocking: reads, writes, schema changes.
          ╭╴
        2 + set lock_timeout = '1s';
          ╰╴
        warning[require-statement-timeout]: Missing `set statement_timeout` before potentially slow operations
          ╭▸ 
        2 │ REINDEX (CONCURRENTLY false) INDEX idx;
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `statement_timeout` before this statement
          ╭╴
        2 + set statement_timeout = '5s';
          ╰╴
        ");
    }

    #[test]
    fn err_vacuum_full_false_uses_non_full_lock() {
        let sql = r#"
VACUUM (FULL false) t;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RequireTimeoutSettings), @"
        warning[require-lock-timeout]: Missing `set lock_timeout` before potentially slow SHARE UPDATE EXCLUSIVE lock operations
          ╭▸ 
        2 │ VACUUM (FULL false) t;
          │ ━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `lock_timeout` before this statement. Statement requires: SHARE UPDATE EXCLUSIVE lock; blocking: schema changes.
          ╭╴
        2 + set lock_timeout = '1s';
          ╰╴
        warning[require-statement-timeout]: Missing `set statement_timeout` before potentially slow operations
          ╭▸ 
        2 │ VACUUM (FULL false) t;
          │ ━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `statement_timeout` before this statement
          ╭╴
        2 + set statement_timeout = '5s';
          ╰╴
        ");
    }

    #[test]
    fn err_lock_access_share_uses_correct_article() {
        let sql = r#"
LOCK TABLE t IN ACCESS SHARE MODE;
        "#;
        assert_snapshot!(lint_errors(sql, Rule::RequireTimeoutSettings), @"
        warning[require-lock-timeout]: Missing `set lock_timeout` before potentially slow ACCESS SHARE lock operations
          ╭▸ 
        2 │ LOCK TABLE t IN ACCESS SHARE MODE;
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `lock_timeout` before this statement. Statement requires: ACCESS SHARE lock; blocking: schema changes.
          ╭╴
        2 + set lock_timeout = '1s';
          ╰╴
        warning[require-statement-timeout]: Missing `set statement_timeout` before potentially slow operations
          ╭▸ 
        2 │ LOCK TABLE t IN ACCESS SHARE MODE;
          │ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
          │
          ├ help: Configure a `statement_timeout` before this statement
          ╭╴
        2 + set statement_timeout = '5s';
          ╰╴
        ");
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
