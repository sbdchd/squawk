-- simple
revoke select, insert, update, delete, truncate, references, trigger, maintain
  on t
  from current_user;

-- on_table
revoke grant option for 
  all privileges 
  on table t, b, c
  from current_user, current_role
  granted by public
  cascade;

revoke grant option for all privileges
  on t
  from current_user;

-- on_all_tables
revoke grant option for 
  all privileges 
  on all tables in schema foo, bar
  from current_role
  restrict;

-- columns
revoke select(a, b, c)
  on t 
  from current_user;

revoke insert(a, b, c)
  on t 
  from current_user;

revoke update(a, b, c)
  on t 
  from current_user;

revoke references(a, b, c)
  on t 
  from current_user;

revoke all privileges(a)
  on t 
  from current_user;

revoke all(a)
  on t 
  from current_user;

-- sequence
revoke select on sequence s
  from current_user;

revoke usage on sequence a, b, c
  from current_user;

revoke update on sequence x
  from current_user;

revoke all on all sequences in schema s
  from current_user;

revoke all privileges on all sequences in schema a, b, c
  from current_user;

-- database
revoke grant option for create
  on database a, b, c
  from current_user;

revoke create, connect, temporary, temp
  on database a
  from current_user;

revoke all
  on database a
  from current_user;

-- domain
revoke grant option for usage
  on domain a, b, c
  from current_user;

revoke all privileges
  on domain d
  from current_user;

-- foreign_data
revoke grant option for usage
  on foreign data wrapper a, b, c
  from current_user;

revoke all privileges
  on foreign data wrapper d
  from current_user;

-- foreign_server
revoke grant option for usage
  on foreign server a, b, c
  from current_user;

revoke all privileges
  on foreign server d
  from current_user;

-- function
revoke grant option for execute
  on function foo, bar
  from current_user;

revoke execute
  on procedure foo(in a text, out b numeric, bigint), bar(), z(int)
  from current_user;

revoke all
  on routine r
  from current_user;

revoke all
  on all functions in schema a, b, c
  from current_user;

revoke all
  on all procedures in schema s
  from current_user;

revoke all
  on all routines in schema s
  from current_user;

revoke all privileges
  on procedure foo(in a text, out b numeric, bigint), bar
  from current_user;

-- language
revoke usage 
  on language foo, bar, buzz
  from current_user;

revoke all 
  on language foo, bar, buzz
  from current_user;

revoke all privileges 
  on language foo, bar, buzz
  from current_user;

-- large_object
revoke select, update 
  on large object 1012, 1231
  from current_user;

revoke all privileges
  on large object 1012, 1231
  from current_user;

-- param
revoke set, alter system
  on parameter foo, bar, buzz
  from current_user;

revoke alter system
  on parameter begin
  from current_user;

revoke all
  on parameter begin
  from current_user;

-- edge_case
revoke set option for 
  set, set, set from current_user; 

revoke set, set, set from current_user; 

-- schema
revoke create
  on schema s
  from current_user;

revoke create, usage
  on schema a, b, c
  from current_user;

revoke all
  on schema a, b, c
  from current_user;

-- tablespace
revoke create
  on tablespace foo, bar, buzz
  from current_user;

revoke all
  on tablespace foo, bar, buzz
  from current_user;

-- type_
revoke usage
  on type a, b, c
  from current_user;

revoke all 
  on type t
  from current_user;

-- option
revoke admin option for
  public
  from current_user;

revoke inherit option for
  public
  from current_user;

revoke set option for
  public
  from current_user;

revoke public
  from current_user;

