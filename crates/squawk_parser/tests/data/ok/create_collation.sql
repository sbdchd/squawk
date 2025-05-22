-- simple
create collation c from d;

-- from_full
create collation if not exists foo.c from bar.d;

-- with_options
create collation if not exists foo.c (
  locale = 'foo',
  lc_collate = 1,
  lc_ctype = false,
  provider = 'foo',
  deterministic = false,
  rules = r,
  version = '100'
);

