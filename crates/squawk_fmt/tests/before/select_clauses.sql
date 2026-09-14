with cte as (select 1) select x from cte where x > 0 group by x having count(*) > 0 window win as (partition by x) order by x for update limit 10 offset 2 rows;

select x from foo fetch first 5 rows with ties;

with a_very_long_common_table_expression_name as (select a_very_long_source_column_name from a_very_long_source_relation_name) select a_very_long_result_column_name from a_very_long_common_table_expression_name where a_very_long_filter_column_name > a_very_long_filter_threshold_value group by a_very_long_result_column_name having count(*) > a_very_long_having_threshold_value window a_very_long_window_name as (partition by a_very_long_partition_column_name order by a_very_long_order_column_name) order by a_very_long_result_column_name desc for no key update of a_very_long_common_table_expression_name skip locked limit a_very_long_limit_expression offset a_very_long_offset_expression rows;

with /* before recursive */ recursive /* before cte */ cte /* before columns */ (/* before column */ x /* before columns close */) /* before as */ as /* before materialized */ materialized /* before query open */ (/* before query */ select 1 /* before query close */) /* before outer select */ select /* before target */ x /* before from */ from /* before relation */ cte /* before where */ where /* before where expression */ x > 0 /* before group */ group /* before by */ by /* before group expression */ x /* before having */ having /* before having expression */ count(*) > 0 /* before window */ window /* before window name */ win /* before window as */ as /* before window open */ (/* before partition */ partition /* before partition by */ by /* before partition expression */ x /* before window close */) /* before order */ order /* before order by */ by /* before order expression */ x /* before desc */ desc /* before locking */ for /* before lock strength */ update /* before locking of */ of /* before locked relation */ cte /* before lock wait */ nowait /* before limit */ limit /* before limit value */ 10 /* before offset */ offset /* before offset value */ 2 /* before rows */ rows /* before semicolon */;

select x /* before fetch */ fetch /* before first */ first /* before quantity */ 5 /* before rows */ rows /* before with ties */ with /* before ties */ ties;

select ''::text as five, unique1, unique2, stringu1 from onek order by unique1 /* before offset */ offset 990 /* before limit */ limit 5;

select thousand from onek where thousand < 5 order by thousand /* before fetch */ fetch first 1 row with ties /* before locking */ for update skip locked;

select thousand from onek where thousand < 995 order by thousand /* before offset */ offset 10 /* before fetch */ fetch first 5 rows only;

select x from foo offset 1 limit 2;
select x from foo limit 2 for update;

select * from t window w as (partition by aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa), w2 as (partition by bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb);

select 1 order by a, -- order by
b;

select 1 order by (select count(*) from a_very_long_relation_name_that_forces_the_parenthesized_subquery_to_wrap);

select 1 order /* before by */ by /* before expression */ (/* before select */ select count(*) from a_very_long_relation_name_that_forces_the_parenthesized_subquery_to_wrap /* before close */) /* before semicolon */;

select
  category,
  current_price > (
    select avg(current_price)
    from a_very_long_relation_name_that_forces_the_parenthesized_subquery_to_wrap
  ) as above_average,
  count(*) filter (where current_price > (
    select avg(current_price)
    from a_very_long_relation_name_that_forces_the_parenthesized_subquery_to_wrap
  )) as expensive_count
from item i
join category_prices p on i.current_price > (
  select avg(current_price)
  from a_very_long_relation_name_that_forces_the_parenthesized_subquery_to_wrap
)
where current_price > (
  select avg(current_price)
  from item
  where category = i.category or color = i.color
)
group by category
having count(*) > (
  select count(*)
  from a_very_long_relation_name_that_forces_the_parenthesized_subquery_to_wrap
)
order by count;

select 1
from t
where current_price = any(select 10000000000000000000000000000000000000000000000000
);

select 1
from t
where current_price = any(select 1 -- foo
);

select * from item where exists(select current_price from a_very_long_relation_name_abcdefghijklmn);

select * from item where my_predicate_function(aaaaaaaaaaaaaaaaaaaaaaaaaa, bbbbbbbbbbbbbbbbbbbbbbbbb);

select * from item where cast(aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa as boolean);

select * from item where /* before predicate */ exists(/* before subquery */ select current_price from a_very_long_relation_name_abcdefghijklmn /* before close */);

select * from item where -- before predicate
exists(select current_price from a_very_long_relation_name_abcdefghijklmn);

select 1 from t where a = array[a_very_long_first_array_expression, a_very_long_second_array_expression, a_very_long_third_array_expressiona_very_long_second_array_expression];

select * from item where aaaaaaaaaaaaaaaaaaaaaaaaaa = 1 and my_predicate_function(bbbbbbbbbbbbbbbbbbbbbbbbb);

select * from t order by -- before sort expression
f(a);

select * from t order by -- before sort expression
a desc;

select * from t order by a_very_long_sort_expression_name_that_does_not_fit_on_a_single_line_abcdef;

select * from a join b on -- before condition
a.id = f(b.id);

select * from a join b on -- before condition
a.id = b.id;

select * from a join b on
-- own line
a.id = b.id;

select * from t where aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa = calculate_something();
