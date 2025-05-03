-- can't have out params in aggregates
create aggregate a(in x, out y) (
  sfunc = f,
  stype = t
);
