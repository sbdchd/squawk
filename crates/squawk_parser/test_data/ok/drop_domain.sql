-- simple
drop domain s;

-- full
drop domain if exists a, foo.b, c cascade;
drop domain if exists a, b, c restrict;

