with t as (
  select 1
), -- <--- extra comma!
select * from t;
