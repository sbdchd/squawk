-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

create function postgres_fdw_disconnect(text) returns boolean
  language c
  strict
  parallel restricted;

create function postgres_fdw_disconnect_all() returns boolean
  language c
  strict
  parallel restricted;

create function postgres_fdw_get_connections(
  check_conn boolean default false,
  out server_name text,
  out user_name text,
  out valid boolean,
  out used_in_xact boolean,
  out closed boolean,
  out remote_backend_pid integer
) returns setof record
  language c
  strict
  parallel restricted;

create function postgres_fdw_handler() returns fdw_handler
  language c
  strict;

create function postgres_fdw_validator(text[], oid) returns void
  language c
  strict;

create foreign data wrapper postgres_fdw
  handler postgres_fdw_handler validator postgres_fdw_validator;
