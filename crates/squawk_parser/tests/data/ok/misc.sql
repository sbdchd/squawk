-- https://www.timescale.com/blog/boosting-postgres-insert-performance/
CREATE TABLE sensors (
    sensorid TEXT,
    ts TIMESTAMPTZ,
    value FLOAT8
);
INSERT INTO sensors (ts, sensorid, value) 
  SELECT * 
  FROM unnest(
    $1::timestamptz[], 
    $2::text[], 
    $3::float8[]
);
INSERT INTO sensors (sensorid, ts, value)
VALUES 
  ($1, $2, $3), 
  ($4, $5, $6), 
  --  ..., 
  ($2998, $2999, $3000);


-- https://www.timescale.com/blog/combining-semantic-search-and-full-text-search-in-postgresql-with-cohere-pgvector-and-pgai/
UPDATE movies
SET embedding = ai.cohere_embed(
    'embed-english-v3.0'
    , CONCAT_WS('. ',
        title,
        COALESCE(overview, '')
    )
    , input_type=>'search_document'
    -- , api_key=>%s
    , api_key=>$1
) where embedding is null;

SELECT title, overview
FROM movies, plainto_tsquery('english', $1) query
WHERE to_tsvector('english', title || ' ' || COALESCE(overview, '')) @@ query
ORDER BY ts_rank_cd(to_tsvector('english', title || ' ' || COALESCE(overview, '')), query) DESC
LIMIT 5;

WITH query_embedding AS (
    SELECT cohere_embed(
        'embed-english-v3.0'
        , $1
        , _input_type=>'search_query'
        -- , _api_key=>%s
        , _api_key=>$2
    ) AS embedding
)
SELECT title, overview
FROM movies, query_embedding
ORDER BY movies.embedding <=> query_embedding.embedding
LIMIT 5;

SELECT
    x."index",
    x.document->>'text' as "text",
    x.relevance_score
FROM jsonb_to_recordset(
    ai.cohere_rerank(
        'rerank-english-v3.0',
        -- %s,
        $1,
        -- %s::jsonb,
        $2::jsonb,
        return_documents => true,
        -- api_key=>%s
        api_key=>$3
    )->'results'
) AS x("index" int, "document" jsonb, relevance_score float8)
ORDER BY relevance_score DESC
LIMIT 5;


-- https://www.depesz.com/2024/11/15/grouping-data-into-array-of-sums-fun-with-custom-aggregates/
CREATE FUNCTION sum_per_hour( INOUT p_state int8[], IN p_hour int4, IN p_count int4 ) RETURNS int8[] LANGUAGE plpgsql AS $$
DECLARE
BEGIN
    -- sanity checks
    IF p_hour < 0 THEN
        raise exception 'Hour can''t be < 0 : %', p_hour;
    END IF;
    IF p_hour > 23 THEN
        raise exception 'Hour can''t be > 23 : %', p_hour;
    END IF;
 
    -- actual count modification
    p_state[ p_hour ] := p_state[ p_hour ] + p_count;
 
    RETURN;
END;
$$;

SELECT sum_per_hour( '[0:23]={0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0}', 1, 15 );

CREATE aggregate sum_per_hour( int4, int4 ) (
    sfunc = sum_per_hour,
    stype = int8[],
    initcond = '[0:23]={0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0}'
);

SELECT
    category_id,
    object_id,
    date_trunc( 'day', interaction_ts ) AS interaction_date,
    sum_per_hour( EXTRACT( 'hour' FROM interaction_ts)::int4, interaction_count ) FILTER (WHERE interaction_type = 'a') AS a_counts,
    sum_per_hour( EXTRACT( 'hour' FROM interaction_ts)::int4, interaction_count ) FILTER (WHERE interaction_type = 'b') AS b_counts
FROM
    input_data
GROUP BY 1, 2, 3
ORDER BY 1, 2, 3;

CREATE FUNCTION sum_hour_arrays( IN p_left int8[], IN p_right int8[] ) RETURNS int8[] AS $$
DECLARE
    i int4;
    v_result int8[];
BEGIN
    FOR i IN 0..23 LOOP
        v_result[i] := p_left[i] + p_right[i];
    END LOOP;
    RETURN v_result;
END;
$$ LANGUAGE plpgsql;


-- https://www.crunchydata.com/blog/postgres-partitioning-with-a-default-partition

INSERT INTO partman_test.time_taptest_table (col3) VALUES ('2024-12-25'::date);

CREATE TABLE partman_test.time_taptest_table_default PARTITION OF partman_test.time_taptest_table DEFAULT;

INSERT INTO partman_test.time_taptest_table (col3) VALUES ('2024-12-25'::date);

SELECT * FROM partman_test.time_taptest_table_default;

Table "partman_test.time_taptest_table_p20241124";

Table "partman_test.time_taptest_table_default";

CREATE TABLE partman_test.time_taptest_table_p20241225 PARTITION OF partman_test.time_taptest_table FOR VALUES FROM ('2024-12-25') TO ('2024-12-26');

BEGIN;

CREATE TEMP TABLE clean_default_temp (LIKE partman_test.time_taptest_table_default);

WITH partition_data AS (
    DELETE FROM partman_test.time_taptest_table_default RETURNING *
)
INSERT INTO clean_default_temp (col1, col2, col3) SELECT col1, col2, col3 FROM partition_data;

CREATE TABLE partman_test.time_taptest_table_p20241225 PARTITION OF partman_test.time_taptest_table FOR VALUES FROM ('2024-12-25') TO ('2024-12-26');

WITH partition_data AS (
    DELETE FROM clean_default_temp RETURNING *
)
INSERT INTO partman_test.time_taptest_table (col1, col2, col3) SELECT col1, col2, col3 FROM partition_data;

DROP TABLE clean_default_temp;

COMMIT;

SELECT * FROM partman_test.time_taptest_table_default ;

SELECT * FROM partman_test.time_taptest_table;

SELECT * FROM partman_test.time_taptest_table_p20241225 ;

SELECT * FROM partman.check_default();

SELECT * FROM partman.check_default(false);

select * from partman_test.time_taptest_table_default;

CALL partman.partition_data_proc('partman_test.time_taptest_table');

VACUUM ANALYZE partman_test.time_taptest_table;

SELECT * FROM partman.partition_gap_fill('partman_test.time_taptest_table');


-- https://www.crunchydata.com/blog/smarter-postgres-llm-with-retrieval-augmented-generation
SELECT openai.prompt(
  'You are a science fiction expert!',
  'What is the Star Trek episode where Deanna and her
   mother are kidnapped?'
);

CREATE TABLE tng (
    title text,
    plot text
    );

COPY tng (title, plot)
    FROM PROGRAM 'curl https://raw.githubusercontent.com/pramsey/pgsql-openai/refs/heads/main/examples/rag/tng.txt'
    WITH (
        FORMAT csv,
        DELIMITER E'\t'
        );

-- Enable pgvector
CREATE EXTENSION pgvector;

-- Add an emedding column to the table
ALTER TABLE tng
    ADD COLUMN vec vector;

-- Populate the column with embeddings from an LLM model
UPDATE tng
    SET vec = openai.vector(title || ' -- ' || plot)::vector;

SELECT title
FROM tng
ORDER BY vec <-> (SELECT openai.vector('What is the Star Trek episode where Deanna and her mother are kidnapped?')::vector)
LIMIT 5;

CREATE OR REPLACE FUNCTION trektrivia(query_text TEXT)
    RETURNS TEXT
    LANGUAGE 'plpgsql' AS $$
DECLARE
    query_embedding VECTOR;
    context_chunks TEXT;
BEGIN
    -- Step 1: Get the embedding vector for the query text
    query_embedding := openai.vector(query_text)::VECTOR;

    -- Step 2: Find the 5 closest plot summaries to the query embedding
    -- Step 3: Lump together results into a context lump
    SELECT string_agg('Episode: { Title: ' || title || ' } Summary: {' || plot, E'}}\n\n\n') INTO context_chunks
    FROM (
        SELECT plot, title
        FROM tng
        ORDER BY vec <-> query_embedding
        LIMIT 5
    ) AS similar_plots;

    -- Step 4: Run the query against the LLM with the augmented context
    RETURN openai.prompt(context_chunks, query_text);
END;
$$;

SELECT trektrivia('What is the Star Trek episode where Deanna and her mother are kidnapped?');


-- https://www.crunchydata.com/blog/pg_incremental-incremental-data-processing-in-postgres

/* define the raw data and summary table */
create table events (event_id bigserial, event_time timestamptz, user_id bigint, response_time double precision);
create table view_counts (day timestamptz, user_id bigint, count bigint, primary key (day, user_id));

/* enable fast range scans on the sequence column */
create index on events using brin (event_id);

/* for demo: generate some random data */
insert into events (event_time, user_id, response_time)
select now(), random() * 100, random() from generate_series(1,1000000) s;

/* define a sequence pipeline that periodically upserts view counts */
select incremental.create_sequence_pipeline('view-count-pipeline', 'events',
  $$
    insert into view_counts
    select date_trunc('day', event_time), user_id, count(*)
    from events where event_id between $1 and $2
    group by 1, 2
    on conflict (day, user_id) do update set count = view_counts.count + EXCLUDED.count;
  $$
);

/* get the most active users of today */
select user_id, sum(count) from view_counts where day = now()::date group by 1 order by 2 desc limit 3;


/* create a table with a single JSONB column and a sequence to track new objects */
create table events_json (id bigint generated by default as identity, payload jsonb);
create index on events_json using brin (id);

/* load some data from a local newline-delimited JSON file */
-- \copy events_json (payload) from '2024-12-15-00.json' with (format 'csv', quote e'\x01', delimiter e'\x02', escape e'\x01')


/* periodically unpack the new JSON objects into the events table */
select incremental.create_sequence_pipeline('unpack-json-pipeline', 'events_json',
  $$
    insert into events (event_id, event_time, user_id, response_time)
    select
      nextval('events_event_id_seq'),
      (payload->>'created_at')::timestamptz,
      (payload->'actor'->>'id')::bigint,
      (payload->>'response_time')::double precision
    from events_json
    where id between $1 and $2
  $$
);


/* create a table for number of active users per hour */
create table user_counts (hour timestamptz, user_count bigint, primary key (hour));

/* enable fast range scans on the event_time column */
create index on events using brin (event_time);

/* aggregates a range of 1 hour intervals after an hour has passed */
select incremental.create_time_interval_pipeline('distinct-user-count', '1 hour',
  $$
    insert into view_counts
    select date_trunc('hour', event_time), count(distinct user_id)
    from events where event_time >= $1 and event_time < $2
    group by 1
  $$
);

/* get number of active users per hour */
select hour, user_count from user_counts order by 1;

/* define a function that wraps a COPY TO command to export data */
create or replace function export_events(start_time timestamptz, end_time timestamptz)
returns void language plpgsql as $function$ begin

  /* select all rows in a time range and export them to a Parquet file */
  execute format(
    'copy (select * from events where event_time >= %L and event_time < %L) to %L',
    start_time, end_time, format('s3://mybucket/events/%s.parquet', start_time::date)
  );

end; $function$;

/* export data as 1 file per day, starting at Jan 1st */
select incremental.create_time_interval_pipeline(
  'export-events',
  '1 day',
  'select export_events($1, $2)',

  source_table_name := 'events', /* wait for writes on events to finish */
  batched := false,              /* separate execution for each day     */
  start_time := '2024-01-01'     /* export all days from Jan 1st now    */
);


/* define function that wraps a COPY FROM command to import data */
create or replace function import_events(path text)
returns void language plpgsql as $function$ begin

  /* load a file into the events table */
  execute format('copy events from %L', path);

end; $function$;

/* load all the files under a prefix, and automatically load new files, one at a time */
select incremental.create_file_list_pipeline(
    'import-events',
    's3://mybucket/events/*.csv',
    'select import_events($1)'
);

select jobname, start_time, status, return_message
from cron.job_run_details join cron.job using (jobid)
where jobname like 'pipeline:event-import%' order by 1 desc limit 3;

-- https://supabase.com/blog/calendars-in-postgres-using-foreign-data-wrappers

create foreign data wrapper wasm_wrapper
  handler wasm_fdw_handler
  validator wasm_fdw_validator;

create server cal_server
  foreign data wrapper wasm_wrapper
  options (
    fdw_package_url 'https://github.com/supabase/wrappers/releases/download/wasm_cal_fdw_v0.1.0/cal_fdw.wasm',
    fdw_package_name 'supabase:cal-fdw',
    fdw_package_version '0.1.0',
    fdw_package_checksum '4afe4fac8c51f2caa1de8483b3817d2cec3a14cd8a65a3942c8b4ff6c430f08a',
    api_key '<your Cal.com API key>'
  );

create schema if not exists cal;

create foreign table cal.event_types (
  attrs jsonb
)
  server cal_server
  options (
    object 'event-types'
  );

create foreign table cal.bookings (
  attrs jsonb
)
  server cal_server
  options (
    object 'bookings',
    rowid_column 'attrs'
  );

-- extract event types
select
  etg->'profile'->>'name' as profile,
  et->>'id' as id,
  et->>'title' as title
from cal.event_types t
  cross join json_array_elements((attrs->'eventTypeGroups')::json) etg
  cross join json_array_elements((etg->'eventTypes')::json) et;

-- extract bookings
select
  bk->>'id' as id,
  bk->>'title' as title,
  bk->'responses'->>'name' as name,
  bk->>'startTime' as start_time
from cal.bookings t
  cross join json_array_elements((attrs->'bookings')::json) bk;

-- make a 15 minutes meeting with Elon Musk
insert into cal.bookings(attrs)
values (
  '{
     "start": "2025-01-01T23:30:00.000Z",
     "eventTypeId": 1398027,
     "attendee": {
       "name": "Elon Musk",
       "email": "elon.musk@x.com",
       "timeZone": "America/New_York"
     }
  }'::jsonb
);


-- https://www.tinybird.co/blog-posts/outgrowing-postgres-how-to-run-olap-workloads-on-postgres

CREATE MATERIALIZED VIEW hourly_stats AS
SELECT
     date_trunc('hour', event_time) as hour,
    count(*) as events,
    count(distinct user_id) as users
FROM events GROUP BY 1;
CREATE UNIQUE INDEX ON hourly_stats(hour);

CREATE TABLE events (
    event_time timestamptz,
    user_id int,
    event_type text
) PARTITION BY RANGE (event_time);

CREATE TABLE events (
    -- Frequently queried columns
    event_time timestamptz,
    user_id int,
    event_type text,
    -- Rarely queried metadata
    user_agent text,
    ip_address inet,
    -- Large payload rarely used in aggregations
    event_data jsonb
);

CREATE TABLE events_core (
    event_id bigserial PRIMARY KEY,
    event_time timestamptz,
    user_id int,
    event_type text);

CREATE TABLE events_metadata (
    event_id bigint PRIMARY KEY REFERENCES events_core,
    user_agent text,
    ip_address inet
);

CREATE TABLE events_payload (
    event_id bigint PRIMARY KEY REFERENCES events_core,
    event_data jsonb
);

-- -- Instead of one simple insert
-- INSERT INTO events VALUES (...);

-- -- You now need transaction-wrapped multi-table inserts
--  BEGIN;
--      INSERT INTO events_core (...) RETURNING event_id;
--      INSERT INTO events_metadata (...);
--      INSERT INTO events_payload (...);
--  COMMIT;

-- Simple query becomes a three-way join
SELECT e.event_time, e.event_type, m.user_agent, p.event_data
FROM events_core e
LEFT JOIN events_metadata m
  ON e.event_id = m.event_id
LEFT JOIN events_payload p
  ON e.event_id = p.event_id
WHERE e.event_time >= now() - interval '1 day';

CREATE EXTENSION cstore_fdw;

CREATE SERVER cstore_server FOREIGN DATA WRAPPER cstore_fdw;

CREATE FOREIGN TABLE metrics_columnar (
    day date,
    metric_name text,
    value numeric) SERVER cstore_server
OPTIONS(compression 'pglz');

CREATE EXTENSION pg_analytics;

CREATE FOREIGN TABLE metrics (
    day date,
    metric_name text,
    value numeric
) SERVER parquet_server
OPTIONS (files 's3://bucket/metrics/*.parquet');

CREATE EXTENSION pg_duckdb;

CREATE TABLE metrics ( 
   day date,
    metric_name text,
    value numeric
) USING duckdb;

CREATE EXTENSION pg_mooncake;

CREATE TABLE metrics (
    day date,
    metric_name text,
    value numeric
) USING columnstore;

-- Instead of this complex self-join
SELECT 
  month,
  revenue,
  (
    SELECT AVG(revenue) 
    FROM monthly_revenues m2 
    WHERE m2.month <= m1.month 
    AND m2.month > m1.month - interval '3 months'
  ) as rolling_avg
FROM monthly_revenues m1;

-- Use this cleaner window function
SELECT 
  month,
  revenue,
  AVG(revenue) OVER (
    ORDER BY month 
    ROWS BETWEEN 2 PRECEDING AND CURRENT ROW
  ) as rolling_avg
FROM monthly_revenues;

SELECT
  date_trunc('month', timestamp) as month,
  customer_segment,
  SUM(revenue) as revenue,
  SUM(SUM(revenue)) OVER (
    PARTITION BY customer_segment
    ORDER BY date_trunc('month', timestamp)
  ) as running_total
FROM transactions
GROUP BY 1, 2;

-- Use CTEs to break down complex analytics
WITH monthly_stats AS (
  SELECT 
    date_trunc('month', timestamp) as month,
    COUNT(DISTINCT user_id) as users,
    SUM(revenue) as revenue
  FROM events 
  WHERE timestamp >= NOW() - interval '12 months'
  GROUP BY 1
),
user_segments AS (
  SELECT 
    user_id,
    CASE 
        WHEN lifetime_value > 1000 THEN 'high'
        WHEN lifetime_value > 100 THEN 'medium'
        ELSE 'low'
    END as segment
  FROM users
)
SELECT 
  month,
  segment,
  COUNT(DISTINCT e.user_id) as users,
  SUM(revenue) as revenue
FROM events e
JOIN user_segments s
  ON e.user_id = s.user_id
GROUP BY 1, 2;

EXPLAIN ANALYZE 
SELECT 
  date_trunc('month', timestamp) as month,
  COUNT(*) as events,
  COUNT(DISTINCT user_id) as users
FROM events
GROUP BY 1;

-- Key settings for parallel queries
SET max_parallel_workers_per_gather = 4;  -- Workers per query
SET parallel_setup_cost = 10;             -- Lower = more parallelism
SET parallel_tuple_cost = 0.001;          -- Lower = more parallelism
SET min_parallel_table_scan_size = '8MB'; -- Table size threshold

-- Get detailed buffering info
EXPLAIN (ANALYZE, BUFFERS) 
SELECT /* your query */;

-- Look for:
-- "Parallel Seq Scan" - Is parallelism being used?
-- "Hash Join" vs "Nested Loop" - Right join strategy?
-- "Sort Method: quicksort" vs "external sort" - Enough work_mem?
-- "Rows Removed by Filter" - Could an index help?

-- Sorts intermediate results unnecessarily
SELECT user_id, COUNT(*) 
FROM (
  SELECT DISTINCT user_id 
  FROM events 
  ORDER BY timestamp
) t
GROUP BY 1;

-- Remove unnecessary ORDER BY
SELECT user_id, COUNT(*) 
FROM (
  SELECT DISTINCT user_id 
  FROM events
) t
GROUP BY 1;

-- Gets all rows then filters
SELECT user_id, status
FROM user_status
WHERE (user_id, timestamp) IN (
  SELECT user_id, MAX(timestamp)
  FROM user_status
  GROUP BY user_id
);

-- Uses DISTINCT ON
SELECT DISTINCT ON (user_id) 
  user_id, status
FROM user_status
ORDER BY user_id, timestamp DESC;

CREATE INDEX idx_events_month ON events (
  date_trunc('month', timestamp)
);

CREATE INDEX event_logs_ts_brin ON event_logs
USING brin(timestamp) WITH (pages_per_range = 128);

SELECT corr(ctid::text::float8, timestamp::text::float8) FROM event_logs;

-- https://www.crunchydata.com/blog/indexing-materialized-views-in-postgres
CREATE MATERIALIZED VIEW recent_product_sales AS
SELECT
    p.sku,
    SUM(po.qty) AS total_quantity
FROM
    products p
    JOIN product_orders po ON p.sku = po.sku
    JOIN orders o ON po.order_id = o.order_id
WHERE
    o.status = 'Shipped'
GROUP BY
    p.sku
ORDER BY
    2 DESC;

CREATE INDEX sku_index ON recent_product_sales (sku);

REFRESH MATERIALIZED VIEW recent_product_sales;

CREATE UNIQUE INDEX unique_idx_recent_product_sales ON recent_product_sales(sku);

DROP INDEX sku_index;

REFRESH MATERIALIZED VIEW CONCURRENTLY recent_product_sales;


-- https://www.timescale.com/blog/iot-renewable-energy-models-building-the-future-with-time-series-data
-- Example time series data structure for a solar panel
CREATE TABLE solar_panel_metrics (
  time        TIMESTAMPTZ NOT NULL,
  panel_id    TEXT,
  voltage     DOUBLE PRECISION,
  current     DOUBLE PRECISION,
  temperature DOUBLE PRECISION,
  irradiance  DOUBLE PRECISION
);

-- Create a hypertable for efficient time series operations
SELECT create_hypertable('solar_panel_metrics', 'time');

-- Calculate daily energy production and compare with expected output
SELECT 
    time_bucket('1 day', time) AS day,
    panel_id,
    avg(voltage * current) AS actual_power,
    expected_power,
    ((avg(voltage * current) / expected_power) * 100) AS performance_ratio
FROM solar_panel_metrics
JOIN panel_specifications ON panel_id = specs_id
WHERE time > now() - INTERVAL '30 days'
GROUP BY day, panel_id, expected_power
HAVING performance_ratio < 90;

-- Detect abnormal vibration patterns in wind turbines
WITH baseline_stats AS (
  SELECT 
    turbine_id,
    avg(vibration_level) as avg_vibration,
    stddev(vibration_level) as stddev_vibration
  FROM turbine_metrics
  WHERE time > now() - INTERVAL '90 days'
  GROUP BY turbine_id
)
SELECT 
    time_bucket('5 minutes', time) AS bucket,
    turbine_id,
    vibration_level,
    (vibration_level - avg_vibration) / stddev_vibration AS z_score
FROM turbine_metrics t
JOIN baseline_stats b USING (turbine_id)
WHERE 
    time > now() - INTERVAL '1 day'
    AND abs((vibration_level - avg_vibration) / stddev_vibration) > 2;

-- Monitor aggregate power availability across resources
SELECT 
    time_bucket('5 minutes', time) AS interval,
    resource_type,
    sum(available_power) AS total_power,
    sum(storage_capacity) AS total_storage
FROM vpp_resources
WHERE time > now() - INTERVAL '1 hour'
GROUP BY interval, resource_type
ORDER BY interval DESC;

-- Identify optimal trading windows
WITH price_analysis AS (
  SELECT 
    time_bucket('1 hour', time) AS hour,
    avg(price) AS avg_price,
    percentile_cont(0.75) WITHIN GROUP (ORDER BY price) AS price_75th
  FROM energy_prices
  WHERE time > now() - INTERVAL '30 days'
  GROUP BY hour
)
SELECT 
    hour,
    avg_price,
    price_75th,
    CASE 
        WHEN avg_price > price_75th THEN 'SELL'
        WHEN avg_price < price_75th THEN 'BUY'
        ELSE 'HOLD'
    END AS trading_signal
FROM price_analysis
ORDER BY hour;
