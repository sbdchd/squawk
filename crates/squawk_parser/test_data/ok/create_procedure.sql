-- simple
create procedure p()
language sql;

-- full
create or replace procedure p(
  in a text DEFAULT 'foo',
  out b bigint = 10,
  bigint = 1
)
language sql
transform for type foo.t, for type text
external security invoker
security invoker
external security definer
security definer
set foo to true
set bar = false
set buzz from current
as 'foo'
as 'foo', 'bar'
return 10 + 1;

-- with_select_body
create or replace procedure p()
language sql
begin atomic
  select 1;
  select 2;
end;

