---
id: ban-new-write-restriction
title: ban-new-write-restriction
---

## problem

New constraints, NOT VALID constraints, SET NOT NULL, and unique indexes (including CONCURRENTLY) can reject writes from existing clients. This rule is opt-in. An unconditional CREATE earlier in the file suppresses a warning for that new object.

```sql
ALTER TABLE t ADD CONSTRAINT c CHECK (id > 0) NOT VALID;
```

## solution

Update client writes before adding the restriction.

Enable this rule with `--include ban-new-write-restriction`.
