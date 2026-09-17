do $$
declare
  x int := 1;
begin
  case x
    when 1 then
      null;
  end case;

  case x
    when 1, 2 then
      null;
    when 3 then
      null;
    else
      null;
  end case;

  case
    when x = 1 then
      case x
        when 1 then
          null;
      end case;
    when x = 2 then
      null;
    else
  end case;

  case (case when x = 1 then 1 end)
    when 1 then
  end case;

  case coalesce(case when x = 1 then 1 end, 0)
    when 1 then
      null;
  end case;
end
$$;
