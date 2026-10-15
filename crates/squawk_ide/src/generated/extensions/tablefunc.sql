-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

create type tablefunc_crosstab_2 as (
  row_name text,
  category_1 text,
  category_2 text
);

create type tablefunc_crosstab_3 as (
  row_name text,
  category_1 text,
  category_2 text,
  category_3 text
);

create type tablefunc_crosstab_4 as (
  row_name text,
  category_1 text,
  category_2 text,
  category_3 text,
  category_4 text
);

create function connectby(text, text, text, text, integer) returns SETOF record
  language c stable strict;

create function connectby(text, text, text, text, integer, text) returns SETOF record
  language c stable strict;

create function connectby(text, text, text, text, text, integer) returns SETOF record
  language c stable strict;

create function connectby(text, text, text, text, text, integer, text) returns SETOF record
  language c stable strict;

create function crosstab(text) returns SETOF record
  language c stable strict;

create function crosstab(text, integer) returns SETOF record
  language c stable strict;

create function crosstab(text, text) returns SETOF record
  language c stable strict;

create function crosstab2(text) returns SETOF tablefunc_crosstab_2
  language c stable strict;

create function crosstab3(text) returns SETOF tablefunc_crosstab_3
  language c stable strict;

create function crosstab4(text) returns SETOF tablefunc_crosstab_4
  language c stable strict;

create function normal_rand(integer, double precision, double precision) returns SETOF double precision
  language c strict;

