-- simple
create server s foreign data wrapper f;

-- full
create server if not exists s 
  type 'bar' 
  version 'foo'
  foreign data wrapper f
  options (a 'foo', bar 'b');

-- docs_1
CREATE SERVER myserver FOREIGN DATA WRAPPER postgres_fdw OPTIONS (host 'foo', dbname 'foodb', port '5432');

