-- simple
set work_mem = '1MB';
set work_mem to '1MB';
set work_mem to default;
set work_mem from current;
set search_path = a, b;

-- scopes
set local work_mem = '1MB';
set session work_mem = '1MB';

-- session and local are the configuration parameter
set local = 1;
set local to 1;
set local.x = 1;
set local from current;
set session = 1;
set session.x = 1;
