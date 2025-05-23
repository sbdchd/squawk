-- with_merge
with t as (
  merge into t
    using u on true
    when matched then 
      do nothing
)
select * from t;

-- with_values
with t as (
  values (1)
)
select * from t;

-- with
-- simple
with t as (
  select 1
)
select * from t;

-- column names
with t(a, b) as (
  select 1, 2
)
select * from t;

-- materialized 
with t as materialized (
  select 1
)
select * from t;

-- not materialized 
with t as not materialized (
  select 1
)
select * from t;

-- nested
select 1, (
  with t as (
    select 1, 2, 3 
  )
  select count(*)
  from t
);

-- search depth first (from pg docs)
WITH RECURSIVE search_tree(id, link, data) AS (
    SELECT t.id, t.link, t.data
    FROM tree t
  UNION ALL
    SELECT t.id, t.link, t.data
    FROM tree t, search_tree st
    WHERE t.id = st.link
) SEARCH DEPTH FIRST BY id SET ordercol
SELECT * FROM search_tree ORDER BY ordercol;

-- search breadth first (from pg docs)
WITH RECURSIVE search_tree(id, link, data) AS (
    SELECT t.id, t.link, t.data
    FROM tree t
  UNION ALL
    SELECT t.id, t.link, t.data
    FROM tree t, search_tree st
    WHERE t.id = st.link
) SEARCH BREADTH FIRST BY id SET ordercol
SELECT * FROM search_tree ORDER BY ordercol;


-- search cycle (from pg docs)
WITH RECURSIVE search_graph(id, link, data, depth) AS (
    SELECT g.id, g.link, g.data, 1
    FROM graph g
  UNION ALL
    SELECT g.id, g.link, g.data, sg.depth + 1
    FROM graph g, search_graph sg
    WHERE g.id = sg.link
) CYCLE id SET is_cycle USING path
SELECT * FROM search_graph;

-- search cycle with to and default values
WITH RECURSIVE search_graph(id, link, data, depth, path) AS (
  select 1
) CYCLE id SET is_cycle TO true DEFAULT false USING path
SELECT * FROM search_graph;

-- multi
with t as (
  select 1
),
b as (
  select 2
)
select * from t, b;

-- recursive
with recursive t as (
  select 1
)
select * from t;

with t2 as (
    with t as (
        select 1
    )
    select * from t
)
select * from t2;

-- timestamp_edge_cases
with t(timestamp) as (select 1)
select timestamp from t;

with t(time) as (select 1)
select time from t;

