-- can't have out params in aggregates
create aggregate a(in x, out y, inout z, in out w) (
  sfunc = f,
  stype = t
);
