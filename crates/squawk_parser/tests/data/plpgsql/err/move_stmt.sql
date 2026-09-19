do $$
declare
  c cursor for select 1;
  n int := 2;
  x int;
begin
  move;
  move next c;
  move c into x;
  -- a word in the count position names the cursor, so `from c` is left over
  move n from c;
end
$$;
