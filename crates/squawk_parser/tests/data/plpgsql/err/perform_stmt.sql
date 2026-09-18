do $$
begin
  perform from;
  perform 1 into x;
  perform 1 into strict x;
  perform with x as (select 1) select * from x;
  perform table onecol;
  perform values (1);
end
$$;
