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
  immutable
  not leakproof
  external security invoker
  parallel unsafe
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

-- strict_variants
alter function f returns null on null input;
alter function f strict;

-- volatility_variants
alter function f stable;
alter function f volatile;

-- leakproof_variants
alter function f leakproof;

-- security_variants
alter function f security invoker;
alter function f external security definer;
alter function f security definer;

-- parallel_variants
alter function f parallel restricted;
alter function f parallel safe;

-- depends
alter function f depends on extension e;
alter function f no depends on extension e;

