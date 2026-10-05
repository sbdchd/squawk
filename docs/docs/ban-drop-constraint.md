---
id: ban-drop-constraint
title: ban-drop-constraint
---

## problem

Dropping a table, foreign table, or domain constraint, or setting a table constraint to `NOT ENFORCED`, removes a guarantee that clients can depend on. If an old application uses `INSERT ... ON CONFLICT ON CONSTRAINT c` or infers a dropped unique constraint as its conflict arbiter, its inserts fail immediately. This rule is enabled by default.

```sql
ALTER TABLE t DROP CONSTRAINT IF EXISTS c;
```

## solution

Update clients to not depend on the constraint before dropping it.

Exclude this rule with `--exclude ban-drop-constraint` after checking application compatibility.
