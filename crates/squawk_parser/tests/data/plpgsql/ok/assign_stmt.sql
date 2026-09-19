do $$
declare
  a int[];
  r record;
begin
  x := 1;
  x = 1;
  a[1] := 2;
  a[1:2] := array[1,2];
  a[1:2].i := array[11,12];
  r.f1 := 3;
  r.c1[1].i := 11;
  $1 := 4;
  values := 5;
  perform := 6;
  x := case when r.f1 = 1 then 'a' else 'b' end;
  x := (select max(f1) from onecol);
end
$$;
