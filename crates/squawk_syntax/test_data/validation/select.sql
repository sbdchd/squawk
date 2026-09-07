
from t select c;

from f()select;

from t;

SELECT(o,select)WHERE''::r;

SELECT().n()FROM ONLY p();

with t as (from k)
select * from t;

SELECT(o,select f);

select (select);

create view f as select from((select)a);

select () d;

-- this is okay
select row() d;

-- COLLATE can be a bare alias, but cannot be followed by another alias
SELECT '' COLLATE;
SELECT '' COLLATE AS t;

-- GROUP BY and ORDER BY list cardinality and recovery
select group by;
select 1 group by a, having true;
with q as (select 1 group by a,) select * from q;
create materialized view v as select 1 group by a, with data;
select 1 group by a,;
select 1 order by a,;
select group by a, order by;

-- trailing ORDER BY comma at EOF
select 1 order by a,
