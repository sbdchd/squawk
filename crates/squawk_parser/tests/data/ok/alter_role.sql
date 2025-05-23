-- with_options
alter role r with superuser;

-- all_options
alter role r
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
  password 'bar' password null
  valid until '2042-02-22';

-- rename
alter role r rename to newname;

-- set_config
alter role r set param = 'value';
alter role r set param to 'value';
alter role r set param to default;

-- set_config_from_current
alter role r set param from current;

-- reset_config
alter role r reset param;

-- reset_all
alter role r reset all;

-- in_database
alter role r in database d set param to 'value';

-- using_current_user
alter role current_user with nologin;

-- for_all_roles
alter role all reset all;

