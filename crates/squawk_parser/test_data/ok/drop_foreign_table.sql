-- simple
drop foreign table s;

-- full
drop foreign table if exists a, foo.b, c cascade;
drop foreign table if exists a, b, c restrict;

