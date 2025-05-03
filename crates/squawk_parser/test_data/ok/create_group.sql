-- simple
create group g;

-- full
create group g
  with superuser
  nosuperuser
  nosuperuser
  createdb
  nocreatedb
  createrole
  nocreaterole
  inherit
  noinherit
  login
  nologin
  replication
  noreplication
  bypassrls
  nobypassrls
  connection limit 100
  encrypted password 'foo'
  password 'bar'
  password null
  valid until '2042-02-22'
  in role foo, bar, buzz
  in group foo
  role r, current_user
  admin foo, bar, buzz
  sysid 100;

