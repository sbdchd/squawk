explain select * from records;

explain analyze verbose update records set value = 1;

explain (analyze true, verbose, costs false, format json) select an_intentionally_long_column_name from an_intentionally_long_table_name where an_intentionally_long_column_name > 0;

/* before explain */ EXPLAIN /* before options */ (/* before analyze */ ANALYZE /* before value */ TRUE /* before comma */, /* before format */ FORMAT /* before json */ JSON /* before close */) /* before select */ SELECT /* before target */ 1 /* before semicolon */;
