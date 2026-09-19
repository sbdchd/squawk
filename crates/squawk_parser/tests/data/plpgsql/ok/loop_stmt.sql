do $$
declare
  i int := 0;
begin
  loop
    exit;
  end loop;

  loop
    exit when i > 10;
    continue when i = 2;
    continue;
  end loop;

  while i < 10 loop
    null;
  end loop;

  while case when i = 1 then true else false end loop
    null;
  end loop;

  <<outer>>
  loop
    <<inner>>
    while i < 10 loop
      continue outer when i = 2;
      exit inner;
    end loop inner;
  end loop outer;

  <<blk>>
  begin
    exit blk;
  end blk;
end
$$;
