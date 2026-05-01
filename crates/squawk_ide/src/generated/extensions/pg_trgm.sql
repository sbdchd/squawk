-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

-- size: -1, align: 4
create type public.gtrgm;

create function public.gin_extract_query_trgm(text, internal, smallint, internal, internal, internal, internal) returns internal
  language c;

create function public.gin_extract_value_trgm(text, internal) returns internal
  language c;

create function public.gin_trgm_consistent(internal, smallint, text, integer, internal, internal, internal, internal) returns boolean
  language c;

create function public.gin_trgm_triconsistent(internal, smallint, text, integer, internal, internal, internal) returns "char"
  language c;

create function public.gtrgm_compress(internal) returns internal
  language c;

create function public.gtrgm_consistent(internal, text, smallint, oid, internal) returns boolean
  language c;

create function public.gtrgm_decompress(internal) returns internal
  language c;

create function public.gtrgm_distance(internal, text, smallint, oid, internal) returns double precision
  language c;

create function public.gtrgm_in(cstring) returns gtrgm
  language c;

create function public.gtrgm_options(internal) returns void
  language c;

create function public.gtrgm_out(gtrgm) returns cstring
  language c;

create function public.gtrgm_penalty(internal, internal, internal) returns internal
  language c;

create function public.gtrgm_picksplit(internal, internal) returns internal
  language c;

create function public.gtrgm_same(gtrgm, gtrgm, internal) returns internal
  language c;

create function public.gtrgm_union(internal, internal) returns gtrgm
  language c;

create function public.set_limit(real) returns real
  language c;

create function public.show_limit() returns real
  language c;

create function public.show_trgm(text) returns text[]
  language c;

create function public.similarity(text, text) returns real
  language c;

create function public.similarity_dist(text, text) returns real
  language c;

create function public.similarity_op(text, text) returns boolean
  language c;

create function public.strict_word_similarity(text, text) returns real
  language c;

create function public.strict_word_similarity_commutator_op(text, text) returns boolean
  language c;

create function public.strict_word_similarity_dist_commutator_op(text, text) returns real
  language c;

create function public.strict_word_similarity_dist_op(text, text) returns real
  language c;

create function public.strict_word_similarity_op(text, text) returns boolean
  language c;

create function public.word_similarity(text, text) returns real
  language c;

create function public.word_similarity_commutator_op(text, text) returns boolean
  language c;

create function public.word_similarity_dist_commutator_op(text, text) returns real
  language c;

create function public.word_similarity_dist_op(text, text) returns real
  language c;

create function public.word_similarity_op(text, text) returns boolean
  language c;

create operator public.% (
  leftarg = text,
  rightarg = text,
  function = public.similarity_op
);

create operator public.%> (
  leftarg = text,
  rightarg = text,
  function = public.word_similarity_commutator_op
);

create operator public.%>> (
  leftarg = text,
  rightarg = text,
  function = public.strict_word_similarity_commutator_op
);

create operator public.<% (
  leftarg = text,
  rightarg = text,
  function = public.word_similarity_op
);

create operator public.<-> (
  leftarg = text,
  rightarg = text,
  function = public.similarity_dist
);

create operator public.<->> (
  leftarg = text,
  rightarg = text,
  function = public.word_similarity_dist_commutator_op
);

create operator public.<->>> (
  leftarg = text,
  rightarg = text,
  function = public.strict_word_similarity_dist_commutator_op
);

create operator public.<<% (
  leftarg = text,
  rightarg = text,
  function = public.strict_word_similarity_op
);

create operator public.<<-> (
  leftarg = text,
  rightarg = text,
  function = public.word_similarity_dist_op
);

create operator public.<<<-> (
  leftarg = text,
  rightarg = text,
  function = public.strict_word_similarity_dist_op
);

