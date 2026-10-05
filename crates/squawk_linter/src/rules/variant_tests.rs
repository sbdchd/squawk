use crate::{
    Rule,
    test_utils::{lint_errors, lint_ok},
};

fn check(sql: &str, rule: Rule, count: usize) {
    let errors = lint_errors(sql, rule);
    assert_eq!(errors.matches("warning[").count(), count, "{errors}");
}

#[test]
fn renames() {
    check(
        "ALTER FOREIGN TABLE ft RENAME TO ft2; ALTER ROUTINE f() RENAME TO g;",
        Rule::RenamingObject,
        2,
    );
    check(
        "ALTER FOREIGN TABLE ft RENAME COLUMN a TO b; ALTER VIEW v RENAME COLUMN a TO b; ALTER MATERIALIZED VIEW mv RENAME COLUMN a TO b;",
        Rule::RenamingColumn,
        3,
    );
    lint_ok("ALTER VIEW v RENAME TO v2;", Rule::RenamingColumn);
}

#[test]
fn defaults_and_nullability() {
    check(
        "ALTER DOMAIN d DROP NOT NULL; ALTER FOREIGN TABLE ft ALTER COLUMN c DROP NOT NULL;",
        Rule::BanDropNotNull,
        2,
    );
    check(
        "ALTER DOMAIN d SET NOT NULL; ALTER FOREIGN TABLE ft ALTER COLUMN c SET NOT NULL;",
        Rule::AddingNotNullableField,
        2,
    );
    check(
        "ALTER DOMAIN d DROP DEFAULT; ALTER VIEW v ALTER COLUMN c DROP DEFAULT; ALTER FOREIGN TABLE ft ALTER COLUMN c DROP DEFAULT;",
        Rule::BanDropDefault,
        3,
    );
    check(
        "ALTER DOMAIN d SET DEFAULT 1; ALTER VIEW v ALTER COLUMN c SET DEFAULT 1; ALTER FOREIGN TABLE ft ALTER COLUMN c SET DEFAULT 1;",
        Rule::BanSetDefault,
        3,
    );
    lint_ok("ALTER DOMAIN d SET DEFAULT 1;", Rule::BanDropDefault);
    lint_ok("ALTER DOMAIN d DROP DEFAULT;", Rule::BanSetDefault);
}

#[test]
fn drops_and_revokes() {
    check("DROP FOREIGN TABLE IF EXISTS ft;", Rule::BanDropTable, 1);
    check("DROP ROUTINE IF EXISTS f(int);", Rule::BanDropFunction, 1);
    check("DROP CAST IF EXISTS (text AS int);", Rule::BanDropType, 1);
    check("DROP OWNED BY app; DROP ROLE app;", Rule::BanRevoke, 2);
    lint_ok("REASSIGN OWNED BY app TO admin;", Rule::BanRevoke);
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
    lint_ok("ALTER TABLE t ENABLE TRIGGER tr;", Rule::BanDisableTrigger);
}

#[test]
fn schema_moves() {
    check(
        "ALTER PROCEDURE p() SET SCHEMA s; ALTER ROUTINE f() SET SCHEMA s;",
        Rule::BanSetSchema,
        2,
    );
}
