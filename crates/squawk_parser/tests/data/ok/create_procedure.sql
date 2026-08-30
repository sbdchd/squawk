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
set foo to true
set bar = false
set buzz from current
return 10 + 1;

-- security_variants
create procedure p() language sql security invoker as 'foo';
create procedure p() language sql external security definer as 'foo';
create procedure p() language sql security definer as 'foo';

-- as_with_two_strings
create procedure p() language c as 'foo', 'bar';

-- with_select_body
create or replace procedure p()
language sql
begin atomic
  select 1;
  select 2;
end;

