-- column_set_default
alter view v alter c set default 42;

alter view if exists v alter column c set default current_timestamp;

-- column_drop_default
alter view v alter column c drop default;

-- owner_to
alter view v owner to u;

-- owner_to_current_role
alter view if exists s.v owner to current_role;

-- owner_to_current_user
alter view v owner to current_user;

-- owner_to_session_user
alter view v owner to session_user;

-- rename_to
alter view v rename to n;

-- rename_column
alter view v rename column a to b;
alter view v rename a to b;

-- set_schema
alter view v set schema s;

-- set_options
alter view v set (a = 'x', b = 100, c, d = true);

-- reset_options
alter view v reset (a, b, c);

