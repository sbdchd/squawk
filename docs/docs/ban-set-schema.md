---
id: ban-set-schema
title: ban-set-schema
---

## problem

Clients that use schema-qualified names cannot find objects after `SET SCHEMA`. This includes foreign tables, procedures, and routines.

```sql
ALTER TABLE t SET SCHEMA s;
```

## solution

Update clients to use the new schema before moving the object.
