-- simple
alter function f stable;

-- rename
alter function f rename to g;
alter function s.f rename to g;

-- action_with_params
alter function f(int, text) strict;

-- owner
alter function f owner to current_user;

-- schema
alter function f set schema s;

-- multiple_actions
alter function f 
  called on null input
  returns null on null input
  strict
  immutable
  stable
  volatile
  not leakproof
  leakproof
  external security invoker
  security invoker
  external security definer
  security definer
  parallel unsafe
  parallel restricted
  parallel safe
  cost 100
  rows 10
  support f
  set c to 1
  set c = 1
  set c = default
  set c from current
  reset c
  reset all
  restrict;

-- depends
alter function f depends on extension e;
alter function f no depends on extension e;

