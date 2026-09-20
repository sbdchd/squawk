do $$
declare
  c cursor for select 1;
  c2 cursor (lo int, hi int) for select generate_series(lo, hi);
  rc refcursor;
  x int := 1;
begin
  open c;
  open c2(1, 2);
  open c2 (lo := 1, hi => 2);
  open rc for select 1;
  open rc scroll for select 1;
  open rc no scroll for select 1;
  open rc for values (1), (2);
  open rc for with t as (select 1 as a) select a from t;
  open rc for execute 'select $1' using x;
  open rc for execute format('select %s', 1);
end
$$;

-- `open` is unreserved, so it can name the cursor it opens
do $$
declare
  open refcursor;
begin
  open open for select 1;
end
$$;

-- a record named `open` keeps its field reference: the dot wins over the keyword
do $$
declare
  open record;
begin
  open.x := 1;
end
$$;

create function open_param(c refcursor) returns void as $$
begin
  open $1 for select 1;
end
$$ language plpgsql;
