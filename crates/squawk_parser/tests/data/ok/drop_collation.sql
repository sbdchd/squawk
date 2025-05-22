-- simple
drop collation s;

-- full
drop collation if exists foo.b cascade;
drop collation if exists a restrict;

