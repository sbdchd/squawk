-- set_to
alter system set p to 'v';

-- set_equals
alter system set p = 'v';

-- schema
alter system set foo.p = 'v';
alter system reset foo.p;

-- set_multiple
alter system set p to 'v1', 'v2', 'v3';

-- set_default
alter system set p to default;

-- reset_param
alter system reset p;

-- reset_all
alter system reset all;

