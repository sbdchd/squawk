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
  function = similarity_op,
  commutator = %
);

create operator %> (
  leftarg = text,
  rightarg = text,
  function = word_similarity_commutator_op,
  commutator = <%
);

create operator %>> (
  leftarg = text,
  rightarg = text,
  function = strict_word_similarity_commutator_op,
  commutator = <<%
);

create operator <% (
  leftarg = text,
  rightarg = text,
  function = word_similarity_op,
  commutator = %>
);

create operator <-> (
  leftarg = text,
  rightarg = text,
  function = similarity_dist,
  commutator = <->
);

create operator <->> (
  leftarg = text,
  rightarg = text,
  function = word_similarity_dist_commutator_op,
  commutator = <<->
);

create operator <->>> (
  leftarg = text,
  rightarg = text,
  function = strict_word_similarity_dist_commutator_op,
  commutator = <<<->
);

create operator <<% (
  leftarg = text,
  rightarg = text,
  function = strict_word_similarity_op,
  commutator = %>>
);

create operator <<-> (
  leftarg = text,
  rightarg = text,
  function = word_similarity_dist_op,
  commutator = <->>
);

create operator <<<-> (
  leftarg = text,
  rightarg = text,
  function = strict_word_similarity_dist_op,
  commutator = <->>>
);

create operator family gin_trgm_ops
  using gin;

create operator family gist_trgm_ops
  using gist;

create operator class gin_trgm_ops
  for type text
  using gin
  family gin_trgm_ops as
    function 2 (text, text) gin_extract_value_trgm(text, internal),
    function 3 (text, text) gin_extract_query_trgm(
      text,
      internal,
      smallint,
      internal,
      internal,
      internal,
      internal
    ),
    storage integer;

create operator class gist_trgm_ops
  for type text
  using gist
  family gist_trgm_ops as
    function 1 (text, text) gtrgm_consistent(
      internal,
      text,
      smallint,
      oid,
      internal
    ),
    function 2 (text, text) gtrgm_union(internal, internal),
    function 5 (text, text) gtrgm_penalty(internal, internal, internal),
    function 6 (text, text) gtrgm_picksplit(internal, internal),
    function 7 (text, text) gtrgm_same(gtrgm, gtrgm, internal),
    storage gtrgm;

alter operator family gin_trgm_ops
  using gin
  add
    operator 1 % (text, text),
    operator 3 pg_catalog.~~ (text, text),
    operator 4 pg_catalog.~~* (text, text),
    operator 5 pg_catalog.~ (text, text),
    operator 6 pg_catalog.~* (text, text),
    operator 7 %> (text, text),
    operator 9 %>> (text, text),
    operator 11 pg_catalog.= (text, text),
    function 1 (text, text) pg_catalog.btint4cmp(integer, integer),
    function 4 (text, text) gin_trgm_consistent(
      internal,
      smallint,
      text,
      integer,
      internal,
      internal,
      internal,
      internal
    ),
    function 6 (text, text) gin_trgm_triconsistent(
      internal,
      smallint,
      text,
      integer,
      internal,
      internal,
      internal
    );

alter operator family gist_trgm_ops
  using gist
  add
    operator 1 % (text, text),
    operator 2 <-> (text, text) for order by pg_catalog.float_ops,
    operator 3 pg_catalog.~~ (text, text),
    operator 4 pg_catalog.~~* (text, text),
    operator 5 pg_catalog.~ (text, text),
    operator 6 pg_catalog.~* (text, text),
    operator 7 %> (text, text),
    operator 8 <->> (text, text) for order by pg_catalog.float_ops,
    operator 9 %>> (text, text),
    operator 10 <->>> (text, text) for order by pg_catalog.float_ops,
    operator 11 pg_catalog.= (text, text),
    function 3 (text, text) gtrgm_compress(internal),
    function 4 (text, text) gtrgm_decompress(internal),
    function 8 (text, text) gtrgm_distance(
      internal,
      text,
      smallint,
      oid,
      internal
    ),
    function 10 (text, text) gtrgm_options(internal);
