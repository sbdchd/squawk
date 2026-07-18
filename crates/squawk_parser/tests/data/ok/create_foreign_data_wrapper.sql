-- simple
create foreign data wrapper w;

-- full
create foreign data wrapper w
  connection c.bar
  no connection
  handler foo.bar
  no handler
  validator f.bar
  no validator
  options (a 'foo', b 'bar');

