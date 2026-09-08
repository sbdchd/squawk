select json_array();
select json_array(returning jsonb);
select json_array(select 1);
select json_array(select a from t returning text);
select json_array(1, 2, 3);
select json_array((select 1), 2);
select json_array(1, 2 null on null returning jsonb);
