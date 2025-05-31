CREATE TEMP TABLE x (
	a serial,
	b int,
	c text not null default 'stuff',
	d text,
	e text
) ;

CREATE FUNCTION fn_x_before () RETURNS TRIGGER AS '
  BEGIN
		NEW.e := ''before trigger fired''::text;
		return NEW;
	END;
' LANGUAGE plpgsql;

CREATE FUNCTION fn_x_after () RETURNS TRIGGER AS '
  BEGIN
		UPDATE x set e=''after trigger fired'' where c=''stuff'';
		return NULL;
	END;
' LANGUAGE plpgsql;

CREATE TRIGGER trg_x_after AFTER INSERT ON x
FOR EACH ROW EXECUTE PROCEDURE fn_x_after();

CREATE TRIGGER trg_x_before BEFORE INSERT ON x
FOR EACH ROW EXECUTE PROCEDURE fn_x_before();





-- non-existent column in column list: should fail
-- redundant options
-- incorrect options
-- too many columns in column list: should fail
-- missing data: should fail

-- extra data: should fail

-- various COPY options: delimiters, oids, NULL string, encoding



COPY x TO stdout WHERE a = 1;



-- check results of copy in
SELECT * FROM x;

-- check copy out
COPY x TO stdout;
COPY x (c, e) TO stdout;
COPY x (b, e) TO stdout WITH NULL 'I''m null';

CREATE TEMP TABLE y (
	col1 text,
	col2 text
);

INSERT INTO y VALUES ('Jackson, Sam', E'\\h');
INSERT INTO y VALUES ('It is "perfect".',E'\t');
INSERT INTO y VALUES ('', NULL);

COPY y TO stdout WITH CSV;
COPY y TO stdout WITH CSV QUOTE '''' DELIMITER '|';
COPY y TO stdout WITH CSV FORCE QUOTE col2 ESCAPE E'\\' ENCODING 'sql_ascii';
COPY y TO stdout WITH CSV FORCE QUOTE *;

-- Repeat above tests with new 9.0 option syntax

COPY y TO stdout (FORMAT CSV);
COPY y TO stdout (FORMAT CSV, QUOTE '''', DELIMITER '|');
COPY y TO stdout (FORMAT CSV, FORCE_QUOTE (col2), ESCAPE E'\\');
COPY y TO stdout (FORMAT CSV, FORCE_QUOTE *);


--test that we read consecutive LFs properly

CREATE TEMP TABLE testnl (a int, b text, c int);

-- inside",2

-- test end of copy marker
CREATE TEMP TABLE testeoc (a text);

-- c\.d
-- "\."

COPY testeoc TO stdout CSV;

-- test handling of nonstandard null marker that violates escaping rules

CREATE TEMP TABLE testnull(a int, b text);
INSERT INTO testnull VALUES (1, E'\\0'), (NULL, NULL);

COPY testnull TO stdout WITH NULL AS E'\\0';


SELECT * FROM testnull;

BEGIN;
CREATE TABLE vistest (LIKE testeoc);
COMMIT;
SELECT * FROM vistest;
BEGIN;
TRUNCATE vistest;
SELECT * FROM vistest;
SAVEPOINT s1;
TRUNCATE vistest;
SELECT * FROM vistest;
COMMIT;
SELECT * FROM vistest;

BEGIN;
TRUNCATE vistest;
SELECT * FROM vistest;
SAVEPOINT s1;
TRUNCATE vistest;
SELECT * FROM vistest;
COMMIT;
SELECT * FROM vistest;

BEGIN;
TRUNCATE vistest;
SELECT * FROM vistest;
COMMIT;
TRUNCATE vistest;
BEGIN;
TRUNCATE vistest;
SAVEPOINT s1;
COMMIT;
BEGIN;
INSERT INTO vistest VALUES ('z');
SAVEPOINT s1;
TRUNCATE vistest;
ROLLBACK TO SAVEPOINT s1;
COMMIT;
CREATE FUNCTION truncate_in_subxact() RETURNS VOID AS
$$
BEGIN
	TRUNCATE vistest;
EXCEPTION
  WHEN OTHERS THEN
	INSERT INTO vistest VALUES ('subxact failure');
END;
$$ language plpgsql;
BEGIN;
INSERT INTO vistest VALUES ('z');
SELECT truncate_in_subxact();
SELECT * FROM vistest;
COMMIT;
SELECT * FROM vistest;
-- Test FORCE_NOT_NULL and FORCE_NULL options
CREATE TEMP TABLE forcetest (
    a INT NOT NULL,
    b TEXT NOT NULL,
    c TEXT,
    d TEXT,
    e TEXT
);
-- should succeed with no effect ("b" remains an empty string, "c" remains NULL)
BEGIN;
COMMIT;
SELECT b, c FROM forcetest WHERE a = 1;
-- should succeed, FORCE_NULL and FORCE_NOT_NULL can be both specified
BEGIN;
COMMIT;
SELECT c, d FROM forcetest WHERE a = 2;
-- should fail with not-null constraint violation
BEGIN;
ROLLBACK;
-- should fail with "not referenced by COPY" error
BEGIN;
COMMIT;
SELECT b, c FROM forcetest WHERE a = 4;
-- should succeed with effect ("b" remains an empty string)
BEGIN;
COMMIT;
SELECT b, c FROM forcetest WHERE a = 5;
-- should succeed with effect ("c" remains NULL)
BEGIN;
COMMIT;
SELECT b, c FROM forcetest WHERE a = 6;
-- should fail with "conflicting or redundant options" error
BEGIN;

-- test case with whole-row Var in a check constraint
create table check_con_tbl (f1 int);
create function check_con_function(check_con_tbl) returns bool as $$
begin
  raise notice 'input = %', row_to_json($1);
  return $1.f1 > 0;
end $$ language plpgsql immutable;
alter table check_con_tbl add check (check_con_function(check_con_tbl.*));
copy check_con_tbl from stdin;
-- 1
copy check_con_tbl from stdin;
-- 0
select * from check_con_tbl;

-- test with RLS enabled.
CREATE ROLE regress_rls_copy_user;
CREATE ROLE regress_rls_copy_user_colperms;
CREATE TABLE rls_t1 (a int, b int, c int);


CREATE POLICY p1 ON rls_t1 FOR SELECT USING (a % 2 = 0);
ALTER TABLE rls_t1 ENABLE ROW LEVEL SECURITY;
ALTER TABLE rls_t1 FORCE ROW LEVEL SECURITY;

GRANT SELECT ON TABLE rls_t1 TO regress_rls_copy_user;
GRANT SELECT (a, b) ON TABLE rls_t1 TO regress_rls_copy_user_colperms;

-- all columns
COPY rls_t1 TO stdout;
COPY rls_t1 (a, b, c) TO stdout;

-- subset of columns
COPY rls_t1 (a) TO stdout;
COPY rls_t1 (a, b) TO stdout;

-- column reordering
COPY rls_t1 (b, a) TO stdout;

SET SESSION AUTHORIZATION regress_rls_copy_user;

-- all columns
COPY rls_t1 TO stdout;
COPY rls_t1 (a, b, c) TO stdout;

-- subset of columns
COPY rls_t1 (a) TO stdout;
COPY rls_t1 (a, b) TO stdout;

-- column reordering
COPY rls_t1 (b, a) TO stdout;

RESET SESSION AUTHORIZATION;

SET SESSION AUTHORIZATION regress_rls_copy_user_colperms;

-- attempt all columns (should fail)
COPY rls_t1 TO stdout;
COPY rls_t1 (a, b, c) TO stdout;

-- try to copy column with no privileges (should fail)
COPY rls_t1 (c) TO stdout;

-- subset of columns (should succeed)
COPY rls_t1 (a) TO stdout;
COPY rls_t1 (a, b) TO stdout;

RESET SESSION AUTHORIZATION;

-- test with INSTEAD OF INSERT trigger on a view
CREATE TABLE instead_of_insert_tbl(id serial, name text);
CREATE VIEW instead_of_insert_tbl_view AS SELECT ''::text AS str;


CREATE FUNCTION fun_instead_of_insert_tbl() RETURNS trigger AS $$
BEGIN
  INSERT INTO instead_of_insert_tbl (name) VALUES (NEW.str);
  RETURN NULL;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER trig_instead_of_insert_tbl_view
  INSTEAD OF INSERT ON instead_of_insert_tbl_view
  FOR EACH ROW EXECUTE PROCEDURE fun_instead_of_insert_tbl();


SELECT * FROM instead_of_insert_tbl;

-- Test of COPY optimization with view using INSTEAD OF INSERT
-- trigger when relation is created in the same transaction as
-- when COPY is executed.
BEGIN;
CREATE VIEW instead_of_insert_tbl_view_2 as select ''::text as str;
CREATE TRIGGER trig_instead_of_insert_tbl_view_2
  INSTEAD OF INSERT ON instead_of_insert_tbl_view_2
  FOR EACH ROW EXECUTE PROCEDURE fun_instead_of_insert_tbl();


SELECT * FROM instead_of_insert_tbl;
COMMIT;

-- tests for on_error option
CREATE TABLE check_ign_err (n int, m int[], k int);
-- 5	{5}	5

-- -- want context for notices

-- 5	{5}	5
-- 6	a
-- 7	{7}	a
-- 8	{8}	8

-- tests for on_error option with log_verbosity and null constraint via domain
CREATE DOMAIN dcheck_ign_err2 varchar(15) NOT NULL;
CREATE TABLE check_ign_err2 (n int, m int[], k int, l dcheck_ign_err2);

-- reset context choice

SELECT * FROM check_ign_err;

SELECT * FROM check_ign_err2;

-- test datatype error that can't be handled as soft: should fail
CREATE TABLE hard_err(foo widget);

-- test missing data: should fail

-- test extra data: should fail

-- tests for reject_limit option
-- 10	{10}	10

-- 10	{10}	10

-- clean up
DROP TABLE forcetest;
DROP TABLE vistest;
DROP FUNCTION truncate_in_subxact();
DROP TABLE x, y;
DROP TABLE rls_t1 CASCADE;
DROP ROLE regress_rls_copy_user;
DROP ROLE regress_rls_copy_user_colperms;
DROP FUNCTION fn_x_before();
DROP FUNCTION fn_x_after();
DROP TABLE instead_of_insert_tbl;
DROP VIEW instead_of_insert_tbl_view;
DROP VIEW instead_of_insert_tbl_view_2;
DROP FUNCTION fun_instead_of_insert_tbl();
DROP TABLE check_ign_err;
DROP TABLE check_ign_err2;
DROP DOMAIN dcheck_ign_err2;
DROP TABLE hard_err;

--
-- COPY FROM ... DEFAULT
--

create temp table copy_default (
	id integer primary key,
	text_value text not null default 'test',
	ts_value timestamp without time zone not null default '2022-07-05'
);

-- if DEFAULT is not specified, then the marker will be regular data
copy copy_default from stdin;
-- 1	value	'2022-07-04'
-- 2	\D	'2022-07-05'

select id, text_value, ts_value from copy_default;

truncate copy_default;

copy copy_default from stdin with (format csv);
-- 1,value,2022-07-04
-- 2,\D,2022-07-05

select id, text_value, ts_value from copy_default;

truncate copy_default;

-- DEFAULT cannot be used in binary mode
copy copy_default from stdin with (format binary, default '\D');

-- DEFAULT cannot be new line nor carriage return
copy copy_default from stdin with (default E'\n');
copy copy_default from stdin with (default E'\r');

-- DELIMITER cannot appear in DEFAULT spec
copy copy_default from stdin with (delimiter ';', default 'test;test');

-- CSV quote cannot appear in DEFAULT spec
copy copy_default from stdin with (format csv, quote '"', default 'test"test');

-- NULL and DEFAULT spec must be different
copy copy_default from stdin with (default '\N');

-- cannot use DEFAULT marker in column that has no DEFAULT value
copy copy_default from stdin with (default '\D');
-- 2	\D	'2022-07-05'

copy copy_default from stdin with (format csv, default '\D');
-- 2,\D,2022-07-05

-- The DEFAULT marker must be unquoted and unescaped or it's not recognized
copy copy_default from stdin with (default '\D');
-- 1	\D	'2022-07-04'
-- 2	\\D	'2022-07-04'
-- 3	"\D"	'2022-07-04'

select id, text_value, ts_value from copy_default;

truncate copy_default;

copy copy_default from stdin with (format csv, default '\D');
-- 1,\D,2022-07-04
-- 2,\\D,2022-07-04
-- 3,"\D",2022-07-04

select id, text_value, ts_value from copy_default;

truncate copy_default;

-- successful usage of DEFAULT option in COPY
copy copy_default from stdin with (default '\D');
-- 1	value	'2022-07-04'
-- 2	\D	'2022-07-03'
-- 3	\D	\D

select id, text_value, ts_value from copy_default;

truncate copy_default;

copy copy_default from stdin with (format csv, default '\D');
-- 1,value,2022-07-04
-- 2,\D,2022-07-03
-- 3,\D,\D

select id, text_value, ts_value from copy_default;

truncate copy_default;

-- DEFAULT cannot be used in COPY TO
copy (select 1 as test) TO stdout with (default '\D');
