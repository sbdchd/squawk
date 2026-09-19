do $$
declare
  c cursor for select 1;
  rc refcursor;
begin
  open;
  open rc for;
  open rc for execute;
  open rc select 1;
  open c.x;
end
$$;
