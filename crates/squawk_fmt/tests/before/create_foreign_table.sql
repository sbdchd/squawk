CREATE FOREIGN TABLE t() SERVER s;

create foreign table if not exists public.remote_records (
  id bigint not null,
  name text,
  constraint remote_records_pkey primary key (id)
) inherits (public.base_records) server foreign_server options (schema_name 'public', table_name 'records');

create foreign table partition_records partition of public.records (id with options not null, name) default server foreign_server;

create foreign table ranged_records partition of public.records (id) for values from (1) to (100) server foreign_server options (table_name 'ranged_records');

create foreign table an_intentionally_long_schema_name.an_intentionally_long_foreign_table_name (an_intentionally_long_column_name character varying, another_intentionally_long_column_name timestamp with time zone) server an_intentionally_long_foreign_server_name options (schema_name 'an_intentionally_long_schema_name', table_name 'an_intentionally_long_foreign_table_name');

/* before create */ CREATE /* before foreign */ FOREIGN /* before table */ TABLE /* before if */ IF /* before not */ NOT /* before exists */ EXISTS /* before table name */ public /* before dot */ . /* after dot */ remote_records /* before left paren */ (/* after left paren */ id /* before type */ BIGINT /* before comma */, /* after comma */ name /* before second type */ TEXT /* before right paren */) /* before server */ SERVER /* before server name */ foreign_server /* before options */ OPTIONS /* before options left paren */ (/* after options left paren */ schema_name /* before option value */ 'public' /* before option comma */, /* after option comma */ table_name /* before second option value */ 'records' /* before options right paren */) /* before semicolon */;
