-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

create function pg_get_wal_block_info(
  start_lsn pg_lsn,
  end_lsn pg_lsn,
  show_data boolean default true,
  out start_lsn pg_lsn,
  out end_lsn pg_lsn,
  out prev_lsn pg_lsn,
  out block_id smallint,
  out reltablespace oid,
  out reldatabase oid,
  out relfilenode oid,
  out relforknumber smallint,
  out relblocknumber bigint,
  out xid xid,
  out resource_manager text,
  out record_type text,
  out record_length integer,
  out main_data_length integer,
  out block_data_length integer,
  out block_fpi_length integer,
  out block_fpi_info text[],
  out description text,
  out block_data bytea,
  out block_fpi_data bytea
) returns setof record
  language c
  strict
  parallel safe;

create function pg_get_wal_record_info(
  in_lsn pg_lsn,
  out start_lsn pg_lsn,
  out end_lsn pg_lsn,
  out prev_lsn pg_lsn,
  out xid xid,
  out resource_manager text,
  out record_type text,
  out record_length integer,
  out main_data_length integer,
  out fpi_length integer,
  out description text,
  out block_ref text
) returns record
  language c
  strict
  parallel safe;

create function pg_get_wal_records_info(
  start_lsn pg_lsn,
  end_lsn pg_lsn,
  out start_lsn pg_lsn,
  out end_lsn pg_lsn,
  out prev_lsn pg_lsn,
  out xid xid,
  out resource_manager text,
  out record_type text,
  out record_length integer,
  out main_data_length integer,
  out fpi_length integer,
  out description text,
  out block_ref text
) returns setof record
  language c
  strict
  parallel safe;

create function pg_get_wal_stats(
  start_lsn pg_lsn,
  end_lsn pg_lsn,
  per_record boolean default false,
  out "resource_manager/record_type" text,
  out count bigint,
  out count_percentage double precision,
  out record_size bigint,
  out record_size_percentage double precision,
  out fpi_size bigint,
  out fpi_size_percentage double precision,
  out combined_size bigint,
  out combined_size_percentage double precision
) returns setof record
  language c
  strict
  parallel safe;
