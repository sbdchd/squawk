-- squawk-ignore-file
-- pg version: 18.2
-- update via:
--   cargo xtask sync-builtins

-- size: -1, align: 4
create type public.ghstore;

-- size: -1, align: 4
create type public.hstore;

create function public.akeys(hstore) returns text[]
  language c;

create function public.avals(hstore) returns text[]
  language c;

create function public.defined(hstore, text) returns boolean
  language c;

create function public.delete(hstore, hstore) returns hstore
  language c;

create function public.delete(hstore, text) returns hstore
  language c;

create function public.delete(hstore, text[]) returns hstore
  language c;

create function public.each(hs hstore, OUT key text, OUT value text) returns SETOF record
  language c;

create function public.exist(hstore, text) returns boolean
  language c;

create function public.exists_all(hstore, text[]) returns boolean
  language c;

create function public.exists_any(hstore, text[]) returns boolean
  language c;

create function public.fetchval(hstore, text) returns text
  language c;

create function public.ghstore_compress(internal) returns internal
  language c;

create function public.ghstore_consistent(internal, hstore, smallint, oid, internal) returns boolean
  language c;

create function public.ghstore_decompress(internal) returns internal
  language c;

create function public.ghstore_in(cstring) returns ghstore
  language c;

create function public.ghstore_options(internal) returns void
  language c;

create function public.ghstore_out(ghstore) returns cstring
  language c;

create function public.ghstore_penalty(internal, internal, internal) returns internal
  language c;

create function public.ghstore_picksplit(internal, internal) returns internal
  language c;

create function public.ghstore_same(ghstore, ghstore, internal) returns internal
  language c;

create function public.ghstore_union(internal, internal) returns ghstore
  language c;

create function public.gin_consistent_hstore(internal, smallint, hstore, integer, internal, internal) returns boolean
  language c;

create function public.gin_extract_hstore(hstore, internal) returns internal
  language c;

create function public.gin_extract_hstore_query(hstore, internal, smallint, internal, internal) returns internal
  language c;

create function public.hs_concat(hstore, hstore) returns hstore
  language c;

create function public.hs_contained(hstore, hstore) returns boolean
  language c;

create function public.hs_contains(hstore, hstore) returns boolean
  language c;

create function public.hstore(record) returns hstore
  language c;

create function public.hstore(text, text) returns hstore
  language c;

create function public.hstore(text[]) returns hstore
  language c;

create function public.hstore(text[], text[]) returns hstore
  language c;

create function public.hstore_cmp(hstore, hstore) returns integer
  language c;

create function public.hstore_eq(hstore, hstore) returns boolean
  language c;

create function public.hstore_ge(hstore, hstore) returns boolean
  language c;

create function public.hstore_gt(hstore, hstore) returns boolean
  language c;

create function public.hstore_hash(hstore) returns integer
  language c;

create function public.hstore_hash_extended(hstore, bigint) returns bigint
  language c;

create function public.hstore_in(cstring) returns hstore
  language c;

create function public.hstore_le(hstore, hstore) returns boolean
  language c;

create function public.hstore_lt(hstore, hstore) returns boolean
  language c;

create function public.hstore_ne(hstore, hstore) returns boolean
  language c;

create function public.hstore_out(hstore) returns cstring
  language c;

create function public.hstore_recv(internal) returns hstore
  language c;

create function public.hstore_send(hstore) returns bytea
  language c;

create function public.hstore_subscript_handler(internal) returns internal
  language c;

create function public.hstore_to_array(hstore) returns text[]
  language c;

create function public.hstore_to_json(hstore) returns json
  language c;

create function public.hstore_to_json_loose(hstore) returns json
  language c;

create function public.hstore_to_jsonb(hstore) returns jsonb
  language c;

create function public.hstore_to_jsonb_loose(hstore) returns jsonb
  language c;

create function public.hstore_to_matrix(hstore) returns text[]
  language c;

create function public.hstore_version_diag(hstore) returns integer
  language c;

create function public.isdefined(hstore, text) returns boolean
  language c;

create function public.isexists(hstore, text) returns boolean
  language c;

create function public.populate_record(anyelement, hstore) returns anyelement
  language c;

create function public.skeys(hstore) returns SETOF text
  language c;

create function public.slice(hstore, text[]) returns hstore
  language c;

create function public.slice_array(hstore, text[]) returns text[]
  language c;

create function public.svals(hstore) returns SETOF text
  language c;

create function public.tconvert(text, text) returns hstore
  language c;

create operator public.#<# (
  leftarg = hstore,
  rightarg = hstore,
  function = public.hstore_lt
);

create operator public.#<=# (
  leftarg = hstore,
  rightarg = hstore,
  function = public.hstore_le
);

create operator public.#= (
  leftarg = anyelement,
  rightarg = hstore,
  function = public.populate_record
);

create operator public.#># (
  leftarg = hstore,
  rightarg = hstore,
  function = public.hstore_gt
);

create operator public.#>=# (
  leftarg = hstore,
  rightarg = hstore,
  function = public.hstore_ge
);

create operator public.%# (
  rightarg = hstore,
  function = public.hstore_to_matrix
);

create operator public.%% (
  rightarg = hstore,
  function = public.hstore_to_array
);

create operator public.- (
  leftarg = hstore,
  rightarg = hstore,
  function = public.delete
);

create operator public.- (
  leftarg = hstore,
  rightarg = text,
  function = public.delete
);

create operator public.- (
  leftarg = hstore,
  rightarg = text[],
  function = public.delete
);

create operator public.-> (
  leftarg = hstore,
  rightarg = text,
  function = public.fetchval
);

create operator public.-> (
  leftarg = hstore,
  rightarg = text[],
  function = public.slice_array
);

create operator public.<> (
  leftarg = hstore,
  rightarg = hstore,
  function = public.hstore_ne
);

create operator public.<@ (
  leftarg = hstore,
  rightarg = hstore,
  function = public.hs_contained
);

create operator public.= (
  leftarg = hstore,
  rightarg = hstore,
  function = public.hstore_eq
);

create operator public.? (
  leftarg = hstore,
  rightarg = text,
  function = public.exist
);

create operator public.?& (
  leftarg = hstore,
  rightarg = text[],
  function = public.exists_all
);

create operator public.?| (
  leftarg = hstore,
  rightarg = text[],
  function = public.exists_any
);

create operator public.@> (
  leftarg = hstore,
  rightarg = hstore,
  function = public.hs_contains
);

create operator public.|| (
  leftarg = hstore,
  rightarg = hstore,
  function = public.hs_concat
);

