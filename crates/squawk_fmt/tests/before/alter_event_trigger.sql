alter event trigger ddl_audit enable;

alter event trigger ddl_audit disable;

alter event trigger ddl_audit enable always;

alter event trigger ddl_audit enable replica;

alter event trigger ddl_audit owner to audit_owner;

alter event trigger ddl_audit rename to renamed_ddl_audit;

alter /* event */ event /* trigger */ trigger /* name */ an_exceedingly_long_event_trigger_name_used_for_auditing /* action */ enable /* always */ always /* semicolon */;
