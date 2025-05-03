-- create_schema
create schema myschema;
create schema s authorization foo;
create schema s authorization current_role;
create schema s authorization current_user;
create schema s authorization session_user;

create schema authorization foo;

create schema if not exists s;
create schema if not exists s authorization bar;
create schema if not exists authorization bar;

create schema s
  create table t (a int, b text)
  create table t1 (z int8);

table schema.table;
table database.schema.table;

drop schema myschema;
drop schema myschema cascade;
drop schema if exists myschema restrict;
drop schema if exists a, b, c;

create schema schema_name authorization user_name;

-- create_schema_with_sequence
create schema s
  create sequence s;

-- create_schema_with_trigger
create schema s
  create trigger t after insert
    on u
    execute function f();

-- search_path
show search_path;

set search_path to myschema,public;

set search_path to myschema;

set foo = bar;

set time zone 'America/Los_Angeles';
set time zone default;
set time zone local;

set foo = default;
set foo to a, 10.0, 1, 'foo', true, false;

-- operator
-- binary
select 3 operator(pg_catalog.+) 4;

select 3 operator(+) 4;

select 1 operator(a.&&) 2;

-- unary
select operator(pg_catalog.-) 4;
select operator(-) 4;

select operator(a.b.-) 4;

