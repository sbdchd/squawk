-- simple
drop server s;

-- full
drop server if exists a, b, c cascade;
drop server if exists a, b, c restrict;

