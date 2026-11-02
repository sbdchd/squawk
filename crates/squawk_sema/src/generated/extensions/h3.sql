-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

-- size: 8, align: 8
create type h3index;

create function __h3_cell_to_children_aux(
  index h3index,
  resolution integer,
  current integer
) returns setof h3index
  language plpgsql;

create function bigint_to_h3index(bigint) returns h3index
  language c
  immutable
  strict
  parallel safe;

-- Returns true if the given indices are neighbors.
create function h3_are_neighbor_cells(
  origin h3index,
  destination h3index
) returns boolean
  language c
  immutable
  strict
  parallel safe;

-- Exact area for a specific cell (hexagon or pentagon).
create function h3_cell_area(
  cell h3index,
  unit text default 'km^2'::text
) returns double precision
  language c
  immutable
  strict
  parallel safe;

-- Finds the boundary of the index.
--
-- Use `SET h3.extend_antimeridian TO true` to extend coordinates when crossing 180th meridian.
create function h3_cell_to_boundary(cell h3index) returns polygon
  language c
  immutable
  strict
  parallel safe;

-- DEPRECATED: Use `SET h3.extend_antimeridian TO true` instead.
create function h3_cell_to_boundary(
  cell h3index,
  extend_antimeridian boolean
) returns polygon
  language c
  immutable
  strict
  parallel safe;

-- Returns the center child (finer) index contained by input index at next resolution.
create function h3_cell_to_center_child(cell h3index) returns h3index
  language c
  immutable
  strict
  parallel safe;

-- Returns the center child (finer) index contained by input index at given resolution.
create function h3_cell_to_center_child(
  cell h3index,
  resolution integer
) returns h3index
  language c
  immutable
  strict
  parallel safe;

-- Returns the position of the child cell within an ordered list of all children of the cells parent at the specified resolution parentRes. The order of the ordered list is the same as that returned by cellToChildren. This is the complement of childPosToCell.
create function h3_cell_to_child_pos(
  child h3index,
  parentres integer
) returns bigint
  language c
  immutable
  strict
  parallel safe;

-- Returns the ordered set of children of the given index at the next resolution.
create function h3_cell_to_children(cell h3index) returns setof h3index
  language c
  immutable
  strict
  parallel safe;

-- Returns the ordered set of children of the given index at the target resolution.
create function h3_cell_to_children(
  cell h3index,
  resolution integer
) returns setof h3index
  language c
  immutable
  strict
  parallel safe;

-- Compatibility wrapper that recursively expands one resolution step at a time.
create function h3_cell_to_children_slow(index h3index) returns setof h3index
  language sql;

-- Compatibility wrapper that recursively expands one resolution step at a time.
create function h3_cell_to_children_slow(
  index h3index,
  resolution integer
) returns setof h3index
  language sql;

-- DEPRECATED: Use `h3_cell_to_latlng` instead.
create function h3_cell_to_lat_lng(cell h3index) returns point
  language c
  immutable
  strict
  parallel safe;

-- Finds the centroid of the index.
create function h3_cell_to_latlng(cell h3index) returns point
  language c
  immutable
  strict
  parallel safe;

-- Converts a cell to local IJ coordinates in the coordinate system anchored at origin.
create function h3_cell_to_local_ij(origin h3index, index h3index) returns point
  language c
  immutable
  strict
  parallel safe;

-- Returns the parent of the given index.
create function h3_cell_to_parent(cell h3index) returns h3index
  language c
  immutable
  strict
  parallel safe;

-- Returns the parent of the given index.
create function h3_cell_to_parent(
  cell h3index,
  resolution integer
) returns h3index
  language c
  immutable
  strict
  parallel safe;

-- Returns a single vertex for a given cell, as an H3 index.
create function h3_cell_to_vertex(
  cell h3index,
  vertexnum integer
) returns h3index
  language c
  immutable
  strict
  parallel safe;

-- Returns all vertexes for a given cell, as H3 indexes.
create function h3_cell_to_vertexes(cell h3index) returns setof h3index
  language c
  immutable
  strict
  parallel safe;

-- Returns a unidirectional edge H3 index based on the provided origin and destination.
create function h3_cells_to_directed_edge(
  origin h3index,
  destination h3index
) returns h3index
  language c
  immutable
  strict
  parallel safe;

-- Create a LinkedGeoPolygon describing the outline(s) of a set of hexagons. Polygon outlines will follow GeoJSON MultiPolygon order: Each polygon will have one outer loop, which is first in the list, followed by any holes.
create function h3_cells_to_multi_polygon(
  h3index[],
  out exterior polygon,
  out holes polygon[]
) returns setof record
  language c
  immutable
  strict
  parallel safe;

-- Returns the child cell at a given position within an ordered list of all children of parent at the specified resolution childRes. The order of the ordered list is the same as that returned by cellToChildren. This is the complement of cellToChildPos.
create function h3_child_pos_to_cell(
  childpos bigint,
  parent h3index,
  childres integer
) returns h3index
  language c
  immutable
  strict
  parallel safe;

-- Compacts the given array as best as possible.
create function h3_compact_cells(cells h3index[]) returns setof h3index
  language c
  immutable
  strict
  parallel safe;

-- Builds a valid H3 cell from explicit components: the target resolution, the base cell number, and a digits array ordered from resolution 1 up to the target resolution. The digits array must contain exactly one non-NULL entry per resolution step.
create function h3_construct_cell(
  resolution integer,
  base_cell_number integer,
  digits integer[]
) returns h3index
  language c
  immutable
  strict
  parallel safe;

-- Provides the coordinates defining the unidirectional edge.
create function h3_directed_edge_to_boundary(edge h3index) returns polygon
  language c
  immutable
  strict
  parallel safe;

-- Returns the pair of indices from the given edge.
create function h3_directed_edge_to_cells(
  edge h3index,
  out origin h3index,
  out destination h3index
) returns record
  language c
  immutable
  strict
  parallel safe;

-- Exact length for a specific unidirectional edge.
create function h3_edge_length(
  edge h3index,
  unit text default 'km'::text
) returns double precision
  language c
  immutable
  strict
  parallel safe;

-- Returns the base cell number (0 through 121) associated with the index.
create function h3_get_base_cell_number(h3index) returns integer
  language c
  immutable
  strict
  parallel safe;

-- Returns the destination index from the given edge.
create function h3_get_directed_edge_destination(edge h3index) returns h3index
  language c
  immutable
  strict
  parallel safe;

-- Returns the origin index from the given edge.
create function h3_get_directed_edge_origin(edge h3index) returns h3index
  language c
  immutable
  strict
  parallel safe;

-- Get the currently installed version of the extension.
create function h3_get_extension_version() returns text
  language c
  immutable
  strict
  parallel safe;

-- Average hexagon area in square (kilo)meters at the given resolution.
create function h3_get_hexagon_area_avg(
  resolution integer,
  unit text default 'km'::text
) returns double precision
  language c
  immutable
  strict
  parallel safe;

-- Average hexagon edge length in (kilo)meters at the given resolution.
create function h3_get_hexagon_edge_length_avg(
  resolution integer,
  unit text default 'km'::text
) returns double precision
  language c
  immutable
  strict
  parallel safe;

-- Returns the icosahedron face numbers intersected by the index. Some cells span more than one face.
create function h3_get_icosahedron_faces(h3index) returns integer[]
  language c
  immutable
  strict
  parallel safe;

-- Returns the index digit at a specific resolution step. Resolution numbering is 1-based: pass 1 for the first digit below the base cell, 2 for the next, and so on.
create function h3_get_index_digit(h3index, resolution integer) returns integer
  language c
  immutable
  strict
  parallel safe;

-- Number of unique H3 indexes at the given resolution.
create function h3_get_num_cells(resolution integer) returns bigint
  language c
  immutable
  strict
  parallel safe;

-- All the pentagon H3 indexes at the specified resolution.
create function h3_get_pentagons(resolution integer) returns setof h3index
  language c
  immutable
  strict
  parallel safe;

-- Returns all 122 resolution 0 indexes.
create function h3_get_res_0_cells() returns setof h3index
  language c
  immutable
  strict
  parallel safe;

-- Returns the H3 resolution encoded in the index (0 through 15).
create function h3_get_resolution(h3index) returns integer
  language c
  immutable
  strict
  parallel safe;

-- The great circle distance in radians between two spherical coordinates.
create function h3_great_circle_distance(
  a point,
  b point,
  unit text default 'km'::text
) returns double precision
  language c
  immutable
  strict
  parallel safe;

-- Preferred disk API. Returns all cells with grid distance less than or equal to k from origin, including cases near pentagons. Row order is not guaranteed.
create function h3_grid_disk(
  origin h3index,
  k integer default 1
) returns setof h3index
  language c
  immutable
  strict
  parallel safe;

-- Preferred disk API with distances. Like h3_grid_disk(), but also returns the grid distance from origin for each returned cell. Handles pentagon distortion internally. Row order is not guaranteed.
create function h3_grid_disk_distances(
  origin h3index,
  k integer default 1,
  out index h3index,
  out distance integer
) returns setof record
  language c
  immutable
  strict
  parallel safe;

-- Returns the shortest grid distance between two cells. Raises an error when the cells are not comparable, too far apart, or the path crosses pentagonal distortion.
create function h3_grid_distance(
  origin h3index,
  destination h3index
) returns bigint
  language c
  immutable
  strict
  parallel safe;

-- Returns one shortest grid path from origin to destination, including both endpoints.
--
-- This function may fail to find the line between two indexes, for
-- example if they are very far apart. It may also fail when finding
-- distances for indexes on opposite sides of a pentagon.
create function h3_grid_path_cells(
  origin h3index,
  destination h3index
) returns setof h3index
  language c
  immutable
  strict
  parallel safe;

-- Preferred ring API. Returns the cells exactly "k" grid steps from origin. Continues to work near pentagons, but row order is not guaranteed and the result may contain fewer than 6*k cells when pentagonal distortion removes positions from the ring.
create function h3_grid_ring(
  origin h3index,
  k integer default 1
) returns setof h3index
  language c
  immutable
  strict
  parallel safe;

-- Fast-path ring traversal. When it succeeds it walks the ring in traversal order, but it throws if origin or the traversed ring hits pentagonal distortion. Prefer h3_grid_ring() unless you specifically want fail-fast semantics or ring-walk ordering.
create function h3_grid_ring_unsafe(
  origin h3index,
  k integer default 1
) returns setof h3index
  language c
  immutable
  strict
  parallel safe;

-- Returns true if this index represents a pentagonal cell.
create function h3_is_pentagon(h3index) returns boolean
  language c
  immutable
  strict
  parallel safe;

-- Returns true when the index is at a Class III resolution.
create function h3_is_res_class_iii(h3index) returns boolean
  language c
  immutable
  strict
  parallel safe;

-- Returns true only for valid H3 cell indexes (hexagons or pentagons). Directed edges, vertices, and malformed values return false.
create function h3_is_valid_cell(h3index) returns boolean
  language c
  immutable
  strict
  parallel safe;

-- Returns true if the given edge is valid.
create function h3_is_valid_directed_edge(edge h3index) returns boolean
  language c
  immutable
  strict
  parallel safe;

-- Returns true for any valid H3 index mode: cell, directed edge, or vertex.
create function h3_is_valid_index(h3index) returns boolean
  language c
  immutable
  strict
  parallel safe;

-- Whether the input is a valid H3 vertex.
create function h3_is_valid_vertex(vertex h3index) returns boolean
  language c
  immutable
  strict
  parallel safe;

-- DEPRECATED: Use `h3_latlng_to_cell` instead.
create function h3_lat_lng_to_cell(
  latlng point,
  resolution integer
) returns h3index
  language c
  immutable
  strict
  parallel safe;

-- Indexes the location at the specified resolution.
create function h3_latlng_to_cell(
  latlng point,
  resolution integer
) returns h3index
  language c
  immutable
  strict
  parallel safe;

-- Converts local IJ coordinates in the coordinate system anchored at origin back to a cell.
create function h3_local_ij_to_cell(origin h3index, coord point) returns h3index
  language c
  immutable
  strict
  parallel safe;

-- Returns all unidirectional edges with the given index as origin.
create function h3_origin_to_directed_edges(h3index) returns setof h3index
  language c
  immutable
  strict
  parallel safe;

-- Migrate h3index from pass-by-reference to pass-by-value.
create function h3_pg_migrate_pass_by_reference(h3index) returns h3index
  language c
  immutable
  strict
  parallel safe;

-- Takes an exterior polygon [and a set of hole polygon] and returns the set of hexagons that best fit the structure.
create function h3_polygon_to_cells(
  exterior polygon,
  holes polygon[],
  resolution integer default 1
) returns setof h3index
  language c
  immutable
  parallel safe;

-- Takes an exterior polygon [and a set of hole polygon] and returns the set of hexagons that best fit the structure.
create function h3_polygon_to_cells_experimental(
  exterior polygon,
  holes polygon[],
  resolution integer default 1,
  containment_mode text default 'center'::text
) returns setof h3index
  language c
  immutable
  parallel safe;

-- Returns the directed edge with origin and destination cells reversed.
create function h3_reverse_directed_edge(edge h3index) returns h3index
  language c
  immutable
  strict
  parallel safe;

-- Uncompacts the given array at the resolution one higher than the highest resolution in the set.
create function h3_uncompact_cells(cells h3index[]) returns setof h3index
  language c
  immutable
  strict
  parallel safe;

-- Uncompacts the given array at the given resolution.
create function h3_uncompact_cells(
  cells h3index[],
  resolution integer
) returns setof h3index
  language c
  immutable
  strict
  parallel safe;

-- DEPRECATED: Use `h3_vertex_to_latlng` instead.
create function h3_vertex_to_lat_lng(vertex h3index) returns point
  language c
  immutable
  strict
  parallel safe;

-- Get the geocoordinates of an H3 vertex.
create function h3_vertex_to_latlng(vertex h3index) returns point
  language c
  immutable
  strict
  parallel safe;

create function h3index_cmp(h3index, h3index) returns integer
  language c
  immutable
  strict
  parallel safe;

create function h3index_contained_by(h3index, h3index) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function h3index_contains(h3index, h3index) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function h3index_distance(h3index, h3index) returns bigint
  language c
  immutable
  strict
  parallel safe;

create function h3index_eq(h3index, h3index) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function h3index_ge(h3index, h3index) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function h3index_gist_consistent(
  entry internal,
  query h3index,
  strategy smallint,
  subtype oid,
  recheck internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function h3index_gist_distance(
  entry internal,
  query h3index,
  strategy smallint,
  subtype oid,
  recheck internal
) returns double precision
  language c
  immutable
  strict
  parallel safe;

create function h3index_gist_penalty(
  internal,
  internal,
  internal
) returns internal
  language c
  immutable
  strict
  parallel safe;

create function h3index_gist_picksplit(internal, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function h3index_gist_same(h3index, h3index, internal) returns internal
  language c
  immutable
  strict
  parallel safe;

create function h3index_gist_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function h3index_gist_union(internal, internal) returns h3index
  language c
  immutable
  strict
  parallel safe;

create function h3index_gt(h3index, h3index) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function h3index_hash(h3index) returns integer
  language c
  immutable
  strict
  parallel safe;

create function h3index_hash_extended(h3index, bigint) returns bigint
  language c
  immutable
  strict
  parallel safe;

create function h3index_in(cstring) returns h3index
  language c
  immutable
  strict
  parallel safe;

create function h3index_le(h3index, h3index) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function h3index_lt(h3index, h3index) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function h3index_ne(h3index, h3index) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function h3index_out(h3index) returns cstring
  language c
  immutable
  strict
  parallel safe;

create function h3index_overlaps(h3index, h3index) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function h3index_recv(internal) returns h3index
  language c
  immutable
  strict
  parallel safe;

create function h3index_send(h3index) returns bytea
  language c
  immutable
  strict
  parallel safe;

create function h3index_sortsupport(internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function h3index_spgist_choose(internal, internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function h3index_spgist_config(internal, internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function h3index_spgist_inner_consistent(internal, internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function h3index_spgist_leaf_consistent(
  internal,
  internal
) returns boolean
  language c
  immutable
  strict
  parallel safe;

create function h3index_spgist_picksplit(internal, internal) returns void
  language c
  immutable
  strict
  parallel safe;

create function h3index_to_bigint(h3index) returns bigint
  language c
  immutable
  strict
  parallel safe;

create type h3index (
  input = h3index_in,
  output = h3index_out
);

-- Returns true if the two H3 indexes intersect.
create operator && (
  leftarg = h3index,
  rightarg = h3index,
  function = h3index_overlaps,
  commutator = &&
);

create operator < (
  leftarg = h3index,
  rightarg = h3index,
  function = h3index_lt,
  commutator = >,
  negator = >=
);

-- Returns the distance in grid cells between the two indices after refining the coarser input to its center child at the finer resolution. Returns the maximum bigint value when gridDistance fails (e.g. near pentagons).
create operator <-> (
  leftarg = h3index,
  rightarg = h3index,
  function = h3index_distance,
  commutator = <->
);

create operator <= (
  leftarg = h3index,
  rightarg = h3index,
  function = h3index_le,
  commutator = >=,
  negator = >
);

create operator <> (
  leftarg = h3index,
  rightarg = h3index,
  function = h3index_ne,
  commutator = <>,
  negator = =
);

-- Returns true if A is contained by B.
create operator <@ (
  leftarg = h3index,
  rightarg = h3index,
  function = h3index_contained_by,
  commutator = @>
);

-- Returns true if two indexes are the same.
create operator = (
  leftarg = h3index,
  rightarg = h3index,
  function = h3index_eq,
  commutator = =,
  negator = <>,
  hashes,
  merges
);

create operator > (
  leftarg = h3index,
  rightarg = h3index,
  function = h3index_gt,
  commutator = <,
  negator = <=
);

create operator >= (
  leftarg = h3index,
  rightarg = h3index,
  function = h3index_ge,
  commutator = <=,
  negator = <
);

-- Returns true if A contains B.
create operator @> (
  leftarg = h3index,
  rightarg = h3index,
  function = h3index_contains,
  commutator = <@
);

create operator family h3index_gist_ops_experimental
  using gist;

create operator family h3index_minmax_ops
  using brin;

create operator family h3index_ops
  using btree;

create operator family h3index_ops
  using hash;

create operator family h3index_ops_experimental
  using spgist;

create operator class h3index_gist_ops_experimental
  for type h3index
  using gist
  family h3index_gist_ops_experimental as
    function 1 (h3index, h3index) h3index_gist_consistent(
      internal,
      h3index,
      smallint,
      oid,
      internal
    ),
    function 2 (h3index, h3index) h3index_gist_union(internal, internal),
    function 5 (h3index, h3index) h3index_gist_penalty(
      internal,
      internal,
      internal
    ),
    function 6 (h3index, h3index) h3index_gist_picksplit(internal, internal),
    function 7 (h3index, h3index) h3index_gist_same(h3index, h3index, internal),
    storage h3index;

create operator class h3index_minmax_ops
  default for type h3index
  using brin
  family h3index_minmax_ops as
    operator 1 < (h3index, h3index),
    operator 2 <= (h3index, h3index),
    operator 3 = (h3index, h3index),
    operator 4 >= (h3index, h3index),
    operator 5 > (h3index, h3index),
    function 1 (h3index, h3index) pg_catalog.brin_minmax_opcinfo(internal),
    function 2 (h3index, h3index) pg_catalog.brin_minmax_add_value(
      internal,
      internal,
      internal,
      internal
    ),
    function 3 (h3index, h3index) pg_catalog.brin_minmax_consistent(
      internal,
      internal,
      internal
    ),
    function 4 (h3index, h3index) pg_catalog.brin_minmax_union(
      internal,
      internal,
      internal
    ),
    storage h3index;

create operator class h3index_ops
  default for type h3index
  using btree
  family h3index_ops as
    operator 1 < (h3index, h3index),
    operator 2 <= (h3index, h3index),
    operator 3 = (h3index, h3index),
    operator 4 >= (h3index, h3index),
    operator 5 > (h3index, h3index),
    function 1 (h3index, h3index) h3index_cmp(h3index, h3index),
    storage h3index;

create operator class h3index_ops
  default for type h3index
  using hash
  family h3index_ops as
    operator 1 = (h3index, h3index),
    function 1 (h3index, h3index) h3index_hash(h3index),
    storage h3index;

create operator class h3index_ops_experimental
  for type h3index
  using spgist
  family h3index_ops_experimental as
    function 1 (h3index, h3index) h3index_spgist_config(internal, internal),
    function 2 (h3index, h3index) h3index_spgist_choose(internal, internal),
    function 3 (h3index, h3index) h3index_spgist_picksplit(internal, internal),
    function 4 (h3index, h3index) h3index_spgist_inner_consistent(
      internal,
      internal
    ),
    function 5 (h3index, h3index) h3index_spgist_leaf_consistent(
      internal,
      internal
    ),
    storage h3index;

alter operator family h3index_gist_ops_experimental
  using gist
  add
    operator 3 && (h3index, h3index),
    operator 6 = (h3index, h3index),
    operator 7 @> (h3index, h3index),
    operator 8 <@ (h3index, h3index),
    operator 15 <-> (h3index, h3index) for order by pg_catalog.integer_ops,
    function 8 (h3index, h3index) h3index_gist_distance(
      internal,
      h3index,
      smallint,
      oid,
      internal
    ),
    function 11 (h3index, h3index) h3index_gist_sortsupport(internal);

alter operator family h3index_ops
  using btree
  add
    function 2 (h3index, h3index) h3index_sortsupport(internal);

alter operator family h3index_ops
  using hash
  add
    function 2 (h3index, h3index) h3index_hash_extended(h3index, bigint);

alter operator family h3index_ops_experimental
  using spgist
  add
    operator 6 = (h3index, h3index),
    operator 7 @> (h3index, h3index),
    operator 8 <@ (h3index, h3index);

create cast (bigint as h3index) with function bigint_to_h3index(bigint);

-- Convert bigint to H3 index.
create cast (h3index as bigint) with function h3index_to_bigint(h3index);

-- Convert H3 index to point.
create cast (h3index as point) with function h3_cell_to_latlng(h3index);
