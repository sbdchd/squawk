alter /* user */ user /* role */ reporting_application_user /* in */ in /* database */ database /* name */ exceptionally_long_analytics_database_name /* action */ set /* parameter */ work_mem /* assignment */ to /* value */ '128MB' /* end */;

alter user reporting_application_user with superuser createdb createrole login replication bypassrls connection limit 100 encrypted password 'an-exceptionally-long-secret-value' valid until '2042-02-22';

alter user reporting_application_user rename /* to */ to /* target */ renamed_reporting_application_user;

alter user all in database analytics reset /* all */ all;
