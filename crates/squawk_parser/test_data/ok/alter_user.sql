-- simple
alter user u superuser;

-- full
alter user u with 
 superuser
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
 connection limit 10
 encrypted password ''
 password null
 password 'foo'
 valid until '2025-01-01';

-- rename_user
alter user u rename to v;

-- set
alter user u set p = 'value';
alter user u set p to 'value';
alter user u set p to default;

alter user u in database d set p to 'value';
alter user u set p from current;

-- reset
alter user u reset p;
alter user u reset all;
alter user u in database d reset p;

