do $$
declare
  c cursor for select 1;
  rc refcursor;
  n int := 2;
begin
  move c;
  move from c;
  move in c;
  move rc;
  move 3 from c;
  move -1 from c;
  move all in c;
  move next from c;
  move prior from c;
  move first from c;
  move last from c;
  move forward from c;
  move forward all in c;
  move forward 2 from c;
  move backward from c;
  move backward all in c;
  move backward 2 from c;
  move relative n in c;
  move absolute 0 from c;
end
$$;

-- `move` is unreserved, so it can name the cursor it moves
do $$
declare
  move refcursor;
begin
  move move;
end
$$;

-- a record named `move` keeps its field reference
do $$
declare
  move record;
begin
  move.x := 1;
end
$$;

create function move_param(c refcursor) returns void as $$
begin
  move next from $1;
end
$$ language plpgsql;
