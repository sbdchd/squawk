--
-- Tests for psql features that aren't closely connected to any
-- specific server features
--

-- \set

-- fail: invalid name
-- fail: invalid value for special variable
-- check handling of built-in boolean variable

-- \g and \gx

SELECT 1 as one, 2 as two \g
SELECT 3 as three, 4 as four \gx

-- \gx should work in FETCH_COUNT mode too

SELECT 1 as one, 2 as two \g
SELECT 3 as three, 4 as four \gx


-- \g/\gx with pset options

SELECT 1 as one, 2 as two \g (format=csv csv_fieldsep='\t')
SELECT 1 as one, 2 as two \gx (title='foo bar')

-- \parse (extended query protocol)
SELECT 1 \parse ''
SELECT 2 \parse stmt1
SELECT $1 \parse stmt2
SELECT $1, $2 \parse stmt3

-- \bind_named (extended query protocol)
-- Repeated calls.  The second call generates an error, cleaning up the
-- statement name set by the first call.
-- Last \bind_named wins
-- Multiple \g calls mean multiple executions

-- \close (extended query protocol)
SELECT name, statement FROM pg_prepared_statements ORDER BY name;

-- \bind (extended query protocol)
SELECT 1 \bind \g
SELECT $1 \bind 'foo' \g
SELECT $1, $2 \bind 'foo' 'bar' \g

-- last \bind wins
select $1::int as col \bind 'foo' \bind 2 \g
-- Multiple \g calls mean multiple executions
select $1::int as col \bind 1 \g \bind 2 \g

-- errors
-- parse error
SELECT foo \bind \g
-- tcop error
SELECT 1 \; SELECT 2 \bind \g
-- bind error
SELECT $1, $2 \bind 'foo' \g
-- bind_named error

-- ;

select 10 as test01, 20 as test02, 'Hello' as test03 ; pref01_


-- should fail: bad variable name
select 10 as "bad name"

select 97 as "EOF", 'ok' as _foo ; IGNORE

-- multiple backslash commands in one line
select 1 as x, 2 as y ; pref01_ \\ \echo 'pref01_x'
select 3 as x, 4 as y ; pref01_ \echo 'pref01_x' \echo 'pref01_y'
select 5 as x, 6 as y ; pref01_ \\ \g \echo 'pref01_x' 'pref01_y'
select 7 as x, 8 as y \g ; pref01_ \echo 'pref01_x' 'pref01_y'

-- NULL should unset the variable
select 1 as var1, NULL as var2, 3 as var3 ;

-- ; requires just one tuple
select 10 as test01, 20 as test02 from generate_series(1,3) ;
select 10 as test01, 20 as test02 from generate_series(1,0) ;

-- ; returns no tuples
select a from generate_series(1, 10) as a where a = 11 ;

-- ; should work in FETCH_COUNT mode too

select 1 as x, 2 as y ; pref01_ \\ \echo 'pref01_x'
select 3 as x, 4 as y ; pref01_ \echo 'pref01_x' \echo 'pref01_y'
select 10 as test01, 20 as test02 from generate_series(1,3) ;
select 10 as test01, 20 as test02 from generate_series(1,0) ;


-- \gdesc

SELECT
    NULL AS zero,
    1 AS one,
    2.0 AS two,
    'three' AS three,
    $1 AS four,
    sin($2) as five,
    'foo'::varchar(4) as six,
    CURRENT_DATE AS now

-- should work with tuple-returning utilities, such as EXECUTE
PREPARE test AS SELECT 1 AS first, 2 AS second;
EXECUTE test \gdesc
EXPLAIN EXECUTE test \gdesc

-- should fail cleanly - syntax error
SELECT 1 + \gdesc

-- check behavior with empty results
SELECT \gdesc
CREATE TABLE bububu(a int) \gdesc

-- subject command should not have executed
TABLE bububu;  -- fail

-- query buffer should remain unchanged
SELECT 1 AS x, 'Hello', 2 AS y, true AS "dirty\name"

-- all on one line
SELECT 3 AS x, 'Hello', 4 AS y, true AS "dirty\name" \gdesc \g

-- test for server bug #17983 with empty statement in aborted transaction
set search_path = default;
begin;
bogus;
;
rollback;

-- \gexec

create temporary table gexec_test(a int, b text, c date, d float);
select format('create index on gexec_test(%I)', attname)
from pg_attribute
where attrelid = 'gexec_test'::regclass and attnum > 0
order by attnum

-- \gexec should work in FETCH_COUNT mode too
-- (though the fetch limit applies to the executed queries not the meta query)

select 'select 1 as ones', 'select x.y, x.y*2 as double from generate_series(1,4) as x(y)'
union all
select 'drop table gexec_test', NULL
union all
select 'drop table gexec_test', 'select ''2000-01-01''::date as party_over'


-- \setenv, \getenv

-- ensure MYVAR isn't set
-- in which case, reading it doesn't change the target
-- now set it

-- show all pset options

-- test multi-line headers, wrapping, and newline indicators
-- in aligned, unaligned, and wrapped formats
prepare q as select array_to_string(array_agg(repeat('x',2*n)),E'\n') as "ab

c", array_to_string(array_agg(repeat('y',20-2*n)),E'\n') as "a
bc" from generate_series(1,10) as n(n) group by n>1 order by n>1;



execute q;
execute q;
execute q;

execute q;
execute q;
execute q;

execute q;
execute q;
execute q;


execute q;
execute q;
execute q;

execute q;
execute q;
execute q;

execute q;
execute q;
execute q;



execute q;
execute q;
execute q;

execute q;
execute q;
execute q;

execute q;
execute q;
execute q;


execute q;
execute q;
execute q;

execute q;
execute q;
execute q;

execute q;
execute q;
execute q;

deallocate q;

-- test single-line header and data
prepare q as select repeat('x',2*n) as "0123456789abcdef", repeat('y',20-2*n) as "0123456789" from generate_series(1,10) as n;



execute q;
execute q;
execute q;

execute q;
execute q;
execute q;

execute q;
execute q;
execute q;


execute q;
execute q;
execute q;

execute q;
execute q;
execute q;

execute q;
execute q;
execute q;


execute q;
execute q;
execute q;

execute q;
execute q;
execute q;

execute q;
execute q;
execute q;



execute q;
execute q;
execute q;

execute q;
execute q;
execute q;

execute q;
execute q;
execute q;


execute q;
execute q;
execute q;

execute q;
execute q;
execute q;

execute q;
execute q;
execute q;

deallocate q;


-- support table for output-format tests (useful to create a footer)

create table psql_serial_tab (id serial);

-- test header/footer/tuples_only behavior in aligned/unaligned/wrapped cases


-- empty table is a special case for this format
select 1 where false;





-- check conditional am display

CREATE SCHEMA tableam_display;
CREATE ROLE regress_display_role;
ALTER SCHEMA tableam_display OWNER TO regress_display_role;
SET search_path TO tableam_display;
CREATE ACCESS METHOD heap_psql TYPE TABLE HANDLER heap_tableam_handler;
SET ROLE TO regress_display_role;
-- Use only relations with a physical size of zero.
CREATE TABLE tbl_heap_psql(f1 int, f2 char(100)) using heap_psql;
CREATE TABLE tbl_heap(f1 int, f2 char(100)) using heap;
CREATE VIEW view_heap_psql AS SELECT f1 from tbl_heap_psql;
CREATE MATERIALIZED VIEW mat_view_heap_psql USING heap_psql AS SELECT f1 from tbl_heap_psql;
-- AM is displayed for tables, indexes and materialized views.
-- But not for views and sequences.
-- \d with 'x' enables expanded mode, but only without a pattern
RESET ROLE;
RESET search_path;
DROP SCHEMA tableam_display CASCADE;
DROP ACCESS METHOD heap_psql;
DROP ROLE regress_display_role;

-- test numericlocale (as best we can without control of psql's locale)


select n, -n as m, n * 111 as x, '1e90'::float8 as f
from generate_series(0,3) n;


-- test asciidoc output format



prepare q as
  select 'some|text' as "a|title", '        ' as "empty ", n as int
  from generate_series(1,2) as n;

execute q;

execute q;

execute q;

execute q;

execute q;

execute q;

deallocate q;

-- test csv output format



prepare q as
  select 'some"text' as "a""title", E'  <foo>\n<bar>' as "junk",
         '   ' as "empty", n as int
  from generate_series(1,2) as n;

execute q;

execute q;

deallocate q;

-- special cases
select 'comma,comma' as comma, 'semi;semi' as semi;
select 'comma,comma' as comma, 'semi;semi' as semi;
select '\.' as data;
select '\' as d1, '' as d2;

-- illegal csv separators


-- test html output format



prepare q as
  select 'some"text' as "a&title", E'  <foo>\n<bar>' as "junk",
         '   ' as "empty", n as int
  from generate_series(1,2) as n;

execute q;

execute q;

execute q;

execute q;

execute q;

execute q;

deallocate q;

-- test latex output format



prepare q as
  select 'some\more_text' as "a$title", E'  #<foo>%&^~|\n{bar}' as "junk",
         '   ' as "empty", n as int
  from generate_series(1,2) as n;

execute q;

execute q;

execute q;

execute q;

execute q;

execute q;

execute q;

execute q;

deallocate q;

-- test latex-longtable output format



prepare q as
  select 'some\more_text' as "a$title", E'  #<foo>%&^~|\n{bar}' as "junk",
         '   ' as "empty", n as int
  from generate_series(1,2) as n;

execute q;

execute q;

execute q;

execute q;

execute q;

execute q;

execute q;

execute q;

execute q;

execute q;

deallocate q;

-- test troff-ms output format



prepare q as
  select 'some\text' as "a\title", E'  <foo>\n<bar>' as "junk",
         '   ' as "empty", n as int
  from generate_series(1,2) as n;

execute q;

execute q;

execute q;

execute q;

execute q;

execute q;

deallocate q;

-- check ambiguous format requests


-- clean up after output format tests

drop table psql_serial_tab;


-- \echo and allied features





-- tests for \if ... \endif

  select 'okay';
  select 'still okay';
  not okay;
  still not okay

-- at this point query buffer should still have last valid line

-- \if should work okay on part of a query
select
  \if true
    42
  \else
    (bogus
  \endif
  forty_two;

select \if false \\ (bogus \else \\ 42 \endif \\ forty_two;

-- test a large nested if using a variety of true-equivalents
	\if 1
		\if yes
			\if on
				\echo 'all true'
			\else
				\echo 'should not print #1-1'
			\endif
		\else
			\echo 'should not print #1-2'
		\endif
	\else
		\echo 'should not print #1-3'
	\endif
	\echo 'should not print #1-4'

-- test a variety of false-equivalents in an if/elif/else structure
	\echo 'should not print #2-1'
	\echo 'should not print #2-2'
	\echo 'should not print #2-3'
	\echo 'should not print #2-4'
	\echo 'all false'

-- test true-false elif after initial true branch
	\echo 'should print #2-5'
	\echo 'should not print #2-6'
	\echo 'should not print #2-7'
	\echo 'should not print #2-8'

-- test simple true-then-else
	\echo 'first thing true'
	\echo 'should not print #3-1'

-- test simple false-true-else
	\echo 'should not print #4-1'
	\echo 'second thing true'
	\echo 'should not print #5-1'

-- invalid boolean expressions are false
	\echo 'will not print #6-1'
	\echo 'will print anyway #6-2'

-- test un-matched endif

-- test un-matched else

-- test un-matched elif

-- test double-else error

-- test elif out-of-order

-- test if-endif matching in a false branch
    \if false
        \echo 'should not print #7-1'
    \else
        \echo 'should not print #7-2'
    \endif
    \echo 'should not print #7-3'
    \echo 'should print #7-4'

-- show that vars and backticks are not expanded when ignoring extra args

-- show that vars and backticks are not expanded and commands are ignored
-- when in a false if-branch
	'try_to_quit'
	\echo `nosuchcommand` 'foo' 'foo' :"foo"
	\pset fieldsep | `nosuchcommand` 'foo' 'foo' :"foo"
	\a
	SELECT $1 \bind 1 \g
	\bind_named stmt1 1 2 \g
	\C arg1
	\c arg1 arg2 arg3 arg4
	\cd arg1
	\close stmt1
	\conninfo
	\copy arg1 arg2 arg3 arg4 arg5 arg6
	\copyright
	SELECT 1 as one, 2, 3 \crosstabview
	\dt arg1
	\e arg1 arg2
	\ef whole_line
	\ev whole_line
	\echo arg1 arg2 arg3 arg4 arg5
	\echo arg1
	\encoding arg1
	\endpipeline
	\errverbose
	\f arg1
	\flush
	\flushrequest
	\g arg1
	\gx arg1
	\gexec
	\getresults
	SELECT 1 AS one ;
	\h
	\?
	\html
	\i arg1
	\ir arg1
	\l arg1
	\lo arg1 arg2
	\lo_list
	\o arg1
	\p
	SELECT 1 \parse
	\password arg1
	\prompt arg1 arg2
	\pset arg1 arg2
	\q
	\reset
	\s arg1
	\sendpipeline
	\set arg1 arg2 arg3 arg4 arg5 arg6 arg7
	\setenv arg1 arg2
	\sf whole_line
	\sv whole_line
	\startpipeline
	\syncpipeline
	\t arg1
	\T arg1
	\timing arg1
	\unset arg1
	\w arg1
	\watch arg1 arg2
	\x arg1
	-- \else here is eaten as part of OT_FILEPIPE argument
	\w |/no/such/file \else
	-- \endif here is eaten as part of whole-line argument
	\! whole_line \endif
	\z
	\echo 'should print #8-1'

-- :{?...} defined variable test
  \echo '#9-1 ok, variable i is defined'
  \echo 'should not print #9-2'

  \echo 'should not print #10-1'
  \echo '#10-2 ok, variable no_such_variable is not defined'

SELECT :{?i} AS i_is_defined;

SELECT NOT :{?no_such_var} AS no_such_var_is_not_defined;

-- SHOW_CONTEXT

do $$
begin
  raise notice 'foo';
  raise exception 'bar';
end $$;

do $$
begin
  raise notice 'foo';
  raise exception 'bar';
end $$;

do $$
begin
  raise notice 'foo';
  raise exception 'bar';
end $$;

-- test printing and clearing the query buffer
SELECT 1;
SELECT 2 \r
SELECT 3 \p
UNION SELECT 4 \p
UNION SELECT 5
ORDER BY 1;

-- tests for special result variables

-- working query, 2 rows selected
SELECT 1 AS stuff UNION SELECT 2;

-- syntax error
SELECT 1 UNION;

-- empty query
;
-- must have kept previous values

-- other query error
DROP TABLE this_table_does_not_exist;

-- nondefault verbosity error settings (except verbose, which is too unstable)
SELECT 1 UNION;

SELECT 1/0;


-- working \gdesc
SELECT 3 AS three, 4 AS four \gdesc

-- \gdesc with an error
SELECT 4 AS \gdesc

-- check row count for a query with chunked results
select unique2 from tenk1 order by unique2 limit 19;

-- chunked results with an error after the first chunk
-- (we must disable parallel query here, else the behavior is timing-dependent)
set debug_parallel_query = off;
select 1/(15-unique2) from tenk1 order by unique2 limit 19;
reset debug_parallel_query;


create schema testpart;
create role regress_partitioning_role;

alter schema testpart owner to regress_partitioning_role;

set role to regress_partitioning_role;

-- run test inside own schema and hide other partitions
set search_path to testpart;

create table testtable_apple(logdate date);
create table testtable_orange(logdate date);
create index testtable_apple_index on testtable_apple(logdate);
create index testtable_orange_index on testtable_orange(logdate);

create table testpart_apple(logdate date) partition by range(logdate);
create table testpart_orange(logdate date) partition by range(logdate);

create index testpart_apple_index on testpart_apple(logdate);
create index testpart_orange_index on testpart_orange(logdate);

-- only partition related object should be displayed

drop table testtable_apple;
drop table testtable_orange;
drop table testpart_apple;
drop table testpart_orange;

create table parent_tab (id int) partition by range (id);
create index parent_index on parent_tab (id);
create table child_0_10 partition of parent_tab
  for values from (0) to (10);
create table child_10_20 partition of parent_tab
  for values from (10) to (20);
create table child_20_30 partition of parent_tab
  for values from (20) to (30);
insert into parent_tab values (generate_series(0,29));
create table child_30_40 partition of parent_tab
for values from (30) to (40)
  partition by range(id);
create table child_30_35 partition of child_30_40
  for values from (30) to (35);
create table child_35_40 partition of child_30_40
   for values from (35) to (40);
insert into parent_tab values (generate_series(30,39));




drop table parent_tab cascade;

drop schema testpart;

set search_path to default;

set role to default;
drop role regress_partitioning_role;

-- \d on toast table (use pg_statistic's toast table, which has a known name)

-- check printing info about access methods

-- check \dconfig
set work_mem = 10240;
reset work_mem;

-- check \df, \do with argument specifications

-- check \df+
-- we have to use functions with a predictable owner name, so make a role
create role regress_psql_user superuser;
begin;
set session authorization regress_psql_user;

create function psql_df_internal (float8)
  returns float8
  language internal immutable parallel safe strict
  as 'dsin';
create function psql_df_sql (x integer)
  returns integer
  security definer
  begin atomic select x + 1; end;
create function psql_df_plpgsql ()
  returns void
  language plpgsql
  as $$ begin return; end; $$;
comment on function psql_df_plpgsql () is 'some comment';

rollback;
drop role regress_psql_user;

-- check \sf

-- AUTOCOMMIT

CREATE TABLE ac_test (a int);

INSERT INTO ac_test VALUES (1);
COMMIT;
SELECT * FROM ac_test;
COMMIT;

INSERT INTO ac_test VALUES (2);
ROLLBACK;
SELECT * FROM ac_test;
COMMIT;

BEGIN;
INSERT INTO ac_test VALUES (3);
COMMIT;
SELECT * FROM ac_test;
COMMIT;

BEGIN;
INSERT INTO ac_test VALUES (4);
ROLLBACK;
SELECT * FROM ac_test;
COMMIT;

DROP TABLE ac_test;
SELECT * FROM ac_test;  -- should be gone now

-- ON_ERROR_ROLLBACK

CREATE TABLE oer_test (a int);

BEGIN;
INSERT INTO oer_test VALUES (1);
INSERT INTO oer_test VALUES ('foo');
INSERT INTO oer_test VALUES (3);
COMMIT;
SELECT * FROM oer_test;

BEGIN;
INSERT INTO oer_test VALUES (4);
ROLLBACK;
SELECT * FROM oer_test;

BEGIN;
INSERT INTO oer_test VALUES (5);
COMMIT AND CHAIN;
INSERT INTO oer_test VALUES (6);
COMMIT;
SELECT * FROM oer_test;

DROP TABLE oer_test;

-- ECHO errors
SELECT * FROM notexists;

--
-- combined queries
--
CREATE FUNCTION warn(msg TEXT) RETURNS BOOLEAN LANGUAGE plpgsql
AS $$
  BEGIN RAISE NOTICE 'warn %', msg ; RETURN TRUE ; END
$$;

-- show both
SELECT 1 AS one \; SELECT warn('1.5') \; SELECT 2 AS two ;
-- ; applies to last query only
SELECT 3 AS three \; SELECT warn('3.5') \; SELECT 4 AS four ;
-- syntax error stops all processing
SELECT 5 \; SELECT 6 + \; SELECT warn('6.5') \; SELECT 7 ;
-- with aborted transaction, stop on first error
BEGIN \; SELECT 8 AS eight \; SELECT 9/0 AS nine \; ROLLBACK \; SELECT 10 AS ten ;
-- close previously aborted transaction
ROLLBACK;

-- miscellaneous SQL commands
-- (non SELECT output is sent to stderr, thus is not shown in expected results)
SELECT 'ok' AS "begin" \;
CREATE TABLE psql_comics(s TEXT) \;
INSERT INTO psql_comics VALUES ('Calvin'), ('hobbes') \;

SELECT 1 AS one \; SELECT warn('1.5') \; SELECT 2 AS two ;

DROP FUNCTION warn(TEXT);

--
-- \g with file
--

CREATE TEMPORARY TABLE reload_output(
  lineno int NOT NULL GENERATED ALWAYS AS IDENTITY,
  line text
);

SELECT 1 AS a \g 'g_out_file'
COPY reload_output(line) FROM 'g_out_file';
SELECT 2 AS b\; SELECT 3 AS c\; SELECT 4 AS d \g 'g_out_file'
COPY reload_output(line) FROM 'g_out_file';
COPY (SELECT 'foo') TO STDOUT \; COPY (SELECT 'bar') TO STDOUT \g 'g_out_file'
COPY reload_output(line) FROM 'g_out_file';

SELECT line FROM reload_output ORDER BY lineno;
TRUNCATE TABLE reload_output;

--
-- \o with file
--

SELECT max(unique1) FROM onek;
SELECT 1 AS a\; SELECT 2 AS b\; SELECT 3 AS c;

-- COPY TO file
-- The data goes to 'g_out_file' and the status to 'o_out_file'
COPY (SELECT unique1 FROM onek ORDER BY unique1 LIMIT 10) TO 'g_out_file';
-- DML command status
UPDATE onek SET unique1 = unique1 WHERE false;

-- Check the contents of the files generated.
COPY reload_output(line) FROM 'g_out_file';
SELECT line FROM reload_output ORDER BY lineno;
TRUNCATE TABLE reload_output;
COPY reload_output(line) FROM 'o_out_file';
SELECT line FROM reload_output ORDER BY lineno;
TRUNCATE TABLE reload_output;

-- Multiple COPY TO STDOUT with output file
-- The data goes to 'o_out_file' with no status generated.
COPY (SELECT 'foo1') TO STDOUT \; COPY (SELECT 'bar1') TO STDOUT;
-- Combination of \o and \g file with multiple COPY queries.
COPY (SELECT 'foo2') TO STDOUT \; COPY (SELECT 'bar2') TO STDOUT \g 'g_out_file'

-- Check the contents of the files generated.
COPY reload_output(line) FROM 'g_out_file';
SELECT line FROM reload_output ORDER BY lineno;
TRUNCATE TABLE reload_output;
COPY reload_output(line) FROM 'o_out_file';
SELECT line FROM reload_output ORDER BY lineno;

DROP TABLE reload_output;

--
-- AUTOCOMMIT and combined queries
--
-- BEGIN is now implicit

CREATE TABLE foo(s TEXT) \;
ROLLBACK;

CREATE TABLE foo(s TEXT) \;
INSERT INTO foo(s) VALUES ('hello'), ('world') \;
COMMIT;

DROP TABLE foo \;
ROLLBACK;

-- table foo is still there
SELECT * FROM foo ORDER BY 1 \;
DROP TABLE foo \;
COMMIT;

-- BEGIN now explicit for multi-statement transactions

BEGIN \;
CREATE TABLE foo(s TEXT) \;
INSERT INTO foo(s) VALUES ('hello'), ('world') \;
COMMIT;

BEGIN \;
DROP TABLE foo \;
ROLLBACK \;

-- implicit transactions
SELECT * FROM foo ORDER BY 1 \;
DROP TABLE foo;

--
-- test ON_ERROR_ROLLBACK and combined queries
--
CREATE FUNCTION psql_error(msg TEXT) RETURNS BOOLEAN AS $$
  BEGIN
    RAISE EXCEPTION 'error %', msg;
  END;
$$ LANGUAGE plpgsql;


BEGIN;
CREATE TABLE bla(s NO_SUCH_TYPE);               -- fails
CREATE TABLE bla(s TEXT);                       -- succeeds
SELECT psql_error('oops!');                     -- fails
INSERT INTO bla VALUES ('Calvin'), ('Hobbes');
COMMIT;

SELECT * FROM bla ORDER BY 1;

BEGIN;
INSERT INTO bla VALUES ('Susie');         -- succeeds
-- now with combined queries
INSERT INTO bla VALUES ('Rosalyn') \;     -- will rollback
SELECT 'before error' AS show \;          -- will show nevertheless!
  SELECT psql_error('boum!') \;           -- failure
  SELECT 'after error' AS noshow;         -- hidden by preceding error
INSERT INTO bla(s) VALUES ('Moe') \;      -- will rollback
  SELECT psql_error('bam!');
INSERT INTO bla VALUES ('Miss Wormwood'); -- succeeds
COMMIT;
SELECT * FROM bla ORDER BY 1;

-- some with autocommit off

-- implicit BEGIN
INSERT INTO bla VALUES ('Dad');           -- succeeds
SELECT psql_error('bad!');                -- implicit partial rollback

INSERT INTO bla VALUES ('Mum') \;         -- will rollback
SELECT COUNT(*) AS "#mum"
FROM bla WHERE s = 'Mum' \;               -- but be counted here
SELECT psql_error('bad!');                -- implicit partial rollback
COMMIT;

SELECT COUNT(*) AS "#mum"
FROM bla WHERE s = 'Mum' \;               -- no mum here
SELECT * FROM bla ORDER BY 1;
COMMIT;

-- reset all
DROP TABLE bla;
DROP FUNCTION psql_error;

-- check describing invalid multipart names

-- check that dots within quoted name segments are not counted

-- again, but with dotted schema qualifications.

-- again, but with current database and dotted schema qualifications.

-- again, but with dotted database and dotted schema qualifications.

-- check \drg and \du
CREATE ROLE regress_du_role0;
CREATE ROLE regress_du_role1;
CREATE ROLE regress_du_role2;
CREATE ROLE regress_du_admin;

GRANT regress_du_role0 TO regress_du_admin WITH ADMIN TRUE;
GRANT regress_du_role1 TO regress_du_admin WITH ADMIN TRUE;
GRANT regress_du_role2 TO regress_du_admin WITH ADMIN TRUE;

GRANT regress_du_role0 TO regress_du_role1 WITH ADMIN TRUE,  INHERIT TRUE,  SET TRUE  GRANTED BY regress_du_admin;
GRANT regress_du_role0 TO regress_du_role2 WITH ADMIN TRUE,  INHERIT FALSE, SET FALSE GRANTED BY regress_du_admin;
GRANT regress_du_role1 TO regress_du_role2 WITH ADMIN TRUE , INHERIT FALSE, SET TRUE  GRANTED BY regress_du_admin;
GRANT regress_du_role0 TO regress_du_role1 WITH ADMIN FALSE, INHERIT TRUE,  SET FALSE GRANTED BY regress_du_role1;
GRANT regress_du_role0 TO regress_du_role2 WITH ADMIN FALSE, INHERIT TRUE , SET TRUE  GRANTED BY regress_du_role1;
GRANT regress_du_role0 TO regress_du_role1 WITH ADMIN FALSE, INHERIT FALSE, SET TRUE  GRANTED BY regress_du_role2;
GRANT regress_du_role0 TO regress_du_role2 WITH ADMIN FALSE, INHERIT FALSE, SET FALSE GRANTED BY regress_du_role2;


DROP ROLE regress_du_role0;
DROP ROLE regress_du_role1;
DROP ROLE regress_du_role2;
DROP ROLE regress_du_admin;

-- Test display of empty privileges.
BEGIN;
-- Create an owner for tested objects because output contains owner name.
CREATE ROLE regress_zeropriv_owner;
SET LOCAL ROLE regress_zeropriv_owner;

CREATE DOMAIN regress_zeropriv_domain AS int;
REVOKE ALL ON DOMAIN regress_zeropriv_domain FROM CURRENT_USER, PUBLIC;

CREATE PROCEDURE regress_zeropriv_proc() LANGUAGE sql AS '';
REVOKE ALL ON PROCEDURE regress_zeropriv_proc() FROM CURRENT_USER, PUBLIC;

CREATE TABLE regress_zeropriv_tbl (a int);
REVOKE ALL ON TABLE regress_zeropriv_tbl FROM CURRENT_USER;

CREATE TYPE regress_zeropriv_type AS (a int);
REVOKE ALL ON TYPE regress_zeropriv_type FROM CURRENT_USER, PUBLIC;

ROLLBACK;

-- Test display of default privileges with \pset null.
CREATE TABLE defprivs (a int);
DROP TABLE defprivs;
