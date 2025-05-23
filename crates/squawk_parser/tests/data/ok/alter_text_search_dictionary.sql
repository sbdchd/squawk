-- options
alter text search dictionary foo.d (
  a = 1,
  b
);

-- rename
alter text search dictionary d rename to n;

-- owner
alter text search dictionary d owner to u;
alter text search dictionary d owner to current_user;

-- schema
alter text search dictionary d set schema s;

