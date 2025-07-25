--
-- MERGE
--

CREATE USER regress_merge_privs;
CREATE USER regress_merge_no_privs;
CREATE USER regress_merge_none;

DROP TABLE IF EXISTS target;
DROP TABLE IF EXISTS source;
CREATE TABLE target (tid integer, balance integer)
  WITH (autovacuum_enabled=off);
CREATE TABLE source (sid integer, delta integer) -- no index
  WITH (autovacuum_enabled=off);
INSERT INTO target VALUES (1, 10);
INSERT INTO target VALUES (2, 20);
INSERT INTO target VALUES (3, 30);
SELECT t.ctid is not null as matched, t.*, s.* FROM source s FULL OUTER JOIN target t ON s.sid = t.tid ORDER BY t.tid, s.sid;

ALTER TABLE target OWNER TO regress_merge_privs;
ALTER TABLE source OWNER TO regress_merge_privs;

CREATE TABLE target2 (tid integer, balance integer)
  WITH (autovacuum_enabled=off);
CREATE TABLE source2 (sid integer, delta integer)
  WITH (autovacuum_enabled=off);

ALTER TABLE target2 OWNER TO regress_merge_no_privs;
ALTER TABLE source2 OWNER TO regress_merge_no_privs;

GRANT INSERT ON target TO regress_merge_no_privs;

SET SESSION AUTHORIZATION regress_merge_privs;

EXPLAIN (COSTS OFF)
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED THEN
	DELETE;

--
-- Errors
--
-- MERGE INTO target t RANDOMWORD
-- USING source AS s
-- ON t.tid = s.sid
-- WHEN MATCHED THEN
-- 	UPDATE SET balance = 0;
-- MATCHED/INSERT error
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED THEN
	INSERT DEFAULT VALUES;
-- NOT MATCHED BY SOURCE/INSERT error
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN NOT MATCHED BY SOURCE THEN
	INSERT DEFAULT VALUES;
-- incorrectly specifying INTO target
-- MERGE INTO target t
-- USING source AS s
-- ON t.tid = s.sid
-- WHEN NOT MATCHED THEN
-- 	INSERT INTO target DEFAULT VALUES;
-- Multiple VALUES clause
-- MERGE INTO target t
-- USING source AS s
-- ON t.tid = s.sid
-- WHEN NOT MATCHED THEN
-- 	INSERT VALUES (1,1), (2,2);
-- SELECT query for INSERT
-- MERGE INTO target t
-- USING source AS s
-- ON t.tid = s.sid
-- WHEN NOT MATCHED THEN
-- 	INSERT SELECT (1, 1);
-- NOT MATCHED/UPDATE
-- MERGE INTO target t
-- USING source AS s
-- ON t.tid = s.sid
-- WHEN NOT MATCHED THEN
-- 	UPDATE SET balance = 0;
-- NOT MATCHED BY TARGET/UPDATE
-- MERGE INTO target t
-- USING source AS s
-- ON t.tid = s.sid
-- WHEN NOT MATCHED BY TARGET THEN
-- 	UPDATE SET balance = 0;
-- UPDATE tablename
-- MERGE INTO target t
-- USING source AS s
-- ON t.tid = s.sid
-- WHEN MATCHED THEN
-- 	UPDATE target SET balance = 0;
-- source and target names the same
MERGE INTO target
USING target
ON tid = tid
WHEN MATCHED THEN DO NOTHING;
-- used in a CTE without RETURNING
WITH foo AS (
  MERGE INTO target USING source ON (true)
  WHEN MATCHED THEN DELETE
) SELECT * FROM foo;
-- used in COPY without RETURNING
COPY (
  MERGE INTO target USING source ON (true)
  WHEN MATCHED THEN DELETE
) TO stdout;

-- unsupported relation types
-- materialized view
CREATE MATERIALIZED VIEW mv AS SELECT * FROM target;
MERGE INTO mv t
USING source s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
	INSERT DEFAULT VALUES;
DROP MATERIALIZED VIEW mv;

-- permissions

SET SESSION AUTHORIZATION regress_merge_none;
MERGE INTO target
USING (SELECT 1)
ON true
WHEN MATCHED THEN
	DO NOTHING;

SET SESSION AUTHORIZATION regress_merge_privs;
MERGE INTO target
USING source2
ON target.tid = source2.sid
WHEN MATCHED THEN
	UPDATE SET balance = 0;

GRANT INSERT ON target TO regress_merge_no_privs;
SET SESSION AUTHORIZATION regress_merge_no_privs;

MERGE INTO target
USING source2
ON target.tid = source2.sid
WHEN MATCHED THEN
	UPDATE SET balance = 0;

GRANT UPDATE ON target2 TO regress_merge_privs;
SET SESSION AUTHORIZATION regress_merge_privs;

MERGE INTO target2
USING source
ON target2.tid = source.sid
WHEN MATCHED THEN
	DELETE;

MERGE INTO target2
USING source
ON target2.tid = source.sid
WHEN NOT MATCHED THEN
	INSERT DEFAULT VALUES;

-- check if the target can be accessed from source relation subquery; we should
-- not be able to do so
MERGE INTO target t
USING (SELECT * FROM source WHERE t.tid > sid) s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
	INSERT DEFAULT VALUES;

--
-- initial tests
--
-- zero rows in source has no effect
MERGE INTO target
USING source
ON target.tid = source.sid
WHEN MATCHED THEN
	UPDATE SET balance = 0;

MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED THEN
	UPDATE SET balance = 0;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED THEN
	DELETE;
BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
	INSERT DEFAULT VALUES;
ROLLBACK;

-- insert some non-matching source rows to work from
INSERT INTO source VALUES (4, 40);
SELECT * FROM source ORDER BY sid;
SELECT * FROM target ORDER BY tid;

MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
	DO NOTHING;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED THEN
	UPDATE SET balance = 0;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED THEN
	DELETE;
BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
	INSERT DEFAULT VALUES;
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- DELETE/INSERT not matched by source/target
BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN NOT MATCHED BY SOURCE THEN
	DELETE
WHEN NOT MATCHED BY TARGET THEN
	INSERT VALUES (s.sid, s.delta)
RETURNING merge_action(), old, new, t.*;
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- index plans
INSERT INTO target SELECT generate_series(1000,2500), 0;
ALTER TABLE target ADD PRIMARY KEY (tid);
ANALYZE target;

EXPLAIN (COSTS OFF)
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED THEN
	UPDATE SET balance = 0;
EXPLAIN (COSTS OFF)
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED THEN
	DELETE;
EXPLAIN (COSTS OFF)
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
	INSERT VALUES (4, NULL);
DELETE FROM target WHERE tid > 100;
ANALYZE target;

-- insert some matching source rows to work from
INSERT INTO source VALUES (2, 5);
INSERT INTO source VALUES (3, 20);
SELECT * FROM source ORDER BY sid;
SELECT * FROM target ORDER BY tid;

-- equivalent of an UPDATE join
BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED THEN
	UPDATE SET balance = 0;
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- equivalent of a DELETE join
BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED THEN
	DELETE;
SELECT * FROM target ORDER BY tid;
ROLLBACK;

BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED THEN
	DO NOTHING;
SELECT * FROM target ORDER BY tid;
ROLLBACK;

BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
	INSERT VALUES (4, NULL);
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- duplicate source row causes multiple target row update ERROR
INSERT INTO source VALUES (2, 5);
SELECT * FROM source ORDER BY sid;
SELECT * FROM target ORDER BY tid;
BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED THEN
	UPDATE SET balance = 0;
ROLLBACK;

BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED THEN
	DELETE;
ROLLBACK;

-- remove duplicate MATCHED data from source data
DELETE FROM source WHERE sid = 2;
INSERT INTO source VALUES (2, 5);
SELECT * FROM source ORDER BY sid;
SELECT * FROM target ORDER BY tid;

-- duplicate source row on INSERT should fail because of target_pkey
INSERT INTO source VALUES (4, 40);
BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
  INSERT VALUES (4, NULL);
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- remove duplicate NOT MATCHED data from source data
DELETE FROM source WHERE sid = 4;
INSERT INTO source VALUES (4, 40);
SELECT * FROM source ORDER BY sid;
SELECT * FROM target ORDER BY tid;

-- remove constraints
alter table target drop CONSTRAINT target_pkey;
alter table target alter column tid drop not null;

-- multiple actions
BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
	INSERT VALUES (4, 4)
WHEN MATCHED THEN
	UPDATE SET balance = 0;
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- should be equivalent
BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED THEN
	UPDATE SET balance = 0
WHEN NOT MATCHED THEN
	INSERT VALUES (4, 4);
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- column references
-- do a simple equivalent of an UPDATE join
BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED THEN
	UPDATE SET balance = t.balance + s.delta;
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- do a simple equivalent of an INSERT SELECT
BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
	INSERT VALUES (s.sid, s.delta);
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- and again with duplicate source rows
INSERT INTO source VALUES (5, 50);
INSERT INTO source VALUES (5, 50);

-- do a simple equivalent of an INSERT SELECT
BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
  INSERT VALUES (s.sid, s.delta);
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- removing duplicate source rows
DELETE FROM source WHERE sid = 5;

-- and again with explicitly identified column list
BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
	INSERT (tid, balance) VALUES (s.sid, s.delta);
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- and again with a subtle error: referring to non-existent target row for NOT MATCHED
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
	INSERT (tid, balance) VALUES (t.tid, s.delta);

-- and again with a constant ON clause
BEGIN;
MERGE INTO target t
USING source AS s
ON (SELECT true)
WHEN NOT MATCHED THEN
	INSERT (tid, balance) VALUES (t.tid, s.delta);
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- now the classic UPSERT
BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED THEN
	UPDATE SET balance = t.balance + s.delta
WHEN NOT MATCHED THEN
	INSERT VALUES (s.sid, s.delta);
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- unreachable WHEN clause should ERROR
BEGIN;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED THEN /* Terminal WHEN clause for MATCHED */
	DELETE
WHEN MATCHED THEN
	UPDATE SET balance = t.balance - s.delta;
ROLLBACK;

-- conditional WHEN clause
CREATE TABLE wq_target (tid integer not null, balance integer DEFAULT -1)
  WITH (autovacuum_enabled=off);
CREATE TABLE wq_source (balance integer, sid integer)
  WITH (autovacuum_enabled=off);

INSERT INTO wq_source (sid, balance) VALUES (1, 100);

BEGIN;
-- try a simple INSERT with default values first
MERGE INTO wq_target t
USING wq_source s ON t.tid = s.sid
WHEN NOT MATCHED THEN
	INSERT (tid) VALUES (s.sid);
SELECT * FROM wq_target;
ROLLBACK;

-- this time with a FALSE condition
MERGE INTO wq_target t
USING wq_source s ON t.tid = s.sid
WHEN NOT MATCHED AND FALSE THEN
	INSERT (tid) VALUES (s.sid);
SELECT * FROM wq_target;

-- this time with an actual condition which returns false
MERGE INTO wq_target t
USING wq_source s ON t.tid = s.sid
WHEN NOT MATCHED AND s.balance <> 100 THEN
	INSERT (tid) VALUES (s.sid);
SELECT * FROM wq_target;

BEGIN;
-- and now with a condition which returns true
MERGE INTO wq_target t
USING wq_source s ON t.tid = s.sid
WHEN NOT MATCHED AND s.balance = 100 THEN
	INSERT (tid) VALUES (s.sid);
SELECT * FROM wq_target;
ROLLBACK;

-- conditions in the NOT MATCHED clause can only refer to source columns
BEGIN;
MERGE INTO wq_target t
USING wq_source s ON t.tid = s.sid
WHEN NOT MATCHED AND t.balance = 100 THEN
	INSERT (tid) VALUES (s.sid);
SELECT * FROM wq_target;
ROLLBACK;

MERGE INTO wq_target t
USING wq_source s ON t.tid = s.sid
WHEN NOT MATCHED AND s.balance = 100 THEN
	INSERT (tid) VALUES (s.sid);
SELECT * FROM wq_target;

-- conditions in NOT MATCHED BY SOURCE clause can only refer to target columns
MERGE INTO wq_target t
USING wq_source s ON t.tid = s.sid
WHEN NOT MATCHED BY SOURCE AND s.balance = 100 THEN
	DELETE;

MERGE INTO wq_target t
USING wq_source s ON t.tid = s.sid
WHEN NOT MATCHED BY SOURCE AND t.balance = 100 THEN
    DELETE;

-- conditions in MATCHED clause can refer to both source and target
SELECT * FROM wq_source;
MERGE INTO wq_target t
USING wq_source s ON t.tid = s.sid
WHEN MATCHED AND s.balance = 100 THEN
	UPDATE SET balance = t.balance + s.balance;
SELECT * FROM wq_target;

MERGE INTO wq_target t
USING wq_source s ON t.tid = s.sid
WHEN MATCHED AND t.balance = 100 THEN
	UPDATE SET balance = t.balance + s.balance;
SELECT * FROM wq_target;

-- check if AND works
MERGE INTO wq_target t
USING wq_source s ON t.tid = s.sid
WHEN MATCHED AND t.balance = 99 AND s.balance > 100 THEN
	UPDATE SET balance = t.balance + s.balance;
SELECT * FROM wq_target;

MERGE INTO wq_target t
USING wq_source s ON t.tid = s.sid
WHEN MATCHED AND t.balance = 99 AND s.balance = 100 THEN
	UPDATE SET balance = t.balance + s.balance;
SELECT * FROM wq_target;

-- check if OR works
MERGE INTO wq_target t
USING wq_source s ON t.tid = s.sid
WHEN MATCHED AND t.balance = 99 OR s.balance > 100 THEN
	UPDATE SET balance = t.balance + s.balance;
SELECT * FROM wq_target;

MERGE INTO wq_target t
USING wq_source s ON t.tid = s.sid
WHEN MATCHED AND t.balance = 199 OR s.balance > 100 THEN
	UPDATE SET balance = t.balance + s.balance;
SELECT * FROM wq_target;

-- check source-side whole-row references
BEGIN;
MERGE INTO wq_target t
USING wq_source s ON (t.tid = s.sid)
WHEN matched and t = s or t.tid = s.sid THEN
	UPDATE SET balance = t.balance + s.balance;
SELECT * FROM wq_target;
ROLLBACK;

-- check if subqueries work in the conditions?
MERGE INTO wq_target t
USING wq_source s ON t.tid = s.sid
WHEN MATCHED AND t.balance > (SELECT max(balance) FROM target) THEN
	UPDATE SET balance = t.balance + s.balance;

-- check if we can access system columns in the conditions
MERGE INTO wq_target t
USING wq_source s ON t.tid = s.sid
WHEN MATCHED AND t.xmin = t.xmax THEN
	UPDATE SET balance = t.balance + s.balance;

MERGE INTO wq_target t
USING wq_source s ON t.tid = s.sid
WHEN MATCHED AND t.tableoid >= 0 THEN
	UPDATE SET balance = t.balance + s.balance;
SELECT * FROM wq_target;

DROP TABLE wq_target, wq_source;

-- test triggers
create or replace function merge_trigfunc () returns trigger
language plpgsql as
$$
DECLARE
	line text;
BEGIN
	SELECT INTO line format('%s %s %s trigger%s',
		TG_WHEN, TG_OP, TG_LEVEL, CASE
		WHEN TG_OP = 'INSERT' AND TG_LEVEL = 'ROW'
			THEN format(' row: %s', NEW)
		WHEN TG_OP = 'UPDATE' AND TG_LEVEL = 'ROW'
			THEN format(' row: %s -> %s', OLD, NEW)
		WHEN TG_OP = 'DELETE' AND TG_LEVEL = 'ROW'
			THEN format(' row: %s', OLD)
		END);

	RAISE NOTICE '%', line;
	IF (TG_WHEN = 'BEFORE' AND TG_LEVEL = 'ROW') THEN
		IF (TG_OP = 'DELETE') THEN
			RETURN OLD;
		ELSE
			RETURN NEW;
		END IF;
	ELSE
		RETURN NULL;
	END IF;
END;
$$;
CREATE TRIGGER merge_bsi BEFORE INSERT ON target FOR EACH STATEMENT EXECUTE PROCEDURE merge_trigfunc ();
CREATE TRIGGER merge_bsu BEFORE UPDATE ON target FOR EACH STATEMENT EXECUTE PROCEDURE merge_trigfunc ();
CREATE TRIGGER merge_bsd BEFORE DELETE ON target FOR EACH STATEMENT EXECUTE PROCEDURE merge_trigfunc ();
CREATE TRIGGER merge_asi AFTER INSERT ON target FOR EACH STATEMENT EXECUTE PROCEDURE merge_trigfunc ();
CREATE TRIGGER merge_asu AFTER UPDATE ON target FOR EACH STATEMENT EXECUTE PROCEDURE merge_trigfunc ();
CREATE TRIGGER merge_asd AFTER DELETE ON target FOR EACH STATEMENT EXECUTE PROCEDURE merge_trigfunc ();
CREATE TRIGGER merge_bri BEFORE INSERT ON target FOR EACH ROW EXECUTE PROCEDURE merge_trigfunc ();
CREATE TRIGGER merge_bru BEFORE UPDATE ON target FOR EACH ROW EXECUTE PROCEDURE merge_trigfunc ();
CREATE TRIGGER merge_brd BEFORE DELETE ON target FOR EACH ROW EXECUTE PROCEDURE merge_trigfunc ();
CREATE TRIGGER merge_ari AFTER INSERT ON target FOR EACH ROW EXECUTE PROCEDURE merge_trigfunc ();
CREATE TRIGGER merge_aru AFTER UPDATE ON target FOR EACH ROW EXECUTE PROCEDURE merge_trigfunc ();
CREATE TRIGGER merge_ard AFTER DELETE ON target FOR EACH ROW EXECUTE PROCEDURE merge_trigfunc ();

-- now the classic UPSERT, with a DELETE
BEGIN;
UPDATE target SET balance = 0 WHERE tid = 3;
--EXPLAIN (ANALYZE ON, COSTS OFF, SUMMARY OFF, TIMING OFF)
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED AND t.balance > s.delta THEN
	UPDATE SET balance = t.balance - s.delta
WHEN MATCHED THEN
	DELETE
WHEN NOT MATCHED THEN
	INSERT VALUES (s.sid, s.delta);
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- UPSERT with UPDATE/DELETE when not matched by source
BEGIN;
DELETE FROM SOURCE WHERE sid = 2;
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED AND t.balance > s.delta THEN
    UPDATE SET balance = t.balance - s.delta
WHEN MATCHED THEN
	UPDATE SET balance = 0
WHEN NOT MATCHED THEN
    INSERT VALUES (s.sid, s.delta)
WHEN NOT MATCHED BY SOURCE AND tid = 1 THEN
	UPDATE SET balance = 0
WHEN NOT MATCHED BY SOURCE THEN
	DELETE
RETURNING merge_action(), old, new, t.*;
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- Test behavior of triggers that turn UPDATE/DELETE into no-ops
create or replace function skip_merge_op() returns trigger
language plpgsql as
$$
BEGIN
	RETURN NULL;
END;
$$;

SELECT * FROM target full outer join source on (sid = tid);
create trigger merge_skip BEFORE INSERT OR UPDATE or DELETE
  ON target FOR EACH ROW EXECUTE FUNCTION skip_merge_op();
DO $$
DECLARE
  result integer;
BEGIN
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED AND s.sid = 3 THEN UPDATE SET balance = t.balance + s.delta
WHEN MATCHED THEN DELETE
WHEN NOT MATCHED THEN INSERT VALUES (sid, delta);
IF FOUND THEN
  RAISE NOTICE 'Found';
ELSE
  RAISE NOTICE 'Not found';
END IF;
GET DIAGNOSTICS result := ROW_COUNT;
RAISE NOTICE 'ROW_COUNT = %', result;
END;
$$;
SELECT * FROM target FULL OUTER JOIN source ON (sid = tid);
DROP TRIGGER merge_skip ON target;
DROP FUNCTION skip_merge_op();

-- test from PL/pgSQL
-- make sure MERGE INTO isn't interpreted to mean returning variables like SELECT INTO
BEGIN;
DO LANGUAGE plpgsql $$
BEGIN
MERGE INTO target t
USING source AS s
ON t.tid = s.sid
WHEN MATCHED AND t.balance > s.delta THEN
	UPDATE SET balance = t.balance - s.delta;
END;
$$;
ROLLBACK;

--source constants
BEGIN;
MERGE INTO target t
USING (SELECT 9 AS sid, 57 AS delta) AS s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
	INSERT (tid, balance) VALUES (s.sid, s.delta);
SELECT * FROM target ORDER BY tid;
ROLLBACK;

--source query
BEGIN;
MERGE INTO target t
USING (SELECT sid, delta FROM source WHERE delta > 0) AS s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
	INSERT (tid, balance) VALUES (s.sid, s.delta);
SELECT * FROM target ORDER BY tid;
ROLLBACK;

BEGIN;
MERGE INTO target t
USING (SELECT sid, delta as newname FROM source WHERE delta > 0) AS s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
	INSERT (tid, balance) VALUES (s.sid, s.newname);
SELECT * FROM target ORDER BY tid;
ROLLBACK;

--self-merge
BEGIN;
MERGE INTO target t1
USING target t2
ON t1.tid = t2.tid
WHEN MATCHED THEN
	UPDATE SET balance = t1.balance + t2.balance
WHEN NOT MATCHED THEN
	INSERT VALUES (t2.tid, t2.balance);
SELECT * FROM target ORDER BY tid;
ROLLBACK;

BEGIN;
MERGE INTO target t
USING (SELECT tid as sid, balance as delta FROM target WHERE balance > 0) AS s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
	INSERT (tid, balance) VALUES (s.sid, s.delta);
SELECT * FROM target ORDER BY tid;
ROLLBACK;

BEGIN;
MERGE INTO target t
USING
(SELECT sid, max(delta) AS delta
 FROM source
 GROUP BY sid
 HAVING count(*) = 1
 ORDER BY sid ASC) AS s
ON t.tid = s.sid
WHEN NOT MATCHED THEN
	INSERT (tid, balance) VALUES (s.sid, s.delta);
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- plpgsql parameters and results
BEGIN;
CREATE FUNCTION merge_func (p_id integer, p_bal integer)
RETURNS INTEGER
LANGUAGE plpgsql
AS $$
DECLARE
 result integer;
BEGIN
MERGE INTO target t
USING (SELECT p_id AS sid) AS s
ON t.tid = s.sid
WHEN MATCHED THEN
	UPDATE SET balance = t.balance - p_bal;
IF FOUND THEN
	GET DIAGNOSTICS result := ROW_COUNT;
END IF;
RETURN result;
END;
$$;
SELECT merge_func(3, 4);
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- PREPARE
BEGIN;
prepare foom as merge into target t using (select 1 as sid) s on (t.tid = s.sid) when matched then update set balance = 1;
execute foom;
SELECT * FROM target ORDER BY tid;
ROLLBACK;

BEGIN;
PREPARE foom2 (integer, integer) AS
MERGE INTO target t
USING (SELECT 1) s
ON t.tid = $1
WHEN MATCHED THEN
UPDATE SET balance = $2;
--EXPLAIN (ANALYZE ON, COSTS OFF, SUMMARY OFF, TIMING OFF)
execute foom2 (1, 1);
SELECT * FROM target ORDER BY tid;
ROLLBACK;

-- subqueries in source relation

CREATE TABLE sq_target (tid integer NOT NULL, balance integer)
  WITH (autovacuum_enabled=off);
CREATE TABLE sq_source (delta integer, sid integer, balance integer DEFAULT 0)
  WITH (autovacuum_enabled=off);

INSERT INTO sq_target(tid, balance) VALUES (1,100), (2,200), (3,300);
INSERT INTO sq_source(sid, delta) VALUES (1,10), (2,20), (4,40);

BEGIN;
MERGE INTO sq_target t
USING (SELECT * FROM sq_source) s
ON tid = sid
WHEN MATCHED AND t.balance > delta THEN
	UPDATE SET balance = t.balance + delta;
SELECT * FROM sq_target;
ROLLBACK;

-- try a view
CREATE VIEW v AS SELECT * FROM sq_source WHERE sid < 2;

BEGIN;
MERGE INTO sq_target
USING v
ON tid = sid
WHEN MATCHED THEN
    UPDATE SET balance = v.balance + delta;
SELECT * FROM sq_target;
ROLLBACK;

-- ambiguous reference to a column
BEGIN;
MERGE INTO sq_target
USING v
ON tid = sid
WHEN MATCHED AND tid >= 2 THEN
    UPDATE SET balance = balance + delta
WHEN NOT MATCHED THEN
	INSERT (balance, tid) VALUES (balance + delta, sid)
WHEN MATCHED AND tid < 2 THEN
	DELETE;
ROLLBACK;

BEGIN;
INSERT INTO sq_source (sid, balance, delta) VALUES (-1, -1, -10);
MERGE INTO sq_target t
USING v
ON tid = sid
WHEN MATCHED AND tid >= 2 THEN
    UPDATE SET balance = t.balance + delta
WHEN NOT MATCHED THEN
	INSERT (balance, tid) VALUES (balance + delta, sid)
WHEN MATCHED AND tid < 2 THEN
	DELETE;
SELECT * FROM sq_target;
ROLLBACK;

-- CTEs
BEGIN;
INSERT INTO sq_source (sid, balance, delta) VALUES (-1, -1, -10);
WITH targq AS (
	SELECT * FROM v
)
MERGE INTO sq_target t
USING v
ON tid = sid
WHEN MATCHED AND tid >= 2 THEN
    UPDATE SET balance = t.balance + delta
WHEN NOT MATCHED THEN
	INSERT (balance, tid) VALUES (balance + delta, sid)
WHEN MATCHED AND tid < 2 THEN
	DELETE;
ROLLBACK;

-- RETURNING
SELECT * FROM sq_source ORDER BY sid;
SELECT * FROM sq_target ORDER BY tid;

BEGIN;
CREATE TABLE merge_actions(action text, abbrev text);
INSERT INTO merge_actions VALUES ('INSERT', 'ins'), ('UPDATE', 'upd'), ('DELETE', 'del');
MERGE INTO sq_target t
USING sq_source s
ON tid = sid
WHEN MATCHED AND tid >= 2 THEN
    UPDATE SET balance = t.balance + delta
WHEN NOT MATCHED THEN
    INSERT (balance, tid) VALUES (balance + delta, sid)
WHEN MATCHED AND tid < 2 THEN
    DELETE
RETURNING (SELECT abbrev FROM merge_actions
            WHERE action = merge_action()) AS action,
          old.tid AS old_tid, old.balance AS old_balance,
          new.tid AS new_tid, new.balance AS new_balance,
          (SELECT new.balance - old.balance AS delta_balance), t.*,
          CASE merge_action()
              WHEN 'INSERT' THEN 'Inserted '||t
              WHEN 'UPDATE' THEN 'Added '||delta||' to balance'
              WHEN 'DELETE' THEN 'Removed '||t
          END AS description;
ROLLBACK;

-- error when using merge_action() outside MERGE
SELECT merge_action() FROM sq_target;
UPDATE sq_target SET balance = balance + 1 RETURNING merge_action();

-- RETURNING in CTEs
CREATE TABLE sq_target_merge_log (tid integer NOT NULL, last_change text);
INSERT INTO sq_target_merge_log VALUES (1, 'Original value');
BEGIN;
WITH m AS (
    MERGE INTO sq_target t
    USING sq_source s
    ON tid = sid
    WHEN MATCHED AND tid >= 2 THEN
        UPDATE SET balance = t.balance + delta
    WHEN NOT MATCHED THEN
        INSERT (balance, tid) VALUES (balance + delta, sid)
    WHEN MATCHED AND tid < 2 THEN
        DELETE
    RETURNING merge_action() AS action, old AS old_data, new AS new_data, t.*,
              CASE merge_action()
                  WHEN 'INSERT' THEN 'Inserted '||t
                  WHEN 'UPDATE' THEN 'Added '||delta||' to balance'
                  WHEN 'DELETE' THEN 'Removed '||t
              END AS description
), m2 AS (
    MERGE INTO sq_target_merge_log l
    USING m
    ON l.tid = m.tid
    WHEN MATCHED THEN
        UPDATE SET last_change = description
    WHEN NOT MATCHED THEN
        INSERT VALUES (m.tid, description)
    RETURNING m.*, merge_action() AS log_action, old AS old_log, new AS new_log, l.*
)
SELECT * FROM m2;
SELECT * FROM sq_target_merge_log ORDER BY tid;
ROLLBACK;

-- COPY (MERGE ... RETURNING) TO ...
BEGIN;
COPY (
    MERGE INTO sq_target t
    USING sq_source s
    ON tid = sid
    WHEN MATCHED AND tid >= 2 THEN
        UPDATE SET balance = t.balance + delta
    WHEN NOT MATCHED THEN
        INSERT (balance, tid) VALUES (balance + delta, sid)
    WHEN MATCHED AND tid < 2 THEN
        DELETE
    RETURNING merge_action(), old.*, new.*
) TO stdout;
ROLLBACK;

-- SQL function with MERGE ... RETURNING
BEGIN;
CREATE FUNCTION merge_into_sq_target(sid int, balance int, delta int,
                                     OUT action text, OUT tid int, OUT new_balance int)
LANGUAGE sql AS
$$
    MERGE INTO sq_target t
    USING (VALUES ($1, $2, $3)) AS v(sid, balance, delta)
    ON tid = v.sid
    WHEN MATCHED AND tid >= 2 THEN
        UPDATE SET balance = t.balance + v.delta
    WHEN NOT MATCHED THEN
        INSERT (balance, tid) VALUES (v.balance + v.delta, v.sid)
    WHEN MATCHED AND tid < 2 THEN
        DELETE
    RETURNING merge_action(), t.*;
$$;
SELECT m.*
FROM (VALUES (1, 0, 0), (3, 0, 20), (4, 100, 10)) AS v(sid, balance, delta),
LATERAL (SELECT action, tid, new_balance FROM merge_into_sq_target(sid, balance, delta)) m;
ROLLBACK;

-- SQL SRF with MERGE ... RETURNING
BEGIN;
CREATE FUNCTION merge_sq_source_into_sq_target()
RETURNS TABLE (action text, tid int, balance int)
LANGUAGE sql AS
$$
    MERGE INTO sq_target t
    USING sq_source s
    ON tid = sid
    WHEN MATCHED AND tid >= 2 THEN
        UPDATE SET balance = t.balance + delta
    WHEN NOT MATCHED THEN
        INSERT (balance, tid) VALUES (balance + delta, sid)
    WHEN MATCHED AND tid < 2 THEN
        DELETE
    RETURNING merge_action(), t.*;
$$;
SELECT * FROM merge_sq_source_into_sq_target();
ROLLBACK;

-- PL/pgSQL function with MERGE ... RETURNING ... INTO
BEGIN;
CREATE FUNCTION merge_into_sq_target(sid int, balance int, delta int,
                                     OUT r_action text, OUT r_tid int, OUT r_balance int)
LANGUAGE plpgsql AS
$$
BEGIN
    MERGE INTO sq_target t
    USING (VALUES ($1, $2, $3)) AS v(sid, balance, delta)
    ON tid = v.sid
    WHEN MATCHED AND tid >= 2 THEN
        UPDATE SET balance = t.balance + v.delta
    WHEN NOT MATCHED THEN
        INSERT (balance, tid) VALUES (v.balance + v.delta, v.sid)
    WHEN MATCHED AND tid < 2 THEN
        DELETE
    RETURNING merge_action(), t.* INTO r_action, r_tid, r_balance;
END;
$$;
SELECT m.*
FROM (VALUES (1, 0, 0), (3, 0, 20), (4, 100, 10)) AS v(sid, balance, delta),
LATERAL (SELECT r_action, r_tid, r_balance FROM merge_into_sq_target(sid, balance, delta)) m;
ROLLBACK;

-- EXPLAIN
CREATE TABLE ex_mtarget (a int, b int)
  WITH (autovacuum_enabled=off);
CREATE TABLE ex_msource (a int, b int)
  WITH (autovacuum_enabled=off);
INSERT INTO ex_mtarget SELECT i, i*10 FROM generate_series(1,100,2) i;
INSERT INTO ex_msource SELECT i, i*10 FROM generate_series(1,100,1) i;

CREATE FUNCTION explain_merge(query text) RETURNS SETOF text
LANGUAGE plpgsql AS
$$
DECLARE ln text;
BEGIN
    FOR ln IN
        EXECUTE 'explain (analyze, timing off, summary off, costs off, buffers off) ' ||
		  query
    LOOP
        ln := regexp_replace(ln, '(Memory( Usage)?|Buckets|Batches): \S*',  '\1: xxx', 'g');
        RETURN NEXT ln;
    END LOOP;
END;
$$;

-- only updates
SELECT explain_merge('
MERGE INTO ex_mtarget t USING ex_msource s ON t.a = s.a
WHEN MATCHED THEN
	UPDATE SET b = t.b + 1');

-- only updates to selected tuples
SELECT explain_merge('
MERGE INTO ex_mtarget t USING ex_msource s ON t.a = s.a
WHEN MATCHED AND t.a < 10 THEN
	UPDATE SET b = t.b + 1');

-- updates + deletes
SELECT explain_merge('
MERGE INTO ex_mtarget t USING ex_msource s ON t.a = s.a
WHEN MATCHED AND t.a < 10 THEN
	UPDATE SET b = t.b + 1
WHEN MATCHED AND t.a >= 10 AND t.a <= 20 THEN
	DELETE');

-- only inserts
SELECT explain_merge('
MERGE INTO ex_mtarget t USING ex_msource s ON t.a = s.a
WHEN NOT MATCHED AND s.a < 10 THEN
	INSERT VALUES (a, b)');

-- all three
SELECT explain_merge('
MERGE INTO ex_mtarget t USING ex_msource s ON t.a = s.a
WHEN MATCHED AND t.a < 10 THEN
	UPDATE SET b = t.b + 1
WHEN MATCHED AND t.a >= 30 AND t.a <= 40 THEN
	DELETE
WHEN NOT MATCHED AND s.a < 20 THEN
	INSERT VALUES (a, b)');

-- not matched by source
SELECT explain_merge('
MERGE INTO ex_mtarget t USING ex_msource s ON t.a = s.a
WHEN NOT MATCHED BY SOURCE and t.a < 10 THEN
	DELETE');

-- not matched by source and target
SELECT explain_merge('
MERGE INTO ex_mtarget t USING ex_msource s ON t.a = s.a
WHEN NOT MATCHED BY SOURCE AND t.a < 10 THEN
	DELETE
WHEN NOT MATCHED BY TARGET AND s.a < 20 THEN
	INSERT VALUES (a, b)');

-- nothing
SELECT explain_merge('
MERGE INTO ex_mtarget t USING ex_msource s ON t.a = s.a AND t.a < -1000
WHEN MATCHED AND t.a < 10 THEN
	DO NOTHING');

DROP TABLE ex_msource, ex_mtarget;
DROP FUNCTION explain_merge(text);

-- EXPLAIN SubPlans and InitPlans
CREATE TABLE src (a int, b int, c int, d int);
CREATE TABLE tgt (a int, b int, c int, d int);
CREATE TABLE ref (ab int, cd int);

EXPLAIN (verbose, costs off)
MERGE INTO tgt t
USING (SELECT *, (SELECT count(*) FROM ref r
                   WHERE r.ab = s.a + s.b
                     AND r.cd = s.c - s.d) cnt
         FROM src s) s
ON t.a = s.a AND t.b < s.cnt
WHEN MATCHED AND t.c > s.cnt THEN
  UPDATE SET (b, c) = (SELECT s.b, s.cnt);

DROP TABLE src, tgt, ref;

-- Subqueries
BEGIN;
MERGE INTO sq_target t
USING v
ON tid = sid
WHEN MATCHED THEN
    UPDATE SET balance = (SELECT count(*) FROM sq_target);
SELECT * FROM sq_target WHERE tid = 1;
ROLLBACK;

BEGIN;
MERGE INTO sq_target t
USING v
ON tid = sid
WHEN MATCHED AND (SELECT count(*) > 0 FROM sq_target) THEN
    UPDATE SET balance = 42;
SELECT * FROM sq_target WHERE tid = 1;
ROLLBACK;

BEGIN;
MERGE INTO sq_target t
USING v
ON tid = sid AND (SELECT count(*) > 0 FROM sq_target)
WHEN MATCHED THEN
    UPDATE SET balance = 42;
SELECT * FROM sq_target WHERE tid = 1;
ROLLBACK;

DROP TABLE sq_target, sq_target_merge_log, sq_source CASCADE;

CREATE TABLE pa_target (tid integer, balance float, val text)
	PARTITION BY LIST (tid);

CREATE TABLE part1 PARTITION OF pa_target FOR VALUES IN (1,4)
  WITH (autovacuum_enabled=off);
CREATE TABLE part2 PARTITION OF pa_target FOR VALUES IN (2,5,6)
  WITH (autovacuum_enabled=off);
CREATE TABLE part3 PARTITION OF pa_target FOR VALUES IN (3,8,9)
  WITH (autovacuum_enabled=off);
CREATE TABLE part4 PARTITION OF pa_target DEFAULT
  WITH (autovacuum_enabled=off);

CREATE TABLE pa_source (sid integer, delta float);
-- insert many rows to the source table
INSERT INTO pa_source SELECT id, id * 10  FROM generate_series(1,14) AS id;
-- insert a few rows in the target table (odd numbered tid)
INSERT INTO pa_target SELECT id, id * 100, 'initial' FROM generate_series(1,15,2) AS id;

-- try simple MERGE
BEGIN;
MERGE INTO pa_target t
  USING pa_source s
  ON t.tid = s.sid
  WHEN MATCHED THEN
    UPDATE SET balance = balance + delta, val = val || ' updated by merge'
  WHEN NOT MATCHED THEN
    INSERT VALUES (sid, delta, 'inserted by merge')
  WHEN NOT MATCHED BY SOURCE THEN
    UPDATE SET val = val || ' not matched by source';
SELECT * FROM pa_target ORDER BY tid, val;
ROLLBACK;

-- same with a constant qual
BEGIN;
MERGE INTO pa_target t
  USING pa_source s
  ON t.tid = s.sid AND tid = 1
  WHEN MATCHED THEN
    UPDATE SET balance = balance + delta, val = val || ' updated by merge'
  WHEN NOT MATCHED THEN
    INSERT VALUES (sid, delta, 'inserted by merge')
  WHEN NOT MATCHED BY SOURCE THEN
    UPDATE SET val = val || ' not matched by source';
SELECT * FROM pa_target ORDER BY tid, val;
ROLLBACK;

-- try updating the partition key column
BEGIN;
CREATE FUNCTION merge_func() RETURNS integer LANGUAGE plpgsql AS $$
DECLARE
  result integer;
BEGIN
MERGE INTO pa_target t
  USING pa_source s
  ON t.tid = s.sid
  WHEN MATCHED THEN
    UPDATE SET tid = tid + 1, balance = balance + delta, val = val || ' updated by merge'
  WHEN NOT MATCHED THEN
    INSERT VALUES (sid, delta, 'inserted by merge')
  WHEN NOT MATCHED BY SOURCE THEN
    UPDATE SET tid = 1, val = val || ' not matched by source';
IF FOUND THEN
  GET DIAGNOSTICS result := ROW_COUNT;
END IF;
RETURN result;
END;
$$;
SELECT merge_func();
SELECT * FROM pa_target ORDER BY tid, val;
ROLLBACK;

-- update partition key to partition not initially scanned
BEGIN;
MERGE INTO pa_target t
  USING pa_source s
  ON t.tid = s.sid AND t.tid = 1
  WHEN MATCHED THEN
    UPDATE SET tid = tid + 1, balance = balance + delta, val = val || ' updated by merge'
  RETURNING merge_action(), old, new, t.*;
SELECT * FROM pa_target ORDER BY tid;
ROLLBACK;

-- bug #18871: ExecInitPartitionInfo()'s handling of DO NOTHING actions
BEGIN;
TRUNCATE pa_target;
MERGE INTO pa_target t
  USING (VALUES (10, 100)) AS s(sid, delta)
  ON t.tid = s.sid
  WHEN NOT MATCHED THEN
    INSERT VALUES (1, 10, 'inserted by merge')
  WHEN MATCHED THEN
    DO NOTHING;
SELECT * FROM pa_target ORDER BY tid, val;
ROLLBACK;

DROP TABLE pa_target CASCADE;

-- The target table is partitioned in the same way, but this time by attaching
-- partitions which have columns in different order, dropped columns etc.
CREATE TABLE pa_target (tid integer, balance float, val text)
	PARTITION BY LIST (tid);

CREATE TABLE part1 (tid integer, balance float, val text)
  WITH (autovacuum_enabled=off);
CREATE TABLE part2 (balance float, tid integer, val text)
  WITH (autovacuum_enabled=off);
CREATE TABLE part3 (tid integer, balance float, val text)
  WITH (autovacuum_enabled=off);
CREATE TABLE part4 (extraid text, tid integer, balance float, val text)
  WITH (autovacuum_enabled=off);
ALTER TABLE part4 DROP COLUMN extraid;

ALTER TABLE pa_target ATTACH PARTITION part1 FOR VALUES IN (1,4);
ALTER TABLE pa_target ATTACH PARTITION part2 FOR VALUES IN (2,5,6);
ALTER TABLE pa_target ATTACH PARTITION part3 FOR VALUES IN (3,8,9);
ALTER TABLE pa_target ATTACH PARTITION part4 DEFAULT;

-- insert a few rows in the target table (odd numbered tid)
INSERT INTO pa_target SELECT id, id * 100, 'initial' FROM generate_series(1,15,2) AS id;

-- try simple MERGE
BEGIN;
DO $$
DECLARE
  result integer;
BEGIN
MERGE INTO pa_target t
  USING pa_source s
  ON t.tid = s.sid
  WHEN MATCHED THEN
    UPDATE SET balance = balance + delta, val = val || ' updated by merge'
  WHEN NOT MATCHED THEN
    INSERT VALUES (sid, delta, 'inserted by merge')
  WHEN NOT MATCHED BY SOURCE THEN
    UPDATE SET val = val || ' not matched by source';
GET DIAGNOSTICS result := ROW_COUNT;
RAISE NOTICE 'ROW_COUNT = %', result;
END;
$$;
SELECT * FROM pa_target ORDER BY tid, val;
ROLLBACK;

-- same with a constant qual
BEGIN;
MERGE INTO pa_target t
  USING pa_source s
  ON t.tid = s.sid AND tid IN (1, 5)
  WHEN MATCHED AND tid % 5 = 0 THEN DELETE
  WHEN MATCHED THEN
    UPDATE SET balance = balance + delta, val = val || ' updated by merge'
  WHEN NOT MATCHED THEN
    INSERT VALUES (sid, delta, 'inserted by merge')
  WHEN NOT MATCHED BY SOURCE THEN
    UPDATE SET val = val || ' not matched by source';
SELECT * FROM pa_target ORDER BY tid, val;
ROLLBACK;

-- try updating the partition key column
BEGIN;
DO $$
DECLARE
  result integer;
BEGIN
MERGE INTO pa_target t
  USING pa_source s
  ON t.tid = s.sid
  WHEN MATCHED THEN
    UPDATE SET tid = tid + 1, balance = balance + delta, val = val || ' updated by merge'
  WHEN NOT MATCHED THEN
    INSERT VALUES (sid, delta, 'inserted by merge')
  WHEN NOT MATCHED BY SOURCE THEN
    UPDATE SET tid = 1, val = val || ' not matched by source';
GET DIAGNOSTICS result := ROW_COUNT;
RAISE NOTICE 'ROW_COUNT = %', result;
END;
$$;
SELECT * FROM pa_target ORDER BY tid, val;
ROLLBACK;

-- as above, but blocked by BEFORE DELETE ROW trigger
BEGIN;
CREATE FUNCTION trig_fn() RETURNS trigger LANGUAGE plpgsql AS
  $$ BEGIN RETURN NULL; END; $$;
CREATE TRIGGER del_trig BEFORE DELETE ON pa_target
  FOR EACH ROW EXECUTE PROCEDURE trig_fn();
DO $$
DECLARE
  result integer;
BEGIN
MERGE INTO pa_target t
  USING pa_source s
  ON t.tid = s.sid
  WHEN MATCHED THEN
    UPDATE SET tid = tid + 1, balance = balance + delta, val = val || ' updated by merge'
  WHEN NOT MATCHED THEN
    INSERT VALUES (sid, delta, 'inserted by merge')
  WHEN NOT MATCHED BY SOURCE THEN
    UPDATE SET val = val || ' not matched by source';
GET DIAGNOSTICS result := ROW_COUNT;
RAISE NOTICE 'ROW_COUNT = %', result;
END;
$$;
SELECT * FROM pa_target ORDER BY tid, val;
ROLLBACK;

-- as above, but blocked by BEFORE INSERT ROW trigger
BEGIN;
CREATE FUNCTION trig_fn() RETURNS trigger LANGUAGE plpgsql AS
  $$ BEGIN RETURN NULL; END; $$;
CREATE TRIGGER ins_trig BEFORE INSERT ON pa_target
  FOR EACH ROW EXECUTE PROCEDURE trig_fn();
DO $$
DECLARE
  result integer;
BEGIN
MERGE INTO pa_target t
  USING pa_source s
  ON t.tid = s.sid
  WHEN MATCHED THEN
    UPDATE SET tid = tid + 1, balance = balance + delta, val = val || ' updated by merge'
  WHEN NOT MATCHED THEN
    INSERT VALUES (sid, delta, 'inserted by merge')
  WHEN NOT MATCHED BY SOURCE THEN
    UPDATE SET val = val || ' not matched by source';
GET DIAGNOSTICS result := ROW_COUNT;
RAISE NOTICE 'ROW_COUNT = %', result;
END;
$$;
SELECT * FROM pa_target ORDER BY tid, val;
ROLLBACK;

-- test RLS enforcement
BEGIN;
ALTER TABLE pa_target ENABLE ROW LEVEL SECURITY;
ALTER TABLE pa_target FORCE ROW LEVEL SECURITY;
CREATE POLICY pa_target_pol ON pa_target USING (tid != 0);
MERGE INTO pa_target t
  USING pa_source s
  ON t.tid = s.sid AND t.tid IN (1,2,3,4)
  WHEN MATCHED THEN
    UPDATE SET tid = tid - 1;
ROLLBACK;

DROP TABLE pa_source;
DROP TABLE pa_target CASCADE;

-- Sub-partitioning
CREATE TABLE pa_target (logts timestamp, tid integer, balance float, val text)
	PARTITION BY RANGE (logts);

CREATE TABLE part_m01 PARTITION OF pa_target
	FOR VALUES FROM ('2017-01-01') TO ('2017-02-01')
	PARTITION BY LIST (tid);
CREATE TABLE part_m01_odd PARTITION OF part_m01
	FOR VALUES IN (1,3,5,7,9) WITH (autovacuum_enabled=off);
CREATE TABLE part_m01_even PARTITION OF part_m01
	FOR VALUES IN (2,4,6,8) WITH (autovacuum_enabled=off);
CREATE TABLE part_m02 PARTITION OF pa_target
	FOR VALUES FROM ('2017-02-01') TO ('2017-03-01')
	PARTITION BY LIST (tid);
CREATE TABLE part_m02_odd PARTITION OF part_m02
	FOR VALUES IN (1,3,5,7,9) WITH (autovacuum_enabled=off);
CREATE TABLE part_m02_even PARTITION OF part_m02
	FOR VALUES IN (2,4,6,8) WITH (autovacuum_enabled=off);

CREATE TABLE pa_source (sid integer, delta float)
  WITH (autovacuum_enabled=off);
-- insert many rows to the source table
INSERT INTO pa_source SELECT id, id * 10  FROM generate_series(1,14) AS id;
-- insert a few rows in the target table (odd numbered tid)
INSERT INTO pa_target SELECT '2017-01-31', id, id * 100, 'initial' FROM generate_series(1,9,3) AS id;
INSERT INTO pa_target SELECT '2017-02-28', id, id * 100, 'initial' FROM generate_series(2,9,3) AS id;

-- try simple MERGE
BEGIN;
MERGE INTO pa_target t
  USING (SELECT '2017-01-15' AS slogts, * FROM pa_source WHERE sid < 10) s
  ON t.tid = s.sid
  WHEN MATCHED THEN
    UPDATE SET balance = balance + delta, val = val || ' updated by merge'
  WHEN NOT MATCHED THEN
    INSERT VALUES (slogts::timestamp, sid, delta, 'inserted by merge')
  RETURNING merge_action(), old, new, t.*;
SELECT * FROM pa_target ORDER BY tid;
ROLLBACK;

DROP TABLE pa_source;
DROP TABLE pa_target CASCADE;

-- Partitioned table with primary key

CREATE TABLE pa_target (tid integer PRIMARY KEY) PARTITION BY LIST (tid);
CREATE TABLE pa_targetp PARTITION OF pa_target DEFAULT;
CREATE TABLE pa_source (sid integer);

INSERT INTO pa_source VALUES (1), (2);

EXPLAIN (VERBOSE, COSTS OFF)
MERGE INTO pa_target t USING pa_source s ON t.tid = s.sid
  WHEN NOT MATCHED THEN INSERT VALUES (s.sid);

MERGE INTO pa_target t USING pa_source s ON t.tid = s.sid
  WHEN NOT MATCHED THEN INSERT VALUES (s.sid);

TABLE pa_target;

-- Partition-less partitioned table
-- (the bug we are checking for appeared only if table had partitions before)

DROP TABLE pa_targetp;

EXPLAIN (VERBOSE, COSTS OFF)
MERGE INTO pa_target t USING pa_source s ON t.tid = s.sid
  WHEN NOT MATCHED THEN INSERT VALUES (s.sid);

MERGE INTO pa_target t USING pa_source s ON t.tid = s.sid
  WHEN NOT MATCHED THEN INSERT VALUES (s.sid);

DROP TABLE pa_source;
DROP TABLE pa_target CASCADE;

-- some complex joins on the source side

CREATE TABLE cj_target (tid integer, balance float, val text)
  WITH (autovacuum_enabled=off);
CREATE TABLE cj_source1 (sid1 integer, scat integer, delta integer)
  WITH (autovacuum_enabled=off);
CREATE TABLE cj_source2 (sid2 integer, sval text)
  WITH (autovacuum_enabled=off);
INSERT INTO cj_source1 VALUES (1, 10, 100);
INSERT INTO cj_source1 VALUES (1, 20, 200);
INSERT INTO cj_source1 VALUES (2, 20, 300);
INSERT INTO cj_source1 VALUES (3, 10, 400);
INSERT INTO cj_source2 VALUES (1, 'initial source2');
INSERT INTO cj_source2 VALUES (2, 'initial source2');
INSERT INTO cj_source2 VALUES (3, 'initial source2');

-- source relation is an unaliased join
MERGE INTO cj_target t
USING cj_source1 s1
	INNER JOIN cj_source2 s2 ON sid1 = sid2
ON t.tid = sid1
WHEN NOT MATCHED THEN
	INSERT VALUES (sid1, delta, sval);

-- try accessing columns from either side of the source join
MERGE INTO cj_target t
USING cj_source2 s2
	INNER JOIN cj_source1 s1 ON sid1 = sid2 AND scat = 20
ON t.tid = sid1
WHEN NOT MATCHED THEN
	INSERT VALUES (sid2, delta, sval)
WHEN MATCHED THEN
	DELETE;

-- some simple expressions in INSERT targetlist
MERGE INTO cj_target t
USING cj_source2 s2
	INNER JOIN cj_source1 s1 ON sid1 = sid2
ON t.tid = sid1
WHEN NOT MATCHED THEN
	INSERT VALUES (sid2, delta + scat, sval)
WHEN MATCHED THEN
	UPDATE SET val = val || ' updated by merge';

MERGE INTO cj_target t
USING cj_source2 s2
	INNER JOIN cj_source1 s1 ON sid1 = sid2 AND scat = 20
ON t.tid = sid1
WHEN MATCHED THEN
	UPDATE SET val = val || ' ' || delta::text;

SELECT * FROM cj_target;

-- try it with an outer join and PlaceHolderVar
MERGE INTO cj_target t
USING (SELECT *, 'join input'::text AS phv FROM cj_source1) fj
	FULL JOIN cj_source2 fj2 ON fj.scat = fj2.sid2 * 10
ON t.tid = fj.scat
WHEN NOT MATCHED THEN
	INSERT (tid, balance, val) VALUES (fj.scat, fj.delta, fj.phv);

SELECT * FROM cj_target;

ALTER TABLE cj_source1 RENAME COLUMN sid1 TO sid;
ALTER TABLE cj_source2 RENAME COLUMN sid2 TO sid;

TRUNCATE cj_target;

MERGE INTO cj_target t
USING cj_source1 s1
	INNER JOIN cj_source2 s2 ON s1.sid = s2.sid
ON t.tid = s1.sid
WHEN NOT MATCHED THEN
	INSERT VALUES (s2.sid, delta, sval);

DROP TABLE cj_source2, cj_source1, cj_target;

-- Function scans
CREATE TABLE fs_target (a int, b int, c text)
  WITH (autovacuum_enabled=off);
MERGE INTO fs_target t
USING generate_series(1,100,1) AS id
ON t.a = id
WHEN MATCHED THEN
	UPDATE SET b = b + id
WHEN NOT MATCHED THEN
	INSERT VALUES (id, -1);

MERGE INTO fs_target t
USING generate_series(1,100,2) AS id
ON t.a = id
WHEN MATCHED THEN
	UPDATE SET b = b + id, c = 'updated '|| id.*::text
WHEN NOT MATCHED THEN
	INSERT VALUES (id, -1, 'inserted ' || id.*::text);

SELECT count(*) FROM fs_target;
DROP TABLE fs_target;

-- SERIALIZABLE test
-- handled in isolation tests

-- Inheritance-based partitioning
CREATE TABLE measurement (
    city_id         int not null,
    logdate         date not null,
    peaktemp        int,
    unitsales       int
) WITH (autovacuum_enabled=off);
CREATE TABLE measurement_y2006m02 (
    CHECK ( logdate >= DATE '2006-02-01' AND logdate < DATE '2006-03-01' )
) INHERITS (measurement) WITH (autovacuum_enabled=off);
CREATE TABLE measurement_y2006m03 (
    CHECK ( logdate >= DATE '2006-03-01' AND logdate < DATE '2006-04-01' )
) INHERITS (measurement) WITH (autovacuum_enabled=off);
CREATE TABLE measurement_y2007m01 (
    filler          text,
    peaktemp        int,
    logdate         date not null,
    city_id         int not null,
    unitsales       int
    CHECK ( logdate >= DATE '2007-01-01' AND logdate < DATE '2007-02-01')
) WITH (autovacuum_enabled=off);
ALTER TABLE measurement_y2007m01 DROP COLUMN filler;
ALTER TABLE measurement_y2007m01 INHERIT measurement;
INSERT INTO measurement VALUES (0, '2005-07-21', 5, 15);

CREATE OR REPLACE FUNCTION measurement_insert_trigger()
RETURNS TRIGGER AS $$
BEGIN
    IF ( NEW.logdate >= DATE '2006-02-01' AND
         NEW.logdate < DATE '2006-03-01' ) THEN
        INSERT INTO measurement_y2006m02 VALUES (NEW.*);
    ELSIF ( NEW.logdate >= DATE '2006-03-01' AND
            NEW.logdate < DATE '2006-04-01' ) THEN
        INSERT INTO measurement_y2006m03 VALUES (NEW.*);
    ELSIF ( NEW.logdate >= DATE '2007-01-01' AND
            NEW.logdate < DATE '2007-02-01' ) THEN
        INSERT INTO measurement_y2007m01 (city_id, logdate, peaktemp, unitsales)
            VALUES (NEW.*);
    ELSE
        RAISE EXCEPTION 'Date out of range.  Fix the measurement_insert_trigger() function!';
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql ;
CREATE TRIGGER insert_measurement_trigger
    BEFORE INSERT ON measurement
    FOR EACH ROW EXECUTE PROCEDURE measurement_insert_trigger();
INSERT INTO measurement VALUES (1, '2006-02-10', 35, 10);
INSERT INTO measurement VALUES (1, '2006-02-16', 45, 20);
INSERT INTO measurement VALUES (1, '2006-03-17', 25, 10);
INSERT INTO measurement VALUES (1, '2006-03-27', 15, 40);
INSERT INTO measurement VALUES (1, '2007-01-15', 10, 10);
INSERT INTO measurement VALUES (1, '2007-01-17', 10, 10);

SELECT tableoid::regclass, * FROM measurement ORDER BY city_id, logdate;

CREATE TABLE new_measurement (LIKE measurement) WITH (autovacuum_enabled=off);
INSERT INTO new_measurement VALUES (0, '2005-07-21', 25, 20);
INSERT INTO new_measurement VALUES (1, '2006-03-01', 20, 10);
INSERT INTO new_measurement VALUES (1, '2006-02-16', 50, 10);
INSERT INTO new_measurement VALUES (2, '2006-02-10', 20, 20);
INSERT INTO new_measurement VALUES (1, '2006-03-27', NULL, NULL);
INSERT INTO new_measurement VALUES (1, '2007-01-17', NULL, NULL);
INSERT INTO new_measurement VALUES (1, '2007-01-15', 5, NULL);
INSERT INTO new_measurement VALUES (1, '2007-01-16', 10, 10);

BEGIN;
MERGE INTO ONLY measurement m
 USING new_measurement nm ON
      (m.city_id = nm.city_id and m.logdate=nm.logdate)
WHEN MATCHED AND nm.peaktemp IS NULL THEN DELETE
WHEN MATCHED THEN UPDATE
     SET peaktemp = greatest(m.peaktemp, nm.peaktemp),
        unitsales = m.unitsales + coalesce(nm.unitsales, 0)
WHEN NOT MATCHED THEN INSERT
     (city_id, logdate, peaktemp, unitsales)
   VALUES (city_id, logdate, peaktemp, unitsales);

SELECT tableoid::regclass, * FROM measurement ORDER BY city_id, logdate, peaktemp;
ROLLBACK;

MERGE into measurement m
 USING new_measurement nm ON
      (m.city_id = nm.city_id and m.logdate=nm.logdate)
WHEN MATCHED AND nm.peaktemp IS NULL THEN DELETE
WHEN MATCHED THEN UPDATE
     SET peaktemp = greatest(m.peaktemp, nm.peaktemp),
        unitsales = m.unitsales + coalesce(nm.unitsales, 0)
WHEN NOT MATCHED THEN INSERT
     (city_id, logdate, peaktemp, unitsales)
   VALUES (city_id, logdate, peaktemp, unitsales);

SELECT tableoid::regclass, * FROM measurement ORDER BY city_id, logdate;

BEGIN;
MERGE INTO new_measurement nm
 USING ONLY measurement m ON
      (nm.city_id = m.city_id and nm.logdate=m.logdate)
WHEN MATCHED THEN DELETE;

SELECT * FROM new_measurement ORDER BY city_id, logdate;
ROLLBACK;

MERGE INTO new_measurement nm
 USING measurement m ON
      (nm.city_id = m.city_id and nm.logdate=m.logdate)
WHEN MATCHED THEN DELETE;

SELECT * FROM new_measurement ORDER BY city_id, logdate;

DROP TABLE measurement, new_measurement CASCADE;
DROP FUNCTION measurement_insert_trigger();

--
-- test non-strict join clause
--
CREATE TABLE src (a int, b text);
INSERT INTO src VALUES (1, 'src row');

CREATE TABLE tgt (a int, b text);
INSERT INTO tgt VALUES (NULL, 'tgt row');

MERGE INTO tgt USING src ON tgt.a IS NOT DISTINCT FROM src.a
  WHEN MATCHED THEN UPDATE SET a = src.a, b = src.b
  WHEN NOT MATCHED BY SOURCE THEN DELETE
  RETURNING merge_action(), src.*, tgt.*;

SELECT * FROM tgt;

DROP TABLE src, tgt;

--
-- test for bug #18634 (wrong varnullingrels error)
--
CREATE TABLE bug18634t (a int, b int, c text);
INSERT INTO bug18634t VALUES(1, 10, 'tgt1'), (2, 20, 'tgt2');
CREATE VIEW bug18634v AS
  SELECT * FROM bug18634t WHERE EXISTS (SELECT 1 FROM bug18634t);

CREATE TABLE bug18634s (a int, b int, c text);
INSERT INTO bug18634s VALUES (1, 2, 'src1');

MERGE INTO bug18634v t USING bug18634s s ON s.a = t.a
  WHEN MATCHED THEN UPDATE SET b = s.b
  WHEN NOT MATCHED BY SOURCE THEN DELETE
  RETURNING merge_action(), s.c, t.*;

SELECT * FROM bug18634t;

DROP TABLE bug18634t CASCADE;
DROP TABLE bug18634s;

-- prepare

RESET SESSION AUTHORIZATION;

-- try a system catalog
MERGE INTO pg_class c
USING (SELECT 'pg_depend'::regclass AS oid) AS j
ON j.oid = c.oid
WHEN MATCHED THEN
	UPDATE SET reltuples = reltuples + 1
RETURNING j.oid;

CREATE VIEW classv AS SELECT * FROM pg_class;
MERGE INTO classv c
USING pg_namespace n
ON n.oid = c.relnamespace
WHEN MATCHED AND c.oid = 'pg_depend'::regclass THEN
	UPDATE SET reltuples = reltuples - 1
RETURNING c.oid;

DROP TABLE target, target2;
DROP TABLE source, source2;
DROP FUNCTION merge_trigfunc();
DROP USER regress_merge_privs;
DROP USER regress_merge_no_privs;
DROP USER regress_merge_none;
