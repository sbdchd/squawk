-- no query
do $$
declare r record;
begin
  for r in loop
    null;
  end loop;
end
$$;

-- REVERSE belongs to the integer form
do $$
declare r record;
begin
  for r in reverse select 1 loop
    null;
  end loop;
end
$$;

-- no `loop`
do $$
declare r record;
begin
  for r in select 1
    null;
  end loop;
end
$$;
