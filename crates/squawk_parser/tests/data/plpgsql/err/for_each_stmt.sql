-- no `array`
do $$
declare
  arr int[] := array[1];
  x int;
begin
  foreach x in arr loop
    null;
  end loop;
end
$$;

-- a slice operand that is not an integer
do $$
declare
  arr int[] := array[1];
  x int;
  n int := 1;
begin
  foreach x slice n in array arr loop
    null;
  end loop;
end
$$;

-- no loop variable
do $$
declare
  arr int[] := array[1];
begin
  foreach in array arr loop
    null;
  end loop;
end
$$;
