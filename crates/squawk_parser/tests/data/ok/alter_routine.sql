-- simple
alter routine r
  stable;

-- rename
alter routine r
  rename to n;

-- owner
alter routine r
  owner to current_user;

-- schema
alter routine r
  set schema s;

-- depends
alter routine r
  depends on extension e;

alter routine r
  no depends on extension e;

-- no_depends
alter routine r
  no depends on extension e;

-- with_params
alter routine f(in a text, out b int)
  parallel safe;

-- all_actions
alter routine r
  immutable
  stable
  volatile
  not leakproof
  leakproof
  external security invoker
  security invoker
  security definer
  external security definer
  parallel unsafe
  parallel restricted
  parallel safe
  cost 10
  rows 10
  set c = 1
  set c = default
  set c to true
  reset c
  reset all
  restrict;

