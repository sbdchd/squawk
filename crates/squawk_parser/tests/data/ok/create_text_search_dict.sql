-- simple
create text search dictionary name (
  template = t
);

-- full
create text search dictionary foo.name (
  template  = t,
  foo = bar,
  a = b
);

