-- squawk-ignore-file
-- pg version: 18.2
-- update via:
--   cargo xtask sync-builtins

create function pg_catalog.plpgsql_call_handler() returns language_handler
  language c;

create function pg_catalog.plpgsql_inline_handler(internal) returns void
  language c;

create function pg_catalog.plpgsql_validator(oid) returns void
  language c;

