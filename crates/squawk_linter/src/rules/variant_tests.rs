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
fn remaining_renames() {
    check(
        "ALTER TABLE t RENAME CONSTRAINT old TO renamed; ALTER DOMAIN d RENAME CONSTRAINT old TO renamed; ALTER ROLE app RENAME TO app2; ALTER USER app RENAME TO app2; ALTER GROUP app RENAME TO app2; ALTER DATABASE db RENAME TO db2; ALTER TRIGGER tr ON t RENAME TO tr2; ALTER POLICY p ON t RENAME TO p2;",
        Rule::RenamingObject,
        8,
    );
    check(
        "ALTER TYPE composite RENAME ATTRIBUTE old TO renamed;",
        Rule::RenamingColumn,
        1,
    );
    lint_ok("ALTER POLICY p ON t USING (true);", Rule::RenamingObject);
}

#[test]
fn remaining_constraints_and_attributes() {
    check(
        "ALTER DOMAIN d DROP CONSTRAINT c; ALTER TABLE t ALTER CONSTRAINT c NOT ENFORCED; ALTER FOREIGN TABLE ft DROP CONSTRAINT c;",
        Rule::BanDropConstraint,
        3,
    );
    check(
        "ALTER TYPE composite DROP ATTRIBUTE a; ALTER FOREIGN TABLE ft DROP COLUMN a;",
        Rule::BanDropColumn,
        2,
    );
    check(
        "ALTER TYPE composite ALTER ATTRIBUTE a TYPE text; ALTER FOREIGN TABLE ft ALTER COLUMN a TYPE text;",
        Rule::ChangingColumnType,
        2,
    );
    lint_ok(
        "ALTER TABLE t ALTER CONSTRAINT c ENFORCED;",
        Rule::BanDropConstraint,
    );
}

#[test]
fn remaining_drops_and_ownership() {
    check("DROP AGGREGATE agg(int);", Rule::BanDropFunction, 1);
    check(
        "DROP OPERATOR + (int, int); DROP OPERATOR CLASS op USING btree; DROP OPERATOR FAMILY fam USING btree;",
        Rule::BanDropType,
        3,
    );
    check(
        "ALTER FOREIGN TABLE ft OWNER TO app; ALTER DOMAIN d OWNER TO app;",
        Rule::BanRevoke,
        2,
    );
    lint_ok("ALTER TABLE t SET SCHEMA s;", Rule::BanRevoke);
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
    check(
        "REASSIGN OWNED BY app TO admin; ALTER TABLE t OWNER TO admin; DROP USER app;",
        Rule::BanRevoke,
        3,
    );
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
    lint_ok("ALTER TABLE t ENABLE TRIGGER tr;", Rule::BanDisableTrigger);
}

#[test]
fn schema_moves() {
    check(
        "ALTER PROCEDURE p() SET SCHEMA s; ALTER ROUTINE f() SET SCHEMA s; ALTER FOREIGN TABLE ft SET SCHEMA s;",
        Rule::BanSetSchema,
        3,
    );
}
