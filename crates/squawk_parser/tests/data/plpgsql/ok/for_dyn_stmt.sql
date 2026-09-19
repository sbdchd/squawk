do $$
declare
  r record;
  n int := 1;
begin
  for r in execute 'select 1 as x' loop
    null;
  end loop;
  for r in execute 'select $1 as x' using 1 loop
    null;
  end loop;
  for r in execute 'select $1, $2' using n, n * 2 loop
    null;
  end loop;
  for r.x, r.y in execute 'select 1, 2' loop
    null;
  end loop;
  for r in execute $q$ select 1 as x $q$ loop
    null;
  end loop;
  <<outer>> for r in execute 'select 1' loop
    exit outer;
  end loop outer;
end
$$;
