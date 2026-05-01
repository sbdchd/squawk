-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

-- size: -1, align: 4
create type public.citext;

create function public.citext(boolean) returns citext
  language internal;

create function public.citext(character) returns citext
  language internal;

create function public.citext(inet) returns citext
  language internal;

create function public.citext_cmp(citext, citext) returns integer
  language c;

create function public.citext_eq(citext, citext) returns boolean
  language c;

create function public.citext_ge(citext, citext) returns boolean
  language c;

create function public.citext_gt(citext, citext) returns boolean
  language c;

create function public.citext_hash(citext) returns integer
  language c;

create function public.citext_hash_extended(citext, bigint) returns bigint
  language c;

create function public.citext_larger(citext, citext) returns citext
  language c;

create function public.citext_le(citext, citext) returns boolean
  language c;

create function public.citext_lt(citext, citext) returns boolean
  language c;

create function public.citext_ne(citext, citext) returns boolean
  language c;

create function public.citext_pattern_cmp(citext, citext) returns integer
  language c;

create function public.citext_pattern_ge(citext, citext) returns boolean
  language c;

create function public.citext_pattern_gt(citext, citext) returns boolean
  language c;

create function public.citext_pattern_le(citext, citext) returns boolean
  language c;

create function public.citext_pattern_lt(citext, citext) returns boolean
  language c;

create function public.citext_smaller(citext, citext) returns citext
  language c;

create function public.citextin(cstring) returns citext
  language internal;

create function public.citextout(citext) returns cstring
  language internal;

create function public.citextrecv(internal) returns citext
  language internal;

create function public.citextsend(citext) returns bytea
  language internal;

create aggregate public.max(citext) (
  sfunc = citext_larger,
  stype = citext,
  combinefunc = citext_larger
);

create aggregate public.min(citext) (
  sfunc = citext_smaller,
  stype = citext,
  combinefunc = citext_smaller
);

create function public.regexp_match(string citext, pattern citext) returns text[]
  language sql;

create function public.regexp_match(string citext, pattern citext, flags text) returns text[]
  language sql;

create function public.regexp_matches(string citext, pattern citext) returns SETOF text[]
  language sql;

create function public.regexp_matches(string citext, pattern citext, flags text) returns SETOF text[]
  language sql;

create function public.regexp_replace(string citext, pattern citext, replacement text) returns text
  language sql;

create function public.regexp_replace(string citext, pattern citext, replacement text, flags text) returns text
  language sql;

create function public.regexp_split_to_array(string citext, pattern citext) returns text[]
  language sql;

create function public.regexp_split_to_array(string citext, pattern citext, flags text) returns text[]
  language sql;

create function public.regexp_split_to_table(string citext, pattern citext) returns SETOF text
  language sql;

create function public.regexp_split_to_table(string citext, pattern citext, flags text) returns SETOF text
  language sql;

create function public.replace(citext, citext, citext) returns text
  language sql;

create function public.split_part(citext, citext, integer) returns text
  language sql;

create function public.strpos(citext, citext) returns integer
  language sql;

create function public.texticlike(citext, citext) returns boolean
  language internal;

create function public.texticlike(citext, text) returns boolean
  language internal;

create function public.texticnlike(citext, citext) returns boolean
  language internal;

create function public.texticnlike(citext, text) returns boolean
  language internal;

create function public.texticregexeq(citext, citext) returns boolean
  language internal;

create function public.texticregexeq(citext, text) returns boolean
  language internal;

create function public.texticregexne(citext, citext) returns boolean
  language internal;

create function public.texticregexne(citext, text) returns boolean
  language internal;

create function public.translate(citext, citext, text) returns text
  language sql;

create operator public.!~ (
  leftarg = citext,
  rightarg = citext,
  function = public.texticregexne
);

create operator public.!~ (
  leftarg = citext,
  rightarg = text,
  function = public.texticregexne
);

create operator public.!~* (
  leftarg = citext,
  rightarg = citext,
  function = public.texticregexne
);

create operator public.!~* (
  leftarg = citext,
  rightarg = text,
  function = public.texticregexne
);

create operator public.!~~ (
  leftarg = citext,
  rightarg = citext,
  function = public.texticnlike
);

create operator public.!~~ (
  leftarg = citext,
  rightarg = text,
  function = public.texticnlike
);

create operator public.!~~* (
  leftarg = citext,
  rightarg = citext,
  function = public.texticnlike
);

create operator public.!~~* (
  leftarg = citext,
  rightarg = text,
  function = public.texticnlike
);

create operator public.< (
  leftarg = citext,
  rightarg = citext,
  function = public.citext_lt
);

create operator public.<= (
  leftarg = citext,
  rightarg = citext,
  function = public.citext_le
);

create operator public.<> (
  leftarg = citext,
  rightarg = citext,
  function = public.citext_ne
);

create operator public.= (
  leftarg = citext,
  rightarg = citext,
  function = public.citext_eq
);

create operator public.> (
  leftarg = citext,
  rightarg = citext,
  function = public.citext_gt
);

create operator public.>= (
  leftarg = citext,
  rightarg = citext,
  function = public.citext_ge
);

create operator public.~ (
  leftarg = citext,
  rightarg = citext,
  function = public.texticregexeq
);

create operator public.~ (
  leftarg = citext,
  rightarg = text,
  function = public.texticregexeq
);

create operator public.~* (
  leftarg = citext,
  rightarg = citext,
  function = public.texticregexeq
);

create operator public.~* (
  leftarg = citext,
  rightarg = text,
  function = public.texticregexeq
);

create operator public.~<=~ (
  leftarg = citext,
  rightarg = citext,
  function = public.citext_pattern_le
);

create operator public.~<~ (
  leftarg = citext,
  rightarg = citext,
  function = public.citext_pattern_lt
);

create operator public.~>=~ (
  leftarg = citext,
  rightarg = citext,
  function = public.citext_pattern_ge
);

create operator public.~>~ (
  leftarg = citext,
  rightarg = citext,
  function = public.citext_pattern_gt
);

create operator public.~~ (
  leftarg = citext,
  rightarg = citext,
  function = public.texticlike
);

create operator public.~~ (
  leftarg = citext,
  rightarg = text,
  function = public.texticlike
);

create operator public.~~* (
  leftarg = citext,
  rightarg = citext,
  function = public.texticlike
);

create operator public.~~* (
  leftarg = citext,
  rightarg = text,
  function = public.texticlike
);

