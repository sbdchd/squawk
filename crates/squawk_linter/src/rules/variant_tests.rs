use crate::{
    Rule,
    test_utils::{lint_errors, lint_ok},
};

fn check(sql: &str, rule: Rule, count: usize) {
    for statement in sql.split(';').filter(|s| !s.trim().is_empty()) {
        let parsed = squawk_syntax::SourceFile::parse(&format!("{statement};"));
        assert!(
            parsed.errors().is_empty(),
            "{statement}: {:?}",
            parsed.errors()
        );
    }
    let errors = lint_errors(sql, rule);
    assert_eq!(errors.matches("warning[").count(), count, "{errors}");
}

#[test]
fn replacements() {
    check(
        "CREATE OR REPLACE RULE r AS ON INSERT TO t DO INSTEAD NOTHING; CREATE OR REPLACE AGGREGATE a (int) (SFUNC = int4pl, STYPE = int); CREATE OR REPLACE TRIGGER tr BEFORE INSERT ON t FOR EACH ROW EXECUTE FUNCTION f();",
        Rule::BanReplaceViewFunction,
        3,
    );
    lint_ok(
        "CREATE RULE r AS ON INSERT TO t DO INSTEAD NOTHING;",
        Rule::BanReplaceViewFunction,
    );
}

#[test]
fn routine_options() {
    check(
        "ALTER PROCEDURE p() SECURITY DEFINER; ALTER PROCEDURE p() SET search_path TO private; ALTER PROCEDURE p() RESET ALL; ALTER ROUTINE f() SECURITY INVOKER; ALTER ROUTINE f() SET search_path TO private; ALTER ROUTINE f() RESET ALL;",
        Rule::BanAlterFunctionOptions,
        6,
    );
    lint_ok(
        "ALTER PROCEDURE p() RENAME TO q; ALTER ROUTINE f() SET SCHEMA private; ALTER PROCEDURE p() OWNER TO app;",
        Rule::BanAlterFunctionOptions,
    );
}

#[test]
fn user_options() {
    check(
        "ALTER USER app NOLOGIN; ALTER USER app NOBYPASSRLS; ALTER USER app IN DATABASE db SET search_path TO private; ALTER USER app RESET ALL;",
        Rule::BanAlterRoleOptions,
        4,
    );
    lint_ok("ALTER USER app RENAME TO app2;", Rule::BanAlterRoleOptions);
}

#[test]
fn group_membership_removal() {
    check(
        "ALTER GROUP writers DROP USER app, worker;",
        Rule::BanRevoke,
        1,
    );
    lint_ok(
        "ALTER GROUP writers ADD USER app; ALTER GROUP writers RENAME TO editors;",
        Rule::BanRevoke,
    );
}

#[test]
fn additional_write_constraint_forms() {
    check(
        "ALTER TABLE t ADD CONSTRAINT id_required NOT NULL id; ALTER TABLE t ADD COLUMN c bigint PRIMARY KEY; ALTER FOREIGN TABLE ft ADD COLUMN c int NOT NULL;",
        Rule::BanNewWriteRestriction,
        3,
    );
    lint_ok(
        "CREATE TABLE t (id bigint); ALTER TABLE t ADD CONSTRAINT id_required NOT NULL id; ALTER TABLE t ADD COLUMN c bigint PRIMARY KEY; ALTER FOREIGN TABLE ft ADD COLUMN c int;",
        Rule::BanNewWriteRestriction,
    );
}

#[test]
fn generated_expression() {
    check(
        "ALTER TABLE t ALTER COLUMN c SET EXPRESSION AS (id + 1);",
        Rule::BanAlterGeneratedExpression,
        1,
    );
    lint_ok(
        "ALTER TABLE t ALTER COLUMN c DROP EXPRESSION;",
        Rule::BanAlterGeneratedExpression,
    );
}

#[test]
fn enforcement_and_policy() {
    check(
        "ALTER TABLE t ENABLE REPLICA TRIGGER tr; ALTER TABLE t ENABLE REPLICA RULE r; ALTER TABLE t ENABLE ALWAYS TRIGGER tr; ALTER TABLE t ENABLE ALWAYS RULE r; ALTER TABLE t NO FORCE ROW LEVEL SECURITY;",
        Rule::BanDisableTrigger,
        5,
    );
    lint_ok("ALTER POLICY p ON t USING (true);", Rule::BanDropPolicy);
    check(
        "ALTER TABLE t ENABLE TRIGGER tr;",
        Rule::BanDisableTrigger,
        1,
    );
}
