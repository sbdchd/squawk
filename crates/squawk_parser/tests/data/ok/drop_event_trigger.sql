-- simple
drop event trigger s;

-- full
drop event trigger if exists a cascade;
drop event trigger if exists a restrict;

drop event trigger if exists a, b restrict;
