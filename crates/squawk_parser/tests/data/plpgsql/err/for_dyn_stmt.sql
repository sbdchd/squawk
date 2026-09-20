-- no query
do $$
declare r record;
begin
  for r in execute loop
    null;
  end loop;
end
$$;

-- no `loop`
do $$
declare r record;
begin
  for r in execute 'select 1'
    null;
  end loop;
end
$$;

-- no parameter after `using`
do $$
declare r record;
begin
  for r in execute 'select 1' using loop
    null;
  end loop;
end
$$;

-- a trailing comma in the parameter list
do $$
declare r record;
begin
  for r in execute 'select $1' using 1, loop
    null;
  end loop;
end
$$;
