-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

-- size: -1, align: 4
create type ghstore;

-- size: -1, align: 4
create type hstore;

create function akeys(hstore) returns text[]
  language c;

create function avals(hstore) returns text[]
  language c;

create function defined(hstore, text) returns boolean
  language c;

create function delete(hstore, hstore) returns hstore
  language c;

create function delete(hstore, text) returns hstore
  language c;

create function delete(hstore, text[]) returns hstore
  language c;

create function each(hs hstore, OUT key text, OUT value text) returns SETOF record
  language c;

create function exist(hstore, text) returns boolean
  language c;

create function exists_all(hstore, text[]) returns boolean
  language c;

create function exists_any(hstore, text[]) returns boolean
  language c;

create function fetchval(hstore, text) returns text
  language c;

create function ghstore_compress(internal) returns internal
  language c;

create function ghstore_consistent(internal, hstore, smallint, oid, internal) returns boolean
  language c;

create function ghstore_decompress(internal) returns internal
  language c;

create function ghstore_in(cstring) returns ghstore
  language c;

create function ghstore_options(internal) returns void
  language c;

create function ghstore_out(ghstore) returns cstring
  language c;

create function ghstore_penalty(internal, internal, internal) returns internal
  language c;

create function ghstore_picksplit(internal, internal) returns internal
  language c;

create function ghstore_same(ghstore, ghstore, internal) returns internal
  language c;

create function ghstore_union(internal, internal) returns ghstore
  language c;

create function gin_consistent_hstore(internal, smallint, hstore, integer, internal, internal) returns boolean
  language c;

create function gin_extract_hstore(hstore, internal) returns internal
  language c;

create function gin_extract_hstore_query(hstore, internal, smallint, internal, internal) returns internal
  language c;

create function hs_concat(hstore, hstore) returns hstore
  language c;

create function hs_contained(hstore, hstore) returns boolean
  language c;

create function hs_contains(hstore, hstore) returns boolean
  language c;

create function hstore(record) returns hstore
  language c;

create function hstore(text, text) returns hstore
  language c;

create function hstore(text[]) returns hstore
  language c;

create function hstore(text[], text[]) returns hstore
  language c;

create function hstore_cmp(hstore, hstore) returns integer
  language c;

create function hstore_eq(hstore, hstore) returns boolean
  language c;

create function hstore_ge(hstore, hstore) returns boolean
  language c;

create function hstore_gt(hstore, hstore) returns boolean
  language c;

create function hstore_hash(hstore) returns integer
  language c;

create function hstore_hash_extended(hstore, bigint) returns bigint
  language c;

create function hstore_in(cstring) returns hstore
  language c;

create function hstore_le(hstore, hstore) returns boolean
  language c;

create function hstore_lt(hstore, hstore) returns boolean
  language c;

create function hstore_ne(hstore, hstore) returns boolean
  language c;

create function hstore_out(hstore) returns cstring
  language c;

create function hstore_recv(internal) returns hstore
  language c;

create function hstore_send(hstore) returns bytea
  language c;

create function hstore_subscript_handler(internal) returns internal
  language c;

create function hstore_to_array(hstore) returns text[]
  language c;

create function hstore_to_json(hstore) returns json
  language c;

create function hstore_to_json_loose(hstore) returns json
  language c;

create function hstore_to_jsonb(hstore) returns jsonb
  language c;

create function hstore_to_jsonb_loose(hstore) returns jsonb
  language c;

create function hstore_to_matrix(hstore) returns text[]
  language c;

create function hstore_version_diag(hstore) returns integer
  language c;

create function isdefined(hstore, text) returns boolean
  language c;

create function isexists(hstore, text) returns boolean
  language c;

create function populate_record(anyelement, hstore) returns anyelement
  language c;

create function skeys(hstore) returns SETOF text
  language c;

create function slice(hstore, text[]) returns hstore
  language c;

create function slice_array(hstore, text[]) returns text[]
  language c;

create function svals(hstore) returns SETOF text
  language c;

create function tconvert(text, text) returns hstore
  language c;

create type ghstore (input = ghstore_in, output = ghstore_out);

-- subscript: pg_catalog.text
create type hstore (input = hstore_in, output = hstore_out, subscript = hstore_subscript_handler);

create operator #<# (
  leftarg = hstore,
  rightarg = hstore,
  function = hstore_lt
);

create operator #<=# (
  leftarg = hstore,
  rightarg = hstore,
  function = hstore_le
);

create operator #= (
  leftarg = anyelement,
  rightarg = hstore,
  function = populate_record
);

create operator #># (
  leftarg = hstore,
  rightarg = hstore,
  function = hstore_gt
);

create operator #>=# (
  leftarg = hstore,
  rightarg = hstore,
  function = hstore_ge
);

create operator %# (
  rightarg = hstore,
  function = hstore_to_matrix
);

create operator %% (
  rightarg = hstore,
  function = hstore_to_array
);

create operator - (
  leftarg = hstore,
  rightarg = hstore,
  function = delete
);

create operator - (
  leftarg = hstore,
  rightarg = text,
  function = delete
);

create operator - (
  leftarg = hstore,
  rightarg = text[],
  function = delete
);

create operator -> (
  leftarg = hstore,
  rightarg = text,
  function = fetchval
);

create operator -> (
  leftarg = hstore,
  rightarg = text[],
  function = slice_array
);

create operator <> (
  leftarg = hstore,
  rightarg = hstore,
  function = hstore_ne
);

create operator <@ (
  leftarg = hstore,
  rightarg = hstore,
  function = hs_contained
);

create operator = (
  leftarg = hstore,
  rightarg = hstore,
  function = hstore_eq
);

create operator ? (
  leftarg = hstore,
  rightarg = text,
  function = exist
);

create operator ?& (
  leftarg = hstore,
  rightarg = text[],
  function = exists_all
);

create operator ?| (
  leftarg = hstore,
  rightarg = text[],
  function = exists_any
);

create operator @> (
  leftarg = hstore,
  rightarg = hstore,
  function = hs_contains
);

create operator || (
  leftarg = hstore,
  rightarg = hstore,
  function = hs_concat
);

create cast (hstore as json) with function hstore_to_json(hstore);

create cast (hstore as jsonb) with function hstore_to_jsonb(hstore);

create cast (text[] as hstore) with function hstore(text[]);

create operator family btree_hstore_ops using btree;

create operator family gin_hstore_ops using gin;

create operator family gist_hstore_ops using gist;

create operator family hash_hstore_ops using hash;

create operator class btree_hstore_ops default for type hstore using btree family btree_hstore_ops as storage hstore;

create operator class gin_hstore_ops default for type hstore using gin family gin_hstore_ops as storage text;

create operator class gist_hstore_ops default for type hstore using gist family gist_hstore_ops as storage ghstore;

create operator class hash_hstore_ops default for type hstore using hash family hash_hstore_ops as storage hstore;

