-- rename
alter database d rename to n;

-- owner
alter database d owner to u;
alter database d owner to current_user;

-- tablespace
alter database d set tablespace t;

-- refresh
alter database d refresh collation version;

-- set_param
alter database d set p to v;
alter database d set p = v;
alter database d set p = default;
alter database d set p from current;

-- reset
alter database d reset p;
alter database d reset all;

-- with_option
alter database d with allow_connections true;

-- option_connection_limit
alter database d connection limit 10;

-- option_is_template
alter database d is_template false;

-- with_multiple_options
alter database d with allow_connections true connection limit 10 is_template false;

