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

do $$
declare
  a cursor (x int, , y int) for select 1;
  b cursor (x int,) for select 1;
begin
  null;
end
$$;
