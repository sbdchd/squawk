-- group by all
with x as (
  select * from t group by all
)
select * from x;
