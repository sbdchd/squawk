ALTER /* materialized */ MATERIALIZED /* view */ VIEW /* all */ ALL /* in */ IN /* tablespace */ TABLESPACE /* old */ old_materialized_view_tablespace /* owned */ OWNED /* by */ BY /* role */ analytics_owner_with_a_very_long_name /* set */ SET /* tablespace */ TABLESPACE /* new */ new_materialized_view_tablespace /* nowait */ NOWAIT /* end */;

ALTER MATERIALIZED VIEW /* if */ IF /* exists */ EXISTS /* view */ reporting.materialized_view_with_a_very_long_descriptive_name /* alter */ ALTER /* column */ COLUMN /* name */ measured_value /* set */ SET /* statistics */ STATISTICS /* value */ 1000, /* owner */ OWNER /* to */ TO /* role */ analytics_owner_with_a_very_long_name;

ALTER MATERIALIZED VIEW reporting.summary /* rename */ RENAME /* column */ COLUMN /* old */ old_column /* to */ TO /* new */ new_column;

ALTER MATERIALIZED VIEW reporting.summary /* rename */ RENAME /* to */ TO /* view */ renamed_summary;

ALTER MATERIALIZED VIEW reporting.summary /* set */ SET /* schema */ SCHEMA /* name */ archive;

ALTER MATERIALIZED VIEW reporting.summary /* depends */ DEPENDS /* on */ ON /* extension */ EXTENSION /* name */ tablefunc;

ALTER MATERIALIZED VIEW reporting.summary /* no */ NO /* depends */ DEPENDS /* on */ ON /* extension */ EXTENSION /* name */ tablefunc;

ALTER MATERIALIZED VIEW reporting.summary SET (fillfactor = 80);
