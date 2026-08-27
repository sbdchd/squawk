CALL refresh_materialized_data();

call public.process_record(1, 'record', enabled => true);

call process_a_record_with_an_intentionally_long_procedure_name(an_intentionally_long_argument_name => 'an intentionally long argument value', another_intentionally_long_argument_name => 12345);

/* before */ CALL /* after call */ public /* before dot */ . /* after dot */ process_record /* before left paren */ (/* after left paren */ 1 /* before comma */, /* after comma */ argument_name /* before arrow */ => /* after arrow */ 'value' /* before second comma */, /* after second comma */ VARIADIC /* after variadic */ ARRAY[1, 2] /* before right paren */) /* before semicolon */;
