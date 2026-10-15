-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

-- size: -1, align: 4
create type citext;

create function citext(boolean) returns citext
  language internal;

create function citext(character) returns citext
  language internal;

create function citext(inet) returns citext
  language internal;

create function citext_cmp(citext, citext) returns integer
  language c;

create function citext_eq(citext, citext) returns boolean
  language c;

create function citext_ge(citext, citext) returns boolean
  language c;

create function citext_gt(citext, citext) returns boolean
  language c;

create function citext_hash(citext) returns integer
  language c;

create function citext_hash_extended(citext, bigint) returns bigint
  language c;

create function citext_larger(citext, citext) returns citext
  language c;

create function citext_le(citext, citext) returns boolean
  language c;

create function citext_lt(citext, citext) returns boolean
  language c;

create function citext_ne(citext, citext) returns boolean
  language c;

create function citext_pattern_cmp(citext, citext) returns integer
  language c;

create function citext_pattern_ge(citext, citext) returns boolean
  language c;

create function citext_pattern_gt(citext, citext) returns boolean
  language c;

create function citext_pattern_le(citext, citext) returns boolean
  language c;

create function citext_pattern_lt(citext, citext) returns boolean
  language c;

create function citext_smaller(citext, citext) returns citext
  language c;

create function citextin(cstring) returns citext
  language internal;

create function citextout(citext) returns cstring
  language internal;

create function citextrecv(internal) returns citext
  language internal;

create function citextsend(citext) returns bytea
  language internal;

create function regexp_match(string citext, pattern citext) returns text[]
  language sql;

create function regexp_match(string citext, pattern citext, flags text) returns text[]
  language sql;

create function regexp_matches(string citext, pattern citext) returns SETOF text[]
  language sql;

create function regexp_matches(string citext, pattern citext, flags text) returns SETOF text[]
  language sql;

create function regexp_replace(string citext, pattern citext, replacement text) returns text
  language sql;

create function regexp_replace(string citext, pattern citext, replacement text, flags text) returns text
  language sql;

create function regexp_split_to_array(string citext, pattern citext) returns text[]
  language sql;

create function regexp_split_to_array(string citext, pattern citext, flags text) returns text[]
  language sql;

create function regexp_split_to_table(string citext, pattern citext) returns SETOF text
  language sql;

create function regexp_split_to_table(string citext, pattern citext, flags text) returns SETOF text
  language sql;

create function replace(citext, citext, citext) returns text
  language sql;

create function split_part(citext, citext, integer) returns text
  language sql;

create function strpos(citext, citext) returns integer
  language sql;

create function texticlike(citext, citext) returns boolean
  language internal;

create function texticlike(citext, text) returns boolean
  language internal;

create function texticnlike(citext, citext) returns boolean
  language internal;

create function texticnlike(citext, text) returns boolean
  language internal;

create function texticregexeq(citext, citext) returns boolean
  language internal;

create function texticregexeq(citext, text) returns boolean
  language internal;

create function texticregexne(citext, citext) returns boolean
  language internal;

create function texticregexne(citext, text) returns boolean
  language internal;

create function translate(citext, citext, text) returns text
  language sql;

create type citext (input = citextin, output = citextout, category = 'S');

create operator !~ (
  leftarg = citext,
  rightarg = citext,
  function = texticregexne
);

create operator !~ (
  leftarg = citext,
  rightarg = text,
  function = texticregexne
);

create operator !~* (
  leftarg = citext,
  rightarg = citext,
  function = texticregexne
);

create operator !~* (
  leftarg = citext,
  rightarg = text,
  function = texticregexne
);

create operator !~~ (
  leftarg = citext,
  rightarg = citext,
  function = texticnlike
);

create operator !~~ (
  leftarg = citext,
  rightarg = text,
  function = texticnlike
);

create operator !~~* (
  leftarg = citext,
  rightarg = citext,
  function = texticnlike
);

create operator !~~* (
  leftarg = citext,
  rightarg = text,
  function = texticnlike
);

create operator < (
  leftarg = citext,
  rightarg = citext,
  function = citext_lt
);

create operator <= (
  leftarg = citext,
  rightarg = citext,
  function = citext_le
);

create operator <> (
  leftarg = citext,
  rightarg = citext,
  function = citext_ne
);

create operator = (
  leftarg = citext,
  rightarg = citext,
  function = citext_eq
);

create operator > (
  leftarg = citext,
  rightarg = citext,
  function = citext_gt
);

create operator >= (
  leftarg = citext,
  rightarg = citext,
  function = citext_ge
);

create operator ~ (
  leftarg = citext,
  rightarg = citext,
  function = texticregexeq
);

create operator ~ (
  leftarg = citext,
  rightarg = text,
  function = texticregexeq
);

create operator ~* (
  leftarg = citext,
  rightarg = citext,
  function = texticregexeq
);

create operator ~* (
  leftarg = citext,
  rightarg = text,
  function = texticregexeq
);

create operator ~<=~ (
  leftarg = citext,
  rightarg = citext,
  function = citext_pattern_le
);

create operator ~<~ (
  leftarg = citext,
  rightarg = citext,
  function = citext_pattern_lt
);

create operator ~>=~ (
  leftarg = citext,
  rightarg = citext,
  function = citext_pattern_ge
);

create operator ~>~ (
  leftarg = citext,
  rightarg = citext,
  function = citext_pattern_gt
);

create operator ~~ (
  leftarg = citext,
  rightarg = citext,
  function = texticlike
);

create operator ~~ (
  leftarg = citext,
  rightarg = text,
  function = texticlike
);

create operator ~~* (
  leftarg = citext,
  rightarg = citext,
  function = texticlike
);

create operator ~~* (
  leftarg = citext,
  rightarg = text,
  function = texticlike
);

create aggregate max(citext) (
  sfunc = citext_larger,
  stype = citext,
  combinefunc = citext_larger,
  sortop = OPERATOR(>)
);

create aggregate min(citext) (
  sfunc = citext_smaller,
  stype = citext,
  combinefunc = citext_smaller,
  sortop = OPERATOR(<)
);

create cast (boolean as citext) with function citext(boolean) as assignment;

create cast (character as citext) with function citext(character) as assignment;

create cast (character varying as citext) without function as assignment;

create cast (citext as character) without function as assignment;

create cast (citext as character varying) without function as implicit;

create cast (citext as text) without function as implicit;

create cast (inet as citext) with function citext(inet) as assignment;

create cast (text as citext) without function as assignment;

create operator family citext_ops using btree;

create operator family citext_ops using hash;

create operator family citext_pattern_ops using btree;

create operator class citext_ops default for type citext using btree family citext_ops as storage citext;

create operator class citext_ops default for type citext using hash family citext_ops as storage citext;

create operator class citext_pattern_ops for type citext using btree family citext_pattern_ops as storage citext;

