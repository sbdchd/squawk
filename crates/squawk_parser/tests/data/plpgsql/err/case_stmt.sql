do $$
declare
  x int := 1;
begin
  case x
    else
      null;
  end case;

  case x
    when 1 then
      null;
  end case

  case case when x = 1 then 1 end
    when 1 then
      null;
  end case;

  case
    when case when x = 1 then true end then
      null;
    when x = 2, case when x = 3 then true end then
      null;
    when case when x = 4 then true end, case when x = 5 then true end then
      null;
  end case;
end
$$;
