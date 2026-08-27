CLUSTER;

cluster verbose;

cluster records;

cluster verbose public.records using records_created_at_idx;

cluster records_created_at_idx on public.records;

cluster (verbose true, analyze false) a_very_long_schema_name.an_intentionally_long_table_name using a_very_long_schema_name.an_intentionally_long_index_name;

/* before */ CLUSTER /* after cluster */ (/* after left paren */ VERBOSE /* before comma */, /* after comma */ ANALYZE /* before value */ TRUE /* before right paren */) /* before table */ public /* before table dot */ . /* after table dot */ records /* before using */ USING /* after using */ public /* before index dot */ . /* after index dot */ records_idx /* before semicolon */;

CLUSTER /* before legacy index */ public /* before legacy index dot */ . /* after legacy index dot */ records_idx /* before on */ ON /* after on */ public /* before legacy table dot */ . /* after legacy table dot */ records /* before legacy semicolon */;
