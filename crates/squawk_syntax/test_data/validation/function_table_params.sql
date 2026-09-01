create function f(out x int) returns table (y int) language sql as $$ select 1 $$;

create function f(inout x int) returns table (y int) language sql as $$ select x $$;

create function f(in out x int) returns table (y int) language sql as $$ select x $$;
