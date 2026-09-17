do $$
declare
  a cursor () for select 1;
  b no cursor for select 1;
  c cursor select 1;
  d int;
begin
  null;
end
$$;
