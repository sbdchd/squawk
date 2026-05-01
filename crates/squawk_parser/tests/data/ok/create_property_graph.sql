-- all the temps
create temporary property graph f;
create temp property graph f;
create local temporary property graph f;
create local temp property graph f;
create global temporary property graph f;
create global temp property graph f;
create unlogged property graph f;

create temp property graph foo.bar;

-- vertex & table edges
create property graph foo.bar
  vertex tables (buzz.boo as foo key (a, b) no properties)
  edge tables (foo.bar as z key (x, y)
    source key (a, b) references k (t, y)
    destination key (q, t) references a (r, j)
    properties all columns);

create property graph g
  edge tables (e
    source s
    destination d
    no properties);

create property graph g
  edge tables (e
    source s
    destination d
    properties (a as k, f as y));

-- vertex & table edges abbr
create property graph foo.bar
  vertex tables (boo)
  edge tables (
    bar
      source z
      destination y
  );

-- vertex tables only
create property graph foo
  edge tables (
    e
      source z
      destination y
  );

-- edge tables only
create property graph bar
  edge tables (
    e
      source z
      destination y
  );

-- vertex & table multiple tables
create property graph foo.bar
  vertex tables (boo, buzz)
  edge tables (
    bar
      source z
      destination y,
    foo
      source z
      destination y
  );
