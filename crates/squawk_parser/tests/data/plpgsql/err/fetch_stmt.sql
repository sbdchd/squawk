do $$
declare
  c cursor for select 1;
  x int;
begin
  fetch c;
  fetch next c into x;
  fetch c into;
  fetch c into x[1];
  fetch c into strict x;
  -- FETCH can only return one row, so these are validations, not syntax errors
  fetch all from c into x;
  fetch 2 from c into x;
  fetch forward all from c into x;
  fetch backward 2 from c into x;
end
$$;
