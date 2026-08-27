VACUUM;

vacuum records;

vacuum full freeze verbose analyze public.records;

vacuum full freeze verbose analyse records;

vacuum (full, freeze, verbose, analyze, disable_page_skipping true, skip_locked on, index_cleanup auto, truncate no, process_main yes, parallel 2) public.records (id, name), public.archived_records;

vacuum (analyze, verbose, index_cleanup auto, parallel 4) a_very_long_schema_name.an_intentionally_long_table_name (an_intentionally_long_column_name, another_intentionally_long_column_name), another_very_long_schema_name.another_intentionally_long_table_name;

/* before */ VACUUM /* after vacuum */ (/* after left paren */ FULL /* before comma */, /* after comma */ ANALYZE /* before value */ TRUE /* before second comma */, /* after second comma */ INDEX_CLEANUP /* before name value */ AUTO /* before right paren */) /* before tables */ public /* before table dot */ . /* after table dot */ records /* before columns */ (/* after columns left paren */ id /* before column comma */, /* after column comma */ name /* before columns right paren */) /* before table comma */, /* after table comma */ archived_records /* before semicolon */;

VACUUM /* before full */ FULL /* before freeze */ FREEZE /* before verbose */ VERBOSE /* before analyse */ ANALYSE /* before legacy table */ records /* before legacy semicolon */;
