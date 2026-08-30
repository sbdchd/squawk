select 1 union select 2;

select 1 UNION ALL select 2 INTERSECT DISTINCT select 3 EXCEPT select 4;

(select 1) except (select 2) order by 1;

(select 1);

select 1 union select 2 order by 1 for update limit 10 offset 2 rows;

select 1 union select 2 fetch first 5 rows with ties;

select 1 union select 2 for no key update of foo, bar skip locked;

table foo union values (1), (2);

select /* after select */ a_very_long_first_column_name, a_very_long_second_column_name from a_very_long_first_table_name
/* before union */ union /* before all */ all
/* before rhs */ select /* rhs select */ a_very_long_first_column_name, a_very_long_second_column_name from a_very_long_second_table_name /* before semicolon */;

select 1 /* before operator */ union /* before quantifier */ distinct /* before right select */ select 2;
select 1 -- before operator
union all -- before right select
select 2;

select 1 union select 2 offset 1 limit 2;
select 1 union select 2 limit 1 for update;
