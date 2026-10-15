-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

-- size: -1, align: 4
create type lquery;

-- size: -1, align: 4
create type ltree;

-- size: -1, align: 4
create type ltree_gist;

-- size: -1, align: 4
create type ltxtquery;

create function _lt_q_regex(ltree[], lquery[]) returns boolean
  language c;

create function _lt_q_rregex(lquery[], ltree[]) returns boolean
  language c;

create function _ltq_extract_regex(ltree[], lquery) returns ltree
  language c;

create function _ltq_regex(ltree[], lquery) returns boolean
  language c;

create function _ltq_rregex(lquery, ltree[]) returns boolean
  language c;

create function _ltree_compress(internal) returns internal
  language c;

create function _ltree_consistent(internal, ltree[], smallint, oid, internal) returns boolean
  language c;

create function _ltree_extract_isparent(ltree[], ltree) returns ltree
  language c;

create function _ltree_extract_risparent(ltree[], ltree) returns ltree
  language c;

create function _ltree_gist_options(internal) returns void
  language c;

create function _ltree_isparent(ltree[], ltree) returns boolean
  language c;

create function _ltree_penalty(internal, internal, internal) returns internal
  language c;

create function _ltree_picksplit(internal, internal) returns internal
  language c;

create function _ltree_r_isparent(ltree, ltree[]) returns boolean
  language c;

create function _ltree_r_risparent(ltree, ltree[]) returns boolean
  language c;

create function _ltree_risparent(ltree[], ltree) returns boolean
  language c;

create function _ltree_same(ltree_gist, ltree_gist, internal) returns internal
  language c;

create function _ltree_union(internal, internal) returns ltree_gist
  language c;

create function _ltxtq_exec(ltree[], ltxtquery) returns boolean
  language c;

create function _ltxtq_extract_exec(ltree[], ltxtquery) returns ltree
  language c;

create function _ltxtq_rexec(ltxtquery, ltree[]) returns boolean
  language c;

create function hash_ltree(ltree) returns integer
  language c;

create function hash_ltree_extended(ltree, bigint) returns bigint
  language c;

create function index(ltree, ltree) returns integer
  language c;

create function index(ltree, ltree, integer) returns integer
  language c;

create function lca(ltree, ltree) returns ltree
  language c;

create function lca(ltree, ltree, ltree) returns ltree
  language c;

create function lca(ltree, ltree, ltree, ltree) returns ltree
  language c;

create function lca(ltree, ltree, ltree, ltree, ltree) returns ltree
  language c;

create function lca(ltree, ltree, ltree, ltree, ltree, ltree) returns ltree
  language c;

create function lca(ltree, ltree, ltree, ltree, ltree, ltree, ltree) returns ltree
  language c;

create function lca(ltree, ltree, ltree, ltree, ltree, ltree, ltree, ltree) returns ltree
  language c;

create function lca(ltree[]) returns ltree
  language c;

create function lquery_in(cstring) returns lquery
  language c;

create function lquery_out(lquery) returns cstring
  language c;

create function lquery_recv(internal) returns lquery
  language c;

create function lquery_send(lquery) returns bytea
  language c;

create function lt_q_regex(ltree, lquery[]) returns boolean
  language c;

create function lt_q_rregex(lquery[], ltree) returns boolean
  language c;

create function ltq_regex(ltree, lquery) returns boolean
  language c;

create function ltq_rregex(lquery, ltree) returns boolean
  language c;

create function ltree2text(ltree) returns text
  language c;

create function ltree_addltree(ltree, ltree) returns ltree
  language c;

create function ltree_addtext(ltree, text) returns ltree
  language c;

create function ltree_cmp(ltree, ltree) returns integer
  language c;

create function ltree_compress(internal) returns internal
  language c;

create function ltree_consistent(internal, ltree, smallint, oid, internal) returns boolean
  language c;

create function ltree_decompress(internal) returns internal
  language c;

create function ltree_eq(ltree, ltree) returns boolean
  language c;

create function ltree_ge(ltree, ltree) returns boolean
  language c;

create function ltree_gist_in(cstring) returns ltree_gist
  language c;

create function ltree_gist_options(internal) returns void
  language c;

create function ltree_gist_out(ltree_gist) returns cstring
  language c;

create function ltree_gt(ltree, ltree) returns boolean
  language c;

create function ltree_in(cstring) returns ltree
  language c;

create function ltree_isparent(ltree, ltree) returns boolean
  language c;

create function ltree_le(ltree, ltree) returns boolean
  language c;

create function ltree_lt(ltree, ltree) returns boolean
  language c;

create function ltree_ne(ltree, ltree) returns boolean
  language c;

create function ltree_out(ltree) returns cstring
  language c;

create function ltree_penalty(internal, internal, internal) returns internal
  language c;

create function ltree_picksplit(internal, internal) returns internal
  language c;

create function ltree_recv(internal) returns ltree
  language c;

create function ltree_risparent(ltree, ltree) returns boolean
  language c;

create function ltree_same(ltree_gist, ltree_gist, internal) returns internal
  language c;

create function ltree_send(ltree) returns bytea
  language c;

create function ltree_textadd(text, ltree) returns ltree
  language c;

create function ltree_union(internal, internal) returns ltree_gist
  language c;

create function ltreeparentsel(internal, oid, internal, integer) returns double precision
  language c;

create function ltxtq_exec(ltree, ltxtquery) returns boolean
  language c;

create function ltxtq_in(cstring) returns ltxtquery
  language c;

create function ltxtq_out(ltxtquery) returns cstring
  language c;

create function ltxtq_recv(internal) returns ltxtquery
  language c;

create function ltxtq_rexec(ltxtquery, ltree) returns boolean
  language c;

create function ltxtq_send(ltxtquery) returns bytea
  language c;

create function nlevel(ltree) returns integer
  language c;

create function subltree(ltree, integer, integer) returns ltree
  language c;

create function subpath(ltree, integer) returns ltree
  language c;

create function subpath(ltree, integer, integer) returns ltree
  language c;

create function text2ltree(text) returns ltree
  language c;

create type lquery (input = lquery_in, output = lquery_out);

create type ltree (input = ltree_in, output = ltree_out);

create type ltree_gist (input = ltree_gist_in, output = ltree_gist_out);

create type ltxtquery (input = ltxtq_in, output = ltxtq_out);

create operator < (
  leftarg = ltree,
  rightarg = ltree,
  function = ltree_lt
);

create operator <= (
  leftarg = ltree,
  rightarg = ltree,
  function = ltree_le
);

create operator <> (
  leftarg = ltree,
  rightarg = ltree,
  function = ltree_ne
);

create operator <@ (
  leftarg = ltree,
  rightarg = ltree,
  function = ltree_risparent
);

create operator <@ (
  leftarg = ltree,
  rightarg = ltree[],
  function = _ltree_r_isparent
);

create operator <@ (
  leftarg = ltree[],
  rightarg = ltree,
  function = _ltree_risparent
);

create operator = (
  leftarg = ltree,
  rightarg = ltree,
  function = ltree_eq
);

create operator > (
  leftarg = ltree,
  rightarg = ltree,
  function = ltree_gt
);

create operator >= (
  leftarg = ltree,
  rightarg = ltree,
  function = ltree_ge
);

create operator ? (
  leftarg = lquery[],
  rightarg = ltree,
  function = lt_q_rregex
);

create operator ? (
  leftarg = lquery[],
  rightarg = ltree[],
  function = _lt_q_rregex
);

create operator ? (
  leftarg = ltree,
  rightarg = lquery[],
  function = lt_q_regex
);

create operator ? (
  leftarg = ltree[],
  rightarg = lquery[],
  function = _lt_q_regex
);

create operator ?<@ (
  leftarg = ltree[],
  rightarg = ltree,
  function = _ltree_extract_risparent
);

create operator ?@ (
  leftarg = ltree[],
  rightarg = ltxtquery,
  function = _ltxtq_extract_exec
);

create operator ?@> (
  leftarg = ltree[],
  rightarg = ltree,
  function = _ltree_extract_isparent
);

create operator ?~ (
  leftarg = ltree[],
  rightarg = lquery,
  function = _ltq_extract_regex
);

create operator @ (
  leftarg = ltree,
  rightarg = ltxtquery,
  function = ltxtq_exec
);

create operator @ (
  leftarg = ltree[],
  rightarg = ltxtquery,
  function = _ltxtq_exec
);

create operator @ (
  leftarg = ltxtquery,
  rightarg = ltree,
  function = ltxtq_rexec
);

create operator @ (
  leftarg = ltxtquery,
  rightarg = ltree[],
  function = _ltxtq_rexec
);

create operator @> (
  leftarg = ltree,
  rightarg = ltree,
  function = ltree_isparent
);

create operator @> (
  leftarg = ltree,
  rightarg = ltree[],
  function = _ltree_r_risparent
);

create operator @> (
  leftarg = ltree[],
  rightarg = ltree,
  function = _ltree_isparent
);

create operator ^<@ (
  leftarg = ltree,
  rightarg = ltree,
  function = ltree_risparent
);

create operator ^<@ (
  leftarg = ltree,
  rightarg = ltree[],
  function = _ltree_r_isparent
);

create operator ^<@ (
  leftarg = ltree[],
  rightarg = ltree,
  function = _ltree_risparent
);

create operator ^? (
  leftarg = lquery[],
  rightarg = ltree,
  function = lt_q_rregex
);

create operator ^? (
  leftarg = lquery[],
  rightarg = ltree[],
  function = _lt_q_rregex
);

create operator ^? (
  leftarg = ltree,
  rightarg = lquery[],
  function = lt_q_regex
);

create operator ^? (
  leftarg = ltree[],
  rightarg = lquery[],
  function = _lt_q_regex
);

create operator ^@ (
  leftarg = ltree,
  rightarg = ltxtquery,
  function = ltxtq_exec
);

create operator ^@ (
  leftarg = ltree[],
  rightarg = ltxtquery,
  function = _ltxtq_exec
);

create operator ^@ (
  leftarg = ltxtquery,
  rightarg = ltree,
  function = ltxtq_rexec
);

create operator ^@ (
  leftarg = ltxtquery,
  rightarg = ltree[],
  function = _ltxtq_rexec
);

create operator ^@> (
  leftarg = ltree,
  rightarg = ltree,
  function = ltree_isparent
);

create operator ^@> (
  leftarg = ltree,
  rightarg = ltree[],
  function = _ltree_r_risparent
);

create operator ^@> (
  leftarg = ltree[],
  rightarg = ltree,
  function = _ltree_isparent
);

create operator ^~ (
  leftarg = lquery,
  rightarg = ltree,
  function = ltq_rregex
);

create operator ^~ (
  leftarg = lquery,
  rightarg = ltree[],
  function = _ltq_rregex
);

create operator ^~ (
  leftarg = ltree,
  rightarg = lquery,
  function = ltq_regex
);

create operator ^~ (
  leftarg = ltree[],
  rightarg = lquery,
  function = _ltq_regex
);

create operator || (
  leftarg = ltree,
  rightarg = ltree,
  function = ltree_addltree
);

create operator || (
  leftarg = ltree,
  rightarg = text,
  function = ltree_addtext
);

create operator || (
  leftarg = text,
  rightarg = ltree,
  function = ltree_textadd
);

create operator ~ (
  leftarg = lquery,
  rightarg = ltree,
  function = ltq_rregex
);

create operator ~ (
  leftarg = lquery,
  rightarg = ltree[],
  function = _ltq_rregex
);

create operator ~ (
  leftarg = ltree,
  rightarg = lquery,
  function = ltq_regex
);

create operator ~ (
  leftarg = ltree[],
  rightarg = lquery,
  function = _ltq_regex
);

create operator family gist__ltree_ops using gist;

create operator family gist_ltree_ops using gist;

create operator family hash_ltree_ops using hash;

create operator family ltree_ops using btree;

create operator class gist__ltree_ops default for type ltree[] using gist family gist__ltree_ops as storage ltree_gist;

create operator class gist_ltree_ops default for type ltree using gist family gist_ltree_ops as storage ltree_gist;

create operator class hash_ltree_ops default for type ltree using hash family hash_ltree_ops as storage ltree;

create operator class ltree_ops default for type ltree using btree family ltree_ops as storage ltree;

