-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

-- size: 8, align: 8
create type public.h3index;

create function public.__h3_cell_to_children_aux(index h3index, resolution integer, current integer) returns SETOF h3index
  language plpgsql;

create function public.bigint_to_h3index(bigint) returns h3index
  language c;

-- Returns true if the given indices are neighbors.
create function public.h3_are_neighbor_cells(origin h3index, destination h3index) returns boolean
  language c;

-- Exact area for a specific cell (hexagon or pentagon).
create function public.h3_cell_area(cell h3index, unit text DEFAULT 'km^2'::text) returns double precision
  language c;

-- Finds the boundary of the index.
-- 
-- Use `SET h3.extend_antimeridian TO true` to extend coordinates when crossing 180th meridian.
create function public.h3_cell_to_boundary(cell h3index) returns polygon
  language c;

-- DEPRECATED: Use `SET h3.extend_antimeridian TO true` instead.
create function public.h3_cell_to_boundary(cell h3index, extend_antimeridian boolean) returns polygon
  language c;

-- Returns the center child (finer) index contained by input index at next resolution.
create function public.h3_cell_to_center_child(cell h3index) returns h3index
  language c;

-- Returns the center child (finer) index contained by input index at given resolution.
create function public.h3_cell_to_center_child(cell h3index, resolution integer) returns h3index
  language c;

-- Returns the position of the child cell within an ordered list of all children of the cells parent at the specified resolution parentRes. The order of the ordered list is the same as that returned by cellToChildren. This is the complement of childPosToCell.
create function public.h3_cell_to_child_pos(child h3index, parentres integer) returns bigint
  language c;

-- Returns the set of children of the given index.
create function public.h3_cell_to_children(cell h3index) returns SETOF h3index
  language c;

-- Returns the set of children of the given index.
create function public.h3_cell_to_children(cell h3index, resolution integer) returns SETOF h3index
  language c;

-- Slower version of H3ToChildren but allocates less memory.
create function public.h3_cell_to_children_slow(index h3index) returns SETOF h3index
  language sql;

-- Slower version of H3ToChildren but allocates less memory.
create function public.h3_cell_to_children_slow(index h3index, resolution integer) returns SETOF h3index
  language sql;

-- DEPRECATED: Use `h3_cell_to_latlng` instead.
create function public.h3_cell_to_lat_lng(cell h3index) returns point
  language c;

-- Finds the centroid of the index.
create function public.h3_cell_to_latlng(cell h3index) returns point
  language c;

-- Produces local IJ coordinates for an H3 index anchored by an origin.
create function public.h3_cell_to_local_ij(origin h3index, index h3index) returns point
  language c;

-- Returns the parent of the given index.
create function public.h3_cell_to_parent(cell h3index) returns h3index
  language c;

-- Returns the parent of the given index.
create function public.h3_cell_to_parent(cell h3index, resolution integer) returns h3index
  language c;

-- Returns a single vertex for a given cell, as an H3 index.
create function public.h3_cell_to_vertex(cell h3index, vertexnum integer) returns h3index
  language c;

-- Returns all vertexes for a given cell, as H3 indexes.
create function public.h3_cell_to_vertexes(cell h3index) returns SETOF h3index
  language c;

-- Returns a unidirectional edge H3 index based on the provided origin and destination.
create function public.h3_cells_to_directed_edge(origin h3index, destination h3index) returns h3index
  language c;

-- Create a LinkedGeoPolygon describing the outline(s) of a set of hexagons. Polygon outlines will follow GeoJSON MultiPolygon order: Each polygon will have one outer loop, which is first in the list, followed by any holes.
create function public.h3_cells_to_multi_polygon(h3index[], OUT exterior polygon, OUT holes polygon[]) returns SETOF record
  language c;

-- Returns the child cell at a given position within an ordered list of all children of parent at the specified resolution childRes. The order of the ordered list is the same as that returned by cellToChildren. This is the complement of cellToChildPos.
create function public.h3_child_pos_to_cell(childpos bigint, parent h3index, childres integer) returns h3index
  language c;

-- Compacts the given array as best as possible.
create function public.h3_compact_cells(cells h3index[]) returns SETOF h3index
  language c;

-- Provides the coordinates defining the unidirectional edge.
create function public.h3_directed_edge_to_boundary(edge h3index) returns polygon
  language c;

-- Returns the pair of indices from the given edge.
create function public.h3_directed_edge_to_cells(edge h3index, OUT origin h3index, OUT destination h3index) returns record
  language c;

-- Exact length for a specific unidirectional edge.
create function public.h3_edge_length(edge h3index, unit text DEFAULT 'km'::text) returns double precision
  language c;

-- Returns the base cell number of the index.
create function public.h3_get_base_cell_number(h3index) returns integer
  language c;

-- Returns the destination index from the given edge.
create function public.h3_get_directed_edge_destination(edge h3index) returns h3index
  language c;

-- Returns the origin index from the given edge.
create function public.h3_get_directed_edge_origin(edge h3index) returns h3index
  language c;

-- Get the currently installed version of the extension.
create function public.h3_get_extension_version() returns text
  language c;

-- Average hexagon area in square (kilo)meters at the given resolution.
create function public.h3_get_hexagon_area_avg(resolution integer, unit text DEFAULT 'km'::text) returns double precision
  language c;

-- Average hexagon edge length in (kilo)meters at the given resolution.
create function public.h3_get_hexagon_edge_length_avg(resolution integer, unit text DEFAULT 'km'::text) returns double precision
  language c;

-- Find all icosahedron faces intersected by a given H3 index.
create function public.h3_get_icosahedron_faces(h3index) returns integer[]
  language c;

-- Number of unique H3 indexes at the given resolution.
create function public.h3_get_num_cells(resolution integer) returns bigint
  language c;

-- All the pentagon H3 indexes at the specified resolution.
create function public.h3_get_pentagons(resolution integer) returns SETOF h3index
  language c;

-- Returns all 122 resolution 0 indexes.
create function public.h3_get_res_0_cells() returns SETOF h3index
  language c;

-- Returns the resolution of the index.
create function public.h3_get_resolution(h3index) returns integer
  language c;

-- The great circle distance in radians between two spherical coordinates.
create function public.h3_great_circle_distance(a point, b point, unit text DEFAULT 'km'::text) returns double precision
  language c;

-- Produces indices within "k" distance of the origin index.
create function public.h3_grid_disk(origin h3index, k integer DEFAULT 1) returns SETOF h3index
  language c;

-- Produces indices within "k" distance of the origin index paired with their distance to the origin.
create function public.h3_grid_disk_distances(origin h3index, k integer DEFAULT 1, OUT index h3index, OUT distance integer) returns SETOF record
  language c;

-- Returns the distance in grid cells between the two indices.
create function public.h3_grid_distance(origin h3index, destination h3index) returns bigint
  language c;

-- Given two H3 indexes, return the line of indexes between them (inclusive).
-- 
-- This function may fail to find the line between two indexes, for
-- example if they are very far apart. It may also fail when finding
-- distances for indexes on opposite sides of a pentagon.
create function public.h3_grid_path_cells(origin h3index, destination h3index) returns SETOF h3index
  language c;

-- Returns the hollow hexagonal ring centered at origin with distance "k".
create function public.h3_grid_ring_unsafe(origin h3index, k integer DEFAULT 1) returns SETOF h3index
  language c;

-- Returns true if this index represents a pentagonal cell.
create function public.h3_is_pentagon(h3index) returns boolean
  language c;

-- Returns true if this index has a resolution with Class III orientation.
create function public.h3_is_res_class_iii(h3index) returns boolean
  language c;

-- Returns true if the given H3Index is valid.
create function public.h3_is_valid_cell(h3index) returns boolean
  language c;

-- Returns true if the given edge is valid.
create function public.h3_is_valid_directed_edge(edge h3index) returns boolean
  language c;

-- Whether the input is a valid H3 vertex.
create function public.h3_is_valid_vertex(vertex h3index) returns boolean
  language c;

-- DEPRECATED: Use `h3_latlng_to_cell` instead.
create function public.h3_lat_lng_to_cell(latlng point, resolution integer) returns h3index
  language c;

-- Indexes the location at the specified resolution.
create function public.h3_latlng_to_cell(latlng point, resolution integer) returns h3index
  language c;

-- Produces an H3 index from local IJ coordinates anchored by an origin.
create function public.h3_local_ij_to_cell(origin h3index, coord point) returns h3index
  language c;

-- Returns all unidirectional edges with the given index as origin.
create function public.h3_origin_to_directed_edges(h3index) returns SETOF h3index
  language c;

-- Migrate h3index from pass-by-reference to pass-by-value.
create function public.h3_pg_migrate_pass_by_reference(h3index) returns h3index
  language c;

-- Takes an exterior polygon [and a set of hole polygon] and returns the set of hexagons that best fit the structure.
create function public.h3_polygon_to_cells(exterior polygon, holes polygon[], resolution integer DEFAULT 1) returns SETOF h3index
  language c;

-- Takes an exterior polygon [and a set of hole polygon] and returns the set of hexagons that best fit the structure.
create function public.h3_polygon_to_cells_experimental(exterior polygon, holes polygon[], resolution integer DEFAULT 1, containment_mode text DEFAULT 'center'::text) returns SETOF h3index
  language c;

-- Uncompacts the given array at the resolution one higher than the highest resolution in the set.
create function public.h3_uncompact_cells(cells h3index[]) returns SETOF h3index
  language c;

-- Uncompacts the given array at the given resolution.
create function public.h3_uncompact_cells(cells h3index[], resolution integer) returns SETOF h3index
  language c;

-- DEPRECATED: Use `h3_vertex_to_latlng` instead.
create function public.h3_vertex_to_lat_lng(vertex h3index) returns point
  language c;

-- Get the geocoordinates of an H3 vertex.
create function public.h3_vertex_to_latlng(vertex h3index) returns point
  language c;

create function public.h3index_cmp(h3index, h3index) returns integer
  language c;

create function public.h3index_contained_by(h3index, h3index) returns boolean
  language c;

create function public.h3index_contains(h3index, h3index) returns boolean
  language c;

create function public.h3index_distance(h3index, h3index) returns bigint
  language c;

create function public.h3index_eq(h3index, h3index) returns boolean
  language c;

create function public.h3index_ge(h3index, h3index) returns boolean
  language c;

create function public.h3index_gt(h3index, h3index) returns boolean
  language c;

create function public.h3index_hash(h3index) returns integer
  language c;

create function public.h3index_hash_extended(h3index, bigint) returns bigint
  language c;

create function public.h3index_in(cstring) returns h3index
  language c;

create function public.h3index_le(h3index, h3index) returns boolean
  language c;

create function public.h3index_lt(h3index, h3index) returns boolean
  language c;

create function public.h3index_ne(h3index, h3index) returns boolean
  language c;

create function public.h3index_out(h3index) returns cstring
  language c;

create function public.h3index_overlaps(h3index, h3index) returns boolean
  language c;

create function public.h3index_recv(internal) returns h3index
  language c;

create function public.h3index_send(h3index) returns bytea
  language c;

create function public.h3index_sortsupport(internal) returns void
  language c;

create function public.h3index_spgist_choose(internal, internal) returns void
  language c;

create function public.h3index_spgist_config(internal, internal) returns void
  language c;

create function public.h3index_spgist_inner_consistent(internal, internal) returns void
  language c;

create function public.h3index_spgist_leaf_consistent(internal, internal) returns boolean
  language c;

create function public.h3index_spgist_picksplit(internal, internal) returns void
  language c;

create function public.h3index_to_bigint(h3index) returns bigint
  language c;

-- Returns true if the two H3 indexes intersect.
create operator public.&& (
  leftarg = h3index,
  rightarg = h3index,
  function = public.h3index_overlaps
);

create operator public.< (
  leftarg = h3index,
  rightarg = h3index,
  function = public.h3index_lt
);

-- Returns the distance in grid cells between the two indices (at the lowest resolution of the two).
create operator public.<-> (
  leftarg = h3index,
  rightarg = h3index,
  function = public.h3index_distance
);

create operator public.<= (
  leftarg = h3index,
  rightarg = h3index,
  function = public.h3index_le
);

create operator public.<> (
  leftarg = h3index,
  rightarg = h3index,
  function = public.h3index_ne
);

-- Returns true if A is contained by B.
create operator public.<@ (
  leftarg = h3index,
  rightarg = h3index,
  function = public.h3index_contained_by
);

-- Returns true if two indexes are the same.
create operator public.= (
  leftarg = h3index,
  rightarg = h3index,
  function = public.h3index_eq
);

create operator public.> (
  leftarg = h3index,
  rightarg = h3index,
  function = public.h3index_gt
);

create operator public.>= (
  leftarg = h3index,
  rightarg = h3index,
  function = public.h3index_ge
);

-- Returns true if A contains B.
create operator public.@> (
  leftarg = h3index,
  rightarg = h3index,
  function = public.h3index_contains
);

