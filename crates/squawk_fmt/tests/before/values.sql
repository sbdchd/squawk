values (1, 2), (3, 4);
values (1), (2) order by column1 desc, column2 asc;
/* before values */ values /* before first row */ ( /* before first expression */ 1 /* before expression comma */, /* before second expression */ 2 /* before first row closing paren */ ) /* before row comma */, /* before second row */ ( /* before third expression */ 3 /* before second row closing paren */ ) /* before order */ order /* before by */ by /* before order expression */ column1 /* before desc */ desc /* before semicolon */;
values (a_very_long_first_expression, a_very_long_second_expression, a_very_long_third_expression), (a_very_long_fourth_expression, a_very_long_fifth_expression, a_very_long_sixth_expression) order by a_very_long_first_order_expression desc, a_very_long_second_order_expression asc;
