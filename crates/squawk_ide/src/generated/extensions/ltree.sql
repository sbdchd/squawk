-- squawk-ignore-file
-- pg version: 18.2
-- update via:
--   cargo xtask sync-builtins

-- size: -1, align: 4
create type public.lquery;

-- size: -1, align: 4
create type public.ltree;

-- size: -1, align: 4
create type public.ltree_gist;

-- size: -1, align: 4
create type public.ltxtquery;

create function public._lt_q_regex(ltree[], lquery[]) returns boolean
  language c;

create function public._lt_q_rregex(lquery[], ltree[]) returns boolean
  language c;

create function public._ltq_extract_regex(ltree[], lquery) returns ltree
  language c;

create function public._ltq_regex(ltree[], lquery) returns boolean
  language c;

create function public._ltq_rregex(lquery, ltree[]) returns boolean
  language c;

create function public._ltree_compress(internal) returns internal
  language c;

create function public._ltree_consistent(internal, ltree[], smallint, oid, internal) returns boolean
  language c;

create function public._ltree_extract_isparent(ltree[], ltree) returns ltree
  language c;

create function public._ltree_extract_risparent(ltree[], ltree) returns ltree
  language c;

create function public._ltree_gist_options(internal) returns void
  language c;

create function public._ltree_isparent(ltree[], ltree) returns boolean
  language c;

create function public._ltree_penalty(internal, internal, internal) returns internal
  language c;

create function public._ltree_picksplit(internal, internal) returns internal
  language c;

create function public._ltree_r_isparent(ltree, ltree[]) returns boolean
  language c;

create function public._ltree_r_risparent(ltree, ltree[]) returns boolean
  language c;

create function public._ltree_risparent(ltree[], ltree) returns boolean
  language c;

create function public._ltree_same(ltree_gist, ltree_gist, internal) returns internal
  language c;

create function public._ltree_union(internal, internal) returns ltree_gist
  language c;

create function public._ltxtq_exec(ltree[], ltxtquery) returns boolean
  language c;

create function public._ltxtq_extract_exec(ltree[], ltxtquery) returns ltree
  language c;

create function public._ltxtq_rexec(ltxtquery, ltree[]) returns boolean
  language c;

create function public.hash_ltree(ltree) returns integer
  language c;

create function public.hash_ltree_extended(ltree, bigint) returns bigint
  language c;

create function public.index(ltree, ltree) returns integer
  language c;

create function public.index(ltree, ltree, integer) returns integer
  language c;

create function public.lca(ltree, ltree) returns ltree
  language c;

create function public.lca(ltree, ltree, ltree) returns ltree
  language c;

create function public.lca(ltree, ltree, ltree, ltree) returns ltree
  language c;

create function public.lca(ltree, ltree, ltree, ltree, ltree) returns ltree
  language c;

create function public.lca(ltree, ltree, ltree, ltree, ltree, ltree) returns ltree
  language c;

create function public.lca(ltree, ltree, ltree, ltree, ltree, ltree, ltree) returns ltree
  language c;

create function public.lca(ltree, ltree, ltree, ltree, ltree, ltree, ltree, ltree) returns ltree
  language c;

create function public.lca(ltree[]) returns ltree
  language c;

create function public.lquery_in(cstring) returns lquery
  language c;

create function public.lquery_out(lquery) returns cstring
  language c;

create function public.lquery_recv(internal) returns lquery
  language c;

create function public.lquery_send(lquery) returns bytea
  language c;

create function public.lt_q_regex(ltree, lquery[]) returns boolean
  language c;

create function public.lt_q_rregex(lquery[], ltree) returns boolean
  language c;

create function public.ltq_regex(ltree, lquery) returns boolean
  language c;

create function public.ltq_rregex(lquery, ltree) returns boolean
  language c;

create function public.ltree2text(ltree) returns text
  language c;

create function public.ltree_addltree(ltree, ltree) returns ltree
  language c;

create function public.ltree_addtext(ltree, text) returns ltree
  language c;

create function public.ltree_cmp(ltree, ltree) returns integer
  language c;

create function public.ltree_compress(internal) returns internal
  language c;

create function public.ltree_consistent(internal, ltree, smallint, oid, internal) returns boolean
  language c;

create function public.ltree_decompress(internal) returns internal
  language c;

create function public.ltree_eq(ltree, ltree) returns boolean
  language c;

create function public.ltree_ge(ltree, ltree) returns boolean
  language c;

create function public.ltree_gist_in(cstring) returns ltree_gist
  language c;

create function public.ltree_gist_options(internal) returns void
  language c;

create function public.ltree_gist_out(ltree_gist) returns cstring
  language c;

create function public.ltree_gt(ltree, ltree) returns boolean
  language c;

create function public.ltree_in(cstring) returns ltree
  language c;

create function public.ltree_isparent(ltree, ltree) returns boolean
  language c;

create function public.ltree_le(ltree, ltree) returns boolean
  language c;

create function public.ltree_lt(ltree, ltree) returns boolean
  language c;

create function public.ltree_ne(ltree, ltree) returns boolean
  language c;

create function public.ltree_out(ltree) returns cstring
  language c;

create function public.ltree_penalty(internal, internal, internal) returns internal
  language c;

create function public.ltree_picksplit(internal, internal) returns internal
  language c;

create function public.ltree_recv(internal) returns ltree
  language c;

create function public.ltree_risparent(ltree, ltree) returns boolean
  language c;

create function public.ltree_same(ltree_gist, ltree_gist, internal) returns internal
  language c;

create function public.ltree_send(ltree) returns bytea
  language c;

create function public.ltree_textadd(text, ltree) returns ltree
  language c;

create function public.ltree_union(internal, internal) returns ltree_gist
  language c;

create function public.ltreeparentsel(internal, oid, internal, integer) returns double precision
  language c;

create function public.ltxtq_exec(ltree, ltxtquery) returns boolean
  language c;

create function public.ltxtq_in(cstring) returns ltxtquery
  language c;

create function public.ltxtq_out(ltxtquery) returns cstring
  language c;

create function public.ltxtq_recv(internal) returns ltxtquery
  language c;

create function public.ltxtq_rexec(ltxtquery, ltree) returns boolean
  language c;

create function public.ltxtq_send(ltxtquery) returns bytea
  language c;

create function public.nlevel(ltree) returns integer
  language c;

create function public.subltree(ltree, integer, integer) returns ltree
  language c;

create function public.subpath(ltree, integer) returns ltree
  language c;

create function public.subpath(ltree, integer, integer) returns ltree
  language c;

create function public.text2ltree(text) returns ltree
  language c;

create operator public.< (
  leftarg = ltree,
  rightarg = ltree,
  function = public.ltree_lt
);

create operator public.<= (
  leftarg = ltree,
  rightarg = ltree,
  function = public.ltree_le
);

create operator public.<> (
  leftarg = ltree,
  rightarg = ltree,
  function = public.ltree_ne
);

create operator public.<@ (
  leftarg = ltree,
  rightarg = ltree,
  function = public.ltree_risparent
);

create operator public.<@ (
  leftarg = ltree,
  rightarg = ltree[],
  function = public._ltree_r_isparent
);

create operator public.<@ (
  leftarg = ltree[],
  rightarg = ltree,
  function = public._ltree_risparent
);

create operator public.= (
  leftarg = ltree,
  rightarg = ltree,
  function = public.ltree_eq
);

create operator public.> (
  leftarg = ltree,
  rightarg = ltree,
  function = public.ltree_gt
);

create operator public.>= (
  leftarg = ltree,
  rightarg = ltree,
  function = public.ltree_ge
);

create operator public.? (
  leftarg = lquery[],
  rightarg = ltree,
  function = public.lt_q_rregex
);

create operator public.? (
  leftarg = lquery[],
  rightarg = ltree[],
  function = public._lt_q_rregex
);

create operator public.? (
  leftarg = ltree,
  rightarg = lquery[],
  function = public.lt_q_regex
);

create operator public.? (
  leftarg = ltree[],
  rightarg = lquery[],
  function = public._lt_q_regex
);

create operator public.?<@ (
  leftarg = ltree[],
  rightarg = ltree,
  function = public._ltree_extract_risparent
);

create operator public.?@ (
  leftarg = ltree[],
  rightarg = ltxtquery,
  function = public._ltxtq_extract_exec
);

create operator public.?@> (
  leftarg = ltree[],
  rightarg = ltree,
  function = public._ltree_extract_isparent
);

create operator public.?~ (
  leftarg = ltree[],
  rightarg = lquery,
  function = public._ltq_extract_regex
);

create operator public.@ (
  leftarg = ltree,
  rightarg = ltxtquery,
  function = public.ltxtq_exec
);

create operator public.@ (
  leftarg = ltree[],
  rightarg = ltxtquery,
  function = public._ltxtq_exec
);

create operator public.@ (
  leftarg = ltxtquery,
  rightarg = ltree,
  function = public.ltxtq_rexec
);

create operator public.@ (
  leftarg = ltxtquery,
  rightarg = ltree[],
  function = public._ltxtq_rexec
);

create operator public.@> (
  leftarg = ltree,
  rightarg = ltree,
  function = public.ltree_isparent
);

create operator public.@> (
  leftarg = ltree,
  rightarg = ltree[],
  function = public._ltree_r_risparent
);

create operator public.@> (
  leftarg = ltree[],
  rightarg = ltree,
  function = public._ltree_isparent
);

create operator public.^<@ (
  leftarg = ltree,
  rightarg = ltree,
  function = public.ltree_risparent
);

create operator public.^<@ (
  leftarg = ltree,
  rightarg = ltree[],
  function = public._ltree_r_isparent
);

create operator public.^<@ (
  leftarg = ltree[],
  rightarg = ltree,
  function = public._ltree_risparent
);

create operator public.^? (
  leftarg = lquery[],
  rightarg = ltree,
  function = public.lt_q_rregex
);

create operator public.^? (
  leftarg = lquery[],
  rightarg = ltree[],
  function = public._lt_q_rregex
);

create operator public.^? (
  leftarg = ltree,
  rightarg = lquery[],
  function = public.lt_q_regex
);

create operator public.^? (
  leftarg = ltree[],
  rightarg = lquery[],
  function = public._lt_q_regex
);

create operator public.^@ (
  leftarg = ltree,
  rightarg = ltxtquery,
  function = public.ltxtq_exec
);

create operator public.^@ (
  leftarg = ltree[],
  rightarg = ltxtquery,
  function = public._ltxtq_exec
);

create operator public.^@ (
  leftarg = ltxtquery,
  rightarg = ltree,
  function = public.ltxtq_rexec
);

create operator public.^@ (
  leftarg = ltxtquery,
  rightarg = ltree[],
  function = public._ltxtq_rexec
);

create operator public.^@> (
  leftarg = ltree,
  rightarg = ltree,
  function = public.ltree_isparent
);

create operator public.^@> (
  leftarg = ltree,
  rightarg = ltree[],
  function = public._ltree_r_risparent
);

create operator public.^@> (
  leftarg = ltree[],
  rightarg = ltree,
  function = public._ltree_isparent
);

create operator public.^~ (
  leftarg = lquery,
  rightarg = ltree,
  function = public.ltq_rregex
);

create operator public.^~ (
  leftarg = lquery,
  rightarg = ltree[],
  function = public._ltq_rregex
);

create operator public.^~ (
  leftarg = ltree,
  rightarg = lquery,
  function = public.ltq_regex
);

create operator public.^~ (
  leftarg = ltree[],
  rightarg = lquery,
  function = public._ltq_regex
);

create operator public.|| (
  leftarg = ltree,
  rightarg = ltree,
  function = public.ltree_addltree
);

create operator public.|| (
  leftarg = ltree,
  rightarg = text,
  function = public.ltree_addtext
);

create operator public.|| (
  leftarg = text,
  rightarg = ltree,
  function = public.ltree_textadd
);

create operator public.~ (
  leftarg = lquery,
  rightarg = ltree,
  function = public.ltq_rregex
);

create operator public.~ (
  leftarg = lquery,
  rightarg = ltree[],
  function = public._ltq_rregex
);

create operator public.~ (
  leftarg = ltree,
  rightarg = lquery,
  function = public.ltq_regex
);

create operator public.~ (
  leftarg = ltree[],
  rightarg = lquery,
  function = public._ltq_regex
);

