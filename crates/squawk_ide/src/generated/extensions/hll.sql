-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

-- size: -1, align: 4
create type public.hll;

-- size: 8, align: 8
create type public.hll_hashval;

create function public.hll(hll, integer, boolean) returns hll
  language c;

create function public.hll_add(hll, hll_hashval) returns hll
  language c;

create aggregate public.hll_add_agg(hll_hashval) (
  sfunc = hll_add_trans0,
  stype = internal,
  finalfunc = hll_pack,
  combinefunc = hll_union_internal
);

create aggregate public.hll_add_agg(hll_hashval, integer) (
  sfunc = hll_add_trans1,
  stype = internal,
  finalfunc = hll_pack,
  combinefunc = hll_union_internal
);

create aggregate public.hll_add_agg(hll_hashval, integer, integer) (
  sfunc = hll_add_trans2,
  stype = internal,
  finalfunc = hll_pack,
  combinefunc = hll_union_internal
);

create aggregate public.hll_add_agg(hll_hashval, integer, integer, bigint) (
  sfunc = hll_add_trans3,
  stype = internal,
  finalfunc = hll_pack,
  combinefunc = hll_union_internal
);

create aggregate public.hll_add_agg(hll_hashval, integer, integer, bigint, integer) (
  sfunc = hll_add_trans4,
  stype = internal,
  finalfunc = hll_pack,
  combinefunc = hll_union_internal
);

create function public.hll_add_rev(hll_hashval, hll) returns hll
  language c;

create function public.hll_add_trans0(internal, hll_hashval) returns internal
  language c;

create function public.hll_add_trans1(internal, hll_hashval, integer) returns internal
  language c;

create function public.hll_add_trans2(internal, hll_hashval, integer, integer) returns internal
  language c;

create function public.hll_add_trans3(internal, hll_hashval, integer, integer, bigint) returns internal
  language c;

create function public.hll_add_trans4(internal, hll_hashval, integer, integer, bigint, integer) returns internal
  language c;

create function public.hll_card_unpacked(internal) returns double precision
  language c;

create function public.hll_cardinality(hll) returns double precision
  language c;

create function public.hll_ceil_card_unpacked(internal) returns bigint
  language c;

create function public.hll_deserialize(bytea, internal) returns internal
  language c;

create function public.hll_empty() returns hll
  language c;

create function public.hll_empty(integer) returns hll
  language c;

create function public.hll_empty(integer, integer) returns hll
  language c;

create function public.hll_empty(integer, integer, bigint) returns hll
  language c;

create function public.hll_empty(integer, integer, bigint, integer) returns hll
  language c;

create function public.hll_eq(hll, hll) returns boolean
  language c;

create function public.hll_expthresh(hll, OUT specified bigint, OUT effective bigint) returns record
  language c;

create function public.hll_floor_card_unpacked(internal) returns bigint
  language c;

create function public.hll_hash_any(anyelement, integer DEFAULT 0) returns hll_hashval
  language c;

create function public.hll_hash_bigint(bigint, integer DEFAULT 0) returns hll_hashval
  language c;

create function public.hll_hash_boolean(boolean, integer DEFAULT 0) returns hll_hashval
  language c;

create function public.hll_hash_bytea(bytea, integer DEFAULT 0) returns hll_hashval
  language c;

create function public.hll_hash_integer(integer, integer DEFAULT 0) returns hll_hashval
  language c;

create function public.hll_hash_smallint(smallint, integer DEFAULT 0) returns hll_hashval
  language c;

create function public.hll_hash_text(text, integer DEFAULT 0) returns hll_hashval
  language c;

create function public.hll_hashval(bigint) returns hll_hashval
  language c;

create function public.hll_hashval_eq(hll_hashval, hll_hashval) returns boolean
  language c;

create function public.hll_hashval_in(cstring, oid, integer) returns hll_hashval
  language c;

create function public.hll_hashval_int4(integer) returns hll_hashval
  language c;

create function public.hll_hashval_ne(hll_hashval, hll_hashval) returns boolean
  language c;

create function public.hll_hashval_out(hll_hashval) returns cstring
  language c;

create function public.hll_in(cstring, oid, integer) returns hll
  language c;

create function public.hll_log2m(hll) returns integer
  language c;

create function public.hll_ne(hll, hll) returns boolean
  language c;

create function public.hll_out(hll) returns cstring
  language c;

create function public.hll_pack(internal) returns hll
  language c;

create function public.hll_print(hll) returns cstring
  language c;

create function public.hll_recv(internal) returns hll
  language c;

create function public.hll_regwidth(hll) returns integer
  language c;

create function public.hll_schema_version(hll) returns integer
  language c;

create function public.hll_send(hll) returns bytea
  language c;

create function public.hll_serialize(internal) returns bytea
  language c;

create function public.hll_set_defaults(i_log2m integer, i_regwidth integer, i_expthresh bigint, i_sparseon integer, OUT o_log2m integer, OUT o_regwidth integer, OUT o_expthresh bigint, OUT o_sparseon integer) returns record
  language c;

create function public.hll_set_max_sparse(integer) returns integer
  language c;

create function public.hll_set_output_version(integer) returns integer
  language c;

create function public.hll_sparseon(hll) returns integer
  language c;

create function public.hll_type(hll) returns integer
  language c;

create function public.hll_typmod_in(cstring[]) returns integer
  language c;

create function public.hll_typmod_out(integer) returns cstring
  language c;

create function public.hll_union(hll, hll) returns hll
  language c;

create aggregate public.hll_union_agg(hll) (
  sfunc = hll_union_trans,
  stype = internal,
  finalfunc = hll_pack,
  combinefunc = hll_union_internal
);

create function public.hll_union_internal(internal, internal) returns internal
  language c;

create function public.hll_union_trans(internal, hll) returns internal
  language c;

create operator public.# (
  rightarg = hll,
  function = public.hll_cardinality
);

create operator public.<> (
  leftarg = hll,
  rightarg = hll,
  function = public.hll_ne
);

create operator public.<> (
  leftarg = hll_hashval,
  rightarg = hll_hashval,
  function = public.hll_hashval_ne
);

create operator public.= (
  leftarg = hll,
  rightarg = hll,
  function = public.hll_eq
);

create operator public.= (
  leftarg = hll_hashval,
  rightarg = hll_hashval,
  function = public.hll_hashval_eq
);

create operator public.|| (
  leftarg = hll,
  rightarg = hll,
  function = public.hll_union
);

create operator public.|| (
  leftarg = hll,
  rightarg = hll_hashval,
  function = public.hll_add
);

create operator public.|| (
  leftarg = hll_hashval,
  rightarg = hll,
  function = public.hll_add_rev
);

