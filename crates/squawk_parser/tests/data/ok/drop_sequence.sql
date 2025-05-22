-- simple
drop sequence s;

-- full
drop sequence if exists a, foo.b, c cascade;
drop sequence if exists a, b, c restrict;

