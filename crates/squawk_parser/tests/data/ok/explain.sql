-- simple
explain select * from t;

-- explain_analyze
explain analyze select a from t;

-- all_options
explain (
  analyze,
  verbose,
  costs,
  costs true,
  costs false,
  settings,
  generic_plan,
  buffers,
  serialize,
  serialize none,
  serialize text,
  serialize binary,
  wal,
  timing,
  summary,
  memory,
  format text,
  format xml,
  format json,
  format yaml
)
select 1;

-- doc_example_1
EXPLAIN (FORMAT JSON) SELECT * FROM foo;

-- doc_example_2
EXPLAIN SELECT * FROM foo WHERE i = 4;

-- doc_example_3
EXPLAIN (FORMAT YAML) SELECT * FROM foo WHERE i='4';

-- doc_example_4
EXPLAIN (COSTS FALSE) SELECT * FROM foo WHERE i = 4;

-- doc_example_5
EXPLAIN SELECT sum(i) FROM foo WHERE i < 10;

-- doc_example_6
PREPARE query(int, int) AS SELECT sum(bar) FROM test
    WHERE id > $1 AND id < $2
    GROUP BY foo;

EXPLAIN ANALYZE EXECUTE query(100, 200);

-- doc_example_7
EXPLAIN (GENERIC_PLAN)
  SELECT sum(bar) FROM test
    WHERE id > $1 AND id < $2
    GROUP BY foo;

-- doc_example_8
EXPLAIN (GENERIC_PLAN)
  SELECT sum(bar) FROM test
    WHERE id > $1::integer AND id < $2::integer
    GROUP BY foo;

-- parens_select
explain analyze (((((select 1)))));

-- parens_values
explain analyze (((((values (1))))));

-- boolean off
EXPLAIN (COSTS OFF) SELECT * FROM foo WHERE i = 4;

-- boolean on
EXPLAIN (COSTS ON) SELECT * FROM foo WHERE i = 4;

-- boolean 1
EXPLAIN (COSTS 1) SELECT * FROM foo WHERE i = 4;

-- boolean 0
EXPLAIN (COSTS 0) SELECT * FROM foo WHERE i = 4;

-- compound select
explain select 1 union select 2;

-- select into
explain select a, b, c into t from t2;

-- paren select
explain (select 1);

-- table
explain table t;

-- values
explain values (1), (2);

-- insert
explain insert into t values (1, 2), (3, 4);

-- update
explain update t set a = 1 where a > 10;

-- delete
explain delete from t where x > 1;

-- merge
explain merge into t
  using u
    on t.id = u.id
  when matched then 
    do nothing;

-- execute
explain execute foo;

-- declare
explain declare c cursor for select 1;

-- create table as
explain create table t as
  select * from u where c >= b;

-- create materialized view
explain create materialized view t
  as select 1;
