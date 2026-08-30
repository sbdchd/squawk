-- ok, a composite type may have no fields
create type ty as ();

-- ok, non-empty column lists
create view v (a) as select 1;
create materialized view mv (a, b) as select 1, 2;
with t (a) as (select 1) select * from t;
select * from t x (a);
create function f() returns table (a int) language sql as 'select 1';

-- error, an empty column list needs at least one column
create view v () as select 1;
create materialized view mv () as select 1;
with t () as (select 1) select * from t;
select * from t x ();
create function f() returns table () language sql as 'select 1';
