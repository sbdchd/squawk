-- simple
alter default privileges
  grant select
    on tables
    to r;

-- full
alter default privileges
  for role r, current_user
  in schema s, s2, s3
  grant select
    on tables
    to r;

alter default privileges
  for user session_user
  in schema s
  grant select
    on tables
    to r;

-- grant_tables
alter default privileges
  grant select
    on tables
    to group r;

alter default privileges
  grant insert
    on tables
    to group r;

alter default privileges
  grant update
    on tables
    to public, current_user, group session_user
    with grant option;

alter default privileges
  grant delete
    on tables
    to r;

alter default privileges
  grant truncate
    on tables
    to r;

alter default privileges
  grant references
    on tables
    to r;

alter default privileges
  grant trigger
    on tables
    to r;

alter default privileges
  grant maintain
    on tables
    to r;

alter default privileges
  grant select, insert, update, delete, truncate, references, trigger, maintain
    on tables
    to r;

alter default privileges
  grant all
    on tables
    to r;

alter default privileges
  grant all privileges
    on tables
    to r;

-- grant_sequences
alter default privileges
  grant usage, select, update
  on sequences
  to u, group current_user, group session_user, group current_role
  with grant option;

alter default privileges
  grant usage
  on sequences
  to r;

-- grant_functions
alter default privileges
  grant execute
  on functions
  to u;

alter default privileges
  grant all
  on routines
  to r;

alter default privileges
  grant all privileges
  on routines
  to r
  with grant option;

-- grant_types
alter default privileges
  grant usage
  on types
  to r;

alter default privileges
  grant all privileges
  on types
  to r, group u, public
  with grant option;

-- grant_schemas
alter default privileges
  grant usage, create
  on schemas
  to r;

alter default privileges
  grant all privileges
  on schemas
  to r, group u, public
  with grant option;

-- revoke_tables
alter default privileges
  revoke grant option for
  select, insert, update, delete, truncate, references, trigger, maintain
  on tables
  from r, group current_user
  cascade;

alter default privileges
  revoke select
  on tables
  from r
  restrict;

-- revoke_sequences
alter default privileges
  revoke grant option for
  usage, select, update
  on sequences
  from r, group current_user
  cascade;

alter default privileges
  revoke select
  on sequences
  from r
  restrict;

-- revoke_functions
alter default privileges
  revoke grant option for
  execute
  on functions
  from r, group current_user
  cascade;

alter default privileges
  revoke all privileges
  on routines
  from r
  restrict;

-- revoke_types
alter default privileges
  revoke grant option for
  usage
  on types
  from r, group current_user
  cascade;

alter default privileges
  revoke all privileges
  on types
  from r
  restrict;

-- revoke_schemas
alter default privileges
  revoke grant option for
  usage, create
  on schemas
  from r, group current_user
  cascade;

alter default privileges
  revoke all privileges
  on schemas
  from r
  restrict;

