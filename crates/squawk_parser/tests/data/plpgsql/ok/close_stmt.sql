do $$
declare
  c cursor for select 1;
  rc refcursor;
begin
  close c;
  close rc;
  CLOSE C;
end
$$;

-- `close` is unreserved, so it can also name the cursor it closes
do $$
declare
  close refcursor;
  fetch refcursor;
  "table" refcursor;
begin
  close close;
  close fetch;
  close "table";
end
$$;

-- a record named `close` keeps its field reference
do $$
declare
  close record;
begin
  close.x := 1;
end
$$;

create function close_param(c refcursor) returns void as $$
begin
  close $1;
end
$$ language plpgsql;
