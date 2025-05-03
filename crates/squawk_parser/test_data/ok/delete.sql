-- delete
-- with where
delete from products where price = 10;

-- no where
delete from products;

-- only
delete from only t;

-- *
delete from t *;

-- alias
delete from t as a;
delete from t a;

-- using, where, return
delete from only t as a
  using foo, bar
  where x > 10
  returning *;

-- using
delete from t 
using foo f, b a;

delete from t
using order_items oi
  left join orders o on oi.order_id = o.id;

delete from user_sessions us
using users u,
  (select user_id, max(login_time) as last_login
  from login_history
  group by user_id) lh;

-- where return
delete from t
  where x > 10
  returning *;

-- returning
delete from employees e
where e.department_id in (
  select department_id 
  from departments 
  where budget < 50000
)
returning e.employee_id, e.name, e.department_id;

-- cursor
delete from invoices 
where current of invoice_cursor;

-- returning
delete from t 
returning *, foo + bar, foo.*;


-- with
with q as (
  select 1
)
delete from t as a
  using q as d
  where a.id = d.id;

with recursive q as (
  select 1
)
delete from t as a
  using q as d
  where a.id = d.id;

-- nested
with t2 as (
  with t as (
    select 1
  )
  select * from t
)
delete from t using t2;

-- pg_docs
DELETE FROM films USING producers
  WHERE producer_id = producers.id AND producers.name = 'foo';

DELETE FROM films
  WHERE producer_id IN (SELECT id FROM producers WHERE name = 'foo');

DELETE FROM films WHERE kind <> 'Musical';

DELETE FROM films;

DELETE FROM tasks WHERE status = 'DONE' RETURNING *;

DELETE FROM tasks WHERE CURRENT OF c_tasks;

WITH delete_batch AS (
  SELECT l.ctid FROM user_logs AS l
    WHERE l.status = 'archived'
    ORDER BY l.creation_date
    FOR UPDATE
    LIMIT 10000
)
DELETE FROM user_logs AS dl
  USING delete_batch AS del
  WHERE dl.ctid = del.ctid;

