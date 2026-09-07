select * from json_table('[]', '$' columns ()) x;

select xmlelement(name foo, xmlattributes());

select xmlforest();

select * from graph_table(g match (a) columns ());

alter property graph g alter vertex table t alter label l add properties ();

alter property graph g alter vertex table t drop properties ();

select 1::numeric();

select * from rows from ();

select * from xmltable(xmlnamespaces(), '/a' passing x columns b text);

create table t (a int, exclude using gist ());

select 1 group by grouping sets ();

select 1 group by rollup ();

select 1 group by cube ();

select distinct on () 1;

create table p partition of t for values in ();

create table p partition of t for values from () to ();

alter table t merge partitions () into p;

alter table t split partition p into ();

analyze ();

cluster ();

repack ();

create property graph g vertex tables ();

create property graph g vertex tables (a) edge tables ();

alter property graph g drop edge tables ();

alter property graph g drop vertex tables ();

create publication p for all tables except ();

create statistics s () on a, b from t;

explain () select 1;

alter server s options ();

reindex () index i;

checkpoint ();

vacuum ();

copy t to stdout ();

copy t to stdout (force_quote ());

insert into t values (1) on conflict () do nothing;

insert into t values (1) returning with () *;

update t set (a) = ();

drop database d with ();

create table t (a int) partition by range ();

create index on t ();

create index on t (a) include ();

create index on t (a int4_ops ());

create function f() returns table () as $$ $$ language sql;

create table t (a int) with ();

alter table t reset ();

alter table t alter column a set ();

create collation c ();

create operator @@@ ();

create type x as range ();

create type x ();

create text search parser p ();

alter subscription s skip ();

with t () as (select 1) select * from t;

create view v () as select 1;

create materialized view mv () as select 1;

select * from t x ();

select * from rows from (generate_series(1, 2) as ());

select * from a join b using ();

create table t (a int, unique ());

create table t (a int, unique (a) include ());

create table t (a int, foreign key (a) references u ());

create table t (a int references u (a) on delete set null ());

create table t (a int references u (a) on delete set default ());

insert into t () values (1);

update t set () = (1);

alter publication p add table t ();

grant all () on t to public;

grant select () on t to public;

revoke select () on t from public;

vacuum t ();

copy t () from stdin;

create property graph g vertex tables (t key ());

create property graph g edge tables (e source key () references v (a) destination w);

create property graph g edge tables (e source key (a) references v () destination w);
