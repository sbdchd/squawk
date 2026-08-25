values (1, 2), (3, 4);
values (1), (2) order by column1 desc, column2 asc;
/* before values */ values /* before first row */ ( /* before first expression */ 1 /* before expression comma */, /* before second expression */ 2 /* before first row closing paren */ ) /* before row comma */, /* before second row */ ( /* before third expression */ 3 /* before second row closing paren */ ) /* before order */ order /* before by */ by /* before order expression */ column1 /* before desc */ desc /* before semicolon */;
values (a_very_long_first_expression, a_very_long_second_expression, a_very_long_third_expression), (a_very_long_fourth_expression, a_very_long_fifth_expression, a_very_long_sixth_expression) order by a_very_long_first_order_expression desc, a_very_long_second_order_expression asc;

with cte as (select 1) values (1), (2) order by 1 for update limit 10 offset 2 rows;
values (1) fetch first 5 rows with ties;

values (a_very_long_first_expression, a_very_long_second_expression, a_very_long_third_expression), (a_very_long_fourth_expression, a_very_long_fifth_expression, a_very_long_sixth_expression) order by a_very_long_first_order_expression desc, a_very_long_second_order_expression asc for no key update of a_very_long_relation_name skip locked limit a_very_long_limit_expression offset a_very_long_offset_expression rows;

with /* before recursive */ recursive /* before cte */ cte /* before as */ as /* before query open */ (/* before query */ select 1 /* before query close */) /* before values */ values /* before row */ (/* before expression */ 1 /* before row close */) /* before order */ order /* before by */ by /* before order expression */ 1 /* before desc */ desc /* before locking */ for /* before lock strength */ update /* before locking of */ of /* before locked relation */ cte /* before lock wait */ nowait /* before limit */ limit /* before limit value */ 10 /* before offset */ offset /* before offset value */ 2 /* before rows */ rows /* before semicolon */;

values /* before row */ (/* before expression */ 1 /* before row close */) /* before fetch */ fetch /* before first */ first /* before quantity */ 5 /* before rows */ rows /* before with ties */ with /* before ties */ ties /* before semicolon */;
