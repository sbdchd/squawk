-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

-- size: -1, align: 4
create type gtrgm;

create function gin_extract_query_trgm(
  text,
  internal,
  smallint,
  internal,
  internal,
  internal,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gin_extract_value_trgm(text, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gin_trgm_consistent(
  internal,
  smallint,
  text,
  integer,
  internal,
  internal,
  internal,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gin_trgm_triconsistent(
  internal,
  smallint,
  text,
  integer,
  internal,
  internal,
  internal
) returns "char"
  language c
  immutable
  strict
  parallel safe;

create function gtrgm_compress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gtrgm_consistent(
  internal,
  text,
  smallint,
  oid,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function gtrgm_decompress(internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gtrgm_distance(
  internal,
  text,
  smallint,
  oid,
  internal
) returns double precision
  language c
  immutable
  strict
  parallel safe;

create function gtrgm_in(cstring) returns gtrgm
  language c
  immutable
  strict
  parallel safe;

create function gtrgm_options(internal) returns void
  language c
  immutable
  parallel safe;

create function gtrgm_out(gtrgm) returns cstring
  language c
  immutable
  strict
  parallel safe;

create function gtrgm_penalty(internal, internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gtrgm_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gtrgm_same(gtrgm, gtrgm, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function gtrgm_union(internal, internal) returns gtrgm
  language c
  immutable
  strict
  parallel safe;

create function set_limit(real) returns real
  language c
  strict;

create function show_limit() returns real
  language c
  stable
  strict
  parallel safe;

create function show_trgm(text) returns text[]
  language c
  immutable
  strict
  parallel safe;

create function similarity(text, text) returns real
  language c
  immutable
  strict
  parallel safe;

create function similarity_dist(text, text) returns real
  language c
  immutable
  strict
  parallel safe;

create function similarity_op(text, text) returns boolean
  language c
  stable
  strict
  parallel safe;

create function strict_word_similarity(text, text) returns real
  language c
  immutable
  strict
  parallel safe;

create function strict_word_similarity_commutator_op(text, text) returns boolean
  language c
  stable
  strict
  parallel safe;

create function strict_word_similarity_dist_commutator_op(
  text,
  text
) returns real
  language c
  immutable
  strict
  parallel safe;

create function strict_word_similarity_dist_op(text, text) returns real
  language c
  immutable
  strict
  parallel safe;

create function strict_word_similarity_op(text, text) returns boolean
  language c
  stable
  strict
  parallel safe;

create function word_similarity(text, text) returns real
  language c
  immutable
  strict
  parallel safe;

create function word_similarity_commutator_op(text, text) returns boolean
  language c
  stable
  strict
  parallel safe;

create function word_similarity_dist_commutator_op(text, text) returns real
  language c
  immutable
  strict
  parallel safe;

create function word_similarity_dist_op(text, text) returns real
  language c
  immutable
  strict
  parallel safe;

create function word_similarity_op(text, text) returns boolean
  language c
  stable
  strict
  parallel safe;

create type gtrgm (
  input = gtrgm_in,
  output = gtrgm_out
);

create operator % (
  leftarg = text,
  rightarg = text,
  function = similarity_op
);

create operator %> (
  leftarg = text,
  rightarg = text,
  function = word_similarity_commutator_op
);

create operator %>> (
  leftarg = text,
  rightarg = text,
  function = strict_word_similarity_commutator_op
);

create operator <% (
  leftarg = text,
  rightarg = text,
  function = word_similarity_op
);

create operator <-> (
  leftarg = text,
  rightarg = text,
  function = similarity_dist
);

create operator <->> (
  leftarg = text,
  rightarg = text,
  function = word_similarity_dist_commutator_op
);

create operator <->>> (
  leftarg = text,
  rightarg = text,
  function = strict_word_similarity_dist_commutator_op
);

create operator <<% (
  leftarg = text,
  rightarg = text,
  function = strict_word_similarity_op
);

create operator <<-> (
  leftarg = text,
  rightarg = text,
  function = word_similarity_dist_op
);

create operator <<<-> (
  leftarg = text,
  rightarg = text,
  function = strict_word_similarity_dist_op
);

create operator family gin_trgm_ops
  using gin;

create operator family gist_trgm_ops
  using gist;

create operator class gin_trgm_ops
  for type text
  using gin
  family gin_trgm_ops as
  storage integer;

create operator class gist_trgm_ops
  for type text
  using gist
  family gist_trgm_ops as
  storage gtrgm;
