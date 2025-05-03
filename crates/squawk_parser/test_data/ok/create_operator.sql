-- simple
create operator > (
  function = f
);

-- procedure
create operator # (
  procedure = p
);

-- full
create operator foo.bar.>>-# (
  function = foo.bar.f,
  leftarg = varchar(100),
  rightarg = foo.bigint,
  commutator = &&&&,
  negator = <->,
  restrict = r,
  join = j,
  hashes,
  merges
);

