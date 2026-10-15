-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

-- size: -1, align: 4
create type halfvec;

-- size: -1, align: 4
create type sparsevec;

-- size: -1, align: 4
create type vector;

create function array_to_halfvec(double precision[], integer, boolean) returns halfvec
  language c;

create function array_to_halfvec(integer[], integer, boolean) returns halfvec
  language c;

create function array_to_halfvec(numeric[], integer, boolean) returns halfvec
  language c;

create function array_to_halfvec(real[], integer, boolean) returns halfvec
  language c;

create function array_to_sparsevec(double precision[], integer, boolean) returns sparsevec
  language c;

create function array_to_sparsevec(integer[], integer, boolean) returns sparsevec
  language c;

create function array_to_sparsevec(numeric[], integer, boolean) returns sparsevec
  language c;

create function array_to_sparsevec(real[], integer, boolean) returns sparsevec
  language c;

create function array_to_vector(double precision[], integer, boolean) returns vector
  language c;

create function array_to_vector(integer[], integer, boolean) returns vector
  language c;

create function array_to_vector(numeric[], integer, boolean) returns vector
  language c;

create function array_to_vector(real[], integer, boolean) returns vector
  language c;

create function binary_quantize(halfvec) returns bit
  language c;

create function binary_quantize(vector) returns bit
  language c;

create function cosine_distance(halfvec, halfvec) returns double precision
  language c;

create function cosine_distance(sparsevec, sparsevec) returns double precision
  language c;

create function cosine_distance(vector, vector) returns double precision
  language c;

create function halfvec(halfvec, integer, boolean) returns halfvec
  language c;

create function halfvec_accum(double precision[], halfvec) returns double precision[]
  language c;

create function halfvec_add(halfvec, halfvec) returns halfvec
  language c;

create function halfvec_avg(double precision[]) returns halfvec
  language c;

create function halfvec_cmp(halfvec, halfvec) returns integer
  language c;

create function halfvec_combine(double precision[], double precision[]) returns double precision[]
  language c;

create function halfvec_concat(halfvec, halfvec) returns halfvec
  language c;

create function halfvec_eq(halfvec, halfvec) returns boolean
  language c;

create function halfvec_ge(halfvec, halfvec) returns boolean
  language c;

create function halfvec_gt(halfvec, halfvec) returns boolean
  language c;

create function halfvec_in(cstring, oid, integer) returns halfvec
  language c;

create function halfvec_l2_squared_distance(halfvec, halfvec) returns double precision
  language c;

create function halfvec_le(halfvec, halfvec) returns boolean
  language c;

create function halfvec_lt(halfvec, halfvec) returns boolean
  language c;

create function halfvec_mul(halfvec, halfvec) returns halfvec
  language c;

create function halfvec_ne(halfvec, halfvec) returns boolean
  language c;

create function halfvec_negative_inner_product(halfvec, halfvec) returns double precision
  language c;

create function halfvec_out(halfvec) returns cstring
  language c;

create function halfvec_recv(internal, oid, integer) returns halfvec
  language c;

create function halfvec_send(halfvec) returns bytea
  language c;

create function halfvec_spherical_distance(halfvec, halfvec) returns double precision
  language c;

create function halfvec_sub(halfvec, halfvec) returns halfvec
  language c;

create function halfvec_to_float4(halfvec, integer, boolean) returns real[]
  language c;

create function halfvec_to_sparsevec(halfvec, integer, boolean) returns sparsevec
  language c;

create function halfvec_to_vector(halfvec, integer, boolean) returns vector
  language c;

create function halfvec_typmod_in(cstring[]) returns integer
  language c;

create function hamming_distance(bit, bit) returns double precision
  language c;

create function hnsw_bit_support(internal) returns internal
  language c;

create function hnsw_halfvec_support(internal) returns internal
  language c;

create function hnsw_sparsevec_support(internal) returns internal
  language c;

create function hnswhandler(internal) returns index_am_handler
  language c;

create function inner_product(halfvec, halfvec) returns double precision
  language c;

create function inner_product(sparsevec, sparsevec) returns double precision
  language c;

create function inner_product(vector, vector) returns double precision
  language c;

create function ivfflat_bit_support(internal) returns internal
  language c;

create function ivfflat_halfvec_support(internal) returns internal
  language c;

create function ivfflathandler(internal) returns index_am_handler
  language c;

create function jaccard_distance(bit, bit) returns double precision
  language c;

create function l1_distance(halfvec, halfvec) returns double precision
  language c;

create function l1_distance(sparsevec, sparsevec) returns double precision
  language c;

create function l1_distance(vector, vector) returns double precision
  language c;

create function l2_distance(halfvec, halfvec) returns double precision
  language c;

create function l2_distance(sparsevec, sparsevec) returns double precision
  language c;

create function l2_distance(vector, vector) returns double precision
  language c;

create function l2_norm(halfvec) returns double precision
  language c;

create function l2_norm(sparsevec) returns double precision
  language c;

create function l2_normalize(halfvec) returns halfvec
  language c;

create function l2_normalize(sparsevec) returns sparsevec
  language c;

create function l2_normalize(vector) returns vector
  language c;

create function sparsevec(sparsevec, integer, boolean) returns sparsevec
  language c;

create function sparsevec_cmp(sparsevec, sparsevec) returns integer
  language c;

create function sparsevec_eq(sparsevec, sparsevec) returns boolean
  language c;

create function sparsevec_ge(sparsevec, sparsevec) returns boolean
  language c;

create function sparsevec_gt(sparsevec, sparsevec) returns boolean
  language c;

create function sparsevec_in(cstring, oid, integer) returns sparsevec
  language c;

create function sparsevec_l2_squared_distance(sparsevec, sparsevec) returns double precision
  language c;

create function sparsevec_le(sparsevec, sparsevec) returns boolean
  language c;

create function sparsevec_lt(sparsevec, sparsevec) returns boolean
  language c;

create function sparsevec_ne(sparsevec, sparsevec) returns boolean
  language c;

create function sparsevec_negative_inner_product(sparsevec, sparsevec) returns double precision
  language c;

create function sparsevec_out(sparsevec) returns cstring
  language c;

create function sparsevec_recv(internal, oid, integer) returns sparsevec
  language c;

create function sparsevec_send(sparsevec) returns bytea
  language c;

create function sparsevec_to_halfvec(sparsevec, integer, boolean) returns halfvec
  language c;

create function sparsevec_to_vector(sparsevec, integer, boolean) returns vector
  language c;

create function sparsevec_typmod_in(cstring[]) returns integer
  language c;

create function subvector(halfvec, integer, integer) returns halfvec
  language c;

create function subvector(vector, integer, integer) returns vector
  language c;

create function vector(vector, integer, boolean) returns vector
  language c;

create function vector_accum(double precision[], vector) returns double precision[]
  language c;

create function vector_add(vector, vector) returns vector
  language c;

create function vector_avg(double precision[]) returns vector
  language c;

create function vector_cmp(vector, vector) returns integer
  language c;

create function vector_combine(double precision[], double precision[]) returns double precision[]
  language c;

create function vector_concat(vector, vector) returns vector
  language c;

create function vector_dims(halfvec) returns integer
  language c;

create function vector_dims(vector) returns integer
  language c;

create function vector_eq(vector, vector) returns boolean
  language c;

create function vector_ge(vector, vector) returns boolean
  language c;

create function vector_gt(vector, vector) returns boolean
  language c;

create function vector_in(cstring, oid, integer) returns vector
  language c;

create function vector_l2_squared_distance(vector, vector) returns double precision
  language c;

create function vector_le(vector, vector) returns boolean
  language c;

create function vector_lt(vector, vector) returns boolean
  language c;

create function vector_mul(vector, vector) returns vector
  language c;

create function vector_ne(vector, vector) returns boolean
  language c;

create function vector_negative_inner_product(vector, vector) returns double precision
  language c;

create function vector_norm(vector) returns double precision
  language c;

create function vector_out(vector) returns cstring
  language c;

create function vector_recv(internal, oid, integer) returns vector
  language c;

create function vector_send(vector) returns bytea
  language c;

create function vector_spherical_distance(vector, vector) returns double precision
  language c;

create function vector_sub(vector, vector) returns vector
  language c;

create function vector_to_float4(vector, integer, boolean) returns real[]
  language c;

create function vector_to_halfvec(vector, integer, boolean) returns halfvec
  language c;

create function vector_to_sparsevec(vector, integer, boolean) returns sparsevec
  language c;

create function vector_typmod_in(cstring[]) returns integer
  language c;

create type halfvec (input = halfvec_in, output = halfvec_out);

create type sparsevec (input = sparsevec_in, output = sparsevec_out);

create type vector (input = vector_in, output = vector_out);

create operator * (
  leftarg = halfvec,
  rightarg = halfvec,
  function = halfvec_mul
);

create operator * (
  leftarg = vector,
  rightarg = vector,
  function = vector_mul
);

create operator + (
  leftarg = halfvec,
  rightarg = halfvec,
  function = halfvec_add
);

create operator + (
  leftarg = vector,
  rightarg = vector,
  function = vector_add
);

create operator - (
  leftarg = halfvec,
  rightarg = halfvec,
  function = halfvec_sub
);

create operator - (
  leftarg = vector,
  rightarg = vector,
  function = vector_sub
);

create operator < (
  leftarg = halfvec,
  rightarg = halfvec,
  function = halfvec_lt
);

create operator < (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = sparsevec_lt
);

create operator < (
  leftarg = vector,
  rightarg = vector,
  function = vector_lt
);

create operator <#> (
  leftarg = halfvec,
  rightarg = halfvec,
  function = halfvec_negative_inner_product
);

create operator <#> (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = sparsevec_negative_inner_product
);

create operator <#> (
  leftarg = vector,
  rightarg = vector,
  function = vector_negative_inner_product
);

create operator <%> (
  leftarg = bit,
  rightarg = bit,
  function = jaccard_distance
);

create operator <+> (
  leftarg = halfvec,
  rightarg = halfvec,
  function = l1_distance
);

create operator <+> (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = l1_distance
);

create operator <+> (
  leftarg = vector,
  rightarg = vector,
  function = l1_distance
);

create operator <-> (
  leftarg = halfvec,
  rightarg = halfvec,
  function = l2_distance
);

create operator <-> (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = l2_distance
);

create operator <-> (
  leftarg = vector,
  rightarg = vector,
  function = l2_distance
);

create operator <= (
  leftarg = halfvec,
  rightarg = halfvec,
  function = halfvec_le
);

create operator <= (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = sparsevec_le
);

create operator <= (
  leftarg = vector,
  rightarg = vector,
  function = vector_le
);

create operator <=> (
  leftarg = halfvec,
  rightarg = halfvec,
  function = cosine_distance
);

create operator <=> (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = cosine_distance
);

create operator <=> (
  leftarg = vector,
  rightarg = vector,
  function = cosine_distance
);

create operator <> (
  leftarg = halfvec,
  rightarg = halfvec,
  function = halfvec_ne
);

create operator <> (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = sparsevec_ne
);

create operator <> (
  leftarg = vector,
  rightarg = vector,
  function = vector_ne
);

create operator <~> (
  leftarg = bit,
  rightarg = bit,
  function = hamming_distance
);

create operator = (
  leftarg = halfvec,
  rightarg = halfvec,
  function = halfvec_eq
);

create operator = (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = sparsevec_eq
);

create operator = (
  leftarg = vector,
  rightarg = vector,
  function = vector_eq
);

create operator > (
  leftarg = halfvec,
  rightarg = halfvec,
  function = halfvec_gt
);

create operator > (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = sparsevec_gt
);

create operator > (
  leftarg = vector,
  rightarg = vector,
  function = vector_gt
);

create operator >= (
  leftarg = halfvec,
  rightarg = halfvec,
  function = halfvec_ge
);

create operator >= (
  leftarg = sparsevec,
  rightarg = sparsevec,
  function = sparsevec_ge
);

create operator >= (
  leftarg = vector,
  rightarg = vector,
  function = vector_ge
);

create operator || (
  leftarg = halfvec,
  rightarg = halfvec,
  function = halfvec_concat
);

create operator || (
  leftarg = vector,
  rightarg = vector,
  function = vector_concat
);

create aggregate avg(halfvec) (
  sfunc = halfvec_accum,
  stype = double precision[],
  finalfunc = halfvec_avg,
  combinefunc = halfvec_combine,
  initcond = '{0}'
);

create aggregate avg(vector) (
  sfunc = vector_accum,
  stype = double precision[],
  finalfunc = vector_avg,
  combinefunc = vector_combine,
  initcond = '{0}'
);

create aggregate sum(halfvec) (
  sfunc = halfvec_add,
  stype = halfvec,
  combinefunc = halfvec_add
);

create aggregate sum(vector) (
  sfunc = vector_add,
  stype = vector,
  combinefunc = vector_add
);

create cast (double precision[] as halfvec) with function array_to_halfvec(double precision[], integer, boolean) as assignment;

create cast (double precision[] as sparsevec) with function array_to_sparsevec(double precision[], integer, boolean) as assignment;

create cast (double precision[] as vector) with function array_to_vector(double precision[], integer, boolean) as assignment;

create cast (halfvec as halfvec) with function halfvec(halfvec, integer, boolean) as implicit;

create cast (halfvec as real[]) with function halfvec_to_float4(halfvec, integer, boolean) as assignment;

create cast (halfvec as sparsevec) with function halfvec_to_sparsevec(halfvec, integer, boolean) as implicit;

create cast (halfvec as vector) with function halfvec_to_vector(halfvec, integer, boolean) as assignment;

create cast (integer[] as halfvec) with function array_to_halfvec(integer[], integer, boolean) as assignment;

create cast (integer[] as sparsevec) with function array_to_sparsevec(integer[], integer, boolean) as assignment;

create cast (integer[] as vector) with function array_to_vector(integer[], integer, boolean) as assignment;

create cast (numeric[] as halfvec) with function array_to_halfvec(numeric[], integer, boolean) as assignment;

create cast (numeric[] as sparsevec) with function array_to_sparsevec(numeric[], integer, boolean) as assignment;

create cast (numeric[] as vector) with function array_to_vector(numeric[], integer, boolean) as assignment;

create cast (real[] as halfvec) with function array_to_halfvec(real[], integer, boolean) as assignment;

create cast (real[] as sparsevec) with function array_to_sparsevec(real[], integer, boolean) as assignment;

create cast (real[] as vector) with function array_to_vector(real[], integer, boolean) as assignment;

create cast (sparsevec as halfvec) with function sparsevec_to_halfvec(sparsevec, integer, boolean) as assignment;

create cast (sparsevec as sparsevec) with function sparsevec(sparsevec, integer, boolean) as implicit;

create cast (sparsevec as vector) with function sparsevec_to_vector(sparsevec, integer, boolean) as assignment;

create cast (vector as halfvec) with function vector_to_halfvec(vector, integer, boolean) as implicit;

create cast (vector as real[]) with function vector_to_float4(vector, integer, boolean) as implicit;

create cast (vector as sparsevec) with function vector_to_sparsevec(vector, integer, boolean) as implicit;

create cast (vector as vector) with function vector(vector, integer, boolean) as implicit;

-- hnsw index access method
create access method hnsw type index handler hnswhandler;

-- ivfflat index access method
create access method ivfflat type index handler ivfflathandler;

create operator family bit_hamming_ops using hnsw;

create operator family bit_hamming_ops using ivfflat;

create operator family bit_jaccard_ops using hnsw;

create operator family halfvec_cosine_ops using hnsw;

create operator family halfvec_cosine_ops using ivfflat;

create operator family halfvec_ip_ops using hnsw;

create operator family halfvec_ip_ops using ivfflat;

create operator family halfvec_l1_ops using hnsw;

create operator family halfvec_l2_ops using hnsw;

create operator family halfvec_l2_ops using ivfflat;

create operator family halfvec_ops using btree;

create operator family sparsevec_cosine_ops using hnsw;

create operator family sparsevec_ip_ops using hnsw;

create operator family sparsevec_l1_ops using hnsw;

create operator family sparsevec_l2_ops using hnsw;

create operator family sparsevec_ops using btree;

create operator family vector_cosine_ops using hnsw;

create operator family vector_cosine_ops using ivfflat;

create operator family vector_ip_ops using hnsw;

create operator family vector_ip_ops using ivfflat;

create operator family vector_l1_ops using hnsw;

create operator family vector_l2_ops using hnsw;

create operator family vector_l2_ops using ivfflat;

create operator family vector_ops using btree;

create operator class bit_hamming_ops for type bit using hnsw family bit_hamming_ops as storage bit;

create operator class bit_hamming_ops for type bit using ivfflat family bit_hamming_ops as storage bit;

create operator class bit_jaccard_ops for type bit using hnsw family bit_jaccard_ops as storage bit;

create operator class halfvec_cosine_ops for type halfvec using hnsw family halfvec_cosine_ops as storage halfvec;

create operator class halfvec_cosine_ops for type halfvec using ivfflat family halfvec_cosine_ops as storage halfvec;

create operator class halfvec_ip_ops for type halfvec using hnsw family halfvec_ip_ops as storage halfvec;

create operator class halfvec_ip_ops for type halfvec using ivfflat family halfvec_ip_ops as storage halfvec;

create operator class halfvec_l1_ops for type halfvec using hnsw family halfvec_l1_ops as storage halfvec;

create operator class halfvec_l2_ops for type halfvec using hnsw family halfvec_l2_ops as storage halfvec;

create operator class halfvec_l2_ops for type halfvec using ivfflat family halfvec_l2_ops as storage halfvec;

create operator class halfvec_ops default for type halfvec using btree family halfvec_ops as storage halfvec;

create operator class sparsevec_cosine_ops for type sparsevec using hnsw family sparsevec_cosine_ops as storage sparsevec;

create operator class sparsevec_ip_ops for type sparsevec using hnsw family sparsevec_ip_ops as storage sparsevec;

create operator class sparsevec_l1_ops for type sparsevec using hnsw family sparsevec_l1_ops as storage sparsevec;

create operator class sparsevec_l2_ops for type sparsevec using hnsw family sparsevec_l2_ops as storage sparsevec;

create operator class sparsevec_ops default for type sparsevec using btree family sparsevec_ops as storage sparsevec;

create operator class vector_cosine_ops for type vector using hnsw family vector_cosine_ops as storage vector;

create operator class vector_cosine_ops for type vector using ivfflat family vector_cosine_ops as storage vector;

create operator class vector_ip_ops for type vector using hnsw family vector_ip_ops as storage vector;

create operator class vector_ip_ops for type vector using ivfflat family vector_ip_ops as storage vector;

create operator class vector_l1_ops for type vector using hnsw family vector_l1_ops as storage vector;

create operator class vector_l2_ops for type vector using hnsw family vector_l2_ops as storage vector;

create operator class vector_l2_ops default for type vector using ivfflat family vector_l2_ops as storage vector;

create operator class vector_ops default for type vector using btree family vector_ops as storage vector;

