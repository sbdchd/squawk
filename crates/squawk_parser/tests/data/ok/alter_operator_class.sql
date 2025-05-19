-- rename
alter operator class c using m
  rename to n;

alter operator class s.c using m
  rename to n;

-- owner
alter operator class c using m
  owner to u;

alter operator class c using m
  owner to current_user;

-- schema
alter operator class c using m
  set schema s;

