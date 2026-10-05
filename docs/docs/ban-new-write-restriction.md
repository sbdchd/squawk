---
id: ban-new-write-restriction
title: ban-new-write-restriction
---

## problem

New table, foreign-table, and domain constraints, `NOT VALID` constraints, `SET NOT NULL`, constrained added columns, changed constraint timing, and unique indexes (including `CONCURRENTLY`) can reject writes from existing clients. This rule is opt-in. An unconditional `CREATE TABLE` earlier in the file suppresses a warning for that table. The rule does not know whether an object existed before the migration.

```sql
ALTER TABLE t ADD CONSTRAINT c CHECK (id > 0) NOT VALID;
```

## solution

Update client writes before adding the restriction.

Enable this rule with `--include ban-new-write-restriction`.
