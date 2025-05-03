-- rename
alter procedure p
  rename to q;
alter procedure foo.p(int, text)
  rename to q;

-- owner
alter procedure p
  owner to u;
alter procedure p
  owner to current_user;

-- set_schema
alter procedure p
  set schema s;

-- security
alter procedure p
  security invoker
  external security invoker
  security definer
  external security definer 
  restrict;

-- actions
alter procedure p
  security invoker
  set c = 1
  set d from current
  reset z
  reset all
  restrict;

-- set_param
alter procedure p
  set c to v;
alter procedure p
  set c = v;
alter procedure p
  set c = default;
alter procedure p
  set c from current;

-- reset_param
alter procedure p
  reset c;
alter procedure p
  reset all;

-- depends
alter procedure p
  depends on extension e;
alter procedure p
  no depends on extension e;

