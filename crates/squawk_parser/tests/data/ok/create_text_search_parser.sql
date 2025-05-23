-- simple
create text search parser name (
  start = start_function,
  gettoken = gettoken_function,
  end = end_function,
  lextypes = lextypes_function
);

-- full
create text search parser foo.name (
  start = start_function,
  gettoken = gettoken_function,
  end = end_function,
  lextypes = lextypes_function,
  headline = headline_function
);

