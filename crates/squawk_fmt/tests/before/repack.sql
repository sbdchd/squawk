repack public.records;

repack (verbose true, analyze false) public.records (id, payload), public.archived_records using index public.records_idx;

repack an_intentionally_long_schema_name.an_intentionally_long_table_name_that_makes_this_statement_exceed_eighty_characters;

/* before repack */ REPACK /* before options */ (/* before verbose */ VERBOSE /* before true */ TRUE /* before close */) /* before table */ public /* before dot */ . /* after dot */ records /* before columns */ (/* before column */ id /* before close */) /* before using */ USING /* before index */ INDEX /* before index name */ public /* before index dot */ . /* after index dot */ records_idx /* before semicolon */;
