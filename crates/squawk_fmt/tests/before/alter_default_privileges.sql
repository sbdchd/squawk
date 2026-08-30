alter default privileges grant select on tables to reporting;

alter default privileges grant update on tables to public, current_user, group /* session role */ session_user, group /* current role */ current_role with grant option;

alter default privileges revoke grant option for select on tables from reporting, group /* current user */ current_user cascade;

alter default privileges for role app_owner, migrations in schema public, audit grant all privileges on sequences to app_user, reporting with grant option;

alter default privileges for user app_owner revoke grant option for insert, update on tables from app_user cascade;

alter default privileges grant execute on functions to public;

alter default privileges grant execute on routines to public;

alter default privileges grant usage on schemas to public;

alter default privileges grant usage on types to public;

alter default privileges grant select on large objects to public;

alter /* default keyword */ default /* privileges keyword */ privileges /* for clause */ for /* role keyword */ role /* first role */ exceedingly_long_application_owner, /* second role */ exceedingly_long_migration_owner /* schemas clause */ in /* schema keyword */ schema /* first schema */ exceedingly_long_application_schema, /* second schema */ exceedingly_long_audit_schema /* action */ grant /* privilege */ select, /* another privilege */ update /* on keyword */ on /* target */ tables /* to keyword */ to /* grantee */ exceedingly_long_reporting_role /* with clause */ with /* grant keyword */ grant /* option keyword */ option /* semicolon */;
