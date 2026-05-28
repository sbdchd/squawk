create function f()
returns void
language sql
begin atomic
  insert into t values (1);
  insert into t values (2)
end;

create function g()
returns void
language sql
begin atomic
  insert into t values (1)
  insert into t values (2);
end;
