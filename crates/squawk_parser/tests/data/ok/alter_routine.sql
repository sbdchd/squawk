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
  not leakproof
  external security invoker
  parallel unsafe
  cost 10
  rows 10
  set c = 1
  set c = default
  set c to true
  reset c
  reset all
  restrict;

-- volatility_variants
alter routine r stable;
alter routine r volatile;

-- leakproof_variants
alter routine r leakproof;

-- security_variants
alter routine r security invoker;
alter routine r security definer;
alter routine r external security definer;

-- parallel_variants
alter routine r parallel restricted;
alter routine r parallel safe;

