-- simple
alter operator family f using i add
  operator 1 < (t, u);

-- multi
alter operator family f using i add
  operator 1 && (t, u),
  function 1 f,
  function 1 (t) f(in a text),
  function 1 (t, varchar(100)) f;

-- add_op_for_search
alter operator family f using i add
  operator 1 > (t, u) for search;

-- add_op_for_order
alter operator family f using i add
  operator 1 > (t, u) for order by s;

-- add_func
alter operator family f using i add
  function 1 f(t);

-- add_func_with_params
alter operator family f using i add
  function 1 (t, u) f(a, b);

-- drop_op
alter operator family f using i drop
  operator 1 (t, u);

-- drop_op_single_param
alter operator family f using i drop
  operator 1 (t);

-- drop_func
alter operator family f using i drop
  function 1 (t, u);

-- drop_func_single_param
alter operator family f using i drop
  function 1 (t);

-- drop_multiple
alter operator family f using i drop
  operator 1 (t, u),
  function 2 (t);

-- rename
alter operator family f using i
  rename to n;

-- owner
alter operator family f using i
  owner to u;
alter operator family f using i
  owner to current_user;

-- schema
alter operator family foo.f using i
  set schema s;

