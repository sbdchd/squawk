select "normalize"('abc', 'def');
select "int"(1, 2);
select "position"(1);
select "xmlelement"(1);
select "coalesce"(1);
call "int"();
drop function "int"(int);
drop procedure "int"();
drop routine "int"();
drop aggregate "int"(int);
alter function "int"(int) rename to x;
create function "int"(a int) returns int as $$ select 1 $$ language sql;
create aggregate "int" (int) (sfunc = int4pl, stype = int4);

create aggregate agg1 (int) (sfunc = "int", stype = int4);
create operator ### (function = "int", leftarg = int, rightarg = int);

select 1::"int";
create table typed ("int" "int");

select coalesce(1, 2);
select normalize('abc', nfc);
select substring('abcdef' from 2 for 3);
select position('b' in 'abc');
select trim(both from ' a ');
select least(1, 2);
select row(1, 2);
select 1::int;
select int from int;

select public."int"(1);
select "myfunc"(1);
select "mycol" from "mytable";
select 1::"mytype";
