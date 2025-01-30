use crate::ast::{RawStmt, StmtRoot};
use crate::error::PgQueryError;
use libpg_query::{pg_query_free_parse_result, pg_query_parse};
use serde::Deserialize;
use serde_json::Value;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

fn parse_str_or_none(str_ptr: *mut c_char) -> Option<String> {
    if str_ptr.is_null() {
        None
    } else {
        unsafe { Some(CStr::from_ptr(str_ptr).to_string_lossy().into()) }
    }
}

fn parse_sql_query_base<'a, T>(query: &'a str) -> Result<T, PgQueryError>
where
    T: Deserialize<'a>,
{
    let c_str = CString::new(query)?;
    let pg_parse_result = unsafe { pg_query_parse(c_str.as_ptr()) };

    if !pg_parse_result.error.is_null() {
        unsafe {
            let err = *pg_parse_result.error;
            return Err(PgQueryError::PgParseError(parse_str_or_none(err.message)));
        }
    }

    // not sure if this is ever null, but might as well check
    if pg_parse_result.parse_tree.is_null() {
        return Err(PgQueryError::ParsingCString);
    }

    let parse_tree = unsafe { CStr::from_ptr(pg_parse_result.parse_tree) }.to_str()?;
    let output =
        serde_json::from_str(parse_tree).map_err(|e| PgQueryError::JsonParse(e.to_string()));

    unsafe {
        pg_query_free_parse_result(pg_parse_result);
    };

    output
}

pub fn parse_sql_query_json(query: &str) -> Result<Value, PgQueryError> {
    parse_sql_query_base(query)
}

pub fn parse_sql_query(query: &str) -> Result<Vec<RawStmt>, PgQueryError> {
    let parsed: StmtRoot = parse_sql_query_base(query)?;
    Ok(parsed.stmts)
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_debug_snapshot;

    #[test]
    fn parse_sql_query_json_works() {
        let sql = r"ALTER TABLE table_c ADD column c boolean GENERATED ALWAYS AS (p IS NOT NULL) STORED NOT NULL;";
        let res = parse_sql_query_json(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn span_with_indent() {
        // NOTE: the span information for these starts at 0 even though the SQL
        // is offset.
        let sql = r"   SELECT 1;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }
    #[test]
    fn span_with_new_line_and_indent() {
        let sql = r"
    SELECT 1;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn adding_index_non_concurrently() {
        let sql = r#"
  -- instead of
  CREATE INDEX "field_name_idx" ON "table_name" ("field_name");
  -- use CONCURRENTLY
  CREATE INDEX CONCURRENTLY "field_name_idx" ON "table_name" ("field_name");
  "#;

        let res = parse_sql_query(sql);

        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_index_without_index_name() {
        let sql = "CREATE INDEX ON FOO(BAR);";
        let res = parse_sql_query(sql);
        assert!(res.is_ok());
        assert_debug_snapshot!(res);
    }

    #[test]
    fn error_paths() {
        let sql = r"lsakdjf;asdlfkjasd;lfj";
        let res = parse_sql_query(sql).unwrap_err();
        assert_debug_snapshot!(res);
    }

    #[test]
    fn migration() {
        let sql = r#"
BEGIN;
CREATE INDEX "table_name_field_name_idx" ON "table_name" ("field_name");
CREATE INDEX "table_name_field_name_idx" ON "table_name" ("field_name" varchar_pattern_ops);
COMMIT;
"#;
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn select_string_literal() {
        let sql = r"SELECT 'some string';";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn select_one() {
        let sql = r"SELECT 1;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }
    #[test]
    fn parse_sql_create_index_concurrently() {
        let sql = r#"CREATE INDEX CONCURRENTLY "table_name_idx" ON "table_name" ("table_field");"#;
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parsing_insert_stmt() {
        let sql = r"INSERT INTO table_name VALUES (1, 2, 3);";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parsing_update_stmt() {
        let sql = r"UPDATE table_name SET foo = 'bar' WHERE buzz > 10;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parsing_create_table() {
        let sql = r#"
BEGIN;
CREATE TABLE "core_foo" (
  "id" serial NOT NULL PRIMARY KEY, 
  "created" timestamp with time zone NOT NULL, 
  "modified" timestamp with time zone NOT NULL, 
  "mongo_id" varchar(255) NOT NULL UNIQUE, 
  "description" text NOT NULL, 
  "metadata" jsonb NOT NULL, 
  "kind" varchar(255) NOT NULL, 
  "age" integer NOT NULL, 
  "tenant_id" integer NULL
);
CREATE INDEX "age_index" ON "core_foo" ("age");
COMMIT;
"#;
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parsing_create_table_using_like() {
        let sql =
            r"CREATE TABLE core_bar (LIKE core_foo INCLUDING DEFAULTS INCLUDING CONSTRAINTS);";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_sql_create_index() {
        let sql = r#"CREATE INDEX "table_name_idx" ON "table_name" ("table_field");"#;
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }
    #[test]
    fn parse_sql_create_unique_index_safe() {
        let sql = r#"
ALTER TABLE "legacy_questiongrouppg" 
    ADD CONSTRAINT "legacy_questiongrouppg_mongo_id_1f8f47d9_uniq" UNIQUE 
    USING INDEX "legacy_questiongrouppg_mongo_id_1f8f47d9_uniq_idx";
"#;
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }
    #[test]
    fn parse_delete_stmt() {
        let sql = r#"DELETE FROM "table_name";"#;
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_delete_stmt_2() {
        let sql = r#"DELETE FROM "table_name" WHERE account_age > 10;"#;
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_set_operations_stmt() {
        let sql = r#"SELECT * from "table_name" UNION SELECT * from "table_foo";"#;
        let res = parse_sql_query(sql).unwrap();
        assert_debug_snapshot!(res);

        let sql = r#"SELECT * from "table_name" UNION ALL SELECT * from "table_foo";"#;
        let res = parse_sql_query(sql).unwrap();
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_set_operations_stmt_2() {
        let sql = r#"SELECT * from "table_name" UNION ALL SELECT * from "table_foo";"#;
        let res = parse_sql_query(sql).unwrap();
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_create_schema_stmt() {
        let sql = r"CREATE SCHEMA schema_name;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_replica_identity_stmt() {
        let sql = "ALTER TABLE aa REPLICA IDENTITY FULL;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_alter_table_set_list() {
        let sql = "ALTER TABLE table_name SET (autovacuum_vacuum_scale_factor = 0.0);";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_alter_collation_stmt() {
        let sql = "ALTER COLLATION name RENAME TO new_name;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_alter_domain_stmt() {
        let sql = "ALTER DOMAIN zipcode SET NOT NULL;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parsing_grant_stmt() {
        let sql = "GRANT INSERT ON films TO PUBLIC;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parsing_grant_role() {
        let sql = "GRANT admins TO joe;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_default_privileges_stmt() {
        let sql = "ALTER DEFAULT PRIVILEGES IN SCHEMA myschema GRANT SELECT ON TABLES TO PUBLIC;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parsing_copy_stmt() {
        let sql = "COPY country FROM '/usr1/proj/bray/sql/country_data';";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parsing_variable_set_stmt() {
        let sql = "set session my.vars.id = '1';";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parsing_variable_show_stmt() {
        let sql = "SHOW name";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parsing_create_table_space_stmt() {
        let sql = "CREATE TABLESPACE dbspace LOCATION '/data/dbs';";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parsing_drop_table_space_stmt() {
        let sql = "DROP TABLESPACE dbspace;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_table_space_stmt() {
        let sql = "ALTER TABLESPACE index_space RENAME TO fast_raid;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_extension() {
        let sql = "CREATE EXTENSION hstore;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_table_extension() {
        let sql = "ALTER EXTENSION hstore UPDATE TO '2.0';";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn drop_extension() {
        let sql = "DROP EXTENSION hstore;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_extension_contents_stmt() {
        let sql = "ALTER EXTENSION hstore SET SCHEMA utils;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);

        let sql = "ALTER EXTENSION hstore ADD FUNCTION populate_record(anyelement, hstore);";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_foreign_data_wrapper() {
        let sql = "CREATE FOREIGN DATA WRAPPER dummy;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_foreign_data_wrapper() {
        let sql = "ALTER FOREIGN DATA WRAPPER dbi OPTIONS (ADD foo '1', DROP 'bar');";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_foreign_server_stmt() {
        let sql = "CREATE SERVER myserver FOREIGN DATA WRAPPER postgres_fdw OPTIONS (host 'foo', dbname 'foodb', port '5432');";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_foreign_server_stmt() {
        let sql = "ALTER SERVER foo OPTIONS (host 'foo', dbname 'foodb');";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_foriegn_table_stmt() {
        let sql = r"
CREATE FOREIGN TABLE films (
    code        char(5) NOT NULL,
    title       varchar(40) NOT NULL,
    did         integer NOT NULL,
    date_prod   date,
    kind        varchar(10),
    len         interval hour to minute
)
SERVER film_server;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_user_mapping_stmt() {
        let sql = "CREATE USER MAPPING FOR bob SERVER foo OPTIONS (user 'bob', password 'secret');";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_user_mapping_stmt() {
        let sql = "ALTER USER MAPPING FOR bob SERVER foo OPTIONS (SET password 'public');";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn drop_user_mapping_stmt() {
        let sql = "DROP USER MAPPING IF EXISTS FOR bob SERVER foo;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn import_foreign_schema_stmt() {
        let sql = r"
IMPORT FOREIGN SCHEMA foreign_films
    FROM SERVER film_server INTO films;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_policy_stmt() {
        let sql = "CREATE POLICY name ON table_name FOR ALL;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_policy_stmt() {
        let sql = "ALTER POLICY name ON table_name RENAME TO new_name;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);

        let sql = "ALTER POLICY name ON table_name TO PUBLIC WITH CHECK (account_age > 10);";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_access_method_stmt() {
        let sql = "CREATE ACCESS METHOD heptree TYPE INDEX HANDLER heptree_handler;";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_trigger_stmt() {
        let sql = r"
CREATE TRIGGER check_update
    BEFORE UPDATE ON accounts
    FOR EACH ROW
    EXECUTE PROCEDURE check_account_update();
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_event_trigger_stmt() {
        let sql = r"
CREATE EVENT TRIGGER abort_ddl ON ddl_command_start
   EXECUTE PROCEDURE abort_any_command();
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_event_trigger_stmt() {
        let sql = r"
ALTER EVENT TRIGGER name DISABLE;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    /// Postgres >=11 feature not supported in libpg_query
    #[test]
    fn create_procedure_stmt() {
        let sql = r"
CREATE PROCEDURE insert_data(a integer, b integer)
LANGUAGE SQL
AS $$
INSERT INTO tbl VALUES (a);
INSERT INTO tbl VALUES (b);
$$;

CALL insert_data(1, 2);
";
        let res = parse_sql_query(sql).unwrap();
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_function_stmt() {
        let sql = r"
CREATE FUNCTION populate() RETURNS integer AS $$
DECLARE
    -- declarations
BEGIN
    PERFORM my_function();
END;
$$ LANGUAGE plpgsql;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_plang_stmt() {
        let sql = r"
CREATE TRUSTED PROCEDURAL LANGUAGE plpgsql
    HANDLER plpgsql_call_handler
    VALIDATOR plpgsql_validator;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_role_stmt() {
        let sql = r"
CREATE ROLE miriam 
    WITH LOGIN PASSWORD 'jw8s0F4' 
    VALID UNTIL '2005-01-01';
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_role_stmt() {
        let sql = r"
ALTER ROLE miriam CREATEROLE CREATEDB;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_role_set_stmt() {
        let sql = r"
ALTER ROLE worker_bee SET maintenance_work_mem = 100000;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn drop_role_set_stmt() {
        let sql = r"
DROP ROLE jonathan;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_sequence_stmt() {
        let sql = r"
CREATE SEQUENCE serial START 101;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_sequence_stmt() {
        let sql = r"
ALTER SEQUENCE serial RESTART WITH 105;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn define_stmt() {
        let sql = r"
CREATE AGGREGATE sum (complex)
(
    sfunc = complex_add,
    stype = complex,
    initcond = '(0,0)'
);
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);

        let sql = r"
CREATE OPERATOR === (
    LEFTARG = box,
    RIGHTARG = box,
    PROCEDURE = area_equal_procedure,
    COMMUTATOR = ===,
    NEGATOR = !==,
    RESTRICT = area_restriction_procedure,
    JOIN = area_join_procedure,
    HASHES, MERGES
);
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);

        let sql = r"
CREATE TYPE box (
    INTERNALLENGTH = 16,
    INPUT = my_box_in_function,
    OUTPUT = my_box_out_function,
    ELEMENT = float4
);
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_domain_stmt() {
        let sql = r"
CREATE DOMAIN us_postal_code AS TEXT
CHECK(
   VALUE ~ '^\d{5}$'
OR VALUE ~ '^\d{5}-\d{4}$'
);
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_op_class_stmt() {
        let sql = r"
CREATE OPERATOR CLASS gist__int_ops
    DEFAULT FOR TYPE _int4 USING gist AS
        OPERATOR        3       &&,
        OPERATOR        6       = (anyarray, anyarray),
        OPERATOR        7       @>,
        OPERATOR        8       <@,
        OPERATOR        20      @@ (_int4, query_int),
        FUNCTION        1       g_int_consistent (internal, _int4, int, oid, internal),
        FUNCTION        2       g_int_union (internal, internal),
        FUNCTION        3       g_int_compress (internal),
        FUNCTION        4       g_int_decompress (internal),
        FUNCTION        5       g_int_penalty (internal, internal, internal),
        FUNCTION        6       g_int_picksplit (internal, internal),
        FUNCTION        7       g_int_same (_int4, _int4, internal);
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_op_class_stmt() {
        let sql = r"
ALTER OPERATOR CLASS name USING index_method RENAME TO new_name;
ALTER OPERATOR CLASS name USING index_method
    OWNER TO CURRENT_USER;
ALTER OPERATOR CLASS name USING index_method
    SET SCHEMA new_schema;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_op_family_stmt() {
        let sql = r"
ALTER OPERATOR FAMILY integer_ops USING btree ADD

  -- int4 vs int2
  OPERATOR 1 < (int4, int2) ,
  OPERATOR 2 <= (int4, int2) ,
  OPERATOR 3 = (int4, int2) ,
  OPERATOR 4 >= (int4, int2) ,
  OPERATOR 5 > (int4, int2) ,
  FUNCTION 1 btint42cmp(int4, int2) ,

  -- int2 vs int4
  OPERATOR 1 < (int2, int4) ,
  OPERATOR 2 <= (int2, int4) ,
  OPERATOR 3 = (int2, int4) ,
  OPERATOR 4 >= (int2, int4) ,
  OPERATOR 5 > (int2, int4) ,
  FUNCTION 1 btint24cmp(int2, int4) ;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn truncate_stmt() {
        let sql = r"
TRUNCATE bigtable, fattable, bar RESTART IDENTITY;
TRUNCATE foo CASCADE;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn comment_on_stmt() {
        let sql = r"
COMMENT ON AGGREGATE my_aggregate (double precision) IS 'Computes sample variance';
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn security_label_stmt() {
        let sql = r"
SECURITY LABEL FOR selinux ON TABLE mytable IS 'system_u:object_r:sepgsql_table_t:s0';
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn declare_cursor_stmt() {
        let sql = r"
DECLARE
    curs2 CURSOR FOR SELECT * FROM tenk1;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn close_portal_stmt() {
        let sql = r"
CLOSE curs1;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn fetch_stmt() {
        let sql = r"
FETCH FORWARD 5 FROM foo;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_stats_stmt() {
        let sql = r"
CREATE STATISTICS s1 (dependencies) ON a, b FROM t1;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn explain_stmt() {
        let sql = r"
EXPLAIN ANALYZE SELECT * FROM t1 WHERE (a = 1) AND (b = 0);
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_function_stmt() {
        let sql = r"
ALTER FUNCTION sqrt(integer) RENAME TO square_root;
ALTER FUNCTION sqrt(integer) OWNER TO joe;
ALTER FUNCTION sqrt(integer) SET SCHEMA maths;
ALTER FUNCTION check_password(text) SET search_path = admin, pg_temp;
ALTER FUNCTION check_password(text) RESET search_path;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn do_stmt() {
        let sql = r"
DO $$DECLARE r record;
BEGIN
    FOR r IN SELECT table_schema, table_name FROM information_schema.tables
             WHERE table_type = 'VIEW' AND table_schema = 'public'
    LOOP
        EXECUTE 'GRANT ALL ON ' || quote_ident(r.table_schema) || '.' || quote_ident(r.table_name) || ' TO webuser';
    END LOOP;
END$$;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_object_depends_stmt() {
        let sql = r"
ALTER TRIGGER name ON table_name 
    DEPENDS ON EXTENSION extension_name;
ALTER FUNCTION sqrt(integer) 
    DEPENDS ON EXTENSION extension_name;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_operator_stmt() {
        let sql = r"
ALTER OPERATOR @@ (text, text) OWNER TO joe;
ALTER OPERATOR @@ (text, text) SET SCHEMA bar;
ALTER OPERATOR && (_int4, _int4) SET (RESTRICT = _int_contsel, JOIN = _int_contjoinsel);
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn rule_stmt() {
        let sql = r#"
CREATE RULE "_RETURN" AS
    ON SELECT TO t1
    DO INSTEAD
        SELECT * FROM t2;

CREATE RULE notify_me AS ON UPDATE TO mytable DO ALSO NOTIFY mytable;
"#;
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn notify_stmt() {
        let sql = r"
NOTIFY virtual;
NOTIFY virtual, 'This is the payload';
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn listen_stmt() {
        let sql = r"
LISTEN virtual;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn unlisten_stmt() {
        let sql = r"
UNLISTEN virtual;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn composite_type_stmt() {
        let sql = r"
CREATE TYPE complex AS (
    r       double precision,
    i       double precision
);
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_enum_stmt() {
        let sql = r"
CREATE TYPE happiness AS ENUM ('happy', 'very happy', 'ecstatic');
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_range_stmt() {
        let sql = r"
CREATE TYPE floatrange AS RANGE (
    subtype = float8,
    subtype_diff = float8mi
);
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_enum_stmt() {
        let sql = r"
ALTER TYPE colors ADD VALUE 'orange' AFTER 'red';
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_view_stmt() {
        let sql = r"
CREATE VIEW vista AS SELECT 'Hello World';
CREATE VIEW comedies AS
    SELECT *
    FROM films
    WHERE kind = 'Comedy';
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn load_stmt() {
        let sql = r"
LOAD 'filename';
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_database_stmt() {
        let sql = r"
CREATE DATABASE lusiadas;
CREATE DATABASE sales OWNER salesapp TABLESPACE salesspace;
CREATE DATABASE music ENCODING 'LATIN1' TEMPLATE template0;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_database_stmt() {
        let sql = r"
ALTER DATABASE name RENAME TO new_name;
ALTER DATABASE name OWNER TO new_owner;
ALTER DATABASE name SET TABLESPACE new_tablespace;
ALTER DATABASE name RESET configuration_parameter;
ALTER DATABASE name RESET ALL;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_database_collation() {
        let sql = r"
ALTER DATABASE pipelines REFRESH COLLATION VERSION;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn drop_database_stmt() {
        let sql = r"
DROP DATABASE name;
DROP DATABASE IF EXISTS name;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_system_stmt() {
        let sql = r"
ALTER SYSTEM SET wal_level = hot_standby;
ALTER SYSTEM RESET wal_level;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn cluster_stmt() {
        let sql = r"
CLUSTER employees USING employees_ind;
CLUSTER employees;
CLUSTER;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn vacuum_stmt() {
        let sql = r"
VACUUM (VERBOSE, ANALYZE) foo;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_table_as_stmt() {
        let sql = r"
CREATE TABLE films2 AS
  TABLE films;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn refresh_material_view_stmt() {
        let sql = r"
REFRESH MATERIALIZED VIEW order_summary;
REFRESH MATERIALIZED VIEW annual_statistics_basis WITH NO DATA;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn checkpoint() {
        let sql = r"
CHECKPOINT;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn discard_stmt() {
        let sql = r"
DISCARD PLANS;
DISCARD SEQUENCES;
DISCARD TEMP;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn lock_stmt() {
        let sql = r"
LOCK TABLE films IN SHARE MODE;
LOCK TABLE films IN SHARE ROW EXCLUSIVE MODE;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn set_constraints() {
        let sql = r"
SET CONSTRAINTS ALL DEFERRED;
SET CONSTRAINTS ALL IMMEDIATE;
SET CONSTRAINTS foo IMMEDIATE;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn reindex_stmt() {
        let sql = r"
REINDEX INDEX my_index;
REINDEX TABLE table_name;
REINDEX DATABASE table_name;
REINDEX SYSTEM table_name;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_conversion_stmt() {
        let sql = r"
CREATE CONVERSION myconv FOR 'UTF8' TO 'LATIN1' FROM myfunc;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_cast_stmt() {
        let sql = r"
CREATE CAST (bigint AS int4) WITH FUNCTION int4(bigint) AS ASSIGNMENT;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn regression_update_table() {
        let sql = r#"
        ALTER TABLE "table_name" ALTER COLUMN "column_name" SET DEFAULT false;
        "#;
        let res = parse_sql_query(sql);
        assert!(res.is_ok());
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_column_default_with_function() {
        let sql = r#"
        ALTER TABLE "table_name" ALTER COLUMN "column_name" SET DEFAULT CURRENT_TIMESTAMP;
        "#;
        let res = parse_sql_query(sql);
        assert!(res.is_ok());
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_transform_stmt() {
        let sql = r"
CREATE TRANSFORM FOR hstore LANGUAGE plpythonu (
    FROM SQL WITH FUNCTION hstore_to_plpython(internal),
    TO SQL WITH FUNCTION plpython_to_hstore(internal)
);
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn prepare_stmt() {
        let sql = r"
PREPARE fooplan (int, text, bool, numeric) AS
    INSERT INTO foo VALUES($1, $2, $3, $4);
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn execute_stmt() {
        let sql = r"
EXECUTE fooplan(1, 'Hunter Valley', 't', 200.00);
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn deallocate_stmt() {
        let sql = r"
DEALLOCATE PREPARE ALL;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn drop_owned_stmt() {
        let sql = r"
DROP OWNED BY foo CASCADE;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn reassign_owned_stmt() {
        let sql = r"
REASSIGN OWNED BY old_role TO new_role;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_ts_dictionary_stmt() {
        let sql = r"
ALTER TEXT SEARCH DICTIONARY my_dict ( StopWords = newrussian );
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_ts_configuration_stmt() {
        let sql = r"
ALTER TEXT SEARCH CONFIGURATION astro_en
    ADD MAPPING FOR asciiword WITH astrosyn, english_ispell, english_stem;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_publication_stmt() {
        let sql = r"
CREATE PUBLICATION mypublication FOR TABLE users, departments;
CREATE PUBLICATION insert_only FOR TABLE mydata
    WITH (publish = 'insert');
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_publication() {
        let sql = r"
ALTER PUBLICATION noinsert SET (publish = 'update, delete');
ALTER PUBLICATION mypublication ADD TABLE users, departments;
ALTER PUBLICATION name RENAME TO new_name
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }
    #[test]
    fn parse_func_call() {
        let sql = r"
ALTER TABLE foobar ALTER COLUMN value SET DEFAULT TO_JSON(false);
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn json_index_operator() {
        let sql = r#"
CREATE INDEX CONCURRENTLY IF NOT EXISTS "idx_a_foo_bar" ON "a" ((foo->>'bar'));
"#;
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn create_subscription_stmt() {
        let sql = r"
CREATE SUBSCRIPTION mysub
         CONNECTION 'host=192.168.1.50 port=5432 user=foo dbname=foodb'
        PUBLICATION mypublication, insert_only;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn alter_subscription_stmt() {
        let sql = r"
ALTER SUBSCRIPTION mysub SET PUBLICATION insert_only;
ALTER SUBSCRIPTION mysub DISABLE;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn drop_subscription_stmt() {
        let sql = r"
DROP SUBSCRIPTION mysub;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_inh() {
        let sql = r"
ALTER TABLE ONLY public.tasks
    DROP CONSTRAINT tasks_fk,
    ADD CONSTRAINT tasks_fk
        FOREIGN KEY (job_id) REFERENCES public.jobs(external_id)
            ON DELETE CASCADE NOT VALID;

ALTER TABLE public.tasks VALIDATE CONSTRAINT tasks_fk;
";
        let res = parse_sql_query(sql);
        assert!(res.is_ok());
        assert_debug_snapshot!(res);
    }
    #[test]
    fn parse_alter_constraint_regression() {
        let sql = r#"
ALTER TABLE "table" ALTER CONSTRAINT "constraint" DEFERRABLE INITIALLY IMMEDIATE;

ALTER TABLE "table" ALTER CONSTRAINT "constraint" NOT DEFERRABLE;
"#;
        let res = parse_sql_query(sql);
        assert!(res.is_ok());
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_generated_column() {
        let sql = r"
ALTER TABLE table_c ADD column c boolean GENERATED ALWAYS AS (p IS NOT NULL) STORED NOT NULL;
";
        let res = parse_sql_query(sql);
        assert!(res.is_ok());
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_create_table_regression() {
        let sql = r"
CREATE TABLE example (
    a integer,
    b integer,
    c integer,
    PRIMARY KEY (a, c)
);
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_create_table_partition() {
        let sql = r"
CREATE TABLE measurement_y2006m02 PARTITION OF measurement
    FOR VALUES FROM ('2006-02-01') TO ('2006-03-01');
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_attach_table_partition() {
        let sql = r"
ALTER TABLE measurement ATTACH PARTITION measurement_y2008m02
    FOR VALUES FROM ('2008-02-01') TO ('2008-03-01' );
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn parse_detach_table_partition() {
        let sql = r"
ALTER TABLE measurement
    DETACH PARTITION measurement_y2006m02;
";
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }

    #[test]
    fn drop_index() {
        let sql = r#"
DROP INDEX "email_idx";
DROP INDEX IF EXISTS "email_idx";
DROP INDEX CONCURRENTLY "email_idx";
DROP INDEX CONCURRENTLY "email_idx";
DROP INDEX CONCURRENTLY "email_idx" RESTRICT;

DROP INDEX "email_idx", "name_idx";
DROP INDEX "email_idx", "name_idx" CASCADE;
DROP INDEX "email_idx", "name_idx" RESTRICT;

DROP INDEX IF EXISTS "email_idx", "name_idx";
DROP INDEX IF EXISTS "email_idx", "name_idx" CASCADE;
DROP INDEX IF EXISTS "email_idx", "name_idx" RESTRICT;

DROP INDEX CONCURRENTLY IF EXISTS "email_idx", "name_idx";
DROP INDEX CONCURRENTLY IF EXISTS "email_idx", "name_idx" CASCADE;
DROP INDEX CONCURRENTLY IF EXISTS "email_idx", "name_idx" RESTRICT;
"#;
        let res = parse_sql_query(sql);
        assert_debug_snapshot!(res);
    }
}
