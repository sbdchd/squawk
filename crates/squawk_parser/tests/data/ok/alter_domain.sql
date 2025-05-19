-- set_default
alter domain d
  set default 42;

-- drop_default
alter domain foo.d
  drop default;

-- set_not_null
alter domain d
  set not null;

-- drop_not_null
alter domain d
  drop not null;

-- add_constraint
alter domain d
  add constraint c check (value > 0);
alter domain d
  add check (value > 0) not valid;
alter domain d
  add constraint a check (a > b);

-- drop_constraint
alter domain d
  drop constraint c cascade;
alter domain d
  drop constraint if exists c restrict;

-- rename_constraint
alter domain d
  rename constraint c to n;

-- validate_constraint
alter domain d
  validate constraint c;

-- owner_to
alter domain d
  owner to u;
alter domain d
  owner to current_user;

-- rename_to
alter domain d
  rename to n;

-- set_schema
alter domain d
  set schema s;

