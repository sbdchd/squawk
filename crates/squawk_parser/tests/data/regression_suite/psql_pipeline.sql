--
-- Tests using psql pipelining
--

CREATE TABLE psql_pipeline(a INTEGER PRIMARY KEY, s TEXT);

-- Single query
SELECT $1 \bind 'val1' \sendpipeline
SELECT 'val1';

-- Multiple queries
SELECT $1 \bind 'val1' \sendpipeline
SELECT $1, $2 \bind 'val2' 'val3' \sendpipeline
SELECT $1, $2 \bind 'val2' 'val3' \sendpipeline
SELECT 'val4';
SELECT 'val5', 'val6';

-- Multiple queries in single line, separated by semicolons
SELECT 1; SELECT 2; SELECT 3
;

-- Test \flush
SELECT $1 \bind 'val1' \sendpipeline
SELECT $1, $2 \bind 'val2' 'val3' \sendpipeline
SELECT $1, $2 \bind 'val2' 'val3' \sendpipeline
SELECT 'val4';
SELECT 'val5', 'val6';

-- Send multiple syncs
SELECT $1 \bind 'val1' \sendpipeline
SELECT $1, $2 \bind 'val2' 'val3' \sendpipeline
SELECT $1, $2 \bind 'val4' 'val5' \sendpipeline
SELECT 'val7';
SELECT 'val8';
SELECT 'val9';

-- Query terminated with a semicolon replaces an unnamed prepared
-- statement.
SELECT $1 \parse ''
SELECT 1;

-- Extended query is appended to pipeline by a semicolon after a
-- newline.
SELECT $1 \bind 1
;
SELECT 2;

-- \startpipeline should not have any effect if already in a pipeline.
SELECT $1 \bind 'val1' \sendpipeline

-- Convert an implicit transaction block to an explicit transaction block.
INSERT INTO psql_pipeline VALUES ($1) \bind 1 \sendpipeline
BEGIN \bind \sendpipeline
INSERT INTO psql_pipeline VALUES ($1) \bind 2 \sendpipeline
ROLLBACK \bind \sendpipeline

-- Multiple explicit transactions
BEGIN \bind \sendpipeline
INSERT INTO psql_pipeline VALUES ($1) \bind 1 \sendpipeline
ROLLBACK \bind \sendpipeline
BEGIN \bind \sendpipeline
INSERT INTO psql_pipeline VALUES ($1) \bind 1 \sendpipeline
COMMIT \bind \sendpipeline

-- COPY FROM STDIN
-- with \sendpipeline and \bind
SELECT $1 \bind 'val1' \sendpipeline
-- with semicolon
SELECT 'val1';

-- COPY FROM STDIN with \flushrequest + \getresults
-- with \sendpipeline and \bind
SELECT $1 \bind 'val1' \sendpipeline
-- with semicolon
SELECT 'val1';

-- COPY FROM STDIN with \syncpipeline + \getresults
-- with \bind and \sendpipeline
SELECT $1 \bind 'val1' \sendpipeline
-- with semicolon
SELECT 'val1';

-- COPY TO STDOUT
-- with \bind and \sendpipeline
SELECT $1 \bind 'val1' \sendpipeline
copy psql_pipeline TO STDOUT \bind \sendpipeline
-- with semicolon
SELECT 'val1';
copy psql_pipeline TO STDOUT;

-- COPY TO STDOUT with \flushrequest + \getresults
-- with \bind and \sendpipeline
SELECT $1 \bind 'val1' \sendpipeline
copy psql_pipeline TO STDOUT \bind \sendpipeline
-- with semicolon
SELECT 'val1';
copy psql_pipeline TO STDOUT;

-- COPY TO STDOUT with \syncpipeline + \getresults
-- with \bind and \sendpipeline
SELECT $1 \bind 'val1' \sendpipeline
copy psql_pipeline TO STDOUT \bind \sendpipeline
-- with semicolon
SELECT 'val1';
copy psql_pipeline TO STDOUT;

-- Use \parse and \bind_named
SELECT $1 \parse ''
SELECT $1, $2 \parse ''
SELECT $2 \parse pipeline_1

-- \getresults displays all results preceding a \flushrequest.
SELECT $1 \bind 1 \sendpipeline
SELECT $1 \bind 2 \sendpipeline

-- \getresults displays all results preceding a \syncpipeline.
SELECT $1 \bind 1 \sendpipeline
SELECT $1 \bind 2 \sendpipeline

-- \getresults immediately returns if there is no result to fetch.
SELECT $1 \bind 2 \sendpipeline

-- \getresults only fetches results preceding a \flushrequest.
SELECT $1 \bind 2 \sendpipeline
SELECT $1 \bind 2 \sendpipeline

-- \getresults only fetches results preceding a \syncpipeline.
SELECT $1 \bind 2 \sendpipeline
SELECT $1 \bind 2 \sendpipeline

-- Use pipeline with chunked results for both \getresults and \endpipeline.
SELECT $1 \bind 2 \sendpipeline
SELECT $1 \bind 2 \sendpipeline

-- \getresults with specific number of requested results.
SELECT $1 \bind 1 \sendpipeline
SELECT $1 \bind 2 \sendpipeline
SELECT $1 \bind 3 \sendpipeline
SELECT $1 \bind 4 \sendpipeline

-- \syncpipeline count as one command to fetch for \getresults.
SELECT $1 \bind 1 \sendpipeline

-- \getresults 0 should get all the results.
SELECT $1 \bind 1 \sendpipeline
SELECT $1 \bind 2 \sendpipeline
SELECT $1 \bind 3 \sendpipeline

--
-- Pipeline errors
--

-- \endpipeline outside of pipeline should fail

-- After an aborted pipeline, commands after a \syncpipeline should be
-- displayed.
SELECT $1 \bind \sendpipeline
SELECT $1 \bind 1 \sendpipeline

-- For an incorrect number of parameters, the pipeline is aborted and
-- the following queries will not be executed.
SELECT \bind 'val1' \sendpipeline
SELECT $1 \bind 'val1' \sendpipeline

-- Using a semicolon with a parameter triggers an error and aborts
-- the pipeline.
SELECT $1;
SELECT 1;

-- An explicit transaction with an error needs to be rollbacked after
-- the pipeline.
BEGIN \bind \sendpipeline
INSERT INTO psql_pipeline VALUES ($1) \bind 1 \sendpipeline
ROLLBACK \bind \sendpipeline
ROLLBACK;

-- \watch is not allowed in a pipeline.
SELECT \bind \sendpipeline

-- \gdesc should fail as synchronous commands are not allowed in a pipeline,
-- and the pipeline should still be usable.
SELECT $1 \bind 1 \gdesc
SELECT $1 \bind 1 \sendpipeline

-- ; is not allowed in a pipeline, pipeline should still be usable.
SELECT $1 as i, $2 as j \parse ''
SELECT $1 as k, $2 as l \parse 'second'

-- \g and \gx are not allowed, pipeline should still be usable.
SELECT $1 \bind 1 \g
SELECT $1 \bind 1 \g (format=unaligned tuples_only=on)
SELECT $1 \bind 1 \gx
SELECT $1 \bind 1 \gx (format=unaligned tuples_only=on)
SELECT $1 \bind 1 \sendpipeline

-- \g and \gx warnings should be emitted in an aborted pipeline, with
-- pipeline still usable.
SELECT $1 \bind \sendpipeline
SELECT $1 \bind 1 \g
SELECT $1 \bind 1 \gx

-- \sendpipeline is not allowed outside of a pipeline
SELECT $1 \bind 1 \sendpipeline

-- \sendpipeline is not allowed if not preceded by \bind or \bind_named
SELECT 1 \sendpipeline

-- \gexec is not allowed, pipeline should still be usable.
SELECT 'INSERT INTO psql_pipeline(a) SELECT generate_series(1, 10)' \parse 'insert_stmt'
SELECT COUNT(*) FROM psql_pipeline \bind \sendpipeline

-- After an error, pipeline is aborted and requires \syncpipeline to be
-- reusable.
SELECT $1 \bind \sendpipeline
SELECT $1 \bind 1 \sendpipeline
SELECT $1 \parse a
-- Pipeline is aborted.
SELECT $1 \bind 1 \sendpipeline
SELECT $1 \parse a
-- Sync allows pipeline to recover.
SELECT $1 \bind 1 \sendpipeline
SELECT $1 \parse a

-- In an aborted pipeline, \getresults 1 aborts commands one at a time.
SELECT $1 \bind \sendpipeline
SELECT $1 \bind 1 \sendpipeline
SELECT $1 \parse a

-- Test chunked results with an aborted pipeline.
SELECT $1 \bind \sendpipeline
SELECT $1 \bind \sendpipeline

-- \getresults returns an error when an incorrect number is provided.

-- \getresults when there is no result should not impact the next
-- query executed.
select 1;

-- Error messages accumulate and are repeated.
SELECT 1 \bind \sendpipeline

--
-- Pipelines and transaction blocks
--

-- SET LOCAL will issue a warning when modifying a GUC outside of a
-- transaction block.  The change will still be valid as a pipeline
-- runs within an implicit transaction block.  Sending a sync will
-- commit the implicit transaction block. The first command after a
-- sync will not be seen as belonging to a pipeline.
SET LOCAL statement_timeout='1h' \bind \sendpipeline
SHOW statement_timeout \bind \sendpipeline
SHOW statement_timeout \bind \sendpipeline
SET LOCAL statement_timeout='2h' \bind \sendpipeline
SHOW statement_timeout \bind \sendpipeline

-- REINDEX CONCURRENTLY fails if not the first command in a pipeline.
SELECT $1 \bind 1 \sendpipeline
REINDEX TABLE CONCURRENTLY psql_pipeline \bind \sendpipeline
SELECT $1 \bind 2 \sendpipeline

-- REINDEX CONCURRENTLY works if it is the first command in a pipeline.
REINDEX TABLE CONCURRENTLY psql_pipeline \bind \sendpipeline
SELECT $1 \bind 2 \sendpipeline

-- Subtransactions are not allowed in a pipeline.
SAVEPOINT a \bind \sendpipeline
SELECT $1 \bind 1 \sendpipeline
ROLLBACK TO SAVEPOINT a \bind \sendpipeline
SELECT $1 \bind 2 \sendpipeline

-- LOCK fails as the first command in a pipeline, as not seen in an
-- implicit transaction block.
LOCK psql_pipeline \bind \sendpipeline
SELECT $1 \bind 2 \sendpipeline

-- LOCK succeeds as it is not the first command in a pipeline,
-- seen in an implicit transaction block.
SELECT $1 \bind 1 \sendpipeline
LOCK psql_pipeline \bind \sendpipeline
SELECT $1 \bind 2 \sendpipeline

-- VACUUM works as the first command in a pipeline.
VACUUM psql_pipeline \bind \sendpipeline

-- VACUUM fails when not the first command in a pipeline.
SELECT 1 \bind \sendpipeline
VACUUM psql_pipeline \bind \sendpipeline

-- VACUUM works after a \syncpipeline.
SELECT 1 \bind \sendpipeline
VACUUM psql_pipeline \bind \sendpipeline

-- Clean up
DROP TABLE psql_pipeline;
