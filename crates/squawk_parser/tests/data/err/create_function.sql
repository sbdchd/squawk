create function f()
-- missing comma
returns table (a text  b int)
as '' language sql;

create function foo(arg float = 1 int4 = 4)
-- missing comma                 ^
returns void
as ''
language sql;

-- regression partial definition
create function

