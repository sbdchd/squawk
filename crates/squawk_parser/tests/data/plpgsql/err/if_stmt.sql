do $$
begin
  if 1 = 1
    null;
  end if;

  if 1 = 1 then
    null;
  else
    null;
  elsif 1 = 2 then
    null;
  end if;

  -- `expr_until_then` ends the condition at the case's own `then`
  if case when true then 1 end = 1 then
    null;
  elsif 1 = case when true then 1 end then
    null;
  end if;

  null;
end
$$;
