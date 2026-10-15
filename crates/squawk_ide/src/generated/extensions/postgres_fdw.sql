-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

create function postgres_fdw_disconnect(text) returns boolean
  language c;

create function postgres_fdw_disconnect_all() returns boolean
  language c;

create function postgres_fdw_get_connections(check_conn boolean DEFAULT false, OUT server_name text, OUT user_name text, OUT valid boolean, OUT used_in_xact boolean, OUT closed boolean, OUT remote_backend_pid integer) returns SETOF record
  language c;

create function postgres_fdw_handler() returns fdw_handler
  language c;

create function postgres_fdw_validator(text[], oid) returns void
  language c;

create foreign data wrapper postgres_fdw handler postgres_fdw_handler validator postgres_fdw_validator;

