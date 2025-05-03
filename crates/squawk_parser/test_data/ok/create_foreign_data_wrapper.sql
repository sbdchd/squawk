-- simple
create foreign data wrapper w;

-- full
create foreign data wrapper w
  handler foo.bar
  no handler
  validator f.bar
  no validator
  options (a 'foo', b 'bar');

