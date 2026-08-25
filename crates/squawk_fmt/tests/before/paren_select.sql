(select 1) order by 1 for update limit 10 offset 2 rows;
(select 1) fetch first 5 rows with ties;
with cte as (select 1) (select x from cte);

(select a_very_long_result_expression from a_very_long_source_relation_name) order by a_very_long_first_order_expression desc, a_very_long_second_order_expression asc for no key update of a_very_long_source_relation_name skip locked limit a_very_long_limit_expression offset a_very_long_offset_expression rows;

with /* before recursive */ recursive /* before cte */ cte /* before as */ as /* before query open */ (/* before query */ select 1 /* before query close */) /* before outer open */ (/* before select */ select /* before target */ x /* before from */ from /* before relation */ cte /* before outer close */) /* before order */ order /* before order by */ by /* before order expression */ x /* before desc */ desc /* before locking */ for /* before lock strength */ update /* before locking of */ of /* before locked relation */ cte /* before lock wait */ nowait /* before limit */ limit /* before limit value */ 10 /* before offset */ offset /* before offset value */ 2 /* before rows */ rows /* before semicolon */;

(/* before select */ select 1 /* before close */) /* before fetch */ fetch /* before first */ first /* before quantity */ 5 /* before rows */ rows /* before with ties */ with /* before ties */ ties;
