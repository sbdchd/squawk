ALTER TABLE customers ADD COLUMN email_address varchar(320), ALTER COLUMN customer_description SET DEFAULT 'No description has been provided for this customer', DROP COLUMN IF EXISTS obsolete_customer_code CASCADE;

ALTER TABLE IF EXISTS ONLY (reporting.customer_activity) RENAME COLUMN old_activity_description TO new_activity_description;

ALTER TABLE reporting.customer_activity ADD CONSTRAINT positive_activity_count CHECK (activity_count > 0), ALTER CONSTRAINT positive_activity_count DEFERRABLE INITIALLY DEFERRED;

ALTER TABLE reporting.customer_activity ATTACH PARTITION reporting.customer_activity_2025 FOR VALUES FROM ('2025-01-01') TO ('2026-01-01');

ALTER TABLE reporting.customer_activity DETACH PARTITION reporting.customer_activity_2024 CONCURRENTLY;

ALTER TABLE reporting.customer_activity ENABLE ROW LEVEL SECURITY, FORCE ROW LEVEL SECURITY, DISABLE TRIGGER ALL;

ALTER TABLE reporting.customer_activity SET (fillfactor = 80), RESET (fillfactor), SET TABLESPACE archive_tablespace;

ALTER TABLE ALL IN TABLESPACE old_reporting_tablespace OWNED BY analytics_owner, reporting_owner SET TABLESPACE new_reporting_tablespace NOWAIT;

ALTER /* table */ TABLE /* if */ IF /* exists */ EXISTS /* only */ ONLY /* left paren */ (/* relation */ reporting.commented_customer_activity /* right paren */) /* action */ ALTER /* column */ COLUMN /* column name */ activity_description /* set */ SET /* default */ DEFAULT /* expression */ 'A very long default activity description used to verify formatter wrapping behavior', /* comma */ RENAME /* column */ COLUMN /* old name */ old_activity_code /* to */ TO /* new name */ current_activity_code /* semicolon */;

ALTER /* table */ TABLE /* all */ ALL /* in */ IN /* tablespace keyword */ TABLESPACE /* old tablespace */ old_tablespace /* owned */ OWNED /* by */ BY /* first role */ role_one, /* second role */ role_two /* set */ SET /* tablespace */ TABLESPACE /* new tablespace */ new_tablespace /* nowait */ NOWAIT /* semicolon */;
