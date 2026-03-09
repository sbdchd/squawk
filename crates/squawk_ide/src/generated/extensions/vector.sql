-- squawk-ignore-file
-- pg version: 18.2
-- update via:
--   cargo xtask sync-builtins

-- size: -1, align: 4
create type public.halfvec;

-- size: -1, align: 4
create type public.sparsevec;

-- size: -1, align: 4
create type public.vector;

create function public.array_to_halfvec(double precision[], integer, boolean) returns halfvec
  language c;

create function public.array_to_halfvec(integer[], integer, boolean) returns halfvec
  language c;

create function public.array_to_halfvec(numeric[], integer, boolean) returns halfvec
  language c;

create function public.array_to_halfvec(real[], integer, boolean) returns halfvec
  language c;

create function public.array_to_sparsevec(double precision[], integer, boolean) returns sparsevec
  language c;

create function public.array_to_sparsevec(integer[], integer, boolean) returns sparsevec
  language c;

create function public.array_to_sparsevec(numeric[], integer, boolean) returns sparsevec
  language c;

create function public.array_to_sparsevec(real[], integer, boolean) returns sparsevec
  language c;

create function public.array_to_vector(double precision[], integer, boolean) returns vector
  language c;

create function public.array_to_vector(integer[], integer, boolean) returns vector
  language c;

create function public.array_to_vector(numeric[], integer, boolean) returns vector
  language c;

create function public.array_to_vector(real[], integer, boolean) returns vector
  language c;

create aggregate public.avg(halfvec) (
  sfunc = halfvec_accum,
  stype = double precision[],
  finalfunc = halfvec_avg,
  combinefunc = halfvec_combine,
  initcond = '{0}'
);

create aggregate public.avg(vector) (
  sfunc = vector_accum,
  stype = double precision[],
  finalfunc = vector_avg,
  combinefunc = vector_combine,
  initcond = '{0}'
);

create function public.binary_quantize(halfvec) returns bit
  language c;

create function public.binary_quantize(vector) returns bit
  language c;

create function public.cosine_distance(halfvec, halfvec) returns double precision
  language c;

create function public.cosine_distance(sparsevec, sparsevec) returns double precision
  language c;

create function public.cosine_distance(vector, vector) returns double precision
  language c;

create function public.halfvec(halfvec, integer, boolean) returns halfvec
  language c;

create function public.halfvec_accum(double precision[], halfvec) returns double precision[]
  language c;

create function public.halfvec_add(halfvec, halfvec) returns halfvec
  language c;

create function public.halfvec_avg(double precision[]) returns halfvec
  language c;

create function public.halfvec_cmp(halfvec, halfvec) returns integer
  language c;

create function public.halfvec_combine(double precision[], double precision[]) returns double precision[]
  language c;

create function public.halfvec_concat(halfvec, halfvec) returns halfvec
  language c;

create function public.halfvec_eq(halfvec, halfvec) returns boolean
  language c;

create function public.halfvec_ge(halfvec, halfvec) returns boolean
  language c;

create function public.halfvec_gt(halfvec, halfvec) returns boolean
  language c;

create function public.halfvec_in(cstring, oid, integer) returns halfvec
  language c;

create function public.halfvec_l2_squared_distance(halfvec, halfvec) returns double precision
  language c;

create function public.halfvec_le(halfvec, halfvec) returns boolean
  language c;

create function public.halfvec_lt(halfvec, halfvec) returns boolean
  language c;

create function public.halfvec_mul(halfvec, halfvec) returns halfvec
  language c;

create function public.halfvec_ne(halfvec, halfvec) returns boolean
  language c;

create function public.halfvec_negative_inner_product(halfvec, halfvec) returns double precision
  language c;

create function public.halfvec_out(halfvec) returns cstring
  language c;

create function public.halfvec_recv(internal, oid, integer) returns halfvec
  language c;

create function public.halfvec_send(halfvec) returns bytea
  language c;

create function public.halfvec_spherical_distance(halfvec, halfvec) returns double precision
  language c;

create function public.halfvec_sub(halfvec, halfvec) returns halfvec
  language c;

create function public.halfvec_to_float4(halfvec, integer, boolean) returns real[]
  language c;

create function public.halfvec_to_sparsevec(halfvec, integer, boolean) returns sparsevec
  language c;

create function public.halfvec_to_vector(halfvec, integer, boolean) returns vector
  language c;

create function public.halfvec_typmod_in(cstring[]) returns integer
  language c;

create function public.hamming_distance(bit, bit) returns double precision
  language c;

create function public.hnsw_bit_support(internal) returns internal
  language c;

create function public.hnsw_halfvec_support(internal) returns internal
  language c;

create function public.hnsw_sparsevec_support(internal) returns internal
  language c;

create function public.hnswhandler(internal) returns index_am_handler
  language c;

create function public.inner_product(halfvec, halfvec) returns double precision
  language c;

create function public.inner_product(sparsevec, sparsevec) returns double precision
  language c;

create function public.inner_product(vector, vector) returns double precision
  language c;

create function public.ivfflat_bit_support(internal) returns internal
  language c;

create function public.ivfflat_halfvec_support(internal) returns internal
  language c;

create function public.ivfflathandler(internal) returns index_am_handler
  language c;

create function public.jaccard_distance(bit, bit) returns double precision
  language c;

create function public.l1_distance(halfvec, halfvec) returns double precision
  language c;

create function public.l1_distance(sparsevec, sparsevec) returns double precision
  language c;

create function public.l1_distance(vector, vector) returns double precision
  language c;

create function public.l2_distance(halfvec, halfvec) returns double precision
  language c;

create function public.l2_distance(sparsevec, sparsevec) returns double precision
  language c;

create function public.l2_distance(vector, vector) returns double precision
  language c;

create function public.l2_norm(halfvec) returns double precision
  language c;

create function public.l2_norm(sparsevec) returns double precision
  language c;

create function public.l2_normalize(halfvec) returns halfvec
  language c;

create function public.l2_normalize(sparsevec) returns sparsevec
  language c;

create function public.l2_normalize(vector) returns vector
  language c;

create function public.sparsevec(sparsevec, integer, boolean) returns sparsevec
  language c;

create function public.sparsevec_cmp(sparsevec, sparsevec) returns integer
  language c;

create function public.sparsevec_eq(sparsevec, sparsevec) returns boolean
  language c;

create function public.sparsevec_ge(sparsevec, sparsevec) returns boolean
  language c;

create function public.sparsevec_gt(sparsevec, sparsevec) returns boolean
  language c;

create function public.sparsevec_in(cstring, oid, integer) returns sparsevec
  language c;

create function public.sparsevec_l2_squared_distance(sparsevec, sparsevec) returns double precision
  language c;

create function public.sparsevec_le(sparsevec, sparsevec) returns boolean
  language c;

create function public.sparsevec_lt(sparsevec, sparsevec) returns boolean
  language c;

create function public.sparsevec_ne(sparsevec, sparsevec) returns boolean
  language c;

create function public.sparsevec_negative_inner_product(sparsevec, sparsevec) returns double precision
  language c;

create function public.sparsevec_out(sparsevec) returns cstring
  language c;

create function public.sparsevec_recv(internal, oid, integer) returns sparsevec
  language c;

create function public.sparsevec_send(sparsevec) returns bytea
  language c;

create function public.sparsevec_to_halfvec(sparsevec, integer, boolean) returns halfvec
  language c;

create function public.sparsevec_to_vector(sparsevec, integer, boolean) returns vector
  language c;

create function public.sparsevec_typmod_in(cstring[]) returns integer
  language c;

create function public.subvector(halfvec, integer, integer) returns halfvec
  language c;

create function public.subvector(vector, integer, integer) returns vector
  language c;

create aggregate public.sum(halfvec) (
  sfunc = halfvec_add,
  stype = halfvec,
  combinefunc = halfvec_add
);

create aggregate public.sum(vector) (
  sfunc = vector_add,
  stype = vector,
  combinefunc = vector_add
);

create function public.vector(vector, integer, boolean) returns vector
  language c;

create function public.vector_accum(double precision[], vector) returns double precision[]
  language c;

create function public.vector_add(vector, vector) returns vector
  language c;

create function public.vector_avg(double precision[]) returns vector
  language c;

create function public.vector_cmp(vector, vector) returns integer
  language c;

create function public.vector_combine(double precision[], double precision[]) returns double precision[]
  language c;

create function public.vector_concat(vector, vector) returns vector
  language c;

create function public.vector_dims(halfvec) returns integer
  language c;

create function public.vector_dims(vector) returns integer
  language c;

create function public.vector_eq(vector, vector) returns boolean
  language c;

create function public.vector_ge(vector, vector) returns boolean
  language c;

create function public.vector_gt(vector, vector) returns boolean
  language c;

create function public.vector_in(cstring, oid, integer) returns vector
  language c;

create function public.vector_l2_squared_distance(vector, vector) returns double precision
  language c;

create function public.vector_le(vector, vector) returns boolean
  language c;

create function public.vector_lt(vector, vector) returns boolean
  language c;

create function public.vector_mul(vector, vector) returns vector
  language c;

create function public.vector_ne(vector, vector) returns boolean
  language c;

create function public.vector_negative_inner_product(vector, vector) returns double precision
  language c;

create function public.vector_norm(vector) returns double precision
  language c;

create function public.vector_out(vector) returns cstring
  language c;

create function public.vector_recv(internal, oid, integer) returns vector
  language c;

create function public.vector_send(vector) returns bytea
  language c;

create function public.vector_spherical_distance(vector, vector) returns double precision
  language c;

create function public.vector_sub(vector, vector) returns vector
  language c;

create function public.vector_to_float4(vector, integer, boolean) returns real[]
  language c;

create function public.vector_to_halfvec(vector, integer, boolean) returns halfvec
  language c;

create function public.vector_to_sparsevec(vector, integer, boolean) returns sparsevec
  language c;

create function public.vector_typmod_in(cstring[]) returns integer
  language c;

create operator public.* (
  leftarg = halfvec,
  rightarg = halfvec,
  function = public.halfvec_mul
);

create operator public.* (
  leftarg = vector,
  rightarg = vector,
  function = public.vector_mul
);

create operator public.+ (
  leftarg = halfvec,
  rightarg = halfvec,
  function = public.halfvec_add
);

create operator public.+ (
  leftarg = vector,
  rightarg = vector,
  function = public.vector_add
);

create operator public.- (
  leftarg = halfvec,
  rightarg = halfvec,
  function = public.halfvec_sub
);

create operator public.- (
  leftarg = vector,
  rightarg = vector,
  function = public.vector_sub
);

create operator public.< (
  leftarg = halfvec,
  rightarg = halfvec,
  function = public.halfvec_lt
);

create operator public.< (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = public.sparsevec_lt
);

create operator public.< (
  leftarg = vector,
  rightarg = vector,
  function = public.vector_lt
);

create operator public.<#> (
  leftarg = halfvec,
  rightarg = halfvec,
  function = public.halfvec_negative_inner_product
);

create operator public.<#> (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = public.sparsevec_negative_inner_product
);

create operator public.<#> (
  leftarg = vector,
  rightarg = vector,
  function = public.vector_negative_inner_product
);

create operator public.<%> (
  leftarg = bit,
  rightarg = bit,
  function = public.jaccard_distance
);

create operator public.<+> (
  leftarg = halfvec,
  rightarg = halfvec,
  function = public.l1_distance
);

create operator public.<+> (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = public.l1_distance
);

create operator public.<+> (
  leftarg = vector,
  rightarg = vector,
  function = public.l1_distance
);

create operator public.<-> (
  leftarg = halfvec,
  rightarg = halfvec,
  function = public.l2_distance
);

create operator public.<-> (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = public.l2_distance
);

create operator public.<-> (
  leftarg = vector,
  rightarg = vector,
  function = public.l2_distance
);

create operator public.<= (
  leftarg = halfvec,
  rightarg = halfvec,
  function = public.halfvec_le
);

create operator public.<= (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = public.sparsevec_le
);

create operator public.<= (
  leftarg = vector,
  rightarg = vector,
  function = public.vector_le
);

create operator public.<=> (
  leftarg = halfvec,
  rightarg = halfvec,
  function = public.cosine_distance
);

create operator public.<=> (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = public.cosine_distance
);

create operator public.<=> (
  leftarg = vector,
  rightarg = vector,
  function = public.cosine_distance
);

create operator public.<> (
  leftarg = halfvec,
  rightarg = halfvec,
  function = public.halfvec_ne
);

create operator public.<> (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = public.sparsevec_ne
);

create operator public.<> (
  leftarg = vector,
  rightarg = vector,
  function = public.vector_ne
);

create operator public.<~> (
  leftarg = bit,
  rightarg = bit,
  function = public.hamming_distance
);

create operator public.= (
  leftarg = halfvec,
  rightarg = halfvec,
  function = public.halfvec_eq
);

create operator public.= (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = public.sparsevec_eq
);

create operator public.= (
  leftarg = vector,
  rightarg = vector,
  function = public.vector_eq
);

create operator public.> (
  leftarg = halfvec,
  rightarg = halfvec,
  function = public.halfvec_gt
);

create operator public.> (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = public.sparsevec_gt
);

create operator public.> (
  leftarg = vector,
  rightarg = vector,
  function = public.vector_gt
);

create operator public.>= (
  leftarg = halfvec,
  rightarg = halfvec,
  function = public.halfvec_ge
);

create operator public.>= (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = public.sparsevec_ge
);

create operator public.>= (
  leftarg = vector,
  rightarg = vector,
  function = public.vector_ge
);

create operator public.|| (
  leftarg = halfvec,
  rightarg = halfvec,
  function = public.halfvec_concat
);

create operator public.|| (
  leftarg = vector,
  rightarg = vector,
  function = public.vector_concat
);

