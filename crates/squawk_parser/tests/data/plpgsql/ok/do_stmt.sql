do $$
begin
  do $q$ begin null; end $q$;
  do language plpgsql $q$ begin null; end $q$;
  do $q$ begin null; end $q$ language plpgsql;
end
$$;
