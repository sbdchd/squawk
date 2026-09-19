do $$
begin
  commit work;
  commit prepared 'x';
  rollback to savepoint s;
  commit and;
  commit chain;
end
$$;
