-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

-- size: -1, align: 4
create type ghstore;

-- size: -1, align: 4
create type hstore;

create function akeys(hstore) returns text[]
  language c
  immutable
  strict
  parallel safe;

create function avals(hstore) returns text[]
  language c
  immutable
  strict
  parallel safe;

create function defined(hstore, text) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function delete(hstore, hstore) returns hstore
  language c
  immutable
  strict
  parallel safe;

create function delete(hstore, text) returns hstore
  language c
  immutable
  strict
  parallel safe;

create function delete(hstore, text[]) returns hstore
  language c
  immutable
  strict
  parallel safe;

create function each(
  hs hstore,
  out key text,
  out value text
) returns setof record
  language c
  immutable
  strict
  parallel safe;

create function exist(hstore, text) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function exists_all(hstore, text[]) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function exists_any(hstore, text[]) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function fetchval(hstore, text) returns text
  language c
  immutable
  strict
  parallel safe;

create function ghstore_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function ghstore_consistent(
  internal,
  hstore,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function ghstore_decompress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function ghstore_in(cstring) returns ghstore
  language c
  immutable
  strict
  parallel safe;

create function ghstore_options(internal) returns void
  language c
  immutable
  parallel safe;

create function ghstore_out(ghstore) returns cstring
  language c
  immutable
  strict
  parallel safe;

create function ghstore_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function ghstore_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function ghstore_same(ghstore, ghstore, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function ghstore_union(internal, internal) returns ghstore
  language c
  immutable
  strict
  parallel safe;

create function gin_consistent_hstore(
  internal,
  smallint,
  hstore,
  integer,
  internal,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gin_extract_hstore(hstore, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gin_extract_hstore_query(
  hstore,
  internal,
  smallint,
  internal,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function hs_concat(hstore, hstore) returns hstore
  language c
  immutable
  strict
  parallel safe;

create function hs_contained(hstore, hstore) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function hs_contains(hstore, hstore) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function hstore(record) returns hstore
  language c
  immutable
  parallel safe;

create function hstore(text, text) returns hstore
  language c
  immutable
  parallel safe;

create function hstore(text[]) returns hstore
  language c
  immutable
  strict
  parallel safe;

create function hstore(text[], text[]) returns hstore
  language c
  immutable
  parallel safe;

create function hstore_cmp(hstore, hstore) returns integer
  language c
  immutable
  strict
  parallel safe;

create function hstore_eq(hstore, hstore) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function hstore_ge(hstore, hstore) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function hstore_gt(hstore, hstore) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function hstore_hash(hstore) returns integer
  language c
  immutable
  strict
  parallel safe;

create function hstore_hash_extended(hstore, bigint) returns bigint
  language c
  immutable
  strict
  parallel safe;

create function hstore_in(cstring) returns hstore
  language c
  immutable
  strict
  parallel safe;

create function hstore_le(hstore, hstore) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function hstore_lt(hstore, hstore) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function hstore_ne(hstore, hstore) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function hstore_out(hstore) returns cstring
  language c
  immutable
  strict
  parallel safe;

create function hstore_recv(internal) returns hstore
  language c
  immutable
  strict
  parallel safe;

create function hstore_send(hstore) returns bytea
  language c
  immutable
  strict
  parallel safe;

create function hstore_subscript_handler(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function hstore_to_array(hstore) returns text[]
  language c
  immutable
  strict
  parallel safe;

create function hstore_to_json(hstore) returns json
  language c
  immutable
  strict
  parallel safe;

create function hstore_to_json_loose(hstore) returns json
  language c
  immutable
  strict
  parallel safe;

create function hstore_to_jsonb(hstore) returns jsonb
  language c
  immutable
  strict
  parallel safe;

create function hstore_to_jsonb_loose(hstore) returns jsonb
  language c
  immutable
  strict
  parallel safe;

create function hstore_to_matrix(hstore) returns text[]
  language c
  immutable
  strict
  parallel safe;

create function hstore_version_diag(hstore) returns integer
  language c
  immutable
  strict
  parallel safe;

create function isdefined(hstore, text) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function isexists(hstore, text) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function populate_record(anyelement, hstore) returns anyelement
  language c
  immutable
  parallel safe;

create function skeys(hstore) returns setof text
  language c
  immutable
  strict
  parallel safe;

create function slice(hstore, text[]) returns hstore
  language c
  immutable
  strict
  parallel safe;

create function slice_array(hstore, text[]) returns text[]
  language c
  immutable
  strict
  parallel safe;

create function svals(hstore) returns setof text
  language c
  immutable
  strict
  parallel safe;

create function tconvert(text, text) returns hstore
  language c
  immutable
  parallel safe;

create type ghstore (
  input = ghstore_in,
  output = ghstore_out
);

-- subscript: pg_catalog.text
create type hstore (
  input = hstore_in,
  output = hstore_out,
  subscript = hstore_subscript_handler
);

create operator #<# (
  leftarg = hstore,
  rightarg = hstore,
  function = hstore_lt,
  commutator = #>#,
  negator = #>=#
);

create operator #<=# (
  leftarg = hstore,
  rightarg = hstore,
  function = hstore_le,
  commutator = #>=#,
  negator = #>#
);

create operator #= (
  leftarg = anyelement,
  rightarg = hstore,
  function = populate_record
);

create operator #># (
  leftarg = hstore,
  rightarg = hstore,
  function = hstore_gt,
  commutator = #<#,
  negator = #<=#
);

create operator #>=# (
  leftarg = hstore,
  rightarg = hstore,
  function = hstore_ge,
  commutator = #<=#,
  negator = #<#
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
  function = hstore_ne,
  commutator = <>,
  negator = =
);

create operator <@ (
  leftarg = hstore,
  rightarg = hstore,
  function = hs_contained,
  commutator = @>
);

create operator = (
  leftarg = hstore,
  rightarg = hstore,
  function = hstore_eq,
  commutator = =,
  negator = <>
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
  function = hs_contains,
  commutator = <@
);

create operator || (
  leftarg = hstore,
  rightarg = hstore,
  function = hs_concat
);

create operator family btree_hstore_ops
  using btree;

create operator family gin_hstore_ops
  using gin;

create operator family gist_hstore_ops
  using gist;

create operator family hash_hstore_ops
  using hash;

create operator class btree_hstore_ops
  default for type hstore
  using btree
  family btree_hstore_ops as
    operator 1 #<# (hstore, hstore),
    operator 2 #<=# (hstore, hstore),
    operator 3 = (hstore, hstore),
    operator 4 #>=# (hstore, hstore),
    operator 5 #># (hstore, hstore),
    function 1 (hstore, hstore) hstore_cmp(hstore, hstore),
    storage hstore;

create operator class gin_hstore_ops
  default for type hstore
  using gin
  family gin_hstore_ops as
    function 2 (hstore, hstore) gin_extract_hstore(hstore, internal),
    function 3 (hstore, hstore) gin_extract_hstore_query(
      hstore,
      internal,
      smallint,
      internal,
      internal
    ),
    storage text;

create operator class gist_hstore_ops
  default for type hstore
  using gist
  family gist_hstore_ops as
    function 1 (hstore, hstore) ghstore_consistent(
      internal,
      hstore,
      smallint,
      oid,
      internal
    ),
    function 2 (hstore, hstore) ghstore_union(internal, internal),
    function 5 (hstore, hstore) ghstore_penalty(internal, internal, internal),
    function 6 (hstore, hstore) ghstore_picksplit(internal, internal),
    function 7 (hstore, hstore) ghstore_same(ghstore, ghstore, internal),
    storage ghstore;

create operator class hash_hstore_ops
  default for type hstore
  using hash
  family hash_hstore_ops as
    operator 1 = (hstore, hstore),
    function 1 (hstore, hstore) hstore_hash(hstore),
    storage hstore;

alter operator family gin_hstore_ops
  using gin
  add
    operator 7 @> (hstore, hstore),
    operator 9 ? (hstore, text),
    operator 10 ?| (hstore, text[]),
    operator 11 ?& (hstore, text[]),
    function 1 (hstore, hstore) pg_catalog.bttextcmp(text, text),
    function 4 (hstore, hstore) gin_consistent_hstore(
      internal,
      smallint,
      hstore,
      integer,
      internal,
      internal
    );

alter operator family gist_hstore_ops
  using gist
  add
    operator 7 @> (hstore, hstore),
    operator 9 ? (hstore, text),
    operator 10 ?| (hstore, text[]),
    operator 11 ?& (hstore, text[]),
    function 3 (hstore, hstore) ghstore_compress(internal),
    function 4 (hstore, hstore) ghstore_decompress(internal),
    function 10 (hstore, hstore) ghstore_options(internal);

alter operator family hash_hstore_ops
  using hash
  add
    function 2 (hstore, hstore) hstore_hash_extended(hstore, bigint);

create cast (hstore as json) with function hstore_to_json(hstore);

create cast (hstore as jsonb) with function hstore_to_jsonb(hstore);

create cast (text[] as hstore) with function hstore(text[]);
