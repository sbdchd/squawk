-- simple
drop conversion s;

-- full
drop conversion if exists foo.b cascade;
drop conversion if exists buzz.b restrict;

