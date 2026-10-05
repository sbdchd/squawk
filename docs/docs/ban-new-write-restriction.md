---
id: ban-new-write-restriction
title: ban-new-write-restriction
---

## problem

New table, foreign-table, and domain constraints, `NOT VALID` constraints, `SET NOT NULL`, named `NOT NULL` constraints, constrained added columns (including inline primary keys), changed constraint timing, and unique indexes (including `CONCURRENTLY`) can reject writes from existing clients. This rule is opt-in. An unconditional `CREATE TABLE` earlier in the file suppresses a warning for that table. The rule does not know whether an object existed before the migration.

```sql
ALTER TABLE t ADD CONSTRAINT c CHECK (id > 0) NOT VALID;
ALTER TABLE t ADD CONSTRAINT id_required NOT NULL id;
ALTER TABLE t ADD COLUMN external_id bigint PRIMARY KEY;
ALTER FOREIGN TABLE ft ADD COLUMN external_id bigint NOT NULL;
```

## solution

Update client writes before adding the restriction.

Enable this rule with `--include ban-new-write-restriction`.
