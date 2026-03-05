-- squawk-ignore-file
-- pg version: 18.2
-- update via:
--   cargo xtask sync-builtins

create view public.pg_stat_statements(userid, dbid, toplevel, queryid, query, plans, total_plan_time, min_plan_time, max_plan_time, mean_plan_time, stddev_plan_time, calls, total_exec_time, min_exec_time, max_exec_time, mean_exec_time, stddev_exec_time, rows, shared_blks_hit, shared_blks_read, shared_blks_dirtied, shared_blks_written, local_blks_hit, local_blks_read, local_blks_dirtied, local_blks_written, temp_blks_read, temp_blks_written, shared_blk_read_time, shared_blk_write_time, local_blk_read_time, local_blk_write_time, temp_blk_read_time, temp_blk_write_time, wal_records, wal_fpi, wal_bytes, wal_buffers_full, jit_functions, jit_generation_time, jit_inlining_count, jit_inlining_time, jit_optimization_count, jit_optimization_time, jit_emission_count, jit_emission_time, jit_deform_count, jit_deform_time, parallel_workers_to_launch, parallel_workers_launched, stats_since, minmax_stats_since) as
  select
    null::oid,
    null::oid,
    null::boolean,
    null::bigint,
    null::text,
    null::bigint,
    null::double precision,
    null::double precision,
    null::double precision,
    null::double precision,
    null::double precision,
    null::bigint,
    null::double precision,
    null::double precision,
    null::double precision,
    null::double precision,
    null::double precision,
    null::bigint,
    null::bigint,
    null::bigint,
    null::bigint,
    null::bigint,
    null::bigint,
    null::bigint,
    null::bigint,
    null::bigint,
    null::bigint,
    null::bigint,
    null::double precision,
    null::double precision,
    null::double precision,
    null::double precision,
    null::double precision,
    null::double precision,
    null::bigint,
    null::bigint,
    null::numeric,
    null::bigint,
    null::bigint,
    null::double precision,
    null::bigint,
    null::double precision,
    null::bigint,
    null::double precision,
    null::bigint,
    null::double precision,
    null::bigint,
    null::double precision,
    null::bigint,
    null::bigint,
    null::timestamp with time zone,
    null::timestamp with time zone
;

create view public.pg_stat_statements_info(dealloc, stats_reset) as
  select
    null::bigint,
    null::timestamp with time zone
;

create function public.pg_stat_statements(showtext boolean, OUT userid oid, OUT dbid oid, OUT toplevel boolean, OUT queryid bigint, OUT query text, OUT plans bigint, OUT total_plan_time double precision, OUT min_plan_time double precision, OUT max_plan_time double precision, OUT mean_plan_time double precision, OUT stddev_plan_time double precision, OUT calls bigint, OUT total_exec_time double precision, OUT min_exec_time double precision, OUT max_exec_time double precision, OUT mean_exec_time double precision, OUT stddev_exec_time double precision, OUT rows bigint, OUT shared_blks_hit bigint, OUT shared_blks_read bigint, OUT shared_blks_dirtied bigint, OUT shared_blks_written bigint, OUT local_blks_hit bigint, OUT local_blks_read bigint, OUT local_blks_dirtied bigint, OUT local_blks_written bigint, OUT temp_blks_read bigint, OUT temp_blks_written bigint, OUT shared_blk_read_time double precision, OUT shared_blk_write_time double precision, OUT local_blk_read_time double precision, OUT local_blk_write_time double precision, OUT temp_blk_read_time double precision, OUT temp_blk_write_time double precision, OUT wal_records bigint, OUT wal_fpi bigint, OUT wal_bytes numeric, OUT wal_buffers_full bigint, OUT jit_functions bigint, OUT jit_generation_time double precision, OUT jit_inlining_count bigint, OUT jit_inlining_time double precision, OUT jit_optimization_count bigint, OUT jit_optimization_time double precision, OUT jit_emission_count bigint, OUT jit_emission_time double precision, OUT jit_deform_count bigint, OUT jit_deform_time double precision, OUT parallel_workers_to_launch bigint, OUT parallel_workers_launched bigint, OUT stats_since timestamp with time zone, OUT minmax_stats_since timestamp with time zone) returns SETOF record
  language c;

create function public.pg_stat_statements_info(OUT dealloc bigint, OUT stats_reset timestamp with time zone) returns record
  language c;

create function public.pg_stat_statements_reset(userid oid DEFAULT 0, dbid oid DEFAULT 0, queryid bigint DEFAULT 0, minmax_only boolean DEFAULT false) returns timestamp with time zone
  language c;

