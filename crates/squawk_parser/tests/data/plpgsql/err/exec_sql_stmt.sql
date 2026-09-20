-- the SQL grammar can't start a statement here
do $$
begin
  insert;
end
$$;

-- the SQL grammar stops short, so the rest of the statement is one error
do $$
begin
  drop table exs_t cascade extra;
end
$$;
