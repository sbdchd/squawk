do $$
declare
  perform int := 1;
begin
  perform 1;
  perform;
  perform distinct f1 from onecol;
  perform * from onecol order by 1 limit 1;
  perform 1 union select 2;
  perform (select max(f1) from onecol);
  perform f1 from onecol where f1 > 0 group by f1 having count(*) > 1;
  perform := 2;
end
$$;
