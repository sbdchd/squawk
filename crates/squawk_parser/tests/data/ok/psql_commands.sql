\set ON_ERROR_STOP on
\connect my_database
\conninfo
\d my_table
\dt+ public.*
\pset format aligned
\timing on
\x auto
\o 'output.txt'
\o
\unset unused_variable
\echo 'hello; -- not SQL' "a\b" `echo hello` :'variable'
select 1 \g
select 2 \gset prefix_
select 3;
select 5 \gx
select 6 \gexec
select 7 \gdesc
select 8 \watch 1
select 9 \crosstabview
select 10
\echo executing
\echo 'still executing'
\g
select 11;
select $1 \parse stmt
select 12 \echo x \g
select 13 \r
select 14\; select 15;
\if :condition
\include 'other.sql'
\if :nested_condition
\i 'nested.sql'
\else
\ir 'relative.sql'
\endif
\elif :other_condition
\echo alternate branch
\else
\echo skipped \echo 'second command'
\endif
\copy my_table from 'data.csv' with (format csv)
\! echo C:\tmp
\echo done \\ select 4;
select '\not_a_command', E'\\escaped', $$\also_not_a_command$$;
-- \not_a_command
/* \not_a_command */
\unknown_command raw arguments;
\q
