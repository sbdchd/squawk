do $$
declare
  arr int[] := array[1, 2, 3];
  n int := 3;
  r record;
begin
  for i in 1..10 loop
    null;
  end loop;
  for i in reverse 10..1 loop
    null;
  end loop;
  for i in reverse 10..1 by 2 loop
    null;
  end loop;
  for i in 1 + 1 .. array_upper(arr, 1) loop
    null;
  end loop;
  for i in 1..n loop
    null;
  end loop;
  for i in (select 1)..(select 2) loop
    null;
  end loop;
  for r.x in 1..2 loop
    null;
  end loop;
  <<outer>> for i in 1..3 loop
    for j in 1..3 loop
      exit outer when j = 2;
    end loop;
  end loop outer;
end
$$;

-- a record named `reverse` keeps its field reference: the dot wins over the keyword
do $$
declare
  reverse record;
begin
  for i in reverse.x .. 3 loop
    null;
  end loop;
end
$$;
