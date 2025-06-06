-- JSON()
SELECT JSON();
SELECT JSON(NULL);
SELECT JSON('{ "a" : 1 } ');
SELECT JSON('{ "a" : 1 } ' FORMAT JSON);
SELECT JSON('{ "a" : 1 } ' FORMAT JSON ENCODING UTF8);
SELECT JSON('{ "a" : 1 } '::bytea FORMAT JSON ENCODING UTF8);
SELECT pg_typeof(JSON('{ "a" : 1 } '));

SELECT JSON('   1   '::json);
SELECT JSON('   1   '::jsonb);
SELECT JSON('   1   '::json WITH UNIQUE KEYS);
SELECT JSON(123);

SELECT JSON('{"a": 1, "a": 2}');
SELECT JSON('{"a": 1, "a": 2}' WITH UNIQUE KEYS);
SELECT JSON('{"a": 1, "a": 2}' WITHOUT UNIQUE KEYS);

EXPLAIN (VERBOSE, COSTS OFF) SELECT JSON('123');
EXPLAIN (VERBOSE, COSTS OFF) SELECT JSON('123' FORMAT JSON);
EXPLAIN (VERBOSE, COSTS OFF) SELECT JSON('123'::bytea FORMAT JSON);
EXPLAIN (VERBOSE, COSTS OFF) SELECT JSON('123'::bytea FORMAT JSON ENCODING UTF8);
EXPLAIN (VERBOSE, COSTS OFF) SELECT JSON('123' WITH UNIQUE KEYS);
EXPLAIN (VERBOSE, COSTS OFF) SELECT JSON('123' WITHOUT UNIQUE KEYS);

EXPLAIN (VERBOSE, COSTS OFF) SELECT JSON('123');
SELECT pg_typeof(JSON('123'));

-- JSON_SCALAR()
SELECT JSON_SCALAR();
SELECT JSON_SCALAR(NULL);
SELECT JSON_SCALAR(NULL::int);
SELECT JSON_SCALAR(123);
SELECT JSON_SCALAR(123.45);
SELECT JSON_SCALAR(123.45::numeric);
SELECT JSON_SCALAR(true);
SELECT JSON_SCALAR(false);
SELECT JSON_SCALAR(' 123.45');
SELECT JSON_SCALAR('2020-06-07'::date);
SELECT JSON_SCALAR('2020-06-07 01:02:03'::timestamp);
SELECT JSON_SCALAR('{}'::json);
SELECT JSON_SCALAR('{}'::jsonb);

EXPLAIN (VERBOSE, COSTS OFF) SELECT JSON_SCALAR(123);
EXPLAIN (VERBOSE, COSTS OFF) SELECT JSON_SCALAR('123');

-- JSON_SERIALIZE()
SELECT JSON_SERIALIZE();
SELECT JSON_SERIALIZE(NULL);
SELECT JSON_SERIALIZE(JSON('{ "a" : 1 } '));
SELECT JSON_SERIALIZE('{ "a" : 1 } ');
SELECT JSON_SERIALIZE('1');
SELECT JSON_SERIALIZE('1' FORMAT JSON);
SELECT JSON_SERIALIZE('{ "a" : 1 } ' RETURNING bytea);
SELECT JSON_SERIALIZE('{ "a" : 1 } ' RETURNING varchar);
SELECT pg_typeof(JSON_SERIALIZE(NULL));

-- only string types or bytea allowed
SELECT JSON_SERIALIZE('{ "a" : 1 } ' RETURNING jsonb);


EXPLAIN (VERBOSE, COSTS OFF) SELECT JSON_SERIALIZE('{}');
EXPLAIN (VERBOSE, COSTS OFF) SELECT JSON_SERIALIZE('{}' RETURNING bytea);

-- JSON_OBJECT()
SELECT JSON_OBJECT();
SELECT JSON_OBJECT(RETURNING json);
SELECT JSON_OBJECT(RETURNING json FORMAT JSON);
SELECT JSON_OBJECT(RETURNING jsonb);
SELECT JSON_OBJECT(RETURNING jsonb FORMAT JSON);
SELECT JSON_OBJECT(RETURNING text);
SELECT JSON_OBJECT(RETURNING text FORMAT JSON);
SELECT JSON_OBJECT(RETURNING text FORMAT JSON ENCODING UTF8);
SELECT JSON_OBJECT(RETURNING text FORMAT JSON ENCODING INVALID_ENCODING);
SELECT JSON_OBJECT(RETURNING bytea);
SELECT JSON_OBJECT(RETURNING bytea FORMAT JSON);
SELECT JSON_OBJECT(RETURNING bytea FORMAT JSON ENCODING UTF8);
SELECT JSON_OBJECT(RETURNING bytea FORMAT JSON ENCODING UTF16);
SELECT JSON_OBJECT(RETURNING bytea FORMAT JSON ENCODING UTF32);

SELECT JSON_OBJECT('foo': NULL::int FORMAT JSON);
SELECT JSON_OBJECT('foo': NULL::int FORMAT JSON ENCODING UTF8);
SELECT JSON_OBJECT('foo': NULL::json FORMAT JSON);
SELECT JSON_OBJECT('foo': NULL::json FORMAT JSON ENCODING UTF8);
SELECT JSON_OBJECT('foo': NULL::jsonb FORMAT JSON);
SELECT JSON_OBJECT('foo': NULL::jsonb FORMAT JSON ENCODING UTF8);

SELECT JSON_OBJECT(NULL: 1);
SELECT JSON_OBJECT('a': 2 + 3);
SELECT JSON_OBJECT('a' VALUE 2 + 3);
--SELECT JSON_OBJECT(KEY 'a' VALUE 2 + 3);
SELECT JSON_OBJECT('a' || 2: 1);
SELECT JSON_OBJECT(('a' || 2) VALUE 1);
--SELECT JSON_OBJECT('a' || 2 VALUE 1);
--SELECT JSON_OBJECT(KEY 'a' || 2 VALUE 1);
SELECT JSON_OBJECT('a': 2::text);
SELECT JSON_OBJECT('a' VALUE 2::text);
--SELECT JSON_OBJECT(KEY 'a' VALUE 2::text);
SELECT JSON_OBJECT(1::text: 2);
SELECT JSON_OBJECT((1::text) VALUE 2);
--SELECT JSON_OBJECT(1::text VALUE 2);
--SELECT JSON_OBJECT(KEY 1::text VALUE 2);
SELECT JSON_OBJECT(json '[1]': 123);
SELECT JSON_OBJECT(ARRAY[1,2,3]: 'aaa');

SELECT JSON_OBJECT(
	'a': '123',
	1.23: 123,
	'c': json '[ 1,true,{ } ]',
	'd': jsonb '{ "x" : 123.45 }'
);

SELECT JSON_OBJECT(
	'a': '123',
	1.23: 123,
	'c': json '[ 1,true,{ } ]',
	'd': jsonb '{ "x" : 123.45 }'
	RETURNING jsonb
);

/*
SELECT JSON_OBJECT(
	'a': '123',
	KEY 1.23 VALUE 123,
	'c' VALUE json '[1, true, {}]'
);
*/

SELECT JSON_OBJECT('a': '123', 'b': JSON_OBJECT('a': 111, 'b': 'aaa'));
SELECT JSON_OBJECT('a': '123', 'b': JSON_OBJECT('a': 111, 'b': 'aaa' RETURNING jsonb));

SELECT JSON_OBJECT('a': JSON_OBJECT('b': 1 RETURNING text));
SELECT JSON_OBJECT('a': JSON_OBJECT('b': 1 RETURNING text) FORMAT JSON);
SELECT JSON_OBJECT('a': JSON_OBJECT('b': 1 RETURNING bytea));
SELECT JSON_OBJECT('a': JSON_OBJECT('b': 1 RETURNING bytea) FORMAT JSON);

SELECT JSON_OBJECT('a': '1', 'b': NULL, 'c': 2);
SELECT JSON_OBJECT('a': '1', 'b': NULL, 'c': 2 NULL ON NULL);
SELECT JSON_OBJECT('a': '1', 'b': NULL, 'c': 2 ABSENT ON NULL);

SELECT JSON_OBJECT(1: 1, '2': NULL, '3': 1, repeat('x', 1000): 1, 2: repeat('a', 100) WITH UNIQUE);

SELECT JSON_OBJECT(1: 1, '1': NULL WITH UNIQUE);
SELECT JSON_OBJECT(1: 1, '1': NULL ABSENT ON NULL WITH UNIQUE);
SELECT JSON_OBJECT(1: 1, '1': NULL NULL ON NULL WITH UNIQUE RETURNING jsonb);
SELECT JSON_OBJECT(1: 1, '1': NULL ABSENT ON NULL WITH UNIQUE RETURNING jsonb);

SELECT JSON_OBJECT(1: 1, '2': NULL, '1': 1 NULL ON NULL WITH UNIQUE);
SELECT JSON_OBJECT(1: 1, '2': NULL, '1': 1 ABSENT ON NULL WITH UNIQUE);
SELECT JSON_OBJECT(1: 1, '2': NULL, '1': 1 ABSENT ON NULL WITHOUT UNIQUE);
SELECT JSON_OBJECT(1: 1, '2': NULL, '1': 1 ABSENT ON NULL WITH UNIQUE RETURNING jsonb);
SELECT JSON_OBJECT(1: 1, '2': NULL, '1': 1 ABSENT ON NULL WITHOUT UNIQUE RETURNING jsonb);
SELECT JSON_OBJECT(1: 1, '2': NULL, '3': 1, 4: NULL, '5': 'a' ABSENT ON NULL WITH UNIQUE RETURNING jsonb);

-- BUG: https://postgr.es/m/CADXhmgTJtJZK9A3Na_ry%2BXrq-ghjcejBRhcRMzWZvbd__QdgJA%40mail.gmail.com
-- datum_to_jsonb_internal() didn't catch keys that are casts instead of a simple scalar
CREATE TYPE mood AS ENUM ('happy', 'sad', 'neutral');
CREATE FUNCTION mood_to_json(mood) RETURNS json AS $$
  SELECT to_json($1::text);
$$ LANGUAGE sql IMMUTABLE;
CREATE CAST (mood AS json) WITH FUNCTION mood_to_json(mood) AS IMPLICIT;
SELECT JSON_OBJECT('happy'::mood: '123'::jsonb);
DROP CAST (mood AS json);
DROP FUNCTION mood_to_json;
DROP TYPE mood;

-- JSON_ARRAY()
SELECT JSON_ARRAY();
SELECT JSON_ARRAY(RETURNING json);
SELECT JSON_ARRAY(RETURNING json FORMAT JSON);
SELECT JSON_ARRAY(RETURNING jsonb);
SELECT JSON_ARRAY(RETURNING jsonb FORMAT JSON);
SELECT JSON_ARRAY(RETURNING text);
SELECT JSON_ARRAY(RETURNING text FORMAT JSON);
SELECT JSON_ARRAY(RETURNING text FORMAT JSON ENCODING UTF8);
SELECT JSON_ARRAY(RETURNING text FORMAT JSON ENCODING INVALID_ENCODING);
SELECT JSON_ARRAY(RETURNING bytea);
SELECT JSON_ARRAY(RETURNING bytea FORMAT JSON);
SELECT JSON_ARRAY(RETURNING bytea FORMAT JSON ENCODING UTF8);
SELECT JSON_ARRAY(RETURNING bytea FORMAT JSON ENCODING UTF16);
SELECT JSON_ARRAY(RETURNING bytea FORMAT JSON ENCODING UTF32);

SELECT JSON_ARRAY('aaa', 111, true, array[1,2,3], NULL, json '{"a": [1]}', jsonb '["a",3]');

SELECT JSON_ARRAY('a',  NULL, 'b' NULL   ON NULL);
SELECT JSON_ARRAY('a',  NULL, 'b' ABSENT ON NULL);
SELECT JSON_ARRAY(NULL, NULL, 'b' ABSENT ON NULL);
SELECT JSON_ARRAY('a',  NULL, 'b' NULL   ON NULL RETURNING jsonb);
SELECT JSON_ARRAY('a',  NULL, 'b' ABSENT ON NULL RETURNING jsonb);
SELECT JSON_ARRAY(NULL, NULL, 'b' ABSENT ON NULL RETURNING jsonb);

SELECT JSON_ARRAY(JSON_ARRAY('{ "a" : 123 }' RETURNING text));
SELECT JSON_ARRAY(JSON_ARRAY('{ "a" : 123 }' FORMAT JSON RETURNING text));
SELECT JSON_ARRAY(JSON_ARRAY('{ "a" : 123 }' FORMAT JSON RETURNING text) FORMAT JSON);

SELECT JSON_ARRAY(SELECT i FROM (VALUES (1), (2), (NULL), (4)) foo(i));
SELECT JSON_ARRAY(SELECT i FROM (VALUES (NULL::int[]), ('{1,2}'), (NULL), (NULL), ('{3,4}'), (NULL)) foo(i));
SELECT JSON_ARRAY(SELECT i FROM (VALUES (NULL::int[]), ('{1,2}'), (NULL), (NULL), ('{3,4}'), (NULL)) foo(i) RETURNING jsonb);
--SELECT JSON_ARRAY(SELECT i FROM (VALUES (NULL::int[]), ('{1,2}'), (NULL), (NULL), ('{3,4}'), (NULL)) foo(i) NULL ON NULL);
--SELECT JSON_ARRAY(SELECT i FROM (VALUES (NULL::int[]), ('{1,2}'), (NULL), (NULL), ('{3,4}'), (NULL)) foo(i) NULL ON NULL RETURNING jsonb);
SELECT JSON_ARRAY(SELECT i FROM (VALUES (3), (1), (NULL), (2)) foo(i) ORDER BY i);
SELECT JSON_ARRAY(WITH x AS (SELECT 1) VALUES (TRUE));

-- Should fail
SELECT JSON_ARRAY(SELECT FROM (VALUES (1)) foo(i));
SELECT JSON_ARRAY(SELECT i, i FROM (VALUES (1)) foo(i));
SELECT JSON_ARRAY(SELECT * FROM (VALUES (1, 2)) foo(i, j));

-- JSON_ARRAYAGG()
SELECT	JSON_ARRAYAGG(i) IS NULL,
		JSON_ARRAYAGG(i RETURNING jsonb) IS NULL
FROM generate_series(1, 0) i;

SELECT	JSON_ARRAYAGG(i),
		JSON_ARRAYAGG(i RETURNING jsonb)
FROM generate_series(1, 5) i;

SELECT JSON_ARRAYAGG(i ORDER BY i DESC)
FROM generate_series(1, 5) i;

SELECT JSON_ARRAYAGG(i::text::json)
FROM generate_series(1, 5) i;

SELECT JSON_ARRAYAGG(JSON_ARRAY(i, i + 1 RETURNING text) FORMAT JSON)
FROM generate_series(1, 5) i;

SELECT	JSON_ARRAYAGG(NULL),
		JSON_ARRAYAGG(NULL RETURNING jsonb)
FROM generate_series(1, 5);

SELECT	JSON_ARRAYAGG(NULL NULL ON NULL),
		JSON_ARRAYAGG(NULL NULL ON NULL RETURNING jsonb)
FROM generate_series(1, 5);

SELECT
	JSON_ARRAYAGG(bar) as no_options,
	JSON_ARRAYAGG(bar RETURNING jsonb) as returning_jsonb,
	JSON_ARRAYAGG(bar ABSENT ON NULL) as absent_on_null,
	JSON_ARRAYAGG(bar ABSENT ON NULL RETURNING jsonb) as absentonnull_returning_jsonb,
	JSON_ARRAYAGG(bar NULL ON NULL) as null_on_null,
	JSON_ARRAYAGG(bar NULL ON NULL RETURNING jsonb) as nullonnull_returning_jsonb,
	JSON_ARRAYAGG(foo) as row_no_options,
	JSON_ARRAYAGG(foo RETURNING jsonb) as row_returning_jsonb,
	JSON_ARRAYAGG(foo ORDER BY bar) FILTER (WHERE bar > 2) as row_filtered_agg,
	JSON_ARRAYAGG(foo ORDER BY bar RETURNING jsonb) FILTER (WHERE bar > 2) as row_filtered_agg_returning_jsonb
FROM
	(VALUES (NULL), (3), (1), (NULL), (NULL), (5), (2), (4), (NULL)) foo(bar);

SELECT
	bar, JSON_ARRAYAGG(bar) FILTER (WHERE bar > 2) OVER (PARTITION BY foo.bar % 2)
FROM
	(VALUES (NULL), (3), (1), (NULL), (NULL), (5), (2), (4), (NULL), (5), (4)) foo(bar);

-- JSON_OBJECTAGG()
SELECT	JSON_OBJECTAGG('key': 1) IS NULL,
		JSON_OBJECTAGG('key': 1 RETURNING jsonb) IS NULL
WHERE FALSE;

SELECT JSON_OBJECTAGG(NULL: 1);

SELECT JSON_OBJECTAGG(NULL: 1 RETURNING jsonb);

SELECT
	JSON_OBJECTAGG(i: i),
--	JSON_OBJECTAGG(i VALUE i),
--	JSON_OBJECTAGG(KEY i VALUE i),
	JSON_OBJECTAGG(i: i RETURNING jsonb)
FROM
	generate_series(1, 5) i;

SELECT
	JSON_OBJECTAGG(k: v),
	JSON_OBJECTAGG(k: v NULL ON NULL),
	JSON_OBJECTAGG(k: v ABSENT ON NULL),
	JSON_OBJECTAGG(k: v RETURNING jsonb),
	JSON_OBJECTAGG(k: v NULL ON NULL RETURNING jsonb),
	JSON_OBJECTAGG(k: v ABSENT ON NULL RETURNING jsonb)
FROM
	(VALUES (1, 1), (1, NULL), (2, NULL), (3, 3)) foo(k, v);

SELECT JSON_OBJECTAGG(k: v WITH UNIQUE KEYS)
FROM (VALUES (1, 1), (1, NULL), (2, 2)) foo(k, v);

SELECT JSON_OBJECTAGG(k: v ABSENT ON NULL WITH UNIQUE KEYS)
FROM (VALUES (1, 1), (1, NULL), (2, 2)) foo(k, v);

SELECT JSON_OBJECTAGG(k: v ABSENT ON NULL WITH UNIQUE KEYS)
FROM (VALUES (1, 1), (0, NULL), (3, NULL), (2, 2), (4, NULL)) foo(k, v);

SELECT JSON_OBJECTAGG(k: v WITH UNIQUE KEYS RETURNING jsonb)
FROM (VALUES (1, 1), (1, NULL), (2, 2)) foo(k, v);

SELECT JSON_OBJECTAGG(k: v ABSENT ON NULL WITH UNIQUE KEYS RETURNING jsonb)
FROM (VALUES (1, 1), (1, NULL), (2, 2)) foo(k, v);

SELECT JSON_OBJECTAGG(k: v ABSENT ON NULL WITH UNIQUE KEYS RETURNING jsonb)
FROM (VALUES (1, 1), (0, NULL),(4, null), (5, null),(6, null),(2, 2)) foo(k, v);

SELECT JSON_OBJECTAGG(mod(i,100): (i)::text FORMAT JSON WITH UNIQUE)
FROM generate_series(0, 199) i;

-- Test JSON_OBJECT deparsing
EXPLAIN (VERBOSE, COSTS OFF)
SELECT JSON_OBJECT('foo' : '1' FORMAT JSON, 'bar' : 'baz' RETURNING json);

CREATE VIEW json_object_view AS
SELECT JSON_OBJECT('foo' : '1' FORMAT JSON, 'bar' : 'baz' RETURNING json);


DROP VIEW json_object_view;

SELECT to_json(a) AS a, JSON_OBJECTAGG(k : v WITH UNIQUE KEYS) OVER (ORDER BY k)
FROM (VALUES (1,1), (2,2)) a(k,v);

SELECT to_json(a) AS a, JSON_OBJECTAGG(k : v WITH UNIQUE KEYS) OVER (ORDER BY k)
FROM (VALUES (1,1), (1,2), (2,2)) a(k,v);

SELECT to_json(a) AS a, JSON_OBJECTAGG(k : v ABSENT ON NULL WITH UNIQUE KEYS)
   OVER (ORDER BY k)
FROM (VALUES (1,1), (1,null), (2,2)) a(k,v);

SELECT to_json(a) AS a, JSON_OBJECTAGG(k : v ABSENT ON NULL)
OVER (ORDER BY k)
FROM (VALUES (1,1), (1,null), (2,2)) a(k,v);

SELECT to_json(a) AS a, JSON_OBJECTAGG(k : v ABSENT ON NULL)
OVER (ORDER BY k RANGE BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING)
FROM (VALUES (1,1), (1,null), (2,2)) a(k,v);

-- Test JSON_ARRAY deparsing
EXPLAIN (VERBOSE, COSTS OFF)
SELECT JSON_ARRAY('1' FORMAT JSON, 2 RETURNING json);

CREATE VIEW json_array_view AS
SELECT JSON_ARRAY('1' FORMAT JSON, 2 RETURNING json);


DROP VIEW json_array_view;

-- Test JSON_OBJECTAGG deparsing
EXPLAIN (VERBOSE, COSTS OFF)
SELECT JSON_OBJECTAGG(i: ('111' || i)::bytea FORMAT JSON WITH UNIQUE RETURNING text) FILTER (WHERE i > 3)
FROM generate_series(1,5) i;

EXPLAIN (VERBOSE, COSTS OFF)
SELECT JSON_OBJECTAGG(i: ('111' || i)::bytea FORMAT JSON WITH UNIQUE RETURNING text) OVER (PARTITION BY i % 2)
FROM generate_series(1,5) i;

CREATE VIEW json_objectagg_view AS
SELECT JSON_OBJECTAGG(i: ('111' || i)::bytea FORMAT JSON WITH UNIQUE RETURNING text) FILTER (WHERE i > 3)
FROM generate_series(1,5) i;


DROP VIEW json_objectagg_view;

-- Test JSON_ARRAYAGG deparsing
EXPLAIN (VERBOSE, COSTS OFF)
SELECT JSON_ARRAYAGG(('111' || i)::bytea FORMAT JSON NULL ON NULL RETURNING text) FILTER (WHERE i > 3)
FROM generate_series(1,5) i;

EXPLAIN (VERBOSE, COSTS OFF)
SELECT JSON_ARRAYAGG(('111' || i)::bytea FORMAT JSON NULL ON NULL RETURNING text) OVER (PARTITION BY i % 2)
FROM generate_series(1,5) i;

CREATE VIEW json_arrayagg_view AS
SELECT JSON_ARRAYAGG(('111' || i)::bytea FORMAT JSON NULL ON NULL RETURNING text) FILTER (WHERE i > 3)
FROM generate_series(1,5) i;


DROP VIEW json_arrayagg_view;

-- Test JSON_ARRAY(subquery) deparsing
EXPLAIN (VERBOSE, COSTS OFF)
SELECT JSON_ARRAY(SELECT i FROM (VALUES (1), (2), (NULL), (4)) foo(i) RETURNING jsonb);

CREATE VIEW json_array_subquery_view AS
SELECT JSON_ARRAY(SELECT i FROM (VALUES (1), (2), (NULL), (4)) foo(i) RETURNING jsonb);


DROP VIEW json_array_subquery_view;

-- IS JSON predicate
SELECT NULL IS JSON;
SELECT NULL IS NOT JSON;
SELECT NULL::json IS JSON;
SELECT NULL::jsonb IS JSON;
SELECT NULL::text IS JSON;
SELECT NULL::bytea IS JSON;
SELECT NULL::int IS JSON;

SELECT '' IS JSON;

SELECT bytea '\x00' IS JSON;

CREATE TABLE test_is_json (js text);

INSERT INTO test_is_json VALUES
 (NULL),
 (''),
 ('123'),
 ('"aaa "'),
 ('true'),
 ('null'),
 ('[]'),
 ('[1, "2", {}]'),
 ('{}'),
 ('{ "a": 1, "b": null }'),
 ('{ "a": 1, "a": null }'),
 ('{ "a": 1, "b": [{ "a": 1 }, { "a": 2 }] }'),
 ('{ "a": 1, "b": [{ "a": 1, "b": 0, "a": 2 }] }'),
 ('aaa'),
 ('{a:1}'),
 ('["a",]');

SELECT
	js,
	js IS JSON "IS JSON",
	js IS NOT JSON "IS NOT JSON",
	js IS JSON VALUE "IS VALUE",
	js IS JSON OBJECT "IS OBJECT",
	js IS JSON ARRAY "IS ARRAY",
	js IS JSON SCALAR "IS SCALAR",
	js IS JSON WITHOUT UNIQUE KEYS "WITHOUT UNIQUE",
	js IS JSON WITH UNIQUE KEYS "WITH UNIQUE"
FROM
	test_is_json;

SELECT
	js,
	js IS JSON "IS JSON",
	js IS NOT JSON "IS NOT JSON",
	js IS JSON VALUE "IS VALUE",
	js IS JSON OBJECT "IS OBJECT",
	js IS JSON ARRAY "IS ARRAY",
	js IS JSON SCALAR "IS SCALAR",
	js IS JSON WITHOUT UNIQUE KEYS "WITHOUT UNIQUE",
	js IS JSON WITH UNIQUE KEYS "WITH UNIQUE"
FROM
	(SELECT js::json FROM test_is_json WHERE js IS JSON) foo(js);

SELECT
	js0,
	js IS JSON "IS JSON",
	js IS NOT JSON "IS NOT JSON",
	js IS JSON VALUE "IS VALUE",
	js IS JSON OBJECT "IS OBJECT",
	js IS JSON ARRAY "IS ARRAY",
	js IS JSON SCALAR "IS SCALAR",
	js IS JSON WITHOUT UNIQUE KEYS "WITHOUT UNIQUE",
	js IS JSON WITH UNIQUE KEYS "WITH UNIQUE"
FROM
	(SELECT js, js::bytea FROM test_is_json WHERE js IS JSON) foo(js0, js);

SELECT
	js,
	js IS JSON "IS JSON",
	js IS NOT JSON "IS NOT JSON",
	js IS JSON VALUE "IS VALUE",
	js IS JSON OBJECT "IS OBJECT",
	js IS JSON ARRAY "IS ARRAY",
	js IS JSON SCALAR "IS SCALAR",
	js IS JSON WITHOUT UNIQUE KEYS "WITHOUT UNIQUE",
	js IS JSON WITH UNIQUE KEYS "WITH UNIQUE"
FROM
	(SELECT js::jsonb FROM test_is_json WHERE js IS JSON) foo(js);

-- Test IS JSON deparsing
EXPLAIN (VERBOSE, COSTS OFF)
SELECT '1' IS JSON AS "any", ('1' || i) IS JSON SCALAR AS "scalar", '[]' IS NOT JSON ARRAY AS "array", '{}' IS JSON OBJECT WITH UNIQUE AS "object" FROM generate_series(1, 3) i;

CREATE VIEW is_json_view AS
SELECT '1' IS JSON AS "any", ('1' || i) IS JSON SCALAR AS "scalar", '[]' IS NOT JSON ARRAY AS "array", '{}' IS JSON OBJECT WITH UNIQUE AS "object" FROM generate_series(1, 3) i;


DROP VIEW is_json_view;

-- Test implicit coercion to a fixed-length type specified in RETURNING
SELECT JSON_SERIALIZE('{ "a" : 1 } ' RETURNING varchar(2));
SELECT JSON_OBJECT('a': JSON_OBJECT('b': 1 RETURNING varchar(2)));
SELECT JSON_ARRAY(JSON_ARRAY('{ "a" : 123 }' RETURNING varchar(2)));
SELECT JSON_ARRAYAGG(('111' || i)::bytea FORMAT JSON NULL ON NULL RETURNING varchar(2)) FROM generate_series(1,1) i;
SELECT JSON_OBJECTAGG(i: ('111' || i)::bytea FORMAT JSON WITH UNIQUE RETURNING varchar(2)) FROM generate_series(1, 1) i;

-- Now try domain over fixed-length type
CREATE DOMAIN sqljson_char2 AS char(2) CHECK (VALUE NOT IN ('12'));
SELECT JSON_SERIALIZE('123' RETURNING sqljson_char2);
SELECT JSON_SERIALIZE('12' RETURNING sqljson_char2);

-- Bug #18657: JsonValueExpr.raw_expr was not initialized in ExecInitExprRec()
-- causing the Aggrefs contained in it to also not be initialized, which led
-- to a crash in ExecBuildAggTrans() as mentioned in the bug report:
-- https://postgr.es/m/18657-1b90ccce2b16bdb8@postgresql.org
CREATE FUNCTION volatile_one() RETURNS int AS $$ BEGIN RETURN 1; END; $$ LANGUAGE plpgsql VOLATILE;
CREATE FUNCTION stable_one() RETURNS int AS $$ BEGIN RETURN 1; END; $$ LANGUAGE plpgsql STABLE;
EXPLAIN (VERBOSE, COSTS OFF) SELECT JSON_OBJECT('a': JSON_OBJECTAGG('b': volatile_one() RETURNING text) FORMAT JSON);
SELECT JSON_OBJECT('a': JSON_OBJECTAGG('b': volatile_one() RETURNING text) FORMAT JSON);
EXPLAIN (VERBOSE, COSTS OFF) SELECT JSON_OBJECT('a': JSON_OBJECTAGG('b': stable_one() RETURNING text) FORMAT JSON);
SELECT JSON_OBJECT('a': JSON_OBJECTAGG('b': stable_one() RETURNING text) FORMAT JSON);
EXPLAIN (VERBOSE, COSTS OFF) SELECT JSON_OBJECT('a': JSON_OBJECTAGG('b': 1 RETURNING text) FORMAT JSON);
SELECT JSON_OBJECT('a': JSON_OBJECTAGG('b': 1 RETURNING text) FORMAT JSON);
DROP FUNCTION volatile_one, stable_one;
