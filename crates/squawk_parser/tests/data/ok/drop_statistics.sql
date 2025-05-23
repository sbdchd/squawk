-- simple
drop statistics s;

-- full
drop statistics if exists a, foo.b, c cascade;

drop statistics if exists a, b, c restrict;

