REINDEX INDEX my_index;

REINDEX TABLE my_table;

REINDEX TABLE CONCURRENTLY my_broken_table;

reindex database my_database;

reindex system my_database;

reindex schema my_schema;

reindex (concurrently true, tablespace new_tablespace, verbose false) database concurrently my_database;

reindex (concurrently 'off', verbose yes) table public.my_table;

reindex (concurrently no, verbose auto) index public.my_index;

reindex (concurrently, verbose) table public.my_table;

reindex () table my_table;

reindex (concurrently true, tablespace an_intentionally_long_tablespace_name, verbose false) table concurrently an_intentionally_long_schema_name.an_intentionally_long_table_name;

/* before reindex */ REINDEX /* before left paren */ (/* after left paren */ CONCURRENTLY /* before option value */ TRUE /* before comma */, /* after comma */ TABLESPACE /* before tablespace */ new_tablespace /* before second comma */, /* after second comma */ VERBOSE /* before verbose value */ NO /* before right paren */) /* before target */ TABLE /* before concurrently */ CONCURRENTLY /* before table name */ public /* before dot */ . /* after dot */ records /* before semicolon */;
