-- `reverse` with no `..`: postgres reads it as a query loop and refuses REVERSE
do $$
begin
  for i in reverse 1 loop
    null;
  end loop;
end
$$;

-- no loop variable
do $$
begin
  for in 1..2 loop
    null;
  end loop;
end
$$;

-- no upper bound
do $$
begin
  for i in 1.. loop
    null;
  end loop;
end
$$;

-- no `loop`
do $$
begin
  for i in 1..2
    null;
  end loop;
end
$$;

-- no step after `by`
do $$
begin
  for i in 1..2 by loop
    null;
  end loop;
end
$$;

-- an integer loop takes one variable, not a list
do $$
declare
  i int;
  j int;
begin
  for i, j in 1..2 loop
    null;
  end loop;
end
$$;
