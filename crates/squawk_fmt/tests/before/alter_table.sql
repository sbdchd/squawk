ALTER TABLE customers ADD COLUMN email_address varchar(320), ALTER COLUMN customer_description SET DEFAULT 'No description has been provided for this customer', DROP COLUMN IF EXISTS obsolete_customer_code CASCADE;

ALTER TABLE IF EXISTS ONLY (reporting.customer_activity) RENAME COLUMN old_activity_description TO new_activity_description;

ALTER TABLE reporting.customer_activity ADD CONSTRAINT positive_activity_count CHECK (activity_count > 0), ALTER CONSTRAINT positive_activity_count DEFERRABLE INITIALLY DEFERRED;

ALTER TABLE tx1 RENAME TO a1;

ALTER TABLE temp_view_test.tt1 RENAME TO tmp1;

ALTER /* table */ TABLE /* old table */ temp_view_test.tt1 /* rename */ RENAME /* to */ TO /* new table */ tmp1 /* semicolon */;

ALTER TABLE temporal_children ADD CONSTRAINT temporal_children_parent_fk FOREIGN KEY (parent_id, PERIOD valid_at) REFERENCES temporal_parents (id, PERIOD valid_at);

ALTER TABLE reporting.customer_activity ATTACH PARTITION reporting.customer_activity_2025 FOR VALUES FROM ('2025-01-01') TO ('2026-01-01');

ALTER TABLE reporting.customer_activity DETACH PARTITION reporting.customer_activity_2024 CONCURRENTLY;

ALTER TABLE reporting.customer_activity ENABLE ROW LEVEL SECURITY, FORCE ROW LEVEL SECURITY, DISABLE TRIGGER ALL;

ALTER TABLE reporting.customer_activity SET (fillfactor = 80), RESET (fillfactor), SET TABLESPACE archive_tablespace;

ALTER TABLE ALL IN TABLESPACE old_reporting_tablespace OWNED BY analytics_owner, reporting_owner SET TABLESPACE new_reporting_tablespace NOWAIT;

ALTER /* table */ TABLE /* if */ IF /* exists */ EXISTS /* only */ ONLY /* left paren */ (/* relation */ reporting.commented_customer_activity /* right paren */) /* action */ ALTER /* column */ COLUMN /* column name */ activity_description /* set */ SET /* default */ DEFAULT /* expression */ 'A very long default activity description used to verify formatter wrapping behavior', /* comma */ RENAME /* column */ COLUMN /* old name */ old_activity_code /* to */ TO /* new name */ current_activity_code /* semicolon */;

ALTER /* table */ TABLE /* all */ ALL /* in */ IN /* tablespace keyword */ TABLESPACE /* old tablespace */ old_tablespace /* owned */ OWNED /* by */ BY /* first role */ role_one, /* second role */ role_two /* set */ SET /* tablespace */ TABLESPACE /* new tablespace */ new_tablespace /* nowait */ NOWAIT /* semicolon */;

ALTER TABLE ONLY parent_table ALTER CONSTRAINT inherited_check_constraint ENFORCED;

ALTER TABLE parent_table ALTER CONSTRAINT inherited_check_constraint NOT ENFORCED;

ALTER TABLE parent_table ALTER CONSTRAINT inherited_not_null_constraint INHERIT;

ALTER TABLE parent_table ALTER CONSTRAINT inherited_not_null_constraint NO INHERIT;

ALTER /* table */ TABLE /* relation */ parent_table /* alter */ ALTER /* constraint */ CONSTRAINT /* constraint name */ inherited_check_constraint /* enforced */ ENFORCED /* semicolon */;

ALTER /* table */ TABLE /* relation */ parent_table /* alter */ ALTER /* constraint */ CONSTRAINT /* constraint name */ inherited_check_constraint /* not */ NOT /* enforced */ ENFORCED /* semicolon */;

ALTER /* table */ TABLE /* relation */ parent_table /* alter */ ALTER /* constraint */ CONSTRAINT /* constraint name */ inherited_not_null_constraint /* inherit */ INHERIT /* semicolon */;

ALTER /* table */ TABLE /* relation */ parent_table /* alter */ ALTER /* constraint */ CONSTRAINT /* constraint name */ inherited_not_null_constraint /* no */ NO /* inherit */ INHERIT /* semicolon */;

ALTER TABLE child_table ADD NOT NULL a_very_long_inherited_column_name NO INHERIT;

ALTER /* table */ TABLE /* relation */ child_table /* add */ ADD /* not */ NOT /* null */ NULL /* column */ inherited_column /* no */ NO /* inherit */ INHERIT /* semicolon */;

ALTER TABLE parent_table ALTER CONSTRAINT source_ordered_constraint /* enforced */ ENFORCED /* not */ NOT /* deferrable */ DEFERRABLE;

ALTER TABLE parent_table ALTER CONSTRAINT invalid_constraint /* not */ NOT /* valid */ VALID;

ALTER TABLE comment_test ALTER COLUMN id SET DATA TYPE int COLLATE "C";

ALTER TABLE extraordinarily_long_partition_name ALTER extraordinarily_long_column_name TYPE char (2) COLLATE "POSIX";

ALTER /* table */ TABLE /* relation */ comment_test /* alter */ ALTER /* column */ COLUMN /* column name */ description /* set */ SET /* data */ DATA /* type keyword */ TYPE /* type */ character varying(100) /* collate */ COLLATE /* collation */ "POSIX" /* using */ USING /* expression */ description::character varying /* semicolon */;

ALTER TABLE sales_range MERGE PARTITIONS (sales_jan2022, sales_feb2022) INTO sales_jan_feb2022;

ALTER TABLE partitions_merge_schema.extraordinarily_long_sales_range MERGE PARTITIONS (partitions_merge_schema.extraordinarily_long_sales_january_2022, partitions_merge_schema.extraordinarily_long_sales_february_2022) INTO partitions_merge_schema.extraordinarily_long_sales_january_february_2022;

ALTER /* table */ TABLE /* relation */ partitions_merge_schema.sales_range /* merge */ MERGE /* partitions */ PARTITIONS /* left paren */ (/* first partition */ partitions_merge_schema.sales_jan2022 /* first comma */, /* second partition */ sales_feb2022, /* third partition */ sales_mar2022 /* right paren */) /* into */ INTO /* target partition */ partitions_merge_schema./* target name */sales_jan_feb_mar2022 /* semicolon */;

ALTER TABLE sales_range SPLIT PARTITION sales_feb_mar_apr2022 INTO (PARTITION sales_feb2022 FOR VALUES FROM ('2022-02-01') TO ('2022-03-01'), PARTITION sales_mar2022 FOR VALUES IN ('2022-03-01', '2022-04-01'), PARTITION sales_hash FOR VALUES WITH (MODULUS 4, REMAINDER 1), PARTITION sales_others DEFAULT);

ALTER /* table */ TABLE /* relation */ sales_range /* split */ SPLIT /* partition */ PARTITION /* source */ partition_split_schema./* source name */sales_all /* into */ INTO /* list left */ (/* first partition */ PARTITION /* first name */ partition_split_schema./* name */sales_first /* for */ FOR /* values */ VALUES /* from */ FROM /* lower left */ (/* lower */ '2022-01-01' /* lower right */) /* to */ TO /* upper left */ (/* upper */ '2022-02-01' /* upper right */) /* comma */, /* second partition */ PARTITION /* second name */ sales_second /* for */ FOR /* values */ VALUES /* in */ IN /* in left */ (/* first value */ 'one' /* value comma */, /* second value */ 'two' /* in right */), /* default partition */ PARTITION /* default name */ sales_other /* default */ DEFAULT /* list right */) /* semicolon */;

ALTER TABLE options_test ALTER COLUMN metadata SET (json = constraint);

ALTER /* table */ TABLE /* relation */ options_test /* alter */ ALTER /* column */ COLUMN /* name */ metadata /* set */ SET /* left paren */ (/* option */ json /* equals */ = /* value */ constraint /* right paren */) /* semicolon */;
