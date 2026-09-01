-- can't have out params in aggregates
create aggregate a(in x, out y, inout z, in out w) (
  sfunc = f,
  stype = t
);

create aggregate aggregate_with_default(value integer default 1) (
  sfunc = integer_sum,
  stype = integer
);

create aggregate ordered_aggregate(
  direct integer order by aggregated integer = 1
) (
  sfunc = integer_sum,
  stype = integer
);
