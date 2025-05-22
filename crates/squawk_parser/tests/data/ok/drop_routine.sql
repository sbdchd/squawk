-- simple
drop routine s;

-- full
drop routine if exists a, b(text), foo.c(in f int), b(out text) cascade;
drop routine if exists a restrict;

