-- create_extension
create extension foo;
create extension if not exists foo;

create extension hstore schema addons;

create extension foo
  with schema bar
  version foo
  cascade;

create extension foo
  schema bar
  version 'buzz';

