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


-- when_clauses_all
merge into t
  using u
    on t.id = u.id
  when not matched then
    do nothing
  when matched then 
    do nothing
  when not matched by source then
    do nothing;

-- when_clauses_all_with_conditions
merge into t
  using u on true
  when matched and foo = bar then
    do nothing
  when not matched by source and bar = foo then
    do nothing
  when not matched and buzz = bar then 
    do nothing;

-- returning_all
merge into t
  using u
    on t.id = u.id
  when not matched then
    do nothing
  when matched then 
    do nothing
  when not matched by source then
    do nothing
  returning *;

-- returning_many
merge into t
  using u
    on t.id = u.id
  when not matched then
    do nothing
  when matched then 
    do nothing
  when not matched by source then
    do nothing
  returning *, u as bar, t b, merge_action(), t.*;

-- merge_insert_simple
merge into t
  using u
    on t.id = u.id
  when not matched then
    insert
    default values;

-- merge_insert_default
merge into t
  using u
    on t.id = u.id
  when not matched then
    insert (a, b, c)
    overriding user value
    default values;

-- merge_insert_values
merge into t
  using u
    on t.id = u.id
  when not matched then
    insert
    overriding system value
    values (1, 2, default, 3, 10 * 10 + 2);

-- merge_update
merge into t
  using u
    on t.id = u.id
  when matched then
    update set
      a = default,
      b = 1,
      c = d,
      e = (select 1),
      f = row(1, 2, default),
      g = (1, 2, default),
      h = (default)
  when not matched by source then
    update set foo = bar;

-- merge_delete
merge into t
  using u
    on t.id = u.id
  when matched then
    delete
  when not matched by source then
    delete;

-- with_select
with t as (select 1, 2, 3)
merge into u
  using t
    on t.id = u.id
  when matched then
    do nothing;

-- doc_example_1
MERGE INTO customer_account ca
USING recent_transactions t
ON t.customer_id = ca.customer_id
WHEN MATCHED THEN
  UPDATE SET balance = balance + transaction_value
WHEN NOT MATCHED THEN
  INSERT (customer_id, balance)
  VALUES (t.customer_id, t.transaction_value);

-- doc_example_2
MERGE INTO customer_account ca
USING (SELECT customer_id, transaction_value FROM recent_transactions) AS t
ON t.customer_id = ca.customer_id
WHEN MATCHED THEN
  UPDATE SET balance = balance + transaction_value
WHEN NOT MATCHED THEN
  INSERT (customer_id, balance)
  VALUES (t.customer_id, t.transaction_value);

-- doc_example_3
MERGE INTO wines w
USING wine_stock_changes s
ON s.winename = w.winename
WHEN NOT MATCHED AND s.stock_delta > 0 THEN
  INSERT VALUES(s.winename, s.stock_delta)
WHEN MATCHED AND w.stock + s.stock_delta > 0 THEN
  UPDATE SET stock = w.stock + s.stock_delta
WHEN MATCHED THEN
  DELETE
RETURNING merge_action(), w.*;

-- doc_example_4
MERGE INTO wines w
USING new_wine_list s
ON s.winename = w.winename
WHEN NOT MATCHED BY TARGET THEN
  INSERT VALUES(s.winename, s.stock)
WHEN MATCHED AND w.stock != s.stock THEN
  UPDATE SET stock = s.stock
WHEN NOT MATCHED BY SOURCE THEN
  DELETE;

-- cross_join_data_source
merge into t
  using u cross join v
  on t.id = u.id
  when matched then
    do nothing;

-- natural join
merge into t
  using u natural join v
  on t.id = u.id
  when matched then
    do nothing;
