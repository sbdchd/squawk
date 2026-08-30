-- do only takes one language and one body
do language plpgsql $$ x $$ language sql;

-- repeated function options
create function f() returns int
  language sql
  language sql
  as 'select 1';

create function f() returns int
  immutable
  stable
  as 'select 1';

create function f() returns int
  strict
  called on null input
  as 'select 1';

create function f() returns int
  security invoker
  external security definer
  as 'select 1';

create function f() returns int
  as ''
  as 'foo', 'bar';

alter function f() cost 1 cost 2;

alter function f() leakproof not leakproof;

-- as and inline SQL bodies conflict
create function f() returns int language sql as 'select 1' return 2;
create procedure p() language sql as 'select 1' begin atomic end;

-- repeated set and reset are allowed, the last one wins
alter function f()
  set a = 1
  set a = 2
  reset a
  reset all;

-- the routine body isn't an option
create function f() returns int
  language sql
  begin atomic
    select 1;
  end;
