-- subquery must be the only arg
select json_array(1, select 2);
select json_array(1, 2, select 3);
