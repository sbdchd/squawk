-- simple
alter server s
  version 'v1';

-- full
alter server s
  version 'v1'
  options (add o 'val', drop p);

-- owner
alter server s
  owner to u;
alter server s
  owner to current_user;

-- rename
alter server s
  rename to t;

