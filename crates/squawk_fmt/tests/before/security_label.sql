security label on table public.records is 'system_u:object_r:postgresql_db_t:s0';

security label for selinux on materialized view public.an_intentionally_long_materialized_view_name_that_makes_this_statement_exceed_eighty_characters is null;

security label on function public.process_record(bigint, text) is 'trusted';

/* before security */ SECURITY /* before label */ LABEL /* before for */ FOR /* before provider */ selinux /* before on */ ON /* before foreign */ FOREIGN /* before table */ TABLE /* before name */ public /* before dot */ . /* after dot */ records /* before is */ IS /* before value */ 'trusted' /* before semicolon */;
