do $$
declare
  c cursor for select 1;
  x int;
  y int;
  r record;
begin
  fetch c into x;
  fetch from c into x;
  fetch in c into x;
  fetch next from c into x;
  fetch prior from c into x;
  fetch first from c into x;
  fetch last from c into x;
  fetch absolute 1 from c into x;
  fetch relative 1 from c into x;
  fetch forward from c into x;
  fetch backward from c into x;
  fetch c into x, y;
  fetch c into r;
  fetch c into r.f;
end
$$;

do $$
declare
  fetch refcursor;
  x int;
begin
  fetch fetch into x;
end
$$;

create function fetch_param(c refcursor) returns void as $$
declare
  x int;
begin
  fetch next from $1 into x;
end
$$ language plpgsql;
