-- simple
grant select, insert, update, delete, truncate, references, trigger, maintain
  on t
  to current_user;

-- on_table
grant all privileges 
  on table t, b, c
  to current_user, current_role
  with grant option
  granted by public;

grant all privileges
  on t
  to current_user;

-- on_all_tables
grant all privileges 
  on all tables in schema foo, bar
  to current_role
  with grant option;

-- columns
grant select(a, b, c)
  on t 
  to current_user;

grant insert(a, b, c)
  on t 
  to current_user;

grant update(a, b, c)
  on t 
  to current_user;

grant references(a, b, c)
  on t 
  to current_user;

grant all privileges(a)
  on t 
  to current_user;

grant all(a)
  on t 
  to current_user;

-- sequence
grant select on sequence s
  to current_user;

grant usage on sequence a, b, c
  to current_user;

grant update on sequence x
  to current_user;

grant all on all sequences in schema s
  to current_user;

grant all privileges on all sequences in schema a, b, c
  to current_user;

-- database
grant create
  on database a, b, c
  to current_user
  with grant option;

grant create, connect, temporary, temp
  on database a
  to current_user;

grant all
  on database a
  to current_user;

-- domain
grant usage
  on domain a, b, c
  to current_user
  with grant option;

grant all privileges
  on domain d
  to current_user;

-- foreign_data
grant usage
  on foreign data wrapper a, b, c
  to current_user
  with grant option;

grant all privileges
  on foreign data wrapper d
  to current_user;

-- foreign_server
grant usage
  on foreign server a, b, c
  to current_user
  with grant option;

grant all privileges
  on foreign server d
  to current_user;

-- function
grant execute
  on function foo, bar
  to current_user
  with grant option;

grant execute
  on procedure foo(in a text, out b numeric, bigint), bar(), z(int)
  to current_user;

grant all
  on routine r
  to current_user;

grant all
  on all functions in schema a, b, c
  to current_user;

grant all
  on all procedures in schema s
  to current_user;

grant all
  on all routines in schema s
  to current_user;

grant all privileges
  on procedure foo(in a text, out b numeric, bigint), bar
  to current_user;

-- language
grant usage 
  on language foo, bar, buzz
  to current_user;

grant all 
  on language foo, bar, buzz
  to current_user;

grant all privileges 
  on language foo, bar, buzz
  to current_user;

-- large_object
grant select, update 
  on large object 1012, 1231
  to current_user;

grant all privileges
  on large object 1012, 1231
  to current_user;

-- param
grant set, alter system
  on parameter foo, bar, buzz
  to current_user;

grant alter system
  on parameter begin
  to current_user;

grant all
  on parameter begin
  to current_user;

-- edge_case
grant set, set, set 
  to current_user
  with set option; 

grant set, set, set 
  to current_user; 

-- schema
grant create
  on schema s
  to current_user;

grant create, usage
  on schema a, b, c
  to current_user, public, group foo, current_role, session_user;

grant all
  on schema a, b, c
  to current_user;

-- tablespace
grant create
  on tablespace foo, bar, buzz
  to current_user;

grant all
  on tablespace foo, bar, buzz
  to current_user;

-- type_
grant usage
  on type a, b, c
  to current_user;

grant all 
  on type t
  to current_user;

-- option
grant public
  to current_user
  with admin option;

grant public
  to current_user
  with inherit option;

grant public
  to current_user
  with inherit true;

grant public
  to current_user
  with set false;

grant public, t(a, b)
  to current_user
  with set option;

grant public
  to current_user;

