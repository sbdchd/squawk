-- simple
vacuum;

-- pg_docs
VACUUM (VERBOSE, ANALYZE) onek;

-- full
VACUUM (
    full, 
    full true, 
    full false, 
    analyze,
    analyze true,
    analyze false,
    disable_page_skipping,
    disable_page_skipping true,
    disable_page_skipping false,
    skip_locked,
    skip_locked true,
    skip_locked false,
    index_cleanup auto,
    index_cleanup on,
    index_cleanup off,
    process_main,
    process_main true,
    process_main false,
    truncate,
    truncate true,
    truncate false,
    parallel 100,
    skip_database_stats,
    skip_database_stats true,
    skip_database_stats false,
    only_database_stats,
    only_database_stats true,
    only_database_stats false,
    buffer_usage_limit 10,
    buffer_usage_limit '10 TB'
) t1;

-- pre_pg_9_syntax
vacuum full freeze verbose analyze foo, bar(a, b), c;

