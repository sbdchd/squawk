with values as (select 1) table values;
with update as (select 1) select * from update;
with merge as (select 1) table merge;

with merge (x) as (select 1) select * from merge;
with insert (x) as (select 1) select * from insert;
with update (x) as (select 1) select * from update;
with values (x) as (select 1) select * from values;
