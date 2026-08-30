alter /* role */ role /* name */ reporting_user /* in */ in /* database */ database /* db */ analytics /* action */ set /* parameter */ work_mem /* assignment */ to /* value */ '128MB' /* end */;

alter role exceptionally_long_application_service_account_name with superuser createdb createrole login replication bypassrls connection limit 100 encrypted password 'a-long-secret-value' valid until '2042-02-22';

alter role current_user rename /* to */ to /* target */ renamed_user;

alter role all reset /* all */ all;
