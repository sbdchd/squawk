-- missing comma in column list
insert into t (a, b c)
  values (1, 2, 3)
  on conflict do nothing;

-- missing column in column list & trailing comma
insert into t (a,,c,)
  values (1, 2, 3)
  on conflict do nothing;

-- missing comma in values & trailing comma
insert into t (a, b, c)
  values (4, 5  6,)
  on conflict do nothing;
