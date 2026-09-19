do $$
declare
  a cursor for select 1;
  b scroll cursor for select f1 from int4_tbl;
  c no scroll cursor for select f1 from int4_tbl;
  d cursor (r1 integer, r2 integer) for select * from generate_series(r1, r2) i;
  e cursor (p1 int4_tbl.f1%type) is select 1;
  cursor cursor for table t;
begin
  null;
end
$$;
