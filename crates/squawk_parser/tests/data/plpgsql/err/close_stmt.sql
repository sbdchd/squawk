do $$
declare
  c refcursor;
begin
  close;
  close all;
  close c.x;
  close c[1];
end
$$;
