-- simple
create foreign table t()
  server s;

-- full
create foreign table if not exists t(
  a text, 
  b foo.bigint 
    options (a 'b', c 'd') 
    collate "fr_FR"
    constraint c
    not null
    null
    check (x > b) no inherit
    default 10 * 2
    generated always as (x + 2) stored,
  constraint fooo
    check (a > b)
)
  inherits (foo.bar, bar)
  server s
  options (a 'b', c 'd');

-- partitioned_simple
create foreign table t 
  partition of u
    default
    server s;

-- partitioned
create foreign table if not exists t 
  partition of foo.u(
    a with options not null check (a > 10),
    b null default 'foo',
    check (a > b)
  )
    default
    server s
    options (a 'b', c 'd');

-- partitioned_bound_spec_in
create foreign table if not exists t 
  partition of u(
    a
  )
    for values in (a > b)
    server s;

-- partitioned_bound_spec_from
create foreign table if not exists t 
  partition of u(
    a
  )
    for values from (a > b, minvalue, maxvalue) 
      to (maxvalue, minvalue)
    server s;

-- partitioned_bound_spec_with
create foreign table if not exists t 
  partition of u(
    a
  )
    for values with (modulus 10, remainder 2)
    server s;

-- with schema
create foreign table cal.event_types (
  attrs jsonb
)
  server cal_server
  options (
    object 'event-types'
  );
