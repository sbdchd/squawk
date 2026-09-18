do $$
declare
  return int := 1;
  next rec;
begin
  return;
  return 1;
  return (select max(f1) from onecol);
  return next;
  return next 1;
  return next.x;
  return query select 1;
  return query values (1), (2);
  return query insert into t values (1) returning a;
  return query execute 'select 1';
  return query execute 'select $1' using 7;
  return query execute 'select $1, $2' using 7, 8;
  return := 2;
end
$$;
