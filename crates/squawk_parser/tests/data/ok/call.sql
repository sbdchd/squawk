-- simple
CALL do_db_maintenance();

-- with_schema
CALL partman.partition_data_proc('partman_test.time_taptest_table');

-- with_named_args
CALL do_db_maintenance(x => '1', null, b => 'd');

