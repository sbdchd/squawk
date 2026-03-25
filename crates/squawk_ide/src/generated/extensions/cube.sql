-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

-- multi-dimensional cube '(FLOAT-1, FLOAT-2, ..., FLOAT-N), (FLOAT-1, FLOAT-2, ..., FLOAT-N)'
-- size: -1, align: 8
create type public.cube;

create function public.cube(cube, double precision) returns cube
  language c;

create function public.cube(cube, double precision, double precision) returns cube
  language c;

create function public.cube(double precision) returns cube
  language c;

create function public.cube(double precision, double precision) returns cube
  language c;

create function public.cube(double precision[]) returns cube
  language c;

create function public.cube(double precision[], double precision[]) returns cube
  language c;

-- btree comparison function
create function public.cube_cmp(cube, cube) returns integer
  language c;

-- contained in
create function public.cube_contained(cube, cube) returns boolean
  language c;

-- contains
create function public.cube_contains(cube, cube) returns boolean
  language c;

create function public.cube_coord(cube, integer) returns double precision
  language c;

create function public.cube_coord_llur(cube, integer) returns double precision
  language c;

create function public.cube_dim(cube) returns integer
  language c;

create function public.cube_distance(cube, cube) returns double precision
  language c;

create function public.cube_enlarge(cube, double precision, integer) returns cube
  language c;

-- same as
create function public.cube_eq(cube, cube) returns boolean
  language c;

-- greater than or equal to
create function public.cube_ge(cube, cube) returns boolean
  language c;

-- greater than
create function public.cube_gt(cube, cube) returns boolean
  language c;

create function public.cube_in(cstring) returns cube
  language c;

create function public.cube_inter(cube, cube) returns cube
  language c;

create function public.cube_is_point(cube) returns boolean
  language c;

-- lower than or equal to
create function public.cube_le(cube, cube) returns boolean
  language c;

create function public.cube_ll_coord(cube, integer) returns double precision
  language c;

-- lower than
create function public.cube_lt(cube, cube) returns boolean
  language c;

-- different
create function public.cube_ne(cube, cube) returns boolean
  language c;

create function public.cube_out(cube) returns cstring
  language c;

-- overlaps
create function public.cube_overlap(cube, cube) returns boolean
  language c;

create function public.cube_recv(internal) returns cube
  language c;

create function public.cube_send(cube) returns bytea
  language c;

create function public.cube_size(cube) returns double precision
  language c;

create function public.cube_subset(cube, integer[]) returns cube
  language c;

create function public.cube_union(cube, cube) returns cube
  language c;

create function public.cube_ur_coord(cube, integer) returns double precision
  language c;

create function public.distance_chebyshev(cube, cube) returns double precision
  language c;

create function public.distance_taxicab(cube, cube) returns double precision
  language c;

create function public.g_cube_consistent(internal, cube, smallint, oid, internal) returns boolean
  language c;

create function public.g_cube_distance(internal, cube, smallint, oid, internal) returns double precision
  language c;

create function public.g_cube_penalty(internal, internal, internal) returns internal
  language c;

create function public.g_cube_picksplit(internal, internal) returns internal
  language c;

create function public.g_cube_same(cube, cube, internal) returns internal
  language c;

create function public.g_cube_union(internal, internal) returns cube
  language c;

create operator public.&& (
  leftarg = cube,
  rightarg = cube,
  function = public.cube_overlap
);

create operator public.-> (
  leftarg = cube,
  rightarg = integer,
  function = public.cube_coord
);

create operator public.< (
  leftarg = cube,
  rightarg = cube,
  function = public.cube_lt
);

create operator public.<#> (
  leftarg = cube,
  rightarg = cube,
  function = public.distance_taxicab
);

create operator public.<-> (
  leftarg = cube,
  rightarg = cube,
  function = public.cube_distance
);

create operator public.<= (
  leftarg = cube,
  rightarg = cube,
  function = public.cube_le
);

create operator public.<=> (
  leftarg = cube,
  rightarg = cube,
  function = public.distance_chebyshev
);

create operator public.<> (
  leftarg = cube,
  rightarg = cube,
  function = public.cube_ne
);

create operator public.<@ (
  leftarg = cube,
  rightarg = cube,
  function = public.cube_contained
);

create operator public.= (
  leftarg = cube,
  rightarg = cube,
  function = public.cube_eq
);

create operator public.> (
  leftarg = cube,
  rightarg = cube,
  function = public.cube_gt
);

create operator public.>= (
  leftarg = cube,
  rightarg = cube,
  function = public.cube_ge
);

create operator public.@> (
  leftarg = cube,
  rightarg = cube,
  function = public.cube_contains
);

create operator public.~> (
  leftarg = cube,
  rightarg = integer,
  function = public.cube_coord_llur
);

