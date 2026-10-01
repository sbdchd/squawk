---
id: ban-drop-schema
title: ban-drop-schema
---

## problem

Queries that reference objects in a dropped schema fail.

```sql
DROP SCHEMA IF EXISTS s CASCADE;
```

## solution

Move clients to a replacement schema before dropping the old schema.
