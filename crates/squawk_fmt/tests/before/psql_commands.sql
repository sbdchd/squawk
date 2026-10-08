\set ON_ERROR_STOP on   
\echo 'hello; -- not SQL' "a\b" `echo hello` :'variable'
SELECT   1 \g
select 2   \gset prefix_
SELECT 10
\echo executing
\g
select 12 \echo x \g
select 14\; SELECT 15;
\if :condition
\include 'other.sql'
\endif
\copy my_table from 'data.csv' with (format csv)
\! echo C:\tmp
\echo done \\ SELECT   4;
\q
