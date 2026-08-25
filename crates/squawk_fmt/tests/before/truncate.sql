truncate foo;
TRUNCATE TABLE ONLY foo CONTINUE IDENTITY RESTRICT;
truncate foo *, bar RESTART IDENTITY CASCADE;

/*before*/ TRUNCATE /*a*/ TABLE /*b*/ public /*c*/ . /*d*/ foo /*e*/ * /*f*/, /*g*/ bar /*h*/ CONTINUE /*i*/ IDENTITY /*j*/ RESTRICT /*k*/;

TRUNCATE TABLE a_very_long_schema_name.a_very_long_table_name, another_very_long_schema_name.another_very_long_table_name RESTART IDENTITY CASCADE;
