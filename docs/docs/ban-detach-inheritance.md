---
id: ban-detach-inheritance
title: ban-detach-inheritance
---

## problem

`DETACH PARTITION` and `NO INHERIT` change which rows existing clients access through a parent table. This rule is opt-in. An unconditional CREATE earlier in the file suppresses a warning for that new object.

```sql
ALTER TABLE parent DETACH PARTITION child CONCURRENTLY;
```

## solution

Update clients that use the parent table before detaching the child.

Enable this rule with `--include ban-detach-inheritance`.
