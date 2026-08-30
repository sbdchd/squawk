ALTER /* index */ INDEX /* all */ ALL /* in */ IN /* tablespace */ TABLESPACE /* old */ old_tablespace_with_a_very_long_descriptive_name /* owned */ OWNED /* by */ BY /* role one */ first_role_with_a_long_name, /* role two */ second_role_with_a_long_name /* set */ SET /* tablespace */ TABLESPACE /* new */ new_tablespace_with_a_very_long_descriptive_name /* nowait */ NOWAIT /* end */;

ALTER INDEX /* if */ IF /* exists */ EXISTS /* name */ public.index_with_a_very_long_descriptive_name /* alter */ ALTER /* column */ COLUMN /* number */ 3 /* set */ SET /* statistics */ STATISTICS /* value */ 1000;

ALTER INDEX idx ATTACH /* partition */ PARTITION /* child */ public.child_idx;

ALTER INDEX idx /* depends */ DEPENDS /* on */ ON /* extension */ EXTENSION /* name */ bloom;

ALTER INDEX idx /* no */ NO /* depends */ DEPENDS /* on */ ON /* extension */ EXTENSION /* name */ bloom;

ALTER INDEX idx /* rename */ RENAME /* to */ TO /* name */ renamed_idx;

ALTER INDEX idx /* reset */ RESET /* list */ (/* first */ fillfactor, /* second */ deduplicate_items);

ALTER INDEX idx /* set */ SET /* list */ (/* key */ fillfactor /* equals */ = /* value */ 70, deduplicate_items = off);

ALTER INDEX idx ALTER COLUMN indexed_expression SET (/* key */ n_distinct = /* value */ 100);

ALTER INDEX idx /* set */ SET /* tablespace */ TABLESPACE /* name */ fast_storage;
