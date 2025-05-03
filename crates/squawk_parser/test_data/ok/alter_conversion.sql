-- rename
alter conversion c rename to n;

-- owner
alter conversion c owner to u;
alter conversion c owner to current_user;

-- set_schema
alter conversion c set schema s;

-- qualified_name
alter conversion a.c set schema s;

