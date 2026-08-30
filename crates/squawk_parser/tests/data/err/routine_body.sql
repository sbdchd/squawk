-- the routine body must come after the options
create function f() returns int return 1 language sql;
create function f() returns int begin atomic select 1; end language sql;

-- only one routine body
create function f() returns int language sql return 1 return 2;

-- alter never takes a routine body
alter function f() return 1;
alter procedure p() begin atomic select 1; end;
alter routine r() return 1;
