-- errs
select * from t join u;

select * from t left join u;

select * from t right join u;

select * from t full join u;


-- err, can't use conditions with natural join
select * from t natural join u using (id);
select * from t natural join u on u.id = t.id;

-- err, can't use conditions with cross joins
select * from t cross join u using (id);
select * from t cross join u on u.id = t.id;
select * from t cross join u on true;

-- ok
select * from t natural join u;

select * from t cross join u, b join c using(id);

select * from t join u on u.id = t.id;
select * from t join u on true;

select * from t
  join u using (id)
  join c using (id);
