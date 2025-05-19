-- handler
alter foreign data wrapper w
  handler h;

-- no_handler
alter foreign data wrapper w
  no handler;

-- validator
alter foreign data wrapper w
  validator v;

-- no_validator
alter foreign data wrapper w
  no validator;

-- options
alter foreign data wrapper w
  options (add o 'v', set o 'v', drop o);

-- multiple
alter foreign data wrapper w
  handler s.h
  no handler
  validator s.v
  no validator
  options (add o 'v', set o '', drop d);

-- owner
alter foreign data wrapper w
  owner to u;

-- rename
alter foreign data wrapper w
  rename to n;

