-- missing comma
drop table foo, bar buzz cascade;

-- missing name
drop table foo,   , buzz cascade;

-- trailing comma
drop table p,;
drop table p, cascade;
drop table p,
