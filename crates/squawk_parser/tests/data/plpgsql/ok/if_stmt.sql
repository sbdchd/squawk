do $$
declare
  x int := 1;
begin
  if x > 0 then
    null;
  end if;

  if x = 1 then
    null;
  else
    null;
  end if;

  if x = 1 then
    if x = 2 then
      null;
    end if;
  elsif x = 2 then
    null;
  elseif x = 3 then
    null;
  elsif (case when x = 4 then true else false end) then
    null;
  else
    null;
  end if;

  if x is null then
  end if;

  if coalesce(case when x = 1 then 1 end, 0) = 1 then
    null;
  end if;

  if (array[case when x = 1 then 1 end])[1] = 1 then
    null;
  end if;

  -- PL/pgSQL expressions accept nearly everything that can follow SELECT.
  if true as condition then
    null;
  end if;

  if true condition, false other then
    null;
  end if;

  if flag from (values (true)) as v(flag) where flag then
    null;
  end if;

  if bool_and(flag) from (values (true)) as v(flag) group by flag having bool_and(flag) then
    null;
  end if;

  if bool_and(flag) over w from (values (true)) as v(flag) window w as () then
    null;
  end if;

  if distinct flag from (values (true)) as v(flag) order by flag limit 1 offset 0 then
    null;
  end if;

  if flag from (values (true)) as v(flag) for update then
    null;
  end if;
end
$$;
