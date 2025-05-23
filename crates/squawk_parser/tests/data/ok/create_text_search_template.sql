-- simple
create text search template name (
  lexize = f
);

-- full
create text search template foo.name (
  INIT = init_function,
  lexize = lexize_function
);

