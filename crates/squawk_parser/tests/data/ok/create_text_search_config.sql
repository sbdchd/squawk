-- simple
create text search configuration name (
  parser = parser_name
);

-- full
create text search configuration foo.name (
  config = source_config
);

-- copy
create text search configuration foo.name (
  copy = other_config
);

