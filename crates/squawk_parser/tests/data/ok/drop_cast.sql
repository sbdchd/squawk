-- simple
drop cast (text as int);

-- full
drop cast if exists (a as b) cascade;
drop cast if exists (foo.a as bar.b) restrict;

