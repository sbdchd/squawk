-- a cursor loop takes one variable, not a list
do $$
declare
  c cursor for select 1, 2;
  a int;
  b int;
begin
  for a, b in c loop
    null;
  end loop;
end
$$;

-- a trailing comma in the argument list
do $$
declare
  c2 cursor (lo int) for select lo;
  r record;
begin
  for r in c2(1,) loop
    null;
  end loop;
end
$$;
