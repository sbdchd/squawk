do $$
begin
  begin
    null;
  end
end
$$;

do $$
begin
  <<inner>>
  begin
    null;
  end inner
end
$$;

do $$
begin
  null;
exception
  when division_by_zero or then
    null;
end
$$;
