drop table if exists some_table;

-- schema
drop table if exists foo.some_table;

-- simple
drop table t;

-- duo
drop table a, b;

-- schema
drop table foo.t;

-- if exists
drop table if exists t;

-- cascade
drop table foo, bar cascade;

-- restrict
drop table t restrict;
