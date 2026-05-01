-- squawk-ignore-file
-- pg version: 18.3
-- update via:
--   cargo xtask sync-builtins

-- size: 65, align: 4
create type public.box2d;

-- size: 16, align: 8
create type public.box2df;

-- size: 52, align: 8
create type public.box3d;

-- size: -1, align: 8
create type public.geography;

-- size: -1, align: 8
create type public.geometry;

-- size: -1, align: 8
create type public.gidx;

-- size: 65, align: 8
create type public.spheroid;

create type public.geometry_dump as (
  path integer[],
  geom geometry
);

create type public.valid_detail as (
  valid boolean,
  reason character varying,
  location geometry
);

create table public.spatial_ref_sys (
  srid integer,
  auth_name character varying(256),
  auth_srid integer,
  srtext character varying(2048),
  proj4text character varying(2048)
);

create view public.geography_columns(f_table_catalog, f_table_schema, f_table_name, f_geography_column, coord_dimension, srid, type) as
  select
    null::name,
    null::name,
    null::name,
    null::name,
    null::integer,
    null::integer,
    null::text
;

create view public.geometry_columns(f_table_catalog, f_table_schema, f_table_name, f_geometry_column, coord_dimension, srid, type) as
  select
    null::character varying(256),
    null::name,
    null::name,
    null::name,
    null::integer,
    null::integer,
    null::character varying(30)
;

create function public._postgis_deprecate(oldname text, newname text, version text) returns void
  language plpgsql;

create function public._postgis_index_extent(tbl regclass, col text) returns box2d
  language c;

create function public._postgis_join_selectivity(regclass, text, regclass, text, text DEFAULT '2'::text) returns double precision
  language c;

create function public._postgis_pgsql_version() returns text
  language sql;

create function public._postgis_scripts_pgsql_version() returns text
  language sql;

create function public._postgis_selectivity(tbl regclass, att_name text, geom geometry, mode text DEFAULT '2'::text) returns double precision
  language c;

create function public._postgis_stats(tbl regclass, att_name text, text DEFAULT '2'::text) returns text
  language c;

create function public._st_3ddfullywithin(geom1 geometry, geom2 geometry, double precision) returns boolean
  language c;

create function public._st_3ddwithin(geom1 geometry, geom2 geometry, double precision) returns boolean
  language c;

create function public._st_3dintersects(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public._st_asgml(integer, geometry, integer, integer, text, text) returns text
  language c;

create function public._st_asx3d(integer, geometry, integer, integer, text) returns text
  language c;

create function public._st_bestsrid(geography) returns integer
  language c;

create function public._st_bestsrid(geography, geography) returns integer
  language c;

create function public._st_contains(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public._st_containsproperly(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public._st_coveredby(geog1 geography, geog2 geography) returns boolean
  language c;

create function public._st_coveredby(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public._st_covers(geog1 geography, geog2 geography) returns boolean
  language c;

create function public._st_covers(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public._st_crosses(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public._st_dfullywithin(geom1 geometry, geom2 geometry, double precision) returns boolean
  language c;

create function public._st_distancetree(geography, geography) returns double precision
  language sql;

create function public._st_distancetree(geography, geography, double precision, boolean) returns double precision
  language c;

create function public._st_distanceuncached(geography, geography) returns double precision
  language sql;

create function public._st_distanceuncached(geography, geography, boolean) returns double precision
  language sql;

create function public._st_distanceuncached(geography, geography, double precision, boolean) returns double precision
  language c;

create function public._st_dwithin(geog1 geography, geog2 geography, tolerance double precision, use_spheroid boolean DEFAULT true) returns boolean
  language c;

create function public._st_dwithin(geom1 geometry, geom2 geometry, double precision) returns boolean
  language c;

create function public._st_dwithinuncached(geography, geography, double precision) returns boolean
  language sql;

create function public._st_dwithinuncached(geography, geography, double precision, boolean) returns boolean
  language c;

create function public._st_equals(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public._st_expand(geography, double precision) returns geography
  language c;

create function public._st_geomfromgml(text, integer) returns geometry
  language c;

create function public._st_intersects(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public._st_linecrossingdirection(line1 geometry, line2 geometry) returns integer
  language c;

create function public._st_longestline(geom1 geometry, geom2 geometry) returns geometry
  language c;

create function public._st_maxdistance(geom1 geometry, geom2 geometry) returns double precision
  language c;

create function public._st_orderingequals(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public._st_overlaps(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public._st_pointoutside(geography) returns geography
  language c;

create function public._st_sortablehash(geom geometry) returns bigint
  language c;

create function public._st_touches(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public._st_voronoi(g1 geometry, clip geometry DEFAULT NULL::geometry, tolerance double precision DEFAULT 0.0, return_polygons boolean DEFAULT true) returns geometry
  language c;

create function public._st_within(geom1 geometry, geom2 geometry) returns boolean
  language sql;

create function public.addgeometrycolumn(catalog_name character varying, schema_name character varying, table_name character varying, column_name character varying, new_srid_in integer, new_type character varying, new_dim integer, use_typmod boolean DEFAULT true) returns text
  language plpgsql;

create function public.addgeometrycolumn(schema_name character varying, table_name character varying, column_name character varying, new_srid integer, new_type character varying, new_dim integer, use_typmod boolean DEFAULT true) returns text
  language plpgsql;

create function public.addgeometrycolumn(table_name character varying, column_name character varying, new_srid integer, new_type character varying, new_dim integer, use_typmod boolean DEFAULT true) returns text
  language plpgsql;

create function public.box(box3d) returns box
  language c;

create function public.box(geometry) returns box
  language c;

create function public.box2d(box3d) returns box2d
  language c;

create function public.box2d(geometry) returns box2d
  language c;

create function public.box2d_in(cstring) returns box2d
  language c;

create function public.box2d_out(box2d) returns cstring
  language c;

create function public.box2df_in(cstring) returns box2df
  language c;

create function public.box2df_out(box2df) returns cstring
  language c;

create function public.box3d(box2d) returns box3d
  language c;

create function public.box3d(geometry) returns box3d
  language c;

create function public.box3d_in(cstring) returns box3d
  language c;

create function public.box3d_out(box3d) returns cstring
  language c;

create function public.box3dtobox(box3d) returns box
  language c;

create function public.bytea(geography) returns bytea
  language c;

create function public.bytea(geometry) returns bytea
  language c;

create function public.contains_2d(box2df, box2df) returns boolean
  language c;

create function public.contains_2d(box2df, geometry) returns boolean
  language c;

create function public.contains_2d(geometry, box2df) returns boolean
  language sql;

create function public.dropgeometrycolumn(catalog_name character varying, schema_name character varying, table_name character varying, column_name character varying) returns text
  language plpgsql;

create function public.dropgeometrycolumn(schema_name character varying, table_name character varying, column_name character varying) returns text
  language plpgsql;

create function public.dropgeometrycolumn(table_name character varying, column_name character varying) returns text
  language plpgsql;

create function public.dropgeometrytable(catalog_name character varying, schema_name character varying, table_name character varying) returns text
  language plpgsql;

create function public.dropgeometrytable(schema_name character varying, table_name character varying) returns text
  language sql;

create function public.dropgeometrytable(table_name character varying) returns text
  language sql;

create function public.equals(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.find_srid(character varying, character varying, character varying) returns integer
  language plpgsql;

create function public.geog_brin_inclusion_add_value(internal, internal, internal, internal) returns boolean
  language c;

create function public.geog_brin_inclusion_merge(internal, internal) returns internal
  language c;

create function public.geography(bytea) returns geography
  language c;

create function public.geography(geography, integer, boolean) returns geography
  language c;

create function public.geography(geometry) returns geography
  language c;

create function public.geography_analyze(internal) returns boolean
  language c;

create function public.geography_cmp(geography, geography) returns integer
  language c;

create function public.geography_distance_knn(geography, geography) returns double precision
  language c;

create function public.geography_eq(geography, geography) returns boolean
  language c;

create function public.geography_ge(geography, geography) returns boolean
  language c;

create function public.geography_gist_compress(internal) returns internal
  language c;

create function public.geography_gist_consistent(internal, geography, integer) returns boolean
  language c;

create function public.geography_gist_decompress(internal) returns internal
  language c;

create function public.geography_gist_distance(internal, geography, integer) returns double precision
  language c;

create function public.geography_gist_penalty(internal, internal, internal) returns internal
  language c;

create function public.geography_gist_picksplit(internal, internal) returns internal
  language c;

create function public.geography_gist_same(box2d, box2d, internal) returns internal
  language c;

create function public.geography_gist_union(bytea, internal) returns internal
  language c;

create function public.geography_gt(geography, geography) returns boolean
  language c;

create function public.geography_in(cstring, oid, integer) returns geography
  language c;

create function public.geography_le(geography, geography) returns boolean
  language c;

create function public.geography_lt(geography, geography) returns boolean
  language c;

create function public.geography_out(geography) returns cstring
  language c;

create function public.geography_overlaps(geography, geography) returns boolean
  language c;

create function public.geography_recv(internal, oid, integer) returns geography
  language c;

create function public.geography_send(geography) returns bytea
  language c;

create function public.geography_spgist_choose_nd(internal, internal) returns void
  language c;

create function public.geography_spgist_compress_nd(internal) returns internal
  language c;

create function public.geography_spgist_config_nd(internal, internal) returns void
  language c;

create function public.geography_spgist_inner_consistent_nd(internal, internal) returns void
  language c;

create function public.geography_spgist_leaf_consistent_nd(internal, internal) returns boolean
  language c;

create function public.geography_spgist_picksplit_nd(internal, internal) returns void
  language c;

create function public.geography_typmod_in(cstring[]) returns integer
  language c;

create function public.geography_typmod_out(integer) returns cstring
  language c;

create function public.geom2d_brin_inclusion_add_value(internal, internal, internal, internal) returns boolean
  language c;

create function public.geom2d_brin_inclusion_merge(internal, internal) returns internal
  language c;

create function public.geom3d_brin_inclusion_add_value(internal, internal, internal, internal) returns boolean
  language c;

create function public.geom3d_brin_inclusion_merge(internal, internal) returns internal
  language c;

create function public.geom4d_brin_inclusion_add_value(internal, internal, internal, internal) returns boolean
  language c;

create function public.geom4d_brin_inclusion_merge(internal, internal) returns internal
  language c;

create function public.geometry(box2d) returns geometry
  language c;

create function public.geometry(box3d) returns geometry
  language c;

create function public.geometry(bytea) returns geometry
  language c;

create function public.geometry(geography) returns geometry
  language c;

create function public.geometry(geometry, integer, boolean) returns geometry
  language c;

create function public.geometry(path) returns geometry
  language c;

create function public.geometry(point) returns geometry
  language c;

create function public.geometry(polygon) returns geometry
  language c;

create function public.geometry(text) returns geometry
  language c;

create function public.geometry_above(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_analyze(internal) returns boolean
  language c;

create function public.geometry_below(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_cmp(geom1 geometry, geom2 geometry) returns integer
  language c;

create function public.geometry_contained_3d(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_contains(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_contains_3d(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_contains_nd(geometry, geometry) returns boolean
  language c;

create function public.geometry_distance_box(geom1 geometry, geom2 geometry) returns double precision
  language c;

create function public.geometry_distance_centroid(geom1 geometry, geom2 geometry) returns double precision
  language c;

create function public.geometry_distance_centroid_nd(geometry, geometry) returns double precision
  language c;

create function public.geometry_distance_cpa(geometry, geometry) returns double precision
  language c;

create function public.geometry_eq(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_ge(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_gist_compress_2d(internal) returns internal
  language c;

create function public.geometry_gist_compress_nd(internal) returns internal
  language c;

create function public.geometry_gist_consistent_2d(internal, geometry, integer) returns boolean
  language c;

create function public.geometry_gist_consistent_nd(internal, geometry, integer) returns boolean
  language c;

create function public.geometry_gist_decompress_2d(internal) returns internal
  language c;

create function public.geometry_gist_decompress_nd(internal) returns internal
  language c;

create function public.geometry_gist_distance_2d(internal, geometry, integer) returns double precision
  language c;

create function public.geometry_gist_distance_nd(internal, geometry, integer) returns double precision
  language c;

create function public.geometry_gist_penalty_2d(internal, internal, internal) returns internal
  language c;

create function public.geometry_gist_penalty_nd(internal, internal, internal) returns internal
  language c;

create function public.geometry_gist_picksplit_2d(internal, internal) returns internal
  language c;

create function public.geometry_gist_picksplit_nd(internal, internal) returns internal
  language c;

create function public.geometry_gist_same_2d(geom1 geometry, geom2 geometry, internal) returns internal
  language c;

create function public.geometry_gist_same_nd(geometry, geometry, internal) returns internal
  language c;

create function public.geometry_gist_sortsupport_2d(internal) returns void
  language c;

create function public.geometry_gist_union_2d(bytea, internal) returns internal
  language c;

create function public.geometry_gist_union_nd(bytea, internal) returns internal
  language c;

create function public.geometry_gt(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_hash(geometry) returns integer
  language c;

create function public.geometry_in(cstring) returns geometry
  language c;

create function public.geometry_le(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_left(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_lt(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_neq(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_out(geometry) returns cstring
  language c;

create function public.geometry_overabove(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_overbelow(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_overlaps(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_overlaps_3d(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_overlaps_nd(geometry, geometry) returns boolean
  language c;

create function public.geometry_overleft(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_overright(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_recv(internal) returns geometry
  language c;

create function public.geometry_right(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_same(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_same_3d(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_same_nd(geometry, geometry) returns boolean
  language c;

create function public.geometry_send(geometry) returns bytea
  language c;

create function public.geometry_sortsupport(internal) returns void
  language c;

create function public.geometry_spgist_choose_2d(internal, internal) returns void
  language c;

create function public.geometry_spgist_choose_3d(internal, internal) returns void
  language c;

create function public.geometry_spgist_choose_nd(internal, internal) returns void
  language c;

create function public.geometry_spgist_compress_2d(internal) returns internal
  language c;

create function public.geometry_spgist_compress_3d(internal) returns internal
  language c;

create function public.geometry_spgist_compress_nd(internal) returns internal
  language c;

create function public.geometry_spgist_config_2d(internal, internal) returns void
  language c;

create function public.geometry_spgist_config_3d(internal, internal) returns void
  language c;

create function public.geometry_spgist_config_nd(internal, internal) returns void
  language c;

create function public.geometry_spgist_inner_consistent_2d(internal, internal) returns void
  language c;

create function public.geometry_spgist_inner_consistent_3d(internal, internal) returns void
  language c;

create function public.geometry_spgist_inner_consistent_nd(internal, internal) returns void
  language c;

create function public.geometry_spgist_leaf_consistent_2d(internal, internal) returns boolean
  language c;

create function public.geometry_spgist_leaf_consistent_3d(internal, internal) returns boolean
  language c;

create function public.geometry_spgist_leaf_consistent_nd(internal, internal) returns boolean
  language c;

create function public.geometry_spgist_picksplit_2d(internal, internal) returns void
  language c;

create function public.geometry_spgist_picksplit_3d(internal, internal) returns void
  language c;

create function public.geometry_spgist_picksplit_nd(internal, internal) returns void
  language c;

create function public.geometry_typmod_in(cstring[]) returns integer
  language c;

create function public.geometry_typmod_out(integer) returns cstring
  language c;

create function public.geometry_within(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.geometry_within_nd(geometry, geometry) returns boolean
  language c;

create function public.geometrytype(geography) returns text
  language c;

create function public.geometrytype(geometry) returns text
  language c;

create function public.geomfromewkb(bytea) returns geometry
  language c;

create function public.geomfromewkt(text) returns geometry
  language c;

create function public.get_proj4_from_srid(integer) returns text
  language plpgsql;

create function public.gidx_in(cstring) returns gidx
  language c;

create function public.gidx_out(gidx) returns cstring
  language c;

create function public.gserialized_gist_joinsel_2d(internal, oid, internal, smallint) returns double precision
  language c;

create function public.gserialized_gist_joinsel_nd(internal, oid, internal, smallint) returns double precision
  language c;

create function public.gserialized_gist_sel_2d(internal, oid, internal, integer) returns double precision
  language c;

create function public.gserialized_gist_sel_nd(internal, oid, internal, integer) returns double precision
  language c;

create function public.is_contained_2d(box2df, box2df) returns boolean
  language c;

create function public.is_contained_2d(box2df, geometry) returns boolean
  language c;

create function public.is_contained_2d(geometry, box2df) returns boolean
  language sql;

create function public.json(geometry) returns json
  language c;

create function public.jsonb(geometry) returns jsonb
  language c;

create function public.overlaps_2d(box2df, box2df) returns boolean
  language c;

create function public.overlaps_2d(box2df, geometry) returns boolean
  language c;

create function public.overlaps_2d(geometry, box2df) returns boolean
  language sql;

create function public.overlaps_geog(geography, gidx) returns boolean
  language sql;

create function public.overlaps_geog(gidx, geography) returns boolean
  language c;

create function public.overlaps_geog(gidx, gidx) returns boolean
  language c;

create function public.overlaps_nd(geometry, gidx) returns boolean
  language sql;

create function public.overlaps_nd(gidx, geometry) returns boolean
  language c;

create function public.overlaps_nd(gidx, gidx) returns boolean
  language c;

create function public.path(geometry) returns path
  language c;

create function public.pgis_asflatgeobuf_finalfn(internal) returns bytea
  language c;

create function public.pgis_asflatgeobuf_transfn(internal, anyelement) returns internal
  language c;

create function public.pgis_asflatgeobuf_transfn(internal, anyelement, boolean) returns internal
  language c;

create function public.pgis_asflatgeobuf_transfn(internal, anyelement, boolean, text) returns internal
  language c;

create function public.pgis_asgeobuf_finalfn(internal) returns bytea
  language c;

create function public.pgis_asgeobuf_transfn(internal, anyelement) returns internal
  language c;

create function public.pgis_asgeobuf_transfn(internal, anyelement, text) returns internal
  language c;

create function public.pgis_asmvt_combinefn(internal, internal) returns internal
  language c;

create function public.pgis_asmvt_deserialfn(bytea, internal) returns internal
  language c;

create function public.pgis_asmvt_finalfn(internal) returns bytea
  language c;

create function public.pgis_asmvt_serialfn(internal) returns bytea
  language c;

create function public.pgis_asmvt_transfn(internal, anyelement) returns internal
  language c;

create function public.pgis_asmvt_transfn(internal, anyelement, text) returns internal
  language c;

create function public.pgis_asmvt_transfn(internal, anyelement, text, integer) returns internal
  language c;

create function public.pgis_asmvt_transfn(internal, anyelement, text, integer, text) returns internal
  language c;

create function public.pgis_asmvt_transfn(internal, anyelement, text, integer, text, text) returns internal
  language c;

create function public.pgis_geometry_accum_transfn(internal, geometry) returns internal
  language c;

create function public.pgis_geometry_accum_transfn(internal, geometry, double precision) returns internal
  language c;

create function public.pgis_geometry_accum_transfn(internal, geometry, double precision, integer) returns internal
  language c;

create function public.pgis_geometry_clusterintersecting_finalfn(internal) returns geometry[]
  language c;

create function public.pgis_geometry_clusterwithin_finalfn(internal) returns geometry[]
  language c;

create function public.pgis_geometry_collect_finalfn(internal) returns geometry
  language c;

create function public.pgis_geometry_coverageunion_finalfn(internal) returns geometry
  language c;

create function public.pgis_geometry_makeline_finalfn(internal) returns geometry
  language c;

create function public.pgis_geometry_polygonize_finalfn(internal) returns geometry
  language c;

create function public.pgis_geometry_union_parallel_combinefn(internal, internal) returns internal
  language c;

create function public.pgis_geometry_union_parallel_deserialfn(bytea, internal) returns internal
  language c;

create function public.pgis_geometry_union_parallel_finalfn(internal) returns geometry
  language c;

create function public.pgis_geometry_union_parallel_serialfn(internal) returns bytea
  language c;

create function public.pgis_geometry_union_parallel_transfn(internal, geometry) returns internal
  language c;

create function public.pgis_geometry_union_parallel_transfn(internal, geometry, double precision) returns internal
  language c;

create function public.point(geometry) returns point
  language c;

create function public.polygon(geometry) returns polygon
  language c;

create function public.populate_geometry_columns(tbl_oid oid, use_typmod boolean DEFAULT true) returns integer
  language plpgsql;

create function public.populate_geometry_columns(use_typmod boolean DEFAULT true) returns text
  language plpgsql;

create function public.postgis_addbbox(geometry) returns geometry
  language c;

create function public.postgis_cache_bbox() returns trigger
  language c;

create function public.postgis_constraint_dims(geomschema text, geomtable text, geomcolumn text) returns integer
  language sql;

create function public.postgis_constraint_srid(geomschema text, geomtable text, geomcolumn text) returns integer
  language sql;

create function public.postgis_constraint_type(geomschema text, geomtable text, geomcolumn text) returns character varying
  language sql;

create function public.postgis_dropbbox(geometry) returns geometry
  language c;

create function public.postgis_extensions_upgrade(target_version text DEFAULT NULL::text) returns text
  language plpgsql;

create function public.postgis_full_version() returns text
  language plpgsql;

create function public.postgis_geos_compiled_version() returns text
  language c;

create function public.postgis_geos_noop(geometry) returns geometry
  language c;

create function public.postgis_geos_version() returns text
  language c;

create function public.postgis_getbbox(geometry) returns box2d
  language c;

create function public.postgis_hasbbox(geometry) returns boolean
  language c;

create function public.postgis_index_supportfn(internal) returns internal
  language c;

create function public.postgis_lib_build_date() returns text
  language c;

create function public.postgis_lib_revision() returns text
  language c;

create function public.postgis_lib_version() returns text
  language c;

create function public.postgis_libjson_version() returns text
  language c;

create function public.postgis_liblwgeom_version() returns text
  language c;

create function public.postgis_libprotobuf_version() returns text
  language c;

create function public.postgis_libxml_version() returns text
  language c;

create function public.postgis_noop(geometry) returns geometry
  language c;

create function public.postgis_proj_compiled_version() returns text
  language c;

create function public.postgis_proj_version() returns text
  language c;

create function public.postgis_scripts_build_date() returns text
  language sql;

create function public.postgis_scripts_installed() returns text
  language sql;

create function public.postgis_scripts_released() returns text
  language c;

create function public.postgis_srs(auth_name text, auth_srid text) returns TABLE(auth_name text, auth_srid text, srname text, srtext text, proj4text text, point_sw geometry, point_ne geometry)
  language c;

create function public.postgis_srs_all() returns TABLE(auth_name text, auth_srid text, srname text, srtext text, proj4text text, point_sw geometry, point_ne geometry)
  language c;

create function public.postgis_srs_codes(auth_name text) returns SETOF text
  language c;

create function public.postgis_srs_search(bounds geometry, authname text DEFAULT 'EPSG'::text) returns TABLE(auth_name text, auth_srid text, srname text, srtext text, proj4text text, point_sw geometry, point_ne geometry)
  language c;

create function public.postgis_svn_version() returns text
  language sql;

create function public.postgis_transform_geometry(geom geometry, text, text, integer) returns geometry
  language c;

create function public.postgis_transform_pipeline_geometry(geom geometry, pipeline text, forward boolean, to_srid integer) returns geometry
  language c;

create function public.postgis_type_name(geomname character varying, coord_dimension integer, use_new_name boolean DEFAULT true) returns character varying
  language sql;

create function public.postgis_typmod_dims(integer) returns integer
  language c;

create function public.postgis_typmod_srid(integer) returns integer
  language c;

create function public.postgis_typmod_type(integer) returns text
  language c;

create function public.postgis_version() returns text
  language c;

create function public.postgis_wagyu_version() returns text
  language c;

create function public.spheroid_in(cstring) returns spheroid
  language c;

create function public.spheroid_out(spheroid) returns cstring
  language c;

create function public.st_3dclosestpoint(geom1 geometry, geom2 geometry) returns geometry
  language c;

create function public.st_3ddfullywithin(geom1 geometry, geom2 geometry, double precision) returns boolean
  language c;

create function public.st_3ddistance(geom1 geometry, geom2 geometry) returns double precision
  language c;

create function public.st_3ddwithin(geom1 geometry, geom2 geometry, double precision) returns boolean
  language c;

create aggregate public.st_3dextent(geometry) (
  sfunc = public.st_combinebbox,
  stype = box3d,
  combinefunc = public.st_combinebbox
);

create function public.st_3dintersects(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.st_3dlength(geometry) returns double precision
  language c;

create function public.st_3dlineinterpolatepoint(geometry, double precision) returns geometry
  language c;

create function public.st_3dlongestline(geom1 geometry, geom2 geometry) returns geometry
  language c;

create function public.st_3dmakebox(geom1 geometry, geom2 geometry) returns box3d
  language c;

create function public.st_3dmaxdistance(geom1 geometry, geom2 geometry) returns double precision
  language c;

create function public.st_3dperimeter(geometry) returns double precision
  language c;

create function public.st_3dshortestline(geom1 geometry, geom2 geometry) returns geometry
  language c;

create function public.st_addmeasure(geometry, double precision, double precision) returns geometry
  language c;

create function public.st_addpoint(geom1 geometry, geom2 geometry) returns geometry
  language c;

create function public.st_addpoint(geom1 geometry, geom2 geometry, integer) returns geometry
  language c;

create function public.st_affine(geometry, double precision, double precision, double precision, double precision, double precision, double precision) returns geometry
  language sql;

create function public.st_affine(geometry, double precision, double precision, double precision, double precision, double precision, double precision, double precision, double precision, double precision, double precision, double precision, double precision) returns geometry
  language c;

create function public.st_angle(line1 geometry, line2 geometry) returns double precision
  language sql;

create function public.st_angle(pt1 geometry, pt2 geometry, pt3 geometry, pt4 geometry DEFAULT '0101000000000000000000F87F000000000000F87F'::geometry) returns double precision
  language c;

create function public.st_area(geog geography, use_spheroid boolean DEFAULT true) returns double precision
  language c;

create function public.st_area(geometry) returns double precision
  language c;

create function public.st_area(text) returns double precision
  language sql;

create function public.st_area2d(geometry) returns double precision
  language c;

create function public.st_asbinary(geography) returns bytea
  language c;

create function public.st_asbinary(geography, text) returns bytea
  language c;

create function public.st_asbinary(geometry) returns bytea
  language c;

create function public.st_asbinary(geometry, text) returns bytea
  language c;

create function public.st_asencodedpolyline(geom geometry, nprecision integer DEFAULT 5) returns text
  language c;

create function public.st_asewkb(geometry) returns bytea
  language c;

create function public.st_asewkb(geometry, text) returns bytea
  language c;

create function public.st_asewkt(geography) returns text
  language c;

create function public.st_asewkt(geography, integer) returns text
  language c;

create function public.st_asewkt(geometry) returns text
  language c;

create function public.st_asewkt(geometry, integer) returns text
  language c;

create function public.st_asewkt(text) returns text
  language sql;

create aggregate public.st_asflatgeobuf(anyelement) (
  sfunc = public.pgis_asflatgeobuf_transfn,
  stype = internal,
  finalfunc = pgis_asflatgeobuf_finalfn
);

create aggregate public.st_asflatgeobuf(anyelement, boolean) (
  sfunc = public.pgis_asflatgeobuf_transfn,
  stype = internal,
  finalfunc = pgis_asflatgeobuf_finalfn
);

create aggregate public.st_asflatgeobuf(anyelement, boolean, text) (
  sfunc = public.pgis_asflatgeobuf_transfn,
  stype = internal,
  finalfunc = pgis_asflatgeobuf_finalfn
);

create aggregate public.st_asgeobuf(anyelement) (
  sfunc = public.pgis_asgeobuf_transfn,
  stype = internal,
  finalfunc = pgis_asgeobuf_finalfn
);

create aggregate public.st_asgeobuf(anyelement, text) (
  sfunc = public.pgis_asgeobuf_transfn,
  stype = internal,
  finalfunc = pgis_asgeobuf_finalfn
);

create function public.st_asgeojson(geog geography, maxdecimaldigits integer DEFAULT 9, options integer DEFAULT 0) returns text
  language c;

create function public.st_asgeojson(geom geometry, maxdecimaldigits integer DEFAULT 9, options integer DEFAULT 8) returns text
  language c;

create function public.st_asgeojson(r record, geom_column text DEFAULT ''::text, maxdecimaldigits integer DEFAULT 9, pretty_bool boolean DEFAULT false, id_column text DEFAULT ''::text) returns text
  language c;

create function public.st_asgeojson(text) returns text
  language sql;

create function public.st_asgml(geog geography, maxdecimaldigits integer DEFAULT 15, options integer DEFAULT 0, nprefix text DEFAULT 'gml'::text, id text DEFAULT ''::text) returns text
  language c;

create function public.st_asgml(geom geometry, maxdecimaldigits integer DEFAULT 15, options integer DEFAULT 0) returns text
  language c;

create function public.st_asgml(text) returns text
  language sql;

create function public.st_asgml(version integer, geog geography, maxdecimaldigits integer DEFAULT 15, options integer DEFAULT 0, nprefix text DEFAULT 'gml'::text, id text DEFAULT ''::text) returns text
  language c;

create function public.st_asgml(version integer, geom geometry, maxdecimaldigits integer DEFAULT 15, options integer DEFAULT 0, nprefix text DEFAULT NULL::text, id text DEFAULT NULL::text) returns text
  language c;

create function public.st_ashexewkb(geometry) returns text
  language c;

create function public.st_ashexewkb(geometry, text) returns text
  language c;

create function public.st_askml(geog geography, maxdecimaldigits integer DEFAULT 15, nprefix text DEFAULT ''::text) returns text
  language c;

create function public.st_askml(geom geometry, maxdecimaldigits integer DEFAULT 15, nprefix text DEFAULT ''::text) returns text
  language c;

create function public.st_askml(text) returns text
  language sql;

create function public.st_aslatlontext(geom geometry, tmpl text DEFAULT ''::text) returns text
  language c;

create function public.st_asmarc21(geom geometry, format text DEFAULT 'hdddmmss'::text) returns text
  language c;

create aggregate public.st_asmvt(anyelement) (
  sfunc = public.pgis_asmvt_transfn,
  stype = internal,
  finalfunc = pgis_asmvt_finalfn,
  combinefunc = pgis_asmvt_combinefn
);

create aggregate public.st_asmvt(anyelement, text) (
  sfunc = public.pgis_asmvt_transfn,
  stype = internal,
  finalfunc = pgis_asmvt_finalfn,
  combinefunc = pgis_asmvt_combinefn
);

create aggregate public.st_asmvt(anyelement, text, integer) (
  sfunc = public.pgis_asmvt_transfn,
  stype = internal,
  finalfunc = pgis_asmvt_finalfn,
  combinefunc = pgis_asmvt_combinefn
);

create aggregate public.st_asmvt(anyelement, text, integer, text) (
  sfunc = public.pgis_asmvt_transfn,
  stype = internal,
  finalfunc = pgis_asmvt_finalfn,
  combinefunc = pgis_asmvt_combinefn
);

create aggregate public.st_asmvt(anyelement, text, integer, text, text) (
  sfunc = public.pgis_asmvt_transfn,
  stype = internal,
  finalfunc = pgis_asmvt_finalfn,
  combinefunc = pgis_asmvt_combinefn
);

create function public.st_asmvtgeom(geom geometry, bounds box2d, extent integer DEFAULT 4096, buffer integer DEFAULT 256, clip_geom boolean DEFAULT true) returns geometry
  language c;

create function public.st_assvg(geog geography, rel integer DEFAULT 0, maxdecimaldigits integer DEFAULT 15) returns text
  language c;

create function public.st_assvg(geom geometry, rel integer DEFAULT 0, maxdecimaldigits integer DEFAULT 15) returns text
  language c;

create function public.st_assvg(text) returns text
  language sql;

create function public.st_astext(geography) returns text
  language c;

create function public.st_astext(geography, integer) returns text
  language c;

create function public.st_astext(geometry) returns text
  language c;

create function public.st_astext(geometry, integer) returns text
  language c;

create function public.st_astext(text) returns text
  language sql;

create function public.st_astwkb(geom geometry, prec integer DEFAULT NULL::integer, prec_z integer DEFAULT NULL::integer, prec_m integer DEFAULT NULL::integer, with_sizes boolean DEFAULT NULL::boolean, with_boxes boolean DEFAULT NULL::boolean) returns bytea
  language c;

create function public.st_astwkb(geom geometry[], ids bigint[], prec integer DEFAULT NULL::integer, prec_z integer DEFAULT NULL::integer, prec_m integer DEFAULT NULL::integer, with_sizes boolean DEFAULT NULL::boolean, with_boxes boolean DEFAULT NULL::boolean) returns bytea
  language c;

create function public.st_asx3d(geom geometry, maxdecimaldigits integer DEFAULT 15, options integer DEFAULT 0) returns text
  language sql;

create function public.st_azimuth(geog1 geography, geog2 geography) returns double precision
  language c;

create function public.st_azimuth(geom1 geometry, geom2 geometry) returns double precision
  language c;

create function public.st_bdmpolyfromtext(text, integer) returns geometry
  language plpgsql;

create function public.st_bdpolyfromtext(text, integer) returns geometry
  language plpgsql;

create function public.st_boundary(geometry) returns geometry
  language c;

create function public.st_boundingdiagonal(geom geometry, fits boolean DEFAULT false) returns geometry
  language c;

create function public.st_box2dfromgeohash(text, integer DEFAULT NULL::integer) returns box2d
  language c;

create function public.st_buffer(geography, double precision) returns geography
  language sql;

create function public.st_buffer(geography, double precision, integer) returns geography
  language sql;

create function public.st_buffer(geography, double precision, text) returns geography
  language sql;

create function public.st_buffer(geom geometry, radius double precision, options text DEFAULT ''::text) returns geometry
  language c;

create function public.st_buffer(geom geometry, radius double precision, quadsegs integer) returns geometry
  language sql;

create function public.st_buffer(text, double precision) returns geometry
  language sql;

create function public.st_buffer(text, double precision, integer) returns geometry
  language sql;

create function public.st_buffer(text, double precision, text) returns geometry
  language sql;

create function public.st_buildarea(geometry) returns geometry
  language c;

create function public.st_centroid(geography, use_spheroid boolean DEFAULT true) returns geography
  language c;

create function public.st_centroid(geometry) returns geometry
  language c;

create function public.st_centroid(text) returns geometry
  language sql;

create function public.st_chaikinsmoothing(geometry, integer DEFAULT 1, boolean DEFAULT false) returns geometry
  language c;

create function public.st_cleangeometry(geometry) returns geometry
  language c;

create function public.st_clipbybox2d(geom geometry, box box2d) returns geometry
  language c;

create function public.st_closestpoint(geography, geography, use_spheroid boolean DEFAULT true) returns geography
  language c;

create function public.st_closestpoint(geom1 geometry, geom2 geometry) returns geometry
  language c;

create function public.st_closestpoint(text, text) returns geometry
  language sql;

create function public.st_closestpointofapproach(geometry, geometry) returns double precision
  language c;

create function public.st_clusterdbscan(geometry, eps double precision, minpoints integer) returns integer
  language c;

create aggregate public.st_clusterintersecting(geometry) (
  sfunc = public.pgis_geometry_accum_transfn,
  stype = internal,
  finalfunc = pgis_geometry_clusterintersecting_finalfn
);

create function public.st_clusterintersecting(geometry[]) returns geometry[]
  language c;

create function public.st_clusterintersectingwin(geometry) returns integer
  language c;

create function public.st_clusterkmeans(geom geometry, k integer, max_radius double precision DEFAULT NULL::double precision) returns integer
  language c;

create aggregate public.st_clusterwithin(geometry, double precision) (
  sfunc = public.pgis_geometry_accum_transfn,
  stype = internal,
  finalfunc = pgis_geometry_clusterwithin_finalfn
);

create function public.st_clusterwithin(geometry[], double precision) returns geometry[]
  language c;

create function public.st_clusterwithinwin(geometry, distance double precision) returns integer
  language c;

create function public.st_collect(geom1 geometry, geom2 geometry) returns geometry
  language c;

create aggregate public.st_collect(geometry) (
  sfunc = public.pgis_geometry_accum_transfn,
  stype = internal,
  finalfunc = pgis_geometry_collect_finalfn
);

create function public.st_collect(geometry[]) returns geometry
  language c;

create function public.st_collectionextract(geometry) returns geometry
  language c;

create function public.st_collectionextract(geometry, integer) returns geometry
  language c;

create function public.st_collectionhomogenize(geometry) returns geometry
  language c;

create function public.st_combinebbox(box2d, geometry) returns box2d
  language c;

create function public.st_combinebbox(box3d, box3d) returns box3d
  language c;

create function public.st_combinebbox(box3d, geometry) returns box3d
  language c;

create function public.st_concavehull(param_geom geometry, param_pctconvex double precision, param_allow_holes boolean DEFAULT false) returns geometry
  language c;

create function public.st_contains(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.st_containsproperly(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.st_convexhull(geometry) returns geometry
  language c;

create function public.st_coorddim(geometry geometry) returns smallint
  language c;

create function public.st_coverageclean(geom geometry, gapmaximumwidth double precision DEFAULT 0.0, snappingdistance double precision DEFAULT '-1.0'::numeric, overlapmergestrategy text DEFAULT 'MERGE_LONGEST_BORDER'::text) returns geometry
  language c;

create function public.st_coverageinvalidedges(geom geometry, tolerance double precision DEFAULT 0.0) returns geometry
  language c;

create function public.st_coveragesimplify(geom geometry, tolerance double precision, simplifyboundary boolean DEFAULT true) returns geometry
  language c;

create aggregate public.st_coverageunion(geometry) (
  sfunc = public.pgis_geometry_accum_transfn,
  stype = internal,
  finalfunc = pgis_geometry_coverageunion_finalfn
);

create function public.st_coverageunion(geometry[]) returns geometry
  language c;

create function public.st_coveredby(geog1 geography, geog2 geography) returns boolean
  language c;

create function public.st_coveredby(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.st_coveredby(text, text) returns boolean
  language sql;

create function public.st_covers(geog1 geography, geog2 geography) returns boolean
  language c;

create function public.st_covers(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.st_covers(text, text) returns boolean
  language sql;

create function public.st_cpawithin(geometry, geometry, double precision) returns boolean
  language c;

create function public.st_crosses(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.st_curven(geometry geometry, i integer) returns geometry
  language c;

create function public.st_curvetoline(geom geometry, tol double precision DEFAULT 32, toltype integer DEFAULT 0, flags integer DEFAULT 0) returns geometry
  language c;

create function public.st_delaunaytriangles(g1 geometry, tolerance double precision DEFAULT 0.0, flags integer DEFAULT 0) returns geometry
  language c;

create function public.st_dfullywithin(geom1 geometry, geom2 geometry, double precision) returns boolean
  language c;

create function public.st_difference(geom1 geometry, geom2 geometry, gridsize double precision DEFAULT '-1.0'::numeric) returns geometry
  language c;

create function public.st_dimension(geometry) returns integer
  language c;

create function public.st_disjoint(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.st_distance(geog1 geography, geog2 geography, use_spheroid boolean DEFAULT true) returns double precision
  language c;

create function public.st_distance(geom1 geometry, geom2 geometry) returns double precision
  language c;

create function public.st_distance(text, text) returns double precision
  language sql;

create function public.st_distancecpa(geometry, geometry) returns double precision
  language c;

create function public.st_distancesphere(geom1 geometry, geom2 geometry) returns double precision
  language sql;

create function public.st_distancesphere(geom1 geometry, geom2 geometry, radius double precision) returns double precision
  language c;

create function public.st_distancespheroid(geom1 geometry, geom2 geometry) returns double precision
  language c;

create function public.st_distancespheroid(geom1 geometry, geom2 geometry, spheroid) returns double precision
  language c;

create function public.st_dump(geometry) returns SETOF geometry_dump
  language c;

create function public.st_dumppoints(geometry) returns SETOF geometry_dump
  language c;

create function public.st_dumprings(geometry) returns SETOF geometry_dump
  language c;

create function public.st_dumpsegments(geometry) returns SETOF geometry_dump
  language c;

create function public.st_dwithin(geog1 geography, geog2 geography, tolerance double precision, use_spheroid boolean DEFAULT true) returns boolean
  language c;

create function public.st_dwithin(geom1 geometry, geom2 geometry, double precision) returns boolean
  language c;

create function public.st_dwithin(text, text, double precision) returns boolean
  language sql;

create function public.st_endpoint(geometry) returns geometry
  language c;

create function public.st_envelope(geometry) returns geometry
  language c;

create function public.st_equals(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.st_estimatedextent(text, text) returns box2d
  language c;

create function public.st_estimatedextent(text, text, text) returns box2d
  language c;

create function public.st_estimatedextent(text, text, text, boolean) returns box2d
  language c;

create function public.st_expand(box box2d, dx double precision, dy double precision) returns box2d
  language c;

create function public.st_expand(box box3d, dx double precision, dy double precision, dz double precision DEFAULT 0) returns box3d
  language c;

create function public.st_expand(box2d, double precision) returns box2d
  language c;

create function public.st_expand(box3d, double precision) returns box3d
  language c;

create function public.st_expand(geom geometry, dx double precision, dy double precision, dz double precision DEFAULT 0, dm double precision DEFAULT 0) returns geometry
  language c;

create function public.st_expand(geometry, double precision) returns geometry
  language c;

create aggregate public.st_extent(geometry) (
  sfunc = public.st_combinebbox,
  stype = box3d,
  finalfunc = public.box2d,
  combinefunc = public.st_combinebbox
);

create function public.st_exteriorring(geometry) returns geometry
  language c;

create function public.st_filterbym(geometry, double precision, double precision DEFAULT NULL::double precision, boolean DEFAULT false) returns geometry
  language c;

create function public.st_findextent(text, text) returns box2d
  language plpgsql;

create function public.st_findextent(text, text, text) returns box2d
  language plpgsql;

create function public.st_flipcoordinates(geometry) returns geometry
  language c;

create function public.st_force2d(geometry) returns geometry
  language c;

create function public.st_force3d(geom geometry, zvalue double precision DEFAULT 0.0) returns geometry
  language sql;

create function public.st_force3dm(geom geometry, mvalue double precision DEFAULT 0.0) returns geometry
  language c;

create function public.st_force3dz(geom geometry, zvalue double precision DEFAULT 0.0) returns geometry
  language c;

create function public.st_force4d(geom geometry, zvalue double precision DEFAULT 0.0, mvalue double precision DEFAULT 0.0) returns geometry
  language c;

create function public.st_forcecollection(geometry) returns geometry
  language c;

create function public.st_forcecurve(geometry) returns geometry
  language c;

create function public.st_forcepolygonccw(geometry) returns geometry
  language sql;

create function public.st_forcepolygoncw(geometry) returns geometry
  language c;

create function public.st_forcerhr(geometry) returns geometry
  language c;

create function public.st_forcesfs(geometry) returns geometry
  language c;

create function public.st_forcesfs(geometry, version text) returns geometry
  language c;

create function public.st_frechetdistance(geom1 geometry, geom2 geometry, double precision DEFAULT '-1'::integer) returns double precision
  language c;

create function public.st_fromflatgeobuf(anyelement, bytea) returns SETOF anyelement
  language c;

create function public.st_fromflatgeobuftotable(text, text, bytea) returns void
  language c;

create function public.st_generatepoints(area geometry, npoints integer) returns geometry
  language c;

create function public.st_generatepoints(area geometry, npoints integer, seed integer) returns geometry
  language c;

create function public.st_geogfromtext(text) returns geography
  language c;

create function public.st_geogfromwkb(bytea) returns geography
  language c;

create function public.st_geographyfromtext(text) returns geography
  language c;

create function public.st_geohash(geog geography, maxchars integer DEFAULT 0) returns text
  language c;

create function public.st_geohash(geom geometry, maxchars integer DEFAULT 0) returns text
  language c;

create function public.st_geomcollfromtext(text) returns geometry
  language sql;

create function public.st_geomcollfromtext(text, integer) returns geometry
  language sql;

create function public.st_geomcollfromwkb(bytea) returns geometry
  language sql;

create function public.st_geomcollfromwkb(bytea, integer) returns geometry
  language sql;

create function public.st_geometricmedian(g geometry, tolerance double precision DEFAULT NULL::double precision, max_iter integer DEFAULT 10000, fail_if_not_converged boolean DEFAULT false) returns geometry
  language c;

create function public.st_geometryfromtext(text) returns geometry
  language c;

create function public.st_geometryfromtext(text, integer) returns geometry
  language c;

create function public.st_geometryn(geometry, integer) returns geometry
  language c;

create function public.st_geometrytype(geometry) returns text
  language c;

create function public.st_geomfromewkb(bytea) returns geometry
  language c;

create function public.st_geomfromewkt(text) returns geometry
  language c;

create function public.st_geomfromgeohash(text, integer DEFAULT NULL::integer) returns geometry
  language sql;

create function public.st_geomfromgeojson(json) returns geometry
  language sql;

create function public.st_geomfromgeojson(jsonb) returns geometry
  language sql;

create function public.st_geomfromgeojson(text) returns geometry
  language c;

create function public.st_geomfromgml(text) returns geometry
  language sql;

create function public.st_geomfromgml(text, integer) returns geometry
  language c;

create function public.st_geomfromkml(text) returns geometry
  language c;

create function public.st_geomfrommarc21(marc21xml text) returns geometry
  language c;

create function public.st_geomfromtext(text) returns geometry
  language c;

create function public.st_geomfromtext(text, integer) returns geometry
  language c;

create function public.st_geomfromtwkb(bytea) returns geometry
  language c;

create function public.st_geomfromwkb(bytea) returns geometry
  language c;

create function public.st_geomfromwkb(bytea, integer) returns geometry
  language sql;

create function public.st_gmltosql(text) returns geometry
  language sql;

create function public.st_gmltosql(text, integer) returns geometry
  language c;

create function public.st_hasarc(geometry geometry) returns boolean
  language c;

create function public.st_hasm(geometry) returns boolean
  language c;

create function public.st_hasz(geometry) returns boolean
  language c;

create function public.st_hausdorffdistance(geom1 geometry, geom2 geometry) returns double precision
  language c;

create function public.st_hausdorffdistance(geom1 geometry, geom2 geometry, double precision) returns double precision
  language c;

create function public.st_hexagon(size double precision, cell_i integer, cell_j integer, origin geometry DEFAULT '010100000000000000000000000000000000000000'::geometry) returns geometry
  language c;

create function public.st_hexagongrid(size double precision, bounds geometry, OUT geom geometry, OUT i integer, OUT j integer) returns SETOF record
  language c;

create function public.st_interiorringn(geometry, integer) returns geometry
  language c;

create function public.st_interpolatepoint(line geometry, point geometry) returns double precision
  language c;

create function public.st_intersection(geography, geography) returns geography
  language sql;

create function public.st_intersection(geom1 geometry, geom2 geometry, gridsize double precision DEFAULT '-1'::integer) returns geometry
  language c;

create function public.st_intersection(text, text) returns geometry
  language sql;

create function public.st_intersects(geog1 geography, geog2 geography) returns boolean
  language c;

create function public.st_intersects(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.st_intersects(text, text) returns boolean
  language sql;

create function public.st_inversetransformpipeline(geom geometry, pipeline text, to_srid integer DEFAULT 0) returns geometry
  language sql;

create function public.st_isclosed(geometry) returns boolean
  language c;

create function public.st_iscollection(geometry) returns boolean
  language c;

create function public.st_isempty(geometry) returns boolean
  language c;

create function public.st_ispolygonccw(geometry) returns boolean
  language c;

create function public.st_ispolygoncw(geometry) returns boolean
  language c;

create function public.st_isring(geometry) returns boolean
  language c;

create function public.st_issimple(geometry) returns boolean
  language c;

create function public.st_isvalid(geometry) returns boolean
  language c;

create function public.st_isvalid(geometry, integer) returns boolean
  language sql;

create function public.st_isvaliddetail(geom geometry, flags integer DEFAULT 0) returns valid_detail
  language c;

create function public.st_isvalidreason(geometry) returns text
  language c;

create function public.st_isvalidreason(geometry, integer) returns text
  language sql;

create function public.st_isvalidtrajectory(geometry) returns boolean
  language c;

create function public.st_largestemptycircle(geom geometry, tolerance double precision DEFAULT 0.0, boundary geometry DEFAULT '0101000000000000000000F87F000000000000F87F'::geometry, OUT center geometry, OUT nearest geometry, OUT radius double precision) returns record
  language c;

create function public.st_length(geog geography, use_spheroid boolean DEFAULT true) returns double precision
  language c;

create function public.st_length(geometry) returns double precision
  language c;

create function public.st_length(text) returns double precision
  language sql;

create function public.st_length2d(geometry) returns double precision
  language c;

create function public.st_length2dspheroid(geometry, spheroid) returns double precision
  language c;

create function public.st_lengthspheroid(geometry, spheroid) returns double precision
  language c;

create function public.st_letters(letters text, font json DEFAULT NULL::json) returns geometry
  language plpgsql;

create function public.st_linecrossingdirection(line1 geometry, line2 geometry) returns integer
  language c;

create function public.st_lineextend(geom geometry, distance_forward double precision, distance_backward double precision DEFAULT 0.0) returns geometry
  language c;

create function public.st_linefromencodedpolyline(txtin text, nprecision integer DEFAULT 5) returns geometry
  language c;

create function public.st_linefrommultipoint(geometry) returns geometry
  language c;

create function public.st_linefromtext(text) returns geometry
  language sql;

create function public.st_linefromtext(text, integer) returns geometry
  language sql;

create function public.st_linefromwkb(bytea) returns geometry
  language sql;

create function public.st_linefromwkb(bytea, integer) returns geometry
  language sql;

create function public.st_lineinterpolatepoint(geography, double precision, use_spheroid boolean DEFAULT true) returns geography
  language c;

create function public.st_lineinterpolatepoint(geometry, double precision) returns geometry
  language c;

create function public.st_lineinterpolatepoint(text, double precision) returns geometry
  language sql;

create function public.st_lineinterpolatepoints(geography, double precision, use_spheroid boolean DEFAULT true, repeat boolean DEFAULT true) returns geography
  language c;

create function public.st_lineinterpolatepoints(geometry, double precision, repeat boolean DEFAULT true) returns geometry
  language c;

create function public.st_lineinterpolatepoints(text, double precision) returns geometry
  language sql;

create function public.st_linelocatepoint(geography, geography, use_spheroid boolean DEFAULT true) returns double precision
  language c;

create function public.st_linelocatepoint(geom1 geometry, geom2 geometry) returns double precision
  language c;

create function public.st_linelocatepoint(text, text) returns double precision
  language sql;

create function public.st_linemerge(geometry) returns geometry
  language c;

create function public.st_linemerge(geometry, boolean) returns geometry
  language c;

create function public.st_linestringfromwkb(bytea) returns geometry
  language sql;

create function public.st_linestringfromwkb(bytea, integer) returns geometry
  language sql;

create function public.st_linesubstring(geography, double precision, double precision) returns geography
  language c;

create function public.st_linesubstring(geometry, double precision, double precision) returns geometry
  language c;

create function public.st_linesubstring(text, double precision, double precision) returns geometry
  language sql;

create function public.st_linetocurve(geometry geometry) returns geometry
  language c;

create function public.st_locatealong(geometry geometry, measure double precision, leftrightoffset double precision DEFAULT 0.0) returns geometry
  language c;

create function public.st_locatebetween(geometry geometry, frommeasure double precision, tomeasure double precision, leftrightoffset double precision DEFAULT 0.0) returns geometry
  language c;

create function public.st_locatebetweenelevations(geometry geometry, fromelevation double precision, toelevation double precision) returns geometry
  language c;

create function public.st_longestline(geom1 geometry, geom2 geometry) returns geometry
  language sql;

create function public.st_m(geometry) returns double precision
  language c;

create function public.st_makebox2d(geom1 geometry, geom2 geometry) returns box2d
  language c;

create function public.st_makeenvelope(double precision, double precision, double precision, double precision, integer DEFAULT 0) returns geometry
  language c;

create function public.st_makeline(geom1 geometry, geom2 geometry) returns geometry
  language c;

create aggregate public.st_makeline(geometry) (
  sfunc = public.pgis_geometry_accum_transfn,
  stype = internal,
  finalfunc = pgis_geometry_makeline_finalfn
);

create function public.st_makeline(geometry[]) returns geometry
  language c;

create function public.st_makepoint(double precision, double precision) returns geometry
  language c;

create function public.st_makepoint(double precision, double precision, double precision) returns geometry
  language c;

create function public.st_makepoint(double precision, double precision, double precision, double precision) returns geometry
  language c;

create function public.st_makepointm(double precision, double precision, double precision) returns geometry
  language c;

create function public.st_makepolygon(geometry) returns geometry
  language c;

create function public.st_makepolygon(geometry, geometry[]) returns geometry
  language c;

create function public.st_makevalid(geom geometry, params text) returns geometry
  language c;

create function public.st_makevalid(geometry) returns geometry
  language c;

create function public.st_maxdistance(geom1 geometry, geom2 geometry) returns double precision
  language sql;

create function public.st_maximuminscribedcircle(geometry, OUT center geometry, OUT nearest geometry, OUT radius double precision) returns record
  language c;

create aggregate public.st_memcollect(geometry) (
  sfunc = public.st_collect,
  stype = geometry,
  combinefunc = public.st_collect
);

create function public.st_memsize(geometry) returns integer
  language c;

create aggregate public.st_memunion(geometry) (
  sfunc = public.st_union,
  stype = geometry,
  combinefunc = public.st_union
);

create function public.st_minimumboundingcircle(inputgeom geometry, segs_per_quarter integer DEFAULT 48) returns geometry
  language c;

create function public.st_minimumboundingradius(geometry, OUT center geometry, OUT radius double precision) returns record
  language c;

create function public.st_minimumclearance(geometry) returns double precision
  language c;

create function public.st_minimumclearanceline(geometry) returns geometry
  language c;

create function public.st_mlinefromtext(text) returns geometry
  language sql;

create function public.st_mlinefromtext(text, integer) returns geometry
  language sql;

create function public.st_mlinefromwkb(bytea) returns geometry
  language sql;

create function public.st_mlinefromwkb(bytea, integer) returns geometry
  language sql;

create function public.st_mpointfromtext(text) returns geometry
  language sql;

create function public.st_mpointfromtext(text, integer) returns geometry
  language sql;

create function public.st_mpointfromwkb(bytea) returns geometry
  language sql;

create function public.st_mpointfromwkb(bytea, integer) returns geometry
  language sql;

create function public.st_mpolyfromtext(text) returns geometry
  language sql;

create function public.st_mpolyfromtext(text, integer) returns geometry
  language sql;

create function public.st_mpolyfromwkb(bytea) returns geometry
  language sql;

create function public.st_mpolyfromwkb(bytea, integer) returns geometry
  language sql;

create function public.st_multi(geometry) returns geometry
  language c;

create function public.st_multilinefromwkb(bytea) returns geometry
  language sql;

create function public.st_multilinestringfromtext(text) returns geometry
  language sql;

create function public.st_multilinestringfromtext(text, integer) returns geometry
  language sql;

create function public.st_multipointfromtext(text) returns geometry
  language sql;

create function public.st_multipointfromwkb(bytea) returns geometry
  language sql;

create function public.st_multipointfromwkb(bytea, integer) returns geometry
  language sql;

create function public.st_multipolyfromwkb(bytea) returns geometry
  language sql;

create function public.st_multipolyfromwkb(bytea, integer) returns geometry
  language sql;

create function public.st_multipolygonfromtext(text) returns geometry
  language sql;

create function public.st_multipolygonfromtext(text, integer) returns geometry
  language sql;

create function public.st_ndims(geometry) returns smallint
  language c;

create function public.st_node(g geometry) returns geometry
  language c;

create function public.st_normalize(geom geometry) returns geometry
  language c;

create function public.st_npoints(geometry) returns integer
  language c;

create function public.st_nrings(geometry) returns integer
  language c;

create function public.st_numcurves(geometry geometry) returns integer
  language c;

create function public.st_numgeometries(geometry) returns integer
  language c;

create function public.st_numinteriorring(geometry) returns integer
  language c;

create function public.st_numinteriorrings(geometry) returns integer
  language c;

create function public.st_numpatches(geometry) returns integer
  language c;

create function public.st_numpoints(geometry) returns integer
  language c;

create function public.st_offsetcurve(line geometry, distance double precision, params text DEFAULT ''::text) returns geometry
  language c;

create function public.st_orderingequals(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.st_orientedenvelope(geometry) returns geometry
  language c;

create function public.st_overlaps(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.st_patchn(geometry, integer) returns geometry
  language c;

create function public.st_perimeter(geog geography, use_spheroid boolean DEFAULT true) returns double precision
  language c;

create function public.st_perimeter(geometry) returns double precision
  language c;

create function public.st_perimeter2d(geometry) returns double precision
  language c;

create function public.st_point(double precision, double precision) returns geometry
  language c;

create function public.st_point(double precision, double precision, srid integer) returns geometry
  language c;

create function public.st_pointfromgeohash(text, integer DEFAULT NULL::integer) returns geometry
  language c;

create function public.st_pointfromtext(text) returns geometry
  language sql;

create function public.st_pointfromtext(text, integer) returns geometry
  language sql;

create function public.st_pointfromwkb(bytea) returns geometry
  language sql;

create function public.st_pointfromwkb(bytea, integer) returns geometry
  language sql;

create function public.st_pointinsidecircle(geometry, double precision, double precision, double precision) returns boolean
  language c;

create function public.st_pointm(xcoordinate double precision, ycoordinate double precision, mcoordinate double precision, srid integer DEFAULT 0) returns geometry
  language c;

create function public.st_pointn(geometry, integer) returns geometry
  language c;

create function public.st_pointonsurface(geometry) returns geometry
  language c;

create function public.st_points(geometry) returns geometry
  language c;

create function public.st_pointz(xcoordinate double precision, ycoordinate double precision, zcoordinate double precision, srid integer DEFAULT 0) returns geometry
  language c;

create function public.st_pointzm(xcoordinate double precision, ycoordinate double precision, zcoordinate double precision, mcoordinate double precision, srid integer DEFAULT 0) returns geometry
  language c;

create function public.st_polyfromtext(text) returns geometry
  language sql;

create function public.st_polyfromtext(text, integer) returns geometry
  language sql;

create function public.st_polyfromwkb(bytea) returns geometry
  language sql;

create function public.st_polyfromwkb(bytea, integer) returns geometry
  language sql;

create function public.st_polygon(geometry, integer) returns geometry
  language sql;

create function public.st_polygonfromtext(text) returns geometry
  language sql;

create function public.st_polygonfromtext(text, integer) returns geometry
  language sql;

create function public.st_polygonfromwkb(bytea) returns geometry
  language sql;

create function public.st_polygonfromwkb(bytea, integer) returns geometry
  language sql;

create aggregate public.st_polygonize(geometry) (
  sfunc = public.pgis_geometry_accum_transfn,
  stype = internal,
  finalfunc = pgis_geometry_polygonize_finalfn
);

create function public.st_polygonize(geometry[]) returns geometry
  language c;

create function public.st_project(geog geography, distance double precision, azimuth double precision) returns geography
  language c;

create function public.st_project(geog_from geography, geog_to geography, distance double precision) returns geography
  language c;

create function public.st_project(geom1 geometry, distance double precision, azimuth double precision) returns geometry
  language c;

create function public.st_project(geom1 geometry, geom2 geometry, distance double precision) returns geometry
  language c;

create function public.st_quantizecoordinates(g geometry, prec_x integer, prec_y integer DEFAULT NULL::integer, prec_z integer DEFAULT NULL::integer, prec_m integer DEFAULT NULL::integer) returns geometry
  language c;

create function public.st_reduceprecision(geom geometry, gridsize double precision) returns geometry
  language c;

create function public.st_relate(geom1 geometry, geom2 geometry) returns text
  language c;

create function public.st_relate(geom1 geometry, geom2 geometry, integer) returns text
  language c;

create function public.st_relate(geom1 geometry, geom2 geometry, text) returns boolean
  language c;

create function public.st_relatematch(text, text) returns boolean
  language c;

create function public.st_removeirrelevantpointsforview(geometry, box2d, boolean DEFAULT false) returns geometry
  language c;

create function public.st_removepoint(geometry, integer) returns geometry
  language c;

create function public.st_removerepeatedpoints(geom geometry, tolerance double precision DEFAULT 0.0) returns geometry
  language c;

create function public.st_removesmallparts(geometry, double precision, double precision) returns geometry
  language c;

create function public.st_reverse(geometry) returns geometry
  language c;

create function public.st_rotate(geometry, double precision) returns geometry
  language sql;

create function public.st_rotate(geometry, double precision, double precision, double precision) returns geometry
  language sql;

create function public.st_rotate(geometry, double precision, geometry) returns geometry
  language sql;

create function public.st_rotatex(geometry, double precision) returns geometry
  language sql;

create function public.st_rotatey(geometry, double precision) returns geometry
  language sql;

create function public.st_rotatez(geometry, double precision) returns geometry
  language sql;

create function public.st_scale(geometry, double precision, double precision) returns geometry
  language sql;

create function public.st_scale(geometry, double precision, double precision, double precision) returns geometry
  language sql;

create function public.st_scale(geometry, geometry) returns geometry
  language c;

create function public.st_scale(geometry, geometry, origin geometry) returns geometry
  language c;

create function public.st_scroll(geometry, geometry) returns geometry
  language c;

create function public.st_segmentize(geog geography, max_segment_length double precision) returns geography
  language c;

create function public.st_segmentize(geometry, double precision) returns geometry
  language c;

create function public.st_seteffectivearea(geometry, double precision DEFAULT '-1'::integer, integer DEFAULT 1) returns geometry
  language c;

create function public.st_setpoint(geometry, integer, geometry) returns geometry
  language c;

create function public.st_setsrid(geog geography, srid integer) returns geography
  language c;

create function public.st_setsrid(geom geometry, srid integer) returns geometry
  language c;

create function public.st_sharedpaths(geom1 geometry, geom2 geometry) returns geometry
  language c;

create function public.st_shiftlongitude(geometry) returns geometry
  language c;

create function public.st_shortestline(geography, geography, use_spheroid boolean DEFAULT true) returns geography
  language c;

create function public.st_shortestline(geom1 geometry, geom2 geometry) returns geometry
  language c;

create function public.st_shortestline(text, text) returns geometry
  language sql;

create function public.st_simplify(geometry, double precision) returns geometry
  language c;

create function public.st_simplify(geometry, double precision, boolean) returns geometry
  language c;

create function public.st_simplifypolygonhull(geom geometry, vertex_fraction double precision, is_outer boolean DEFAULT true) returns geometry
  language c;

create function public.st_simplifypreservetopology(geometry, double precision) returns geometry
  language c;

create function public.st_simplifyvw(geometry, double precision) returns geometry
  language c;

create function public.st_snap(geom1 geometry, geom2 geometry, double precision) returns geometry
  language c;

create function public.st_snaptogrid(geom1 geometry, geom2 geometry, double precision, double precision, double precision, double precision) returns geometry
  language c;

create function public.st_snaptogrid(geometry, double precision) returns geometry
  language sql;

create function public.st_snaptogrid(geometry, double precision, double precision) returns geometry
  language sql;

create function public.st_snaptogrid(geometry, double precision, double precision, double precision, double precision) returns geometry
  language c;

create function public.st_split(geom1 geometry, geom2 geometry) returns geometry
  language c;

create function public.st_square(size double precision, cell_i integer, cell_j integer, origin geometry DEFAULT '010100000000000000000000000000000000000000'::geometry) returns geometry
  language c;

create function public.st_squaregrid(size double precision, bounds geometry, OUT geom geometry, OUT i integer, OUT j integer) returns SETOF record
  language c;

create function public.st_srid(geog geography) returns integer
  language c;

create function public.st_srid(geom geometry) returns integer
  language c;

create function public.st_startpoint(geometry) returns geometry
  language c;

create function public.st_subdivide(geom geometry, maxvertices integer DEFAULT 256, gridsize double precision DEFAULT '-1.0'::numeric) returns SETOF geometry
  language c;

create function public.st_summary(geography) returns text
  language c;

create function public.st_summary(geometry) returns text
  language c;

create function public.st_swapordinates(geom geometry, ords cstring) returns geometry
  language c;

create function public.st_symdifference(geom1 geometry, geom2 geometry, gridsize double precision DEFAULT '-1.0'::numeric) returns geometry
  language c;

create function public.st_symmetricdifference(geom1 geometry, geom2 geometry) returns geometry
  language sql;

create function public.st_tileenvelope(zoom integer, x integer, y integer, bounds geometry DEFAULT '0102000020110F00000200000093107C45F81B73C193107C45F81B73C193107C45F81B734193107C45F81B7341'::geometry, margin double precision DEFAULT 0.0) returns geometry
  language c;

create function public.st_touches(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.st_transform(geom geometry, from_proj text, to_proj text) returns geometry
  language sql;

create function public.st_transform(geom geometry, from_proj text, to_srid integer) returns geometry
  language sql;

create function public.st_transform(geom geometry, to_proj text) returns geometry
  language sql;

create function public.st_transform(geometry, integer) returns geometry
  language c;

create function public.st_transformpipeline(geom geometry, pipeline text, to_srid integer DEFAULT 0) returns geometry
  language sql;

create function public.st_translate(geometry, double precision, double precision) returns geometry
  language sql;

create function public.st_translate(geometry, double precision, double precision, double precision) returns geometry
  language sql;

create function public.st_transscale(geometry, double precision, double precision, double precision, double precision) returns geometry
  language sql;

create function public.st_triangulatepolygon(g1 geometry) returns geometry
  language c;

create function public.st_unaryunion(geometry, gridsize double precision DEFAULT '-1.0'::numeric) returns geometry
  language c;

create function public.st_union(geom1 geometry, geom2 geometry) returns geometry
  language c;

create function public.st_union(geom1 geometry, geom2 geometry, gridsize double precision) returns geometry
  language c;

create aggregate public.st_union(geometry) (
  sfunc = public.pgis_geometry_union_parallel_transfn,
  stype = internal,
  finalfunc = pgis_geometry_union_parallel_finalfn,
  combinefunc = pgis_geometry_union_parallel_combinefn
);

create aggregate public.st_union(geometry, gridsize double precision) (
  sfunc = public.pgis_geometry_union_parallel_transfn,
  stype = internal,
  finalfunc = pgis_geometry_union_parallel_finalfn,
  combinefunc = pgis_geometry_union_parallel_combinefn
);

create function public.st_union(geometry[]) returns geometry
  language c;

create function public.st_voronoilines(g1 geometry, tolerance double precision DEFAULT 0.0, extend_to geometry DEFAULT NULL::geometry) returns geometry
  language sql;

create function public.st_voronoipolygons(g1 geometry, tolerance double precision DEFAULT 0.0, extend_to geometry DEFAULT NULL::geometry) returns geometry
  language sql;

create function public.st_within(geom1 geometry, geom2 geometry) returns boolean
  language c;

create function public.st_wkbtosql(wkb bytea) returns geometry
  language c;

create function public.st_wkttosql(text) returns geometry
  language c;

create function public.st_wrapx(geom geometry, wrap double precision, move double precision) returns geometry
  language c;

create function public.st_x(geometry) returns double precision
  language c;

create function public.st_xmax(box3d) returns double precision
  language c;

create function public.st_xmin(box3d) returns double precision
  language c;

create function public.st_y(geometry) returns double precision
  language c;

create function public.st_ymax(box3d) returns double precision
  language c;

create function public.st_ymin(box3d) returns double precision
  language c;

create function public.st_z(geometry) returns double precision
  language c;

create function public.st_zmax(box3d) returns double precision
  language c;

create function public.st_zmflag(geometry) returns smallint
  language c;

create function public.st_zmin(box3d) returns double precision
  language c;

create function public.text(geometry) returns text
  language c;

create function public.updategeometrysrid(catalogn_name character varying, schema_name character varying, table_name character varying, column_name character varying, new_srid_in integer) returns text
  language plpgsql;

create function public.updategeometrysrid(character varying, character varying, character varying, integer) returns text
  language plpgsql;

create function public.updategeometrysrid(character varying, character varying, integer) returns text
  language plpgsql;

create operator public.&& (
  leftarg = box2df,
  rightarg = box2df,
  function = public.overlaps_2d
);

create operator public.&& (
  leftarg = box2df,
  rightarg = geometry,
  function = public.overlaps_2d
);

create operator public.&& (
  leftarg = geography,
  rightarg = geography,
  function = public.geography_overlaps
);

create operator public.&& (
  leftarg = geography,
  rightarg = gidx,
  function = public.overlaps_geog
);

create operator public.&& (
  leftarg = geometry,
  rightarg = box2df,
  function = public.overlaps_2d
);

create operator public.&& (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_overlaps
);

create operator public.&& (
  leftarg = gidx,
  rightarg = geography,
  function = public.overlaps_geog
);

create operator public.&& (
  leftarg = gidx,
  rightarg = gidx,
  function = public.overlaps_geog
);

create operator public.&&& (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_overlaps_nd
);

create operator public.&&& (
  leftarg = geometry,
  rightarg = gidx,
  function = public.overlaps_nd
);

create operator public.&&& (
  leftarg = gidx,
  rightarg = geometry,
  function = public.overlaps_nd
);

create operator public.&&& (
  leftarg = gidx,
  rightarg = gidx,
  function = public.overlaps_nd
);

create operator public.&/& (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_overlaps_3d
);

create operator public.&< (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_overleft
);

create operator public.&<| (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_overbelow
);

create operator public.&> (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_overright
);

create operator public.< (
  leftarg = geography,
  rightarg = geography,
  function = public.geography_lt
);

create operator public.< (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_lt
);

create operator public.<#> (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_distance_box
);

create operator public.<-> (
  leftarg = geography,
  rightarg = geography,
  function = public.geography_distance_knn
);

create operator public.<-> (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_distance_centroid
);

create operator public.<< (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_left
);

create operator public.<<->> (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_distance_centroid_nd
);

create operator public.<<@ (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_contained_3d
);

create operator public.<<| (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_below
);

create operator public.<= (
  leftarg = geography,
  rightarg = geography,
  function = public.geography_le
);

create operator public.<= (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_le
);

create operator public.<> (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_neq
);

create operator public.= (
  leftarg = geography,
  rightarg = geography,
  function = public.geography_eq
);

create operator public.= (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_eq
);

create operator public.> (
  leftarg = geography,
  rightarg = geography,
  function = public.geography_gt
);

create operator public.> (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_gt
);

create operator public.>= (
  leftarg = geography,
  rightarg = geography,
  function = public.geography_ge
);

create operator public.>= (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_ge
);

create operator public.>> (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_right
);

create operator public.@ (
  leftarg = box2df,
  rightarg = box2df,
  function = public.is_contained_2d
);

create operator public.@ (
  leftarg = box2df,
  rightarg = geometry,
  function = public.is_contained_2d
);

create operator public.@ (
  leftarg = geometry,
  rightarg = box2df,
  function = public.is_contained_2d
);

create operator public.@ (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_within
);

create operator public.@>> (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_contains_3d
);

create operator public.@@ (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_within_nd
);

create operator public.|&> (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_overabove
);

create operator public.|=| (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_distance_cpa
);

create operator public.|>> (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_above
);

create operator public.~ (
  leftarg = box2df,
  rightarg = box2df,
  function = public.contains_2d
);

create operator public.~ (
  leftarg = box2df,
  rightarg = geometry,
  function = public.contains_2d
);

create operator public.~ (
  leftarg = geometry,
  rightarg = box2df,
  function = public.contains_2d
);

create operator public.~ (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_contains
);

create operator public.~= (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_same
);

create operator public.~== (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_same_3d
);

create operator public.~~ (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_contains_nd
);

create operator public.~~= (
  leftarg = geometry,
  rightarg = geometry,
  function = public.geometry_same_nd
);

