-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

create function public.pg_get_wal_block_info(start_lsn pg_lsn, end_lsn pg_lsn, show_data boolean DEFAULT true, OUT start_lsn pg_lsn, OUT end_lsn pg_lsn, OUT prev_lsn pg_lsn, OUT block_id smallint, OUT reltablespace oid, OUT reldatabase oid, OUT relfilenode oid, OUT relforknumber smallint, OUT relblocknumber bigint, OUT xid xid, OUT resource_manager text, OUT record_type text, OUT record_length integer, OUT main_data_length integer, OUT block_data_length integer, OUT block_fpi_length integer, OUT block_fpi_info text[], OUT description text, OUT block_data bytea, OUT block_fpi_data bytea) returns SETOF record
  language c;

create function public.pg_get_wal_record_info(in_lsn pg_lsn, OUT start_lsn pg_lsn, OUT end_lsn pg_lsn, OUT prev_lsn pg_lsn, OUT xid xid, OUT resource_manager text, OUT record_type text, OUT record_length integer, OUT main_data_length integer, OUT fpi_length integer, OUT description text, OUT block_ref text) returns record
  language c;

create function public.pg_get_wal_records_info(start_lsn pg_lsn, end_lsn pg_lsn, OUT start_lsn pg_lsn, OUT end_lsn pg_lsn, OUT prev_lsn pg_lsn, OUT xid xid, OUT resource_manager text, OUT record_type text, OUT record_length integer, OUT main_data_length integer, OUT fpi_length integer, OUT description text, OUT block_ref text) returns SETOF record
  language c;

create function public.pg_get_wal_stats(start_lsn pg_lsn, end_lsn pg_lsn, per_record boolean DEFAULT false, OUT "resource_manager/record_type" text, OUT count bigint, OUT count_percentage double precision, OUT record_size bigint, OUT record_size_percentage double precision, OUT fpi_size bigint, OUT fpi_size_percentage double precision, OUT combined_size bigint, OUT combined_size_percentage double precision) returns SETOF record
  language c;

