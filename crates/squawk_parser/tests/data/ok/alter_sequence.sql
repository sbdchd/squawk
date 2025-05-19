-- full
alter sequence if exists foo.s
  as varchar(100)
  increment by 2
  minvalue 1
  no minvalue
  maxvalue 100
  no maxvalue
  start 10
  start with 10
  restart
  restart 10
  restart with 10
  cache 10
  no cycle
  cycle
  owned by foo.c
  owned by none;

-- set_logged
alter sequence s
  set logged;

alter sequence s
  set unlogged;

-- owner_to
alter sequence s
  owner to u;

alter sequence s
  owner to current_user;

-- rename
alter sequence s
  rename to t;

-- schema
alter sequence s
  set schema x;

