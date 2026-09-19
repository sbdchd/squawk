do $$
declare
  x int;
begin
  get diagnostics x = bogus;
  get diagnostics x = "row_count";
  get diagnostics x row_count;
  get diagnostics x[1] = row_count;
  get diagnostics;
end
$$;
