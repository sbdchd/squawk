-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

-- multi-dimensional cube '(FLOAT-1, FLOAT-2, ..., FLOAT-N), (FLOAT-1, FLOAT-2, ..., FLOAT-N)'
-- size: -1, align: 8
create type cube;

create function cube(cube, double precision) returns cube
  language c;

create function cube(cube, double precision, double precision) returns cube
  language c;

create function cube(double precision) returns cube
  language c;

create function cube(double precision, double precision) returns cube
  language c;

create function cube(double precision[]) returns cube
  language c;

create function cube(double precision[], double precision[]) returns cube
  language c;

-- btree comparison function
create function cube_cmp(cube, cube) returns integer
  language c;

-- contained in
create function cube_contained(cube, cube) returns boolean
  language c;

-- contains
create function cube_contains(cube, cube) returns boolean
  language c;

create function cube_coord(cube, integer) returns double precision
  language c;

create function cube_coord_llur(cube, integer) returns double precision
  language c;

create function cube_dim(cube) returns integer
  language c;

create function cube_distance(cube, cube) returns double precision
  language c;

create function cube_enlarge(cube, double precision, integer) returns cube
  language c;

-- same as
create function cube_eq(cube, cube) returns boolean
  language c;

-- greater than or equal to
create function cube_ge(cube, cube) returns boolean
  language c;

-- greater than
create function cube_gt(cube, cube) returns boolean
  language c;

create function cube_in(cstring) returns cube
  language c;

create function cube_inter(cube, cube) returns cube
  language c;

create function cube_is_point(cube) returns boolean
  language c;

-- lower than or equal to
create function cube_le(cube, cube) returns boolean
  language c;

create function cube_ll_coord(cube, integer) returns double precision
  language c;

-- lower than
create function cube_lt(cube, cube) returns boolean
  language c;

-- different
create function cube_ne(cube, cube) returns boolean
  language c;

create function cube_out(cube) returns cstring
  language c;

-- overlaps
create function cube_overlap(cube, cube) returns boolean
  language c;

create function cube_recv(internal) returns cube
  language c;

create function cube_send(cube) returns bytea
  language c;

create function cube_size(cube) returns double precision
  language c;

create function cube_subset(cube, integer[]) returns cube
  language c;

create function cube_union(cube, cube) returns cube
  language c;

create function cube_ur_coord(cube, integer) returns double precision
  language c;

create function distance_chebyshev(cube, cube) returns double precision
  language c;

create function distance_taxicab(cube, cube) returns double precision
  language c;

create function g_cube_consistent(internal, cube, smallint, oid, internal) returns boolean
  language c;

create function g_cube_distance(internal, cube, smallint, oid, internal) returns double precision
  language c;

create function g_cube_penalty(internal, internal, internal) returns internal
  language c;

create function g_cube_picksplit(internal, internal) returns internal
  language c;

create function g_cube_same(cube, cube, internal) returns internal
  language c;

create function g_cube_union(internal, internal) returns cube
  language c;

create type cube (input = cube_in, output = cube_out);

create operator && (
  leftarg = cube,
  rightarg = cube,
  function = cube_overlap
);

create operator -> (
  leftarg = cube,
  rightarg = integer,
  function = cube_coord
);

create operator < (
  leftarg = cube,
  rightarg = cube,
  function = cube_lt
);

create operator <#> (
  leftarg = cube,
  rightarg = cube,
  function = distance_taxicab
);

create operator <-> (
  leftarg = cube,
  rightarg = cube,
  function = cube_distance
);

create operator <= (
  leftarg = cube,
  rightarg = cube,
  function = cube_le
);

create operator <=> (
  leftarg = cube,
  rightarg = cube,
  function = distance_chebyshev
);

create operator <> (
  leftarg = cube,
  rightarg = cube,
  function = cube_ne
);

create operator <@ (
  leftarg = cube,
  rightarg = cube,
  function = cube_contained
);

create operator = (
  leftarg = cube,
  rightarg = cube,
  function = cube_eq
);

create operator > (
  leftarg = cube,
  rightarg = cube,
  function = cube_gt
);

create operator >= (
  leftarg = cube,
  rightarg = cube,
  function = cube_ge
);

create operator @> (
  leftarg = cube,
  rightarg = cube,
  function = cube_contains
);

create operator ~> (
  leftarg = cube,
  rightarg = integer,
  function = cube_coord_llur
);

create operator family cube_ops using btree;

create operator family gist_cube_ops using gist;

create operator class cube_ops default for type cube using btree family cube_ops as storage cube;

create operator class gist_cube_ops default for type cube using gist family gist_cube_ops as storage cube;

