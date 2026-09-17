do $$
begin
  null;
exception
  when others then
    null;
  when division_by_zero or sqlstate '22012' then
    null;
end
$$;

do $$
<<lbl>>
begin
  begin
    null;
  exception
    when no_data_found then
      null;
  end;
end lbl;
$$;
