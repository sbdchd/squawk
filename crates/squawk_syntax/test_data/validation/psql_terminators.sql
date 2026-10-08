SELECT 1
\echo executing
SELECT 2
\g
SELECT 3;

SELECT 1
\echo executing
SELECT 2;

select $1 \parse stmt
select 4 \sendpipeline
select 5 \r
select 6\; select 7;
