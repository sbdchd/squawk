-- setof is only allowed in function return types
create table t (a setof int);

alter table t add column b setof int;

create type c as (a setof int);

alter type c add attribute b setof int;

select * from f() as t(x setof int);

create function f(a setof int) returns int language sql return 1;

create function f() returns table (a setof int) language sql return 1;
