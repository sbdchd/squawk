with t as (
  select 1
), -- <--- extra comma!
select * from t;

-- search depth missing item
with t as (select 1)
search depth first by a, , c set ordercol
select * from t order by ordercol;

-- search depth missing comma
with t as (select 1)
search depth first by a, b c set ordercol
select * from t order by ordercol;
