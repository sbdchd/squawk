-- simple
drop function s;

-- full
drop function if exists a, b(text), foo.c(in f int), b(out text) cascade;
drop function if exists a restrict;

