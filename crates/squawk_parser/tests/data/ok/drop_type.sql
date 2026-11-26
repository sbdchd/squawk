-- drop_type
drop type t;
drop type foo.t;
drop type a, b, c;
drop type if exists a, b, c;

drop type if exists a cascade;
drop type a restrict;

drop type if exists foo;



-- full type names
drop type double precision, character varying, int[];
