-- simple
merge into t
  using u
    on t.id = u.id
  when matched then 
    do nothing;

-- aliases_with_as
merge into foo as f
  using bar as b
    on f.id = b.id
  when matched then 
    do nothing;

-- using_rows_from
merge into t
  using rows from (f(1, 2))
    on true
  when matched then 
    do nothing;

-- aliases_no_as
merge into foo f
  using bar b
    on f.id = b.id
  when matched then 
    do nothing;

-- table_with_star
merge into t *
  using u
    on t.id = u.id
  when matched then 
    do nothing;

-- table_with_only
merge into only t
  using u
    on t.id = u.id
  when matched then 
    do nothing;

-- paren_query
merge into only t
  using (select id from bar)
    on t.id = u.id
  when matched then 
    do nothing;

merge into only t
  using (select id from bar join foo as f on f.id = bar.id) as u
    on t.id = u.id
  when matched then 
    do nothing;

merge into only t
  using (select id from bar) u
    on t.id = u.id
  when matched then 
    do nothing;

-- when_clauses_dupe
merge into t
  using u
    on t.id = u.id
  when matched then 
    do nothing
  when matched then 
    do nothing;
