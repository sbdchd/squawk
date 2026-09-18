do $$
declare
  commit int := 1;
  rollback int := 1;
begin
  commit;
  commit and chain;
  commit and no chain;
  rollback;
  rollback and chain;
  rollback and no chain;
  commit := 2;
  rollback := 3;
end
$$;
