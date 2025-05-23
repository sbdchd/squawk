-- simple
create policy p on t;

-- as_
create policy p on t
  as permissive;

create policy p on t
  as restrictive;

-- full
create policy p on foo.t
  as permissive
  for all
  to public, current_role, current_user, session_user
  using (x > b and 1 < 2)
  with check ( x < 1);

-- for_
create policy p on t
  for all;

create policy p on t
  for select;

create policy p on t
  for insert;

create policy p on t
  for update;

create policy p on t
  for delete;

