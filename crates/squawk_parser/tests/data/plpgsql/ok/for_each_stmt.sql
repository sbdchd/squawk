do $$
declare
  arr int[] := array[1, 2, 3];
  pairs int[][] := array[[1, 2], [3, 4]];
  x int;
  y int;
  s int[];
  r record;
  slice int;
begin
  foreach x in array arr loop
    null;
  end loop;
  foreach s slice 1 in array pairs loop
    null;
  end loop;
  foreach s slice 0 in array arr loop
    null;
  end loop;
  foreach x, y in array arr loop
    null;
  end loop;
  foreach r.x in array arr loop
    null;
  end loop;
  -- `slice` is unreserved, so it can name the loop variable
  foreach slice in array arr loop
    null;
  end loop;
  <<outer>> foreach x in array arr loop
    exit outer;
  end loop outer;
end
$$;

-- a positional parameter works as both the loop variable and the array
create function foreach_test(anyarray) returns void as $$
declare
  x int;
begin
  foreach x in array $1 loop
    null;
  end loop;
end
$$ language plpgsql;
