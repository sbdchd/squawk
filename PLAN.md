# Plan

> For the future of Squawk

## General

### PL/pgSQL strings

parse & lint in things like function definitions

should support all the features of normal SQL

[src/pl/plpgsql/src/pl_gram.y](https://github.com/postgres/postgres/blob/78c5e141e9c139fc2ff36a220334e4aa25e1b0eb/src/pl/plpgsql/src/pl_gram.y#L4C39-L4C39)

### Notebook / Juypter

linter and IDE support

- https://jupyterlab.readthedocs.io/en/latest/user/lsp.html
- https://github.com/denoland/deno/blob/57dd66ec3dae2a0fe376f6a43c476dfade421c84/cli/tools/jupyter/server.rs
- https://docs.astral.sh/ruff/configuration/#jupyter-notebook-discovery

### Embedded SQL Strings

support SQL embedded in other languages

- requires supporting their various formatting strings among other things

- https://peps.python.org/pep-0249/#paramstyle
  - qmark Question mark style, e.g. ...WHERE name=?
  - numeric Numeric, positional style, e.g. ...WHERE name=:1
  - named Named style, e.g. ...WHERE name=:name
  - format ANSI C printf format codes, e.g. ...WHERE name=%s
  - pyformat Python extended format codes, e.g. ...WHERE name=%(name)s
- https://fljd.in/en/2024/11/25/substituting-a-variable-in-a-sql-script/

- https://www.timescale.com/blog/how-to-build-an-iot-pipeline-for-real-time-analytics-in-postgresql

  - `$__timeFrom()`, `$__timeTo()`, and `$sensor_id`
  - https://grafana.com/docs/grafana/latest/dashboards/variables/

- https://news.ycombinator.com/item?id=43514935
  - https://github.com/jmbuhr/otter.nvim
  - https://code.visualstudio.com/api/language-extensions/embedded-languages#request-forwarding

### Psql Meta Commands

parse and warn, helps with copy pasting examples

### Other Dialects

support Trino, BigQuery, Aurora DSLQ, etc.

### Formatter

```shell
squawk format
```

example:

```sql
-- some query comment
SELECT name username, email,
        "weird-column-name"
    FROM    bar
```

becomes

```sql
-- some query comment
select name username, email, "weird-column-name" from bar
```

- autofix trailing commas
  - http://peter.eisentraut.org/blog/2025/02/11/how-about-trailing-commas-in-sql
  - clickhouse & duckdb support them
- comments maintained

- disable for sections / files with special comment

config ideas:

- lower / upper case keywords (default lowercase)
- indent (default 2)

links:

- https://mcyoung.xyz/2025/03/11/formatters/
- https://archive.jlongster.com/A-Prettier-Formatter
- https://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf
- [datagrip code formatting](https://blog.jetbrains.com/datagrip/2019/03/11/top-9-sql-features-of-datagrip-you-have-to-know/#code_formatting)

### LSIF/SCIP

- https://github.com/sourcegraph/scip

### Benchmark & Profile Parser

https://www.dolthub.com/blog/2024-10-10-yacc-union-types/

- https://github.com/akopytov/sysbench/tree/master

sql for benchmarks maybe?

- pg_graphql

  https://github.com/supabase/pg_graphql/blob/bd0283718abaf329d98c69808f862594e9df5edc/pg_graphql--0.4.0.sql

- postgrest

  https://github.com/PostgREST/postgrest/blob/9ae885486735bff682de2b5639591ac5314272bb/test/spec/fixtures/schema.sql#L4

- hasura

  https://github.com/hasura/graphql-engine/blob/master/server/src-rsr/initialise.sql

- pg_vector

  https://github.com/pgvector/pgvector/blob/master/sql/vector.sql

### CLI

from `deno`

- `check` command for type checking
- `lsp` for language server
- `lint` for linter

### Type Checker

https://www.postgresql.org/docs/17/datatype-pseudo.html

context aware errors

- aka disambgiuate types with their namespace if necessary: https://gleam.run//news/context-aware-compilation#context-aware-errors

type checking should allow us to give type errors without the user having to run the query

support more advanced lint rules

### PGO

- https://github.com/rust-lang/rust-analyzer/pull/19582#issue-2992471459

## Linter

Support for auto fixes on the command line and in an IDE for rules where it makes sense.

### Rule: validating json path expressions

Make sure they've valid and won't error at runtime

https://www.postgresql.org/docs/17/functions-json.html#FUNCTIONS-SQLJSON-PATH-OPERATORS

### Rule: limit-missing-order-by

https://www.postgresql.org/docs/17/queries-limit.html

warn about missing `order by` with `limit`

not important for a lot of queries, like:

```sql
select * from t limit 10;
```

### Rule: simple array types

```sql
foo array[]

-- becomes

foo[]
```

### Rule: simple alter column

```sql
alter table t alter column c;

-- becomes

alter table t alter c;
```

```sql
alter table t
  alter column c set data type int;

-- becomes

alter table t
  alter column c type int;
```

### Rule: pointless cascade / restrict

> These key words do not have any effect, since there are no dependencies on \$name.

https://www.postgresql.org/docs/17/sql-dropcast.html

Applies to:

- `drop cast`
- `drop conversion`
- `drop policy`
- `drop publication`
- `drop statistics`
- `drop subscription`

### Rule: unnecessary casts

```sql
select a::int from t;
--     ^^^^^^ pointless cast, `a` is already an `int`
```

### Rule: simple joins

#### Inner join

```sql
select * from t inner join u using (id);

-- becomes:

select * from t join u using (id);
```

#### Left join

```sql
select * from t left outer join u using (id);

-- becomes:

select * from t left join u using (id);
```

#### Right join

```sql
select * from t right outer join u using (id);

-- becomes:

select * from t right join u using (id);
```

#### Full join

```sql
select * from t full outer join u using (id);

-- becomes:

select * from t full join u using (id);
```

### Rule: british spellings

```sql
   analyse foo.bar(a, b);
-- ^^^^^^^ Prefer analyze
```

### Rule: unsupported sequence function

```sql
select c from unnest(sequence(1, 10, 2)) as t(c);
--                   ^^^^^^^^ unknown function `sequence`, did you mean `generate_series`?

-- quick fix gives:

select c from generate_series(1, 10, 2) as t(c);
```

Trino has [`sequence`](https://trino.io/docs/current/functions/array.html#sequence) instead of [`generate_series`](https://www.postgresql.org/docs/17/functions-srf.html).

### Rule: unsupported suffixed literal

```sql
set session query_max_run_time = 10m;
```

Trino has support for [suffixed literals](https://trino.io/docs/current/admin/properties.html#duration)

Running this in Postgres gives you:

```
Query 1 ERROR at Line 1: : ERROR:  trailing junk after numeric literal at or near "10m"
LINE 1: set session query_max_run_time = 10m;
                                         ^
```

### Rule: cross join

```sql
select * from t join u on true;
select * from t1, t2;
```

suggests / autofixes to:

```sql
select * from t cross join u;
select * from t1 cross join t2;
```

with config to change desired destination format

### Rule: natural join

warn about natural joins and autofix to the equivalent

```sql
select * from t natural join u;
```

suggests / autofixes to:

```sql
select * from t join u using (id, name, ip, description, meta);
```

### Rule: using unsupported lambdas

This actually parsers in Postgres, but could work off a heuristic

```sql
select array_filter(
  array[1, 2, 2, 3],
  e -> (e % 2) = 0
--^^^^^^^^^^^^^^^^ Rule: lambdas aren't support in postgres
);

-- suggest

select (
  select coalesce(
    array_agg(e),
    array[]::int[]
  )
  from unnest(array[1, 2, 2, 3]) t(e)
  where (e % 2) = 0
);
```

https://blog.jooq.org/when-sql-meets-lambda-expressions/

### Rule: friendly sql

https://duckdb.org/docs/stable/sql/dialect/friendly_sql

Parse friendly sql syntax, like leading `from` clause, and warn with an autofix to convert to valid postgres syntax.

### Rule: values to scalars

```sql
SELECT * FROM machines
WHERE ip_address IN (VALUES('192.168.0.1'::inet), ('192.168.0.10'), ('192.168.1.43'));
```

becomes:

```sql
SELECT * FROM machines
WHERE ip_address IN ('192.168.0.1', '192.168.0.10', '192.168.1.43');
```

via: <https://www.postgresql.org/docs/17/sql-values.html>

> Tip: For simple IN tests, it's better to rely on the list-of-scalars form of IN than to write a VALUES query as shown above. The list of scalars method requires less writing and is often more efficient.

example to test this:

```sql
with t(name) as (select '1 month'::interval)
select count(*) from t where t.name in ('1 month', '2 month')
-- type checks!
```

### Rule: dialect: now() to dest

should support various fixes so people can write in one dialect of SQL and have it easily convert to the other one

### Rule: unused column

```sql
SELECT customer_id, total_amount
FROM (
  SELECT customer_id, SUM(amount) AS total_amount, min(created_at)
  FROM orders
  GROUP BY customer_id
) AS customer_totals
WHERE total_amount > 1000;
```

### Rule: sum(boolean) to case stmt

```sql
with t(x) as (select 1, 2, 3)
select sum(x > 1) from t;
```

```
Query 1 ERROR at Line 2: : ERROR:  function sum(boolean) does not exist
LINE 2: select sum(x  > 1) from t;
               ^
HINT:  No function matches the given name and argument types. You might need to add explicit type casts.
```

should instead offer an autofix to change it to a `case` statement:

```sql
with t(x) as (select 1, 2, 3)
select sum(case when x > 1 then 1 else 0 end) from t;
```

rel: <https://duckdb.org/2025/03/06/gems-of-duckdb-1-2.html>

### Rule: missing column in group by

suggest using an aggregate or grouping by

### Rule: field does not exist

### Rule: table does not exist

### Rule: ambiguous column name

Provide options to select from in quick fix

### Rule: column label is the same as an existing column

```sql
create table t (
    a int,
    b int,
    c int
);

select a, b c from t;
--          ^warn: column label takes the same name as a table column
```

### Rule: exists to count equal 0

```sql
select u.* from users u
where 0 = (select count(*) from addresses a where a.user_id = u.id);
--    ^^^warn: NOT EXISTS instead of comparing against zero

-- instead:

select u.* from users u
where NOT EXISTS (select from addresses a where a.user_id = u.id);
```

via: https://www.depesz.com/2024/12/01/sql-best-practices-dont-compare-count-with-0/

### Rule: unnamed columns in view

via: https://www.postgresql.org/docs/17/sql-createview.html

> ```sql
> CREATE VIEW vista AS SELECT 'Hello World';
> ```
>
> is bad form because the column name defaults to `?column?`; also, the column data type defaults to text, which might not be what you wanted. Better style for a string literal in a view's result is something like:
>
> ```sql
> CREATE VIEW vista AS SELECT text 'Hello World' AS hello;
> ```

### Rule: `drop role` to `drop group`

> DROP GROUP is now an alias for DROP ROLE.

https://www.postgresql.org/docs/17/sql-dropgroup.html

### Rule: `create table as` to `select into`

```sql
SELECT * INTO films_recent FROM films WHERE date_prod >= '2002-01-01';

-- becomes

CREATE TABLE films_recent AS
SELECT * FROM films WHERE date_prod >= '2002-01-01';
```

> CREATE TABLE AS is functionally similar to SELECT INTO. CREATE TABLE AS is the recommended syntax, since this form of SELECT INTO is not available in ECPG or PL/pgSQL, because they interpret the INTO clause differently. Furthermore, CREATE TABLE AS offers a superset of the functionality provided by SELECT INTO.
>
> In contrast to CREATE TABLE AS, SELECT INTO does not allow specifying properties like a table's access method with USING method or the table's tablespace with TABLESPACE tablespace_name. Use CREATE TABLE AS if necessary. Therefore, the default table access method is chosen for the new table. See default_table_access_method for more information.

via https://www.postgresql.org/docs/17/sql-selectinto.html

### Rule: delete without a where clause

### Rule: overflow warnings

https://sql-info.de/postgresql/postgres-gotchas.html#1_4

```sql
select 256 * 256 * 256 * 256;
-- Query 1 ERROR at Line 1: : ERROR:  integer out of range
```

```sql
select 256::int8 * 256 * 256 * 256;
-- ?column?
-- 4294967296
```

## IDE

### Find References

https://rust-analyzer.github.io/blog/2019/11/13/find-usages.html

### Go to Definition

#### Column / table

```sql
select name username, email, "weird-column-name" from bar
--      ^$ option+click
```

navigates to schema definition file or if setup, navigates to the the source code from a user provided command

#### Function

```sql
select json_array_length('["foo", "bar", "buzz"]');
--      ^$ option+click
```

for builtins, we return the same stuff as:

```sql
select pg_get_functiondef('pg_catalog.json_array_length'::regproc)
```

so in this case, we'd get:

```sql
CREATE OR REPLACE FUNCTION pg_catalog.json_array_length(json)
 RETURNS integer
 LANGUAGE internal
 IMMUTABLE PARALLEL SAFE STRICT
AS $function$json_array_length$function$
```

maybe simplify to:

```sql
create function pg_catalog.json_array_length(
  json
) returns integer
  language internal immutable parallel safe strict
  as $$json_array_length$$
```

non-builtin:

```sql
select inc(10)
--     ^$ option+click
```

gives:

```sql
create function inc(int) returns int
  language sql immutable
  as 'select $1 + 1';
```

if we call that postgres function to get the def we get:

```sql
CREATE OR REPLACE FUNCTION public.inc(integer)
 RETURNS integer
 LANGUAGE sql
 IMMUTABLE
AS $function$SELECT $1 + 1$function$
```

```sql
create function pg_catalog.jsonb_extract_path(
  from_json jsonb,
  variadic path_elems text[]
) returns jsonb
  language internal immutable parallel safe strict
  as $$jsonb_extract_path$$
```

### Autocomplete

- [datagrip postfix completion](https://blog.jetbrains.com/datagrip/2019/03/11/top-9-sql-features-of-datagrip-you-have-to-know/#postfix_completion)
  - https://www.jetbrains.com/help/datagrip/settings-postfix-completion.html
- [datagrip abbr completion](https://blog.jetbrains.com/datagrip/2019/03/11/top-9-sql-features-of-datagrip-you-have-to-know/#abbreviation_completion)

#### Joins

```sql
select id, email, created_at from users
join login_att
--            ^$:suggest
```

suggests

```sql
select id, email, created_at from users
join login_attempts on login_attempts.user_id = users.id
```

or maybe it does the snippet style thing:

```sql
select id, email, created_at from users
join login_attempts on login_attempts.$0 = $1
```

data grips has nice snippet support: https://www.jetbrains.com/datagrip/features/editor.html#live

#### CTEs

We should make writing and using CTEs easy since they're common in analytical queries

```sql
WITH customer_totals AS (
  SELECT customer_id, SUM(amount) AS total_amount
  FROM orders
  GROUP BY customer_id
)
SELECT c.customer_id, c.total_amount
FROM customer_totals c
WHERE c.total_amount > 1000;
```

### Hover Info

#### Column

```sql
create table bar (name varchar(255), email text, "weird-column-name" boolean);
select name username, email, "weird-column-name" from bar;
--      ^$ hover
```

gives:

```
-- size = 24 (0x18)
name: nullable varchar(255)
--------------------------------------
The name of the user in the system.
```

- https://r.ena.to/blog/optimizing-postgres-table-layout-for-maximum-efficiency/

rust analyzer gives:

```
squawk_parser::event::Event
Error { pub(crate) msg: String, }
size = 24 (0x18), align = 0x8, needs Drop
```

does alignment make sense to have with postgres?

maybe have common values and distribution somehow?

show index size on hover too?

- https://www.peterbe.com/plog/index-size-postgresql

#### Table

another example:

```sql
select name username, email, "weird-column-name" from bar
--                                                     ^$ hover
```

gives:

```sql
-- size = 48 (0x30)
-- storage = 150 million rows, 500GB
create table user (
  id integer seq_123123,
  name varchar(255),
  email varchar(125)
)
--------------------------------
Users of the platform.
```

#### Function

another example:

```sql
SELECT customer_id, SUM(amount) AS total_amount, min(created_at)
--                  ^$ hover
FROM orders
```

```sql
-- sum of expression across all input values
sum(expression smallint | int) -> bigint
sum(expression bigint) -> numeric
sum(expression double precision) -> double precision
sum(expression real) -> real
sum(expression interval) -> interval
sum(expression numeric) -> numeric
```

#### Column Number

another example:

```sql
SELECT customer_id, sum(cost) from cpus
group by 1;
--       ^$ hover
```

```
field: cpus.customer_id
type: string
-- id of the customer that rents the CPU
```

### Semantic Syntax Highlighting

https://code.visualstudio.com/api/language-extensions/semantic-highlight-guide

### VSCode Syntax Highlighting

Aka a non-semantic version

### Monaco Support

- Monaco Syntax Highlighting

### Codemirror Support

### Show Syntax Tree Command

replicate what we have in WASM in IDE

also show lex command

### Snippets

- [datagrip live templates](https://blog.jetbrains.com/datagrip/2019/03/11/top-9-sql-features-of-datagrip-you-have-to-know/#live_templates)
- [postgresql-snippets](https://github.com/Manuel7806/postgresql-snippets/blob/main/snippets/snippets.code-snippets)

### Quick Fix: alias query

```sql
select * from bar
--              ^$ action:rename-alias
```

becomes after filling in alias name with `b`

```sql
select b.* from bar b
```

another example:

```sql
select name, email from bar
--                       ^$ action:rename-alias
```

becomes after filling in alias name with `b`

```sql
select b.name, b.email from bar
```

should prompt for table name for each entry when there is an ambigous column

related:

- https://blog.jetbrains.com/datagrip/2019/03/11/top-9-sql-features-of-datagrip-you-have-to-know/#introduce_alias

### Quick Fix: table to select

```sql
table t;
-- ^$ action: convert to select

select * from t;
-- ^$ action: convert to table
```

### Quick Fix: wrap side-effect explain analyze in a transaction

```sql
explain analyze insert into t values (1);
-- ^$ action: wrap possible side effect in a transaction

-- becomes

begin;
explain analyze insert into t values (1);
rollback;
```

> **Important**
>
> Keep in mind that the statement is actually executed when the `ANALYZE` option is used. Although `EXPLAIN` will discard any output that a `SELECT` would return, other side effects of the statement will happen as usual. If you wish to use `EXPLAIN ANALYZE` on an `INSERT`, `UPDATE`, `DELETE`, `MERGE`, `CREATE TABLE AS`, or `EXECUTE` statement without letting the command affect your data, use this approach:
>
> ```sql
> BEGIN;
> EXPLAIN ANALYZE ...;
> ROLLBACK;
> ```

via: https://www.postgresql.org/docs/17/sql-explain.html

### Quick Fix: expand star

```sql
select * from bar
--     ^$ action:expand
```

becomes

```sql
select bar.name, bar.email, bar.buzz, bar.foo, bar."weird-column-name" from bar
```

or maybe

```sql
select name, email, buzz, foo, "weird-column-name" from bar
```

related:

- [datagrips expand wildcard](https://blog.jetbrains.com/datagrip/2019/03/11/top-9-sql-features-of-datagrip-you-have-to-know/#expand_wildcard)

### Quick Fix: field rename

```sql
select name, email, buzz, foo, "weird-column-name" from bar
--      ^$ action:rename
```

becomes:

```sql
select name as username, email, buzz, foo, "weird-column-name" from bar
```

or maybe:

```sql
select name username, email, buzz, foo, "weird-column-name" from bar
```

### Quick Fix: strings and quoted idents

if the quoted ident doesn't exist, the user probably meant to use single quotes to create a string literal. Add an auto fix suggestion.

```sql
select foo, "a" from t;
--          ^^^ unknown column `"a"`, did you mean to convert this to write a string literal?
--          ^^^ Quick Fix: convert to string literal

-- gives

select foo, 'a' from t;
```

### Quick Fix: subquery to CTE

### Quick Fix: CTE to subquery
