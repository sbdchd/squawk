-- simple
drop materialized view s;

-- full
drop materialized view if exists a, foo.b, c cascade;
drop materialized view if exists a, b, c restrict;

