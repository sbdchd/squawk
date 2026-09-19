do $$
declare
  q text := 'select 1';
  t text := 'dynexec_t';
  x int;
  y int;
  r record;
begin
  execute 'select 1';
  execute q;
  execute 'select a from ' || quote_ident(t);
  execute q into x;
  execute q into strict x;
  execute q into r;
  execute q into r.a;
  execute 'select $1' using 1;
  execute 'select $1, $2' using 1, 2;
  execute 'select $1' into x using 1;
  -- postgres takes the two clauses in either order
  execute 'select $1' using 1 into x;
  execute 'select $1, $2' into strict x using 1, 2;
  execute 'select 1, 2' into strict x, y;
end
$$;

-- a positional parameter is both the query and a USING value
create function dynexec_test(text) returns int as $$
declare
  x int;
begin
  execute $1 into x using $1;
  return x;
end
$$ language plpgsql;
