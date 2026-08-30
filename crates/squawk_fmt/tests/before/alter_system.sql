ALTER SYSTEM SET work_mem = '64MB';

ALTER SYSTEM SET archive_command TO 'cp %p /very/long/archive/destination/with/a/descriptive/directory/%f';

ALTER SYSTEM SET shared_preload_libraries = 'pg_stat_statements', /* second value */ 'auto_explain';

ALTER SYSTEM SET effective_cache_size FROM CURRENT;

ALTER SYSTEM RESET work_mem;

ALTER SYSTEM RESET ALL;

ALTER /* system */ SYSTEM /* action */ SET /* parameter */ autovacuum_vacuum_cost_limit /* equals */ = /* value */ 4000 /* semicolon */;

ALTER /* system */ SYSTEM /* reset */ RESET /* parameter */ autovacuum_analyze_scale_factor /* semicolon */;
