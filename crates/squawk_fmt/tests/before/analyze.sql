analyze;

analyse verbose records;

analyze public.records (id, payload), public.archived_records;

analyze (verbose true, skip_locked false, buffer_usage_limit '4MB') public.records;

analyze an_intentionally_long_schema_name.an_intentionally_long_table_name_that_makes_this_statement_exceed_eighty_characters (an_intentionally_long_column_name);

/* before analyze */ ANALYZE /* before options */ (/* before verbose */ VERBOSE /* before true */ TRUE /* before comma */, /* before skip locked */ SKIP_LOCKED /* before false */ FALSE /* before close */) /* before table */ public /* before dot */ . /* after dot */ records /* before columns */ (/* before id */ id /* before column comma */, /* before payload */ payload /* before columns close */) /* before table comma */, /* before second table */ archived_records /* before semicolon */;
