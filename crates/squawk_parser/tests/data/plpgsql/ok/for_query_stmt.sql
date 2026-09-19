do $$
declare
  r record;
begin
  for r in select 1 as x loop
    null;
  end loop;
  for r in select * from generate_series(1, 3) as g(x) loop
    null;
  end loop;
  for r in select * from forq_t loop
    null;
  end loop;
  for r in select * from forq_t t loop
    null;
  end loop;
  for r in values (1, 2), (3, 4) loop
    null;
  end loop;
  for r in with c as (select 1 as x) select * from c loop
    null;
  end loop;
  for r in table forq_t loop
    null;
  end loop;
  for r.x, r.y in select 1, 2 loop
    null;
  end loop;
  <<outer>> for r in select 1 loop
    exit outer;
  end loop outer;
end
$$;

create function for_query_returning() returns void as $$
declare r record;
begin
  for r in update forq_t set x = x + 1 returning x loop
    null;
  end loop;
end
$$ language plpgsql;
