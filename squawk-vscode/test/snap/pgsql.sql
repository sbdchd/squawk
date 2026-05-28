select count(1) from tbl;
select 'hello'::text;
select 'line\nvalue';
select E'line\nvalue';
select U&'d\0061t\0061';
select $$hello$$;
select $tag$hello$tag$;
select 0xFF;
select 0o77;
select 0b1010;
select 1_000_000;
select 1.5;
select 1e10;
select "Col" from "Tbl";
select 1 as U&"foo";
select B'1010';
select X'FF';
select $1;
-- comment
\echo hi
create function foo()
returns int
as $$
select 1;
$$
language sql;
