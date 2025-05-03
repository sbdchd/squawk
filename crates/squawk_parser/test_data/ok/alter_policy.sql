-- rename
alter policy p on t
  rename to q;

-- to_role
alter policy p on t
  to r;

-- to_multiple_roles
alter policy p on t
  to r, public, current_user;

-- using_expr
alter policy p on t
  using (a = b);

-- with_check
alter policy p on t
  with check (c > d);

-- full
alter policy p on t
  to r, s
  using (a = b)
  with check (c > d);

