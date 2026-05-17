create rule r as on update to t do instead (
  insert into t values (1)
  insert into t values (2);
);
