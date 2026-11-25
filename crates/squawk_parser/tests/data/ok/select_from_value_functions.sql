-- sql value functions
select * from current_date;
select * from localtime with ordinality as t(a, b);
select * from current_time(2) with ordinality;
select * from pg_class c, lateral current_date t;

-- casts
select * from cast(1 as int) with ordinality t;
select * from lateral cast(1 as int) t;

-- collation for
select * from collation for ('a') with ordinality;

-- rows from
select * from rows from (current_date, cast(1 as int)) with ordinality;
select * from rows from (cast(null as record) as (a int));

