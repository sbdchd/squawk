-- rename
alter tablespace t rename to u;

-- owner
alter tablespace t owner to u;
alter tablespace t owner to current_user;

-- set_option
alter tablespace t set (o = v);

-- set_multiple_options
alter tablespace t set (o1 = v1, o2 = v2);

-- reset_option
alter tablespace t reset (o);

-- reset_multiple_options
alter tablespace t reset (o1, o2);

