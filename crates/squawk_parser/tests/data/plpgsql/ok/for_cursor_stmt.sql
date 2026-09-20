do $$
declare
  c cursor for select 1 as x;
  c2 cursor (lo int, hi int) for select lo + hi as x;
  value cursor for select 1 as x;
  r record;
begin
  for r in c loop
    null;
  end loop;
  for r in c2(1, 2) loop
    null;
  end loop;
  for r in c2(hi := 2, lo => 1) loop
    null;
  end loop;
  for r.x in c loop
    null;
  end loop;
  for r in value loop
    null;
  end loop;
  <<outer>> for r in c loop
    exit outer;
  end loop outer;
end
$$;

create function for_cursor_param(cur refcursor) returns void as $$
declare r record;
begin
  for r in $1 loop
    null;
  end loop;
end
$$ language plpgsql;
