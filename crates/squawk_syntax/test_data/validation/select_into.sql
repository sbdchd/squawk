-- okay, top level
select 1 a into t;
select 1 a into t union select 2 a;
((select 1 a into t));
explain select 1 into x; 
prepare p as select 1 into x;

-- error, only the left most select in a union is allowed to be a select into
select 1 a union select 2 a into t union select 3 a;
select 4 a union select 5 a into t;
select 1 a union (((select 2 a into t)));
select 1 union ((select 2 into x) union select 3);

-- error, nested
with t as (select 1 a into t) select * from t;
with t as (select 1 into x union select 2) select * from t;

-- error, nested
select * from u
  where a in (select 1 a into t);
select json_array(select into t from u);

-- error, nested
insert into x select 1 into t;

-- error, nested
declare c cursor for select 1 into t;

-- error, nested
create table k as select 1 into t;

-- error, nested
create view k as select 1 into t;
