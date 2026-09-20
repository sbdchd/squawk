-- no query
do $$
begin
  execute;
end
$$;

-- the query can't start at INTO
do $$
declare
  x int;
begin
  execute into x;
end
$$;

-- or at USING
do $$
begin
  execute using 1;
end
$$;

-- no INTO target
do $$
declare
  q text := 'select 1';
begin
  execute q into;
end
$$;

-- INTO twice
do $$
declare
  q text := 'select 1';
  x int;
  y int;
begin
  execute q into x into y;
end
$$;
