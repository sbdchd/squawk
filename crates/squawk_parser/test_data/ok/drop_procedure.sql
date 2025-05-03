-- simple
drop procedure s;

-- full
drop procedure if exists a, b(text), foo.c(in f int), b(out text) cascade;
drop procedure if exists a restrict;

