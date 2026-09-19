do $$
begin
  raise notice 'x' using bogus = 1;
  raise notice 'x' using errcode;
  raise notice 'x' using errcode = ;
  raise sqlstate;
  raise exception;
end
$$;
