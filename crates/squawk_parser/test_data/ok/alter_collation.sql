-- refresh
alter collation c
  refresh version;

-- rename
alter collation c
  rename to d;

-- owner
alter collation c
  owner to u;
alter collation s.c
  owner to current_role;

-- set_schema
alter collation c
  set schema s;

