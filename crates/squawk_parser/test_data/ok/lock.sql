-- simple
lock t;

-- table_names
lock table t, only b, c *;

-- lock_mode
lock t in access share mode;
lock t in row share mode;
lock t in row exclusive mode;
lock t in share update exclusive mode;
lock t in share mode;
lock t in share row exclusive mode;
lock t in exclusive mode;
lock t in access exclusive mode;

-- all
lock table t, a *, only c in row exclusive mode nowait;

